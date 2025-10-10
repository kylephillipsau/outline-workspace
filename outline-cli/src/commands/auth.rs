use anyhow::Result;
use clap::Subcommand;

use outline_api::auth;

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Set API token for authentication
    SetToken {
        /// Your Outline API token (get this from Settings > API & Apps)
        token: String,
    },

    /// Clear stored authentication credentials
    Logout,

    /// Check authentication status
    Status,
}

impl AuthCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            AuthCommands::SetToken { token } => {
                auth::set_api_token(token)?;
                println!("API token stored securely in system keyring");
            }

            AuthCommands::Logout => {
                auth::delete_api_token()?;
                println!("Authentication credentials cleared");
            }

            AuthCommands::Status => {
                if auth::has_api_token() {
                    println!("Authenticated: Yes");
                    println!("API token is stored in system keyring");
                } else {
                    println!("Authenticated: No");
                    println!("Run 'outline-cli auth set-token <token>' to authenticate");
                }
            }
        }

        Ok(())
    }
}
