use anyhow::Result;
use clap::Subcommand;

use outline_api::auth::{self, OAuth2Config};

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Authenticate with OAuth2 (recommended)
    Login {
        /// Request read scope
        #[arg(long, default_value_t = true)]
        read: bool,

        /// Request write scope
        #[arg(long, default_value_t = true)]
        write: bool,
    },

    /// Configure OAuth2 client credentials
    ConfigureOauth {
        /// OAuth2 Client ID
        #[arg(long)]
        client_id: String,

        /// OAuth2 Client Secret
        #[arg(long)]
        client_secret: String,

        /// Base URL of your Outline instance (e.g., https://app.getoutline.com)
        #[arg(long)]
        base_url: Option<String>,

        /// OAuth2 Authorization URL (advanced: override auto-constructed URL)
        #[arg(long)]
        auth_url: Option<String>,

        /// OAuth2 Token URL (advanced: override auto-constructed URL)
        #[arg(long)]
        token_url: Option<String>,

        /// OAuth2 Redirect URL (local callback)
        #[arg(long, default_value = "http://localhost:8080/callback")]
        redirect_url: String,
    },

    /// Set API token for authentication (legacy method)
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
            AuthCommands::Login { read, write } => {
                // Check if OAuth2 config exists
                if !auth::has_oauth2_config() {
                    println!("OAuth2 configuration not found.");
                    println!("Please run 'outline-cli auth configure-oauth' first to set up your OAuth2 credentials.");
                    println!();
                    println!("To create OAuth2 credentials:");
                    println!("1. Go to your Outline instance → Settings → API & Apps");
                    println!("2. Create a new OAuth application");
                    println!("3. Set the redirect URL to: http://localhost:8080/callback");
                    println!("4. Copy the Client ID and Client Secret");
                    println!("5. Run: outline-cli auth configure-oauth --base-url <URL> --client-id <ID> --client-secret <SECRET>");
                    return Ok(());
                }

                let config = auth::get_oauth2_config()?;

                // Build scopes list
                let mut scopes = Vec::new();
                if *read {
                    scopes.push("read".to_string());
                }
                if *write {
                    scopes.push("write".to_string());
                }

                if scopes.is_empty() {
                    println!("Error: At least one scope (read or write) must be requested");
                    return Ok(());
                }

                println!("Starting OAuth2 authentication flow...");
                println!("Authorization URL: {}", config.auth_url);
                println!("Scopes: {}", scopes.join(", "));
                println!();

                match auth::oauth2_authorize(config, scopes).await {
                    Ok(tokens) => {
                        println!();
                        println!("✓ Authentication successful!");
                        println!();
                        println!("Access token stored securely in system keyring");
                        if let Some(expires_at) = tokens.expires_at {
                            println!("Token expires at: {}", expires_at.format("%Y-%m-%d %H:%M:%S UTC"));
                        }
                        if tokens.refresh_token.is_some() {
                            println!("Refresh token available - access token will be automatically refreshed");
                        }
                    }
                    Err(e) => {
                        println!();
                        println!("✗ Authentication failed: {}", e);
                        println!();
                        println!("Make sure:");
                        println!("1. Your OAuth2 credentials are correct");
                        println!("2. The redirect URL is set to: http://localhost:8080/callback");
                        println!("3. Port 8080 is not in use");
                        return Err(e);
                    }
                }
            }

            AuthCommands::ConfigureOauth {
                client_id,
                client_secret,
                base_url,
                auth_url,
                token_url,
                redirect_url,
            } => {
                // Determine auth and token URLs
                let (final_auth_url, final_token_url) = match (base_url, auth_url, token_url) {
                    // If explicit URLs provided, use them
                    (_, Some(auth), Some(token)) => (auth.clone(), token.clone()),

                    // If base_url provided, construct OAuth URLs from it
                    (Some(base), _, _) => {
                        let base = base.trim_end_matches('/');
                        (
                            format!("{}/oauth/authorize", base),
                            format!("{}/oauth/token", base),
                        )
                    },

                    // If neither provided, error
                    _ => {
                        println!("Error: Either --base-url OR both --auth-url and --token-url must be provided");
                        println!();
                        println!("Examples:");
                        println!("  # Using base URL (recommended):");
                        println!("  outline-cli auth configure-oauth \\");
                        println!("    --base-url https://outline.example.com \\");
                        println!("    --client-id <ID> \\");
                        println!("    --client-secret <SECRET>");
                        println!();
                        println!("  # Using explicit URLs (advanced):");
                        println!("  outline-cli auth configure-oauth \\");
                        println!("    --auth-url https://outline.example.com/oauth/authorize \\");
                        println!("    --token-url https://outline.example.com/oauth/token \\");
                        println!("    --client-id <ID> \\");
                        println!("    --client-secret <SECRET>");
                        return Ok(());
                    }
                };

                let config = OAuth2Config {
                    client_id: client_id.clone(),
                    client_secret: client_secret.clone(),
                    auth_url: final_auth_url.clone(),
                    token_url: final_token_url.clone(),
                    redirect_url: redirect_url.clone(),
                };

                auth::set_oauth2_config(&config)?;
                println!("✓ OAuth2 configuration saved securely");
                println!();
                println!("Configuration:");
                println!("  Authorization URL: {}", final_auth_url);
                println!("  Token URL: {}", final_token_url);
                println!("  Redirect URL: {}", redirect_url);
                println!();
                println!("You can now authenticate with: outline-cli auth login");
            }

            AuthCommands::SetToken { token } => {
                auth::set_api_token(token)?;
                println!("✓ API token stored securely in system keyring");
                println!();
                println!("Note: For better security and automatic token refresh,");
                println!("consider using OAuth2 instead: outline-cli auth login");
            }

            AuthCommands::Logout => {
                let mut cleared_any = false;

                // Clear OAuth2 tokens
                if auth::has_oauth2_tokens() {
                    auth::delete_oauth2_tokens()?;
                    println!("✓ OAuth2 tokens cleared");
                    cleared_any = true;
                }

                // Clear OAuth2 config (optional, but let's keep it for complete logout)
                if auth::has_oauth2_config() {
                    auth::delete_oauth2_config()?;
                    println!("✓ OAuth2 configuration cleared");
                    cleared_any = true;
                }

                // Clear API token
                if auth::has_api_token() {
                    auth::delete_api_token()?;
                    println!("✓ API token cleared");
                    cleared_any = true;
                }

                if cleared_any {
                    println!();
                    println!("All authentication credentials have been removed");
                } else {
                    println!("No authentication credentials found");
                }
            }

            AuthCommands::Status => {
                let auth_method = auth::get_auth_method();

                match auth_method {
                    auth::AuthMethod::OAuth2 => {
                        println!("✓ Authenticated via OAuth2");
                        println!();

                        if let Ok(tokens) = auth::get_oauth2_tokens() {
                            println!("Scopes: {}", tokens.scopes.join(", "));

                            if let Some(expires_at) = tokens.expires_at {
                                let now = chrono::Utc::now();
                                if expires_at > now {
                                    let duration = expires_at - now;
                                    println!("Token expires: {} ({} remaining)",
                                        expires_at.format("%Y-%m-%d %H:%M:%S UTC"),
                                        format_duration(duration));
                                } else {
                                    println!("Token expired: {}", expires_at.format("%Y-%m-%d %H:%M:%S UTC"));
                                    println!("(Will be automatically refreshed on next API call)");
                                }
                            }

                            println!("Refresh token: {}",
                                if tokens.refresh_token.is_some() { "Yes" } else { "No" });
                        }

                        if auth::has_oauth2_config() {
                            println!();
                            println!("OAuth2 configuration: Configured");
                        }
                    }

                    auth::AuthMethod::ApiToken => {
                        println!("✓ Authenticated via API Token (legacy)");
                        println!();
                        println!("API token is stored in system keyring");
                        println!();
                        println!("Tip: For automatic token refresh, consider using OAuth2:");
                        println!("  outline-cli auth configure-oauth --base-url <URL> --client-id <ID> --client-secret <SECRET>");
                        println!("  outline-cli auth login");
                    }

                    auth::AuthMethod::None => {
                        println!("✗ Not authenticated");
                        println!();
                        println!("To authenticate:");
                        println!();
                        println!("Option 1: OAuth2 (recommended)");
                        println!("  1. Configure: outline-cli auth configure-oauth --base-url <URL> --client-id <ID> --client-secret <SECRET>");
                        println!("  2. Login: outline-cli auth login");
                        println!();
                        println!("Option 2: API Token (legacy)");
                        println!("  outline-cli auth set-token <token>");
                    }
                }
            }
        }

        Ok(())
    }
}

/// Format a duration in a human-readable way
fn format_duration(duration: chrono::Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;

    if hours > 24 {
        let days = hours / 24;
        format!("{} day{}", days, if days == 1 { "" } else { "s" })
    } else if hours > 0 {
        format!("{} hour{} {} minute{}",
            hours, if hours == 1 { "" } else { "s" },
            minutes, if minutes == 1 { "" } else { "s" })
    } else {
        format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" })
    }
}
