use anyhow::Result;
use clap::Subcommand;

use outline_api::{
    OutlineClient,
    auth,
    CreateShareRequest,
    UpdateShareRequest,
    ListSharesRequest,
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum SharesCommands {
    /// Create a public share link
    Create {
        /// Document ID
        document_id: String,

        /// Include child documents
        #[arg(long)]
        include_children: bool,
    },

    /// Get share details
    Get {
        /// Share ID
        id: String,
    },

    /// List all shares
    List {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },

    /// Update share settings
    Update {
        /// Share ID
        id: String,

        /// Publish the share
        #[arg(long)]
        published: Option<bool>,

        /// Include child documents
        #[arg(long)]
        include_children: Option<bool>,
    },

    /// Revoke a share link
    Revoke {
        /// Share ID
        id: String,
    },
}

impl SharesCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;
        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            SharesCommands::Create { document_id, include_children } => {
                let mut request = CreateShareRequest::new(document_id.clone());
                if *include_children {
                    request.include_child_documents = Some(true);
                }

                let share = client.create_share(request).await?;
                println!("✓ Share created: {}", share.id);
                println!("  URL: {}", share.url);
                println!("  Published: {}", share.published);
            }

            SharesCommands::Get { id } => {
                let share = client.get_share(id.clone()).await?;
                println!("🔗 Share: {}", share.id);
                println!("URL: {}", share.url);
                println!("Document: {}", share.document_id);
                println!("Published: {}", share.published);
                println!("Include children: {}", share.include_child_documents);

                println!("Created: {}", share.created_at);
                println!("Updated: {}", share.updated_at);
            }

            SharesCommands::List { offset, limit } => {
                let mut request = ListSharesRequest::new();
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_shares(request).await?;

                println!("Shares (showing {} results):", response.data.len());
                println!();

                for share in response.data {
                    let status = if share.published { " [PUBLISHED]" } else { " [DRAFT]" };
                    println!("🔗 {} - Document: {}{}", share.id, share.document_id, status);
                    println!("   URL: {}", share.url);
                    println!();
                }
            }

            SharesCommands::Update { id, published, include_children } => {
                let mut request = UpdateShareRequest::new(id.clone());
                request.published = *published;
                request.include_child_documents = *include_children;

                let share = client.update_share(request).await?;
                println!("✓ Share updated: {}", share.id);
                println!("  Published: {}", share.published);
            }

            SharesCommands::Revoke { id } => {
                client.revoke_share(id.clone()).await?;
                println!("✓ Share revoked: {}", id);
            }
        }

        Ok(())
    }
}
