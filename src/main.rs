use std::path::PathBuf;

use clap::Parser;
use logger::{ConsoleLogger, JsonLogger, Logger, LoggerType, QuickfixLogger};
use processor::Processor;

mod analysis;
mod logger;
mod parser;
mod processor;
mod utils;

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// The default console format
    Default,
    /// Formats the output as JSON
    Json,
    /// Formats the output as a Vim quickfix list
    #[value(alias("vi"))]
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
    format: Option<OutputFormat>,
}

fn main() {
    let args = Args::parse();
    let mut logger = get_logger(args.format.unwrap_or(OutputFormat::Default));

    let request = analysis::AnalysisRequest {
        path: PathBuf::from(args.cwd),
        source: args.source,
        specifier: args.name,
    };

    Processor::new(request, &mut logger).process();
    logger.end()
}

fn get_logger(format: OutputFormat) -> LoggerType {
    match format {
        OutputFormat::Default => LoggerType::Console(ConsoleLogger::new()),
        OutputFormat::Quickfix => LoggerType::Quickfix(QuickfixLogger::new()),
        OutputFormat::Json => LoggerType::Json(JsonLogger::new()),
    }
}
