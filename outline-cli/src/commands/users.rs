use anyhow::Result;
use clap::Subcommand;

use outline_api::{
    OutlineClient,
    auth,
    UserInfoRequest,
    UpdateUserRequest,
    ListUsersRequest,
    UserFilter,
    InviteUserRequest,
    UserRole,
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum UsersCommands {
    /// Get user information (current user or by ID)
    Get {
        /// User ID (omit for current user)
        #[arg(long)]
        id: Option<String>,
    },

    /// Update user profile
    Update {
        /// User ID
        id: String,

        /// New name
        #[arg(long)]
        name: Option<String>,

        /// New avatar URL
        #[arg(long)]
        avatar_url: Option<String>,

        /// New language
        #[arg(long)]
        language: Option<String>,
    },

    /// List all users
    List {
        /// Filter by query string
        #[arg(long)]
        query: Option<String>,

        /// Filter to suspended users
        #[arg(long)]
        suspended: bool,

        /// Filter to admin users
        #[arg(long)]
        admins: bool,

        /// Filter to invited (not yet joined) users
        #[arg(long)]
        invited: bool,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },

    /// Suspend a user account
    Suspend {
        /// User ID
        id: String,
    },

    /// Activate a suspended user
    Activate {
        /// User ID
        id: String,
    },

    /// Delete a user account
    Delete {
        /// User ID
        id: String,
    },

    /// Promote user to admin
    Promote {
        /// User ID
        id: String,
    },

    /// Demote admin to regular user
    Demote {
        /// User ID
        id: String,
    },

    /// Invite a new user to the team
    Invite {
        /// Email addresses to invite (comma-separated)
        #[arg(long)]
        emails: String,

        /// User name
        #[arg(long)]
        name: Option<String>,

        /// Role (member or viewer)
        #[arg(long, default_value = "member")]
        role: String,
    },
}

impl UsersCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;
        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            UsersCommands::Get { id } => {
                let request = UserInfoRequest { id: id.clone() };
                let user = client.get_user(request).await?;

                println!("👤 {}", user.name);
                println!("ID: {}", user.id);

                if let Some(email) = &user.email {
                    println!("Email: {}", email);
                }

                if user.is_suspended.unwrap_or(false) {
                    println!("⚠️  Status: Suspended");
                }
                if user.is_admin.unwrap_or(false) {
                    println!("🔑 Administrator");
                }
                if user.is_viewer.unwrap_or(false) {
                    println!("👁️  Viewer");
                }

                if let Some(created) = &user.created_at {
                    println!("Created: {}", created);
                }
                if let Some(last_active) = &user.last_active_at {
                    println!("Last active: {}", last_active);
                }
            }

            UsersCommands::Update { id, name, avatar_url, language } => {
                let mut request = UpdateUserRequest::new();
                request.id = Some(id.clone());
                request.name = name.clone();
                request.avatar_url = avatar_url.clone();
                request.language = language.clone();

                let user = client.update_user(request).await?;
                println!("✓ User updated: {}", user.name);
            }

            UsersCommands::List { query, suspended, admins, invited, offset, limit } => {
                let mut request = ListUsersRequest::new();
                request.query = query.clone();
                request.filter = if *suspended {
                    Some(UserFilter::Suspended)
                } else if *admins {
                    Some(UserFilter::Admins)
                } else if *invited {
                    Some(UserFilter::Invited)
                } else {
                    None
                };
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_users(request).await?;

                println!("Users (showing {} results):", response.data.len());
                println!();

                for user in response.data {
                    let status = if user.is_suspended.unwrap_or(false) {
                        " [SUSPENDED]"
                    } else if user.last_active_at.is_none() {
                        " [INVITED]"
                    } else {
                        ""
                    };

                    let admin = if user.is_admin.unwrap_or(false) {
                        " 🔑"
                    } else {
                        ""
                    };

                    println!("👤 {} ({}){}{}", user.name, user.id, admin, status);

                    if let Some(email) = &user.email {
                        println!("   Email: {}", email);
                    }

                    if let Some(last_active) = &user.last_active_at {
                        println!("   Last active: {}", last_active);
                    }
                    println!();
                }
            }

            UsersCommands::Suspend { id } => {
                let user = client.suspend_user(id.clone()).await?;
                println!("⚠️  User suspended: {}", user.name);
            }

            UsersCommands::Activate { id } => {
                let user = client.activate_user(id.clone()).await?;
                println!("✓ User activated: {}", user.name);
            }

            UsersCommands::Delete { id } => {
                client.delete_user(id.clone()).await?;
                println!("✓ User deleted: {}", id);
            }

            UsersCommands::Promote { id } => {
                let user = client.promote_user(id.clone()).await?;
                println!("🔑 User promoted to admin: {}", user.name);
            }

            UsersCommands::Demote { id } => {
                let user = client.demote_user(id.clone()).await?;
                println!("✓ User demoted from admin: {}", user.name);
            }

            UsersCommands::Invite { emails, name, role } => {
                // Parse comma-separated emails
                let email_list: Vec<String> = emails
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if email_list.is_empty() {
                    anyhow::bail!("No valid emails provided");
                }

                // Parse role
                let user_role = match role.to_lowercase().as_str() {
                    "admin" => UserRole::Admin,
                    "member" => UserRole::Member,
                    "viewer" => UserRole::Viewer,
                    _ => anyhow::bail!("Invalid role '{}'. Use: admin, member, or viewer", role),
                };

                // Invite each user separately
                for email in &email_list {
                    let request = InviteUserRequest {
                        email: email.clone(),
                        name: name.clone().unwrap_or_else(|| email.split('@').next().unwrap_or("User").to_string()),
                        role: Some(user_role),
                    };

                    client.invite_user(request).await?;
                }

                println!("✓ Invitation(s) sent to:");
                for email in email_list {
                    println!("  • {}", email);
                }
            }
        }

        Ok(())
    }
}
