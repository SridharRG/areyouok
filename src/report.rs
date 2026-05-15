use crate::link_checker::CheckResult;
use anyhow::Result;
use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_report(
    results: &[CheckResult],
    format: &str,
    base_dir: &Path,
) -> Result<()> {
    let broken_links: Vec<_> = results.iter().filter(|r| !r.is_valid).collect();
    let valid_links: Vec<_> = results.iter().filter(|r| r.is_valid).collect();

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("areyouok_report_{}.{}", timestamp, get_extension(format));

    let report_content = match format.to_lowercase().as_str() {
        "json" => generate_json_report(results),
        "html" => generate_html_report(results, base_dir),
        "github" => generate_github_report(results, valid_links.len(), broken_links.len()),
        "txt" => generate_txt_report(results, valid_links.len(), broken_links.len()),
        _ => return Err(anyhow::anyhow!("Unknown report format: {}", format)),
    };

    let mut file = File::create(&filename)?;
    file.write_all(report_content.as_bytes())?;

    println!("\n✓ Report generated: {}", filename);
    println!("  Valid links: {}", valid_links.len());
    println!("  Broken links: {}", broken_links.len());

    Ok(())
}

fn get_extension(format: &str) -> &str {
    match format.to_lowercase().as_str() {
        "json" => "json",
        "html" => "html",
        "github" => "md",
        _ => "txt",
    }
}

