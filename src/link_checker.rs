use crate::scanner::Link;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub url: String,
    pub status_code: Option<u16>,
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub files: Vec<LinkLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkLocation {
    pub file: String,
    pub line: usize,
}

pub async fn check_links(links: &[Link]) -> Result<Vec<CheckResult>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("AreYouOk/0.1.0")
        .build()?;

    // Group links by URL to avoid checking duplicates
    let mut url_map: HashMap<String, Vec<LinkLocation>> = HashMap::new();

    for link in links {
        let location = LinkLocation {
            file: link.file.to_string_lossy().to_string(),
            line: link.line,
        };

        url_map
            .entry(link.url.clone())
            .or_insert_with(Vec::new)
            .push(location);
    }

    let mut results = Vec::new();

    for (url, locations) in url_map {
        let result = check_single_link(&client, &url, locations).await;
        results.push(result);
    }

    Ok(results)
}

async fn check_single_link(
    client: &reqwest::Client,
    url: &str,
    files: Vec<LinkLocation>,
) -> CheckResult {
    match client.head(url).send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            let is_valid = response.status().is_success();

            CheckResult {
                url: url.to_string(),
                status_code: Some(status),
                is_valid,
                error_message: if !is_valid {
                    Some(format!("HTTP {}", status))
                } else {
                    None
                },
                files,
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            let is_network_error = error_msg.contains("timeout")
                || error_msg.contains("connection")
                || error_msg.contains("dns");

            CheckResult {
                url: url.to_string(),
                status_code: None,
                is_valid: false,
                error_message: Some(if is_network_error {
                    format!("Network error: {}", error_msg)
                } else {
                    error_msg
                }),
                files,
            }
        }
    }
}
