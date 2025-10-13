use anyhow::Result;
use clap::Subcommand;

use outline_api::{
    OutlineClient,
    auth,
    CreateGroupRequest,
    UpdateGroupRequest,
    ListGroupsRequest,
    AddUserToGroupRequest,
    RemoveUserFromGroupRequest,
    GroupMembershipsRequest,
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum GroupsCommands {
    /// Create a new group
    Create {
        /// Group name
        name: String,
    },

    /// Get group details
    Get {
        /// Group ID
        id: String,
    },

    /// List all groups
    List {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },

    /// Update a group
    Update {
        /// Group ID
        id: String,

        /// New name
        #[arg(long)]
        name: String,
    },

    /// Delete a group
    Delete {
        /// Group ID
        id: String,
    },

    /// Add a user to a group
    AddUser {
        /// Group ID
        id: String,

        /// User ID
        user_id: String,
    },

    /// Remove a user from a group
    RemoveUser {
        /// Group ID
        id: String,

        /// User ID
        user_id: String,
    },

    /// List group members
    Memberships {
        /// Group ID
        id: String,

        /// Optional search query
        #[arg(long)]
        query: Option<String>,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },
}

impl GroupsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;

        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            GroupsCommands::Create { name } => {
                let request = CreateGroupRequest::new(name.clone());
                let group = client.create_group(request).await?;
                println!("✓ Group created: {} ({})", group.name, group.id);
            }

            GroupsCommands::Get { id } => {
                let group = client.get_group(id.clone()).await?;
                println!("👥 {}", group.name);
                println!("ID: {}", group.id);
                if let Some(count) = group.member_count {
                    println!("Members: {}", count);
                }
                println!("Created: {}", group.created_at);
                println!("Updated: {}", group.updated_at);
            }

            GroupsCommands::List { offset, limit } => {
                let mut request = ListGroupsRequest::new();
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_groups(request).await?;

                println!("Groups (showing {} results):", response.data.len());
                println!();

                for group in response.data {
                    let count = group.member_count.unwrap_or(0);
                    println!("👥 {} ({})", group.name, group.id);
                    println!("   Members: {}", count);
                    println!();
                }
            }

            GroupsCommands::Update { id, name } => {
                let mut request = UpdateGroupRequest::new(id.clone());
                request.name = Some(name.clone());
                let group = client.update_group(request).await?;
                println!("✓ Group updated: {}", group.name);
            }

            GroupsCommands::Delete { id } => {
                client.delete_group(id.clone()).await?;
                println!("✓ Group deleted: {}", id);
            }

            GroupsCommands::AddUser { id, user_id } => {
                let request = AddUserToGroupRequest::new(id.clone(), user_id.clone());
                client.add_user_to_group(request).await?;
                println!("✓ User {} added to group", user_id);
            }

            GroupsCommands::RemoveUser { id, user_id } => {
                let request = RemoveUserFromGroupRequest::new(id.clone(), user_id.clone());
                client.remove_user_from_group(request).await?;
                println!("✓ User {} removed from group", user_id);
            }

            GroupsCommands::Memberships { id, query: _, offset, limit } => {
                let mut request = GroupMembershipsRequest::new(id.clone());
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_group_memberships(request).await?;

                println!("Group members (showing {} results):", response.data.len());
                println!();

                for membership in response.data {
                    let user = &membership.user;
                    println!("👤 {} ({})", user.name, user.id);
                    if let Some(email) = &user.email {
                        println!("   Email: {}", email);
                    }
                    println!();
                }
            }
        }

        Ok(())
    }
}
