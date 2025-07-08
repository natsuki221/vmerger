use clap::Parser;
use std::process;

mod cli;
mod core;

use cli::Cli;
use core::VideoProcessor;

fn main() {
    let cli = Cli::parse();

    // Create video processor with verbose flag
    let processor = VideoProcessor::new(cli.verbose);

    // Process videos
    if let Err(e) = processor.merge_videos(&cli) {
        eprintln!("❌ Error: {e}");

        // Print the error chain for more context
        let mut source = e.source();
        while let Some(err) = source {
            eprintln!("   Caused by: {err}");
            source = err.source();
        }

        process::exit(1);
    }
}
