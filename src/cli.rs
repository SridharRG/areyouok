use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "AreYouOk",
    about = "Scan files for broken links",
    long_about = "A tool to scan files for broken or dead links. \
                   Supports multiple file types and report formats."
)]
pub struct Args {
    /// Directory path to scan
    #[arg(value_name = "PATH")]
    pub directory: PathBuf,

    /// Type of files to scan (markdown, html, text, or any file extension)
    #[arg(
        short = 't',
        long = "type",
        default_value = "markdown",
        value_name = "TYPE"
    )]
    pub file_types: String,

    /// Files or directories to ignore (comma-separated)
    #[arg(
        short = 'i',
        long = "ignore",
        value_name = "PATTERNS",
        default_value = ""
    )]
    pub ignore_patterns: String,

    /// Report format (json, html, txt, github)
    #[arg(
        short = 'r',
        long = "report",
        default_value = "txt",
        value_name = "FORMAT"
    )]
    pub report_format: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
