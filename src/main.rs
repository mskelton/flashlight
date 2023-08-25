use std::path::PathBuf;

use clap::{Parser, Subcommand};
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The directory to search in.
    #[arg(long, default_value = ".")]
    cwd: String,

    /// The output format. The default format is a human-readable console output.
    #[arg(long)]
    format: Option<OutputFormat>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find imports
    Imports {
        /// The import source (e.g., react)
        #[arg(index = 1)]
        source: String,

        /// Only include imports containing this import specifier (e.g., useState)
        #[arg(index = 2)]
        specifier: Option<String>,
    },
    /// Find JSX tags
    JsxTags {
        /// The name of the tag (e.g., div)
        #[arg(index = 1)]
        name: String,

        /// Only include tags with this attribute (e.g., onClick)
        #[arg(index = 2)]
        attribute: Option<String>,
    },
    /// Find unused modules
    UnusedModules,
}

fn main() {
    let cli = Cli::parse();
    let mut logger = get_logger(cli.format.unwrap_or(OutputFormat::Default));

    match cli.command {
        Commands::Imports { source, specifier } => {
            let request = analysis::AnalysisRequest {
                path: PathBuf::from(cli.cwd),
                source,
                specifier,
            };

            Processor::new(request, &mut logger).process();
        }
        Commands::JsxTags { .. } => {
            todo!();
        }
        Commands::UnusedModules => {
            todo!();
        }
    };

    logger.end();
}

fn get_logger(format: OutputFormat) -> LoggerType {
    match format {
        OutputFormat::Default => LoggerType::Console(ConsoleLogger::new()),
        OutputFormat::Quickfix => LoggerType::Quickfix(QuickfixLogger::new()),
        OutputFormat::Json => LoggerType::Json(JsonLogger::new()),
    }
}
