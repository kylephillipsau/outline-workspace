mod api;
mod auth;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{AuthCommands, CollectionsCommands, ConfigCommands, DocumentsCommands};

/// CLI tool for interacting with Outline documentation server
#[derive(Parser)]
#[command(name = "outline-cli")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage authentication
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Manage documents
    Documents {
        #[command(subcommand)]
        command: DocumentsCommands,
    },

    /// Manage collections
    Collections {
        #[command(subcommand)]
        command: CollectionsCommands,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { command } => command.execute().await,
        Commands::Config { command } => command.execute().await,
        Commands::Documents { command } => command.execute().await,
        Commands::Collections { command } => command.execute().await,
    }
}
