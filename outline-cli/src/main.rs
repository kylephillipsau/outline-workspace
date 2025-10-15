mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    AttachmentsCommands, AuthCommands, CollectionsCommands, CommentsCommands,
    ConfigCommands, DocumentsCommands, GroupsCommands, SharesCommands, UsersCommands
};

/// CLI tool for interacting with Outline documentation server
#[derive(Parser)]
#[command(name = "outline-cli")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output format (json or text)
    #[arg(long, global = true, default_value = "text")]
    output: String,

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

    /// Manage users
    Users {
        #[command(subcommand)]
        command: UsersCommands,
    },

    /// Manage comments
    Comments {
        #[command(subcommand)]
        command: CommentsCommands,
    },

    /// Manage groups
    Groups {
        #[command(subcommand)]
        command: GroupsCommands,
    },

    /// Manage shares
    Shares {
        #[command(subcommand)]
        command: SharesCommands,
    },

    /// Manage attachments
    Attachments {
        #[command(subcommand)]
        command: AttachmentsCommands,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI args
    let cli = Cli::parse();

    // Parse output format
    let output_format = output::OutputFormat::from_str(&cli.output)?;

    match cli.command {
        Commands::Auth { command } => command.execute().await,
        Commands::Config { command } => command.execute().await,
        Commands::Documents { command } => command.execute(output_format).await,
        Commands::Collections { command } => command.execute().await,
        Commands::Users { command } => command.execute().await,
        Commands::Comments { command } => command.execute().await,
        Commands::Groups { command } => command.execute().await,
        Commands::Shares { command } => command.execute().await,
        Commands::Attachments { command } => command.execute().await,
    }
}
