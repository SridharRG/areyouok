mod cli;
mod scanner;
mod link_checker;
mod report;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse_args();

    // Scan files and extract links
    let links = scanner::scan_directory(
        &args.directory,
        &args.file_types,
        &args.ignore_patterns,
    )?;

    println!("Found {} links to check...", links.len());

    // Check links
    let results = link_checker::check_links(&links).await?;

    // Generate report
    report::generate_report(&results, &args.report_format, &args.directory)?;

    Ok(())
}
