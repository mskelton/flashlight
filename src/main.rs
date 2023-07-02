use std::path::PathBuf;

use clap::Parser;
use console::style;

mod analysis;
mod parser;
mod processor;
mod utils;

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// A human-readable format
    Default,
    /// A machine-readable format
    Json,
    /// A machine-readable format
    Quickfix,
}

/// Find usages of imported symbols in your codebase
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to search for imports in
    #[arg(long, default_value = ".")]
    cwd: String,

    /// The import source to search for
    #[arg(short, long)]
    source: String,

    /// The name of a specific import specifier
    #[arg(short, long)]
    name: Option<String>,

    /// The output format. The default format
    #[arg(long)]
    // TODO: Finish this
    // #[arg(long, default_value = OutputFormat::Default)]
    format: Option<OutputFormat>,
}

fn main() {
    let args = Args::parse();
    println!("{}\n", style("Searching for matches...").bold());

    processor::Processor::new(analysis::AnalysisRequest {
        path: PathBuf::from(args.cwd),
        source: args.source,
        specifier: args.name,
    })
    .process();
}
