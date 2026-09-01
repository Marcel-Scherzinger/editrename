use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short = 'p', long)]
    pub directory: Option<PathBuf>,
    #[arg(long, required = false)]
    pub with_extension: bool,
    /// If set, rename the file in sorted order and sleep specified seconds between files
    #[arg(short = 't', long)]
    pub sequential_delay: Option<f64>,
}
