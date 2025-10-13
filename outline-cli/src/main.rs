mod commands;
mod config;

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
    // Parse CLI args, but handle the case where no args are provided
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // If it's a help request or no arguments, print help and exit cleanly
            if e.kind() == clap::error::ErrorKind::DisplayHelp
                || e.kind() == clap::error::ErrorKind::DisplayVersion {
                print!("{}", e);
                std::process::exit(0);
            }
            // For other errors (like missing required args when a subcommand IS provided),
            // still show the error but exit cleanly
            print!("{}", e);
            std::process::exit(0);
        }
    };

    match cli.command {
        Commands::Auth { command } => command.execute().await,
        Commands::Config { command } => command.execute().await,
        Commands::Documents { command } => command.execute().await,
        Commands::Collections { command } => command.execute().await,
        Commands::Users { command } => command.execute().await,
        Commands::Comments { command } => command.execute().await,
        Commands::Groups { command } => command.execute().await,
        Commands::Shares { command } => command.execute().await,
        Commands::Attachments { command } => command.execute().await,
    }
}