fn generate_txt_report(results: &[CheckResult], valid: usize, broken: usize) -> String {
    let mut report = String::new();
    report.push_str("╔════════════════════════════════════════════╗\n");
    report.push_str("║           AREYOUOK LINK REPORT             ║\n");
    report.push_str("╚════════════════════════════════════════════╝\n\n");

    report.push_str(&format!("Timestamp: {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
    report.push_str(&format!("Summary:\n"));
    report.push_str(&format!("  Total Links Checked: {}\n", results.len()));
    report.push_str(&format!("  ✓ Valid: {}\n", valid));
    report.push_str(&format!("  ✗ Broken: {}\n\n", broken));

    let broken: Vec<_> = results.iter().filter(|r| !r.is_valid).collect();

    if !broken.is_empty() {
        report.push_str("═══════════════════════════════════════════\n");
        report.push_str("BROKEN LINKS\n");
        report.push_str("═══════════════════════════════════════════\n\n");

        for result in broken {
            report.push_str(&format!("URL: {}\n", result.url));
            if let Some(msg) = &result.error_message {
                report.push_str(&format!("Error: {}\n", msg));
            }
            report.push_str("Found in:\n");
            for location in &result.files {
                report.push_str(&format!("  - {} (line {})\n", location.file, location.line));
            }
            report.push_str("\n");
        }
    }

    report
}

fn generate_json_report(results: &[CheckResult]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_default()
}

fn generate_html_report(results: &[CheckResult], _base_dir: &Path) -> String {
    let broken: Vec<_> = results.iter().filter(|r| !r.is_valid).collect();
    let valid_count = results.len() - broken.len();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AreYouOk Link Report</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1000px;
            margin: 0 auto;
            background: white;
            border-radius: 8px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.3);
            overflow: hidden;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px;
            text-align: center;
        }
        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
        }
        .header p {
            font-size: 1.1em;
            opacity: 0.9;
        }
        .content {
            padding: 40px;
        }
        .summary {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            margin-bottom: 40px;
        }
        .stat {
            padding: 20px;
            border-radius: 8px;
            text-align: center;
        }
        .stat.valid {
            background: #e8f5e9;
            border-left: 4px solid #4caf50;
        }
        .stat.broken {
            background: #ffebee;
            border-left: 4px solid #f44336;
        }
        .stat-value {
            font-size: 2.5em;
            font-weight: bold;
            margin: 10px 0;
        }
        .stat.valid .stat-value {
            color: #4caf50;
        }
        .stat.broken .stat-value {
            color: #f44336;
        }
        .broken-section {
            margin-top: 40px;
        }
        .broken-section h2 {
            color: #f44336;
            margin-bottom: 20px;
            border-bottom: 2px solid #f44336;
            padding-bottom: 10px;
        }
        .broken-link {
            background: #fff3cd;
            border-left: 4px solid #ff9800;
            padding: 15px;
            margin-bottom: 15px;
            border-radius: 4px;
        }
        .broken-link .url {
            font-family: 'Courier New', monospace;
            color: #d32f2f;
            word-break: break-all;
            margin-bottom: 10px;
        }
        .broken-link .error {
            color: #666;
            font-size: 0.95em;
            margin-bottom: 10px;
        }
        .broken-link .locations {
            background: white;
            padding: 10px;
            border-radius: 4px;
        }
        .broken-link .locations li {
            list-style-position: inside;
            padding: 5px 0;
            color: #555;
            font-family: 'Courier New', monospace;
            font-size: 0.9em;
        }
        .footer {
            background: #f5f5f5;
            padding: 20px;
            text-align: center;
            color: #999;
            font-size: 0.9em;
        }
        .no-broken {
            background: #e8f5e9;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
            color: #4caf50;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔍 AreYouOk</h1>
            <p>Link Validation Report</p>
        </div>
        <div class="content">
"#,
    );

    html.push_str(&format!(
        r#"            <div class="summary">
                <div class="stat valid">
                    <div>Valid Links</div>
                    <div class="stat-value">{}</div>
                </div>
                <div class="stat broken">
                    <div>Broken Links</div>
                    <div class="stat-value">{}</div>
                </div>
            </div>
"#,
        valid_count,
        broken.len()
    ));

    if broken.is_empty() {
        html.push_str(
            r#"            <div class="no-broken">
                ✓ All links are valid! No broken links found.
            </div>
"#,
        );
    } else {
        html.push_str(r#"            <div class="broken-section">
                <h2>Broken Links</h2>
"#);

        for result in &broken {
            html.push_str(&format!(
                r#"                <div class="broken-link">
                    <div class="url">{}</div>
"#,
                html_escape(&result.url)
            ));

            if let Some(err) = &result.error_message {
                html.push_str(&format!(
                    r#"                    <div class="error">Error: {}</div>
"#,
                    html_escape(err)
                ));
            }

            html.push_str(r#"                    <div class="locations">
                        <strong>Found in:</strong>
                        <ul>
"#);

            for location in &result.files {
                html.push_str(&format!(
                    r#"                            <li>{} (line {})</li>
"#,
                    html_escape(&location.file),
                    location.line
                ));
            }

            html.push_str(
                r#"                        </ul>
                    </div>
                </div>
"#,
            );
        }

        html.push_str(r#"            </div>
"#);
    }

    html.push_str(&format!(
        r#"        </div>
        <div class="footer">
            Generated on {} | AreYouOk v0.1.0
        </div>
    </div>
</body>
</html>
"#,
        timestamp
    ));

    html
}

fn generate_github_report(results: &[CheckResult], valid: usize, broken_count: usize) -> String {
    let broken: Vec<_> = results.iter().filter(|r| !r.is_valid).collect();

    let mut report = String::new();
    report.push_str("# 🔍 AreYouOk Link Report\n\n");
    report.push_str(&format!(
        "**Report generated:** {}\n\n",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    report.push_str("## Summary\n\n");
    report.push_str(&format!("| Metric | Count |\n"));
    report.push_str(&format!("| --- | --- |\n"));
    report.push_str(&format!("| Total Links | {} |\n", results.len()));
    report.push_str(&format!("| ✓ Valid | {} |\n", valid));
    report.push_str(&format!("| ✗ Broken | {} |\n\n", broken_count));

    if !broken.is_empty() {
        report.push_str("## Broken Links\n\n");

        for result in broken {
            report.push_str(&format!("### {}\n", result.url));

            if let Some(msg) = &result.error_message {
                report.push_str(&format!("**Error:** {}\n\n", msg));
            }

            report.push_str("**Found in:**\n");
            for location in &result.files {
                report.push_str(&format!("- `{}` (line {})\n", location.file, location.line));
            }
            report.push_str("\n");
        }
    } else {
        report.push_str("## Status\n\n✓ All links are valid! No broken links found.\n");
    }

    report
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
