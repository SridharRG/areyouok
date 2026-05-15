use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Link {
    pub url: String,
    pub file: PathBuf,
    pub line: usize,
}

pub fn scan_directory(
    directory: &Path,
    file_types: &str,
    ignore_patterns: &str,
) -> Result<Vec<Link>> {
    let mut links = Vec::new();
    let file_extensions = parse_file_types(file_types);
    let ignore_list = parse_ignore_patterns(ignore_patterns);

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip ignored patterns
        if should_ignore(path, &ignore_list, directory) {
            continue;
        }

        // Filter by file type
        if !should_process_file(path, &file_extensions) {
            continue;
        }

        // Extract links from file
        if let Ok(file_links) = extract_links(path) {
            links.extend(file_links);
        }
    }

    // Deduplicate links
    let mut seen = HashSet::new();
    links.retain(|link| seen.insert((link.url.clone(), link.file.clone())));

    Ok(links)
}

fn parse_file_types(types: &str) -> Vec<String> {
    match types.to_lowercase().as_str() {
        "markdown" => vec!["md".to_string()],
        "html" => vec!["html".to_string(), "htm".to_string()],
        "text" => vec!["txt".to_string()],
        custom => custom
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    }
}

fn parse_ignore_patterns(patterns: &str) -> Vec<String> {
    if patterns.is_empty() {
        return vec![];
    }
    patterns
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn should_ignore(path: &Path, ignore_list: &[String], base_dir: &Path) -> bool {
    let relative_path = path
        .strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy();

    for pattern in ignore_list {
        if relative_path.contains(pattern) {
            return true;
        }
    }

    false
}

fn should_process_file(path: &Path, extensions: &[String]) -> bool {
    if !path.is_file() {
        return false;
    }

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    extensions.iter().any(|ext| ext == &extension)
}

fn extract_links(path: &Path) -> Result<Vec<Link>> {
    let content = fs::read_to_string(path)?;
    let mut links = Vec::new();

    // Regex patterns for different link formats
    let patterns = vec![
        // Markdown links: [text](url)
        Regex::new(r"\[([^\]]+)\]\((https?://[^\s\)]+)\)")?,
        // HTML links: <a href="url">
        Regex::new(r#"href=["']?(https?://[^\s"')]+)"#)?,
        // Plain URLs
        Regex::new(r"https?://[^\s\)>\]'\x22]+")?,
    ];

    for (line_num, line) in content.lines().enumerate() {
        for pattern in &patterns {
            for cap in pattern.captures_iter(line) {
                let url = if let Some(m) = cap.get(2) {
                    m.as_str().to_string()
                } else if let Some(m) = cap.get(1) {
                    if m.as_str().starts_with("http") {
                        m.as_str().to_string()
                    } else {
                        continue;
                    }
                } else {
                    cap.get(0).unwrap().as_str().to_string()
                };

                // Validate URL format
                if let Ok(_) = Url::parse(&url) {
                    links.push(Link {
                        url,
                        file: path.to_path_buf(),
                        line: line_num + 1,
                    });
                }
            }
        }
    }

    Ok(links)
}
