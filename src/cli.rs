use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The directory whose files should be processed
    #[arg(short = 'p', long)]
    pub directory: Option<PathBuf>,
    /// Include the extension in the file to edit
    #[arg(short = 'e', long, required = false)]
    pub with_extension: bool,
    /// Sleep specified seconds between file renaming
    #[arg(short = 'd', long)]
    pub sequential_delay: Option<f64>,
}
