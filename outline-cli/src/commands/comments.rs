use anyhow::Result;
use clap::Subcommand;

use outline_api::{
    OutlineClient,
    auth,
    CreateCommentRequest,
    CommentData,
    ListCommentsRequest,
    UpdateCommentRequest,
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum CommentsCommands {
    /// Create a comment on a document
    Create {
        /// Document ID
        #[arg(long)]
        document_id: String,

        /// Comment text
        #[arg(long)]
        text: String,

        /// Parent comment ID (for replies)
        #[arg(long)]
        parent_id: Option<String>,
    },

    /// Get comment details
    Get {
        /// Comment ID
        id: String,
    },

    /// List comments on a document
    List {
        /// Document ID
        #[arg(long)]
        document_id: String,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },

    /// Update a comment
    Update {
        /// Comment ID
        id: String,

        /// New comment text
        #[arg(long)]
        text: String,
    },

    /// Delete a comment
    Delete {
        /// Comment ID
        id: String,
    },

    /// Mark a comment thread as resolved
    Resolve {
        /// Comment ID
        id: String,
    },

    /// Mark a comment thread as unresolved
    Unresolve {
        /// Comment ID
        id: String,
    },
}

impl CommentsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;
        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            CommentsCommands::Create { document_id, text, parent_id } => {
                let data = CommentData::new(text.clone());
                let mut request = CreateCommentRequest::new(document_id.clone(), data);
                request.parent_comment_id = parent_id.clone();

                let comment = client.create_comment(request).await?;
                println!("✓ Comment created: {}", comment.id);
                println!("  Text: {}", comment.data.text);
            }

            CommentsCommands::Get { id } => {
                let comment = client.get_comment(id.clone()).await?;

                println!("💬 Comment: {}", comment.id);
                println!("Document: {}", comment.document_id);
                println!("Text: {}", comment.data.text);
                println!("Created by: {}", comment.created_by.name);
                println!("Created at: {}", comment.created_at);
                println!("Updated at: {}", comment.updated_at);

                if let Some(parent_id) = comment.parent_comment_id {
                    println!("Reply to: {}", parent_id);
                }

                if let Some(resolved_at) = comment.resolved_at {
                    println!("✅ Resolved at: {}", resolved_at);
                    if let Some(resolved_by) = comment.resolved_by {
                        println!("   Resolved by: {}", resolved_by.name);
                    }
                }
            }

            CommentsCommands::List { document_id, offset, limit } => {
                let mut request = ListCommentsRequest::new(document_id.clone());
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_comments(request).await?;

                println!("Comments on document {} ({} results):", document_id, response.data.len());
                println!();

                for comment in response.data {
                    let resolved = if comment.resolved_at.is_some() {
                        " ✅"
                    } else {
                        ""
                    };

                    let reply = if comment.parent_comment_id.is_some() {
                        "  ↳ "
                    } else {
                        ""
                    };

                    println!("{}💬 {} ({}){}", reply, comment.id, comment.created_by.name, resolved);
                    println!("{}   {}", reply, comment.data.text);
                    println!("{}   Created: {}", reply, comment.created_at);
                    println!();
                }
            }

            CommentsCommands::Update { id, text } => {
                let data = CommentData::new(text.clone());
                let request = UpdateCommentRequest::new(id.clone(), data);

                let comment = client.update_comment(request).await?;
                println!("✓ Comment updated: {}", comment.id);
                println!("  Text: {}", comment.data.text);
            }

            CommentsCommands::Delete { id } => {
                client.delete_comment(id.clone()).await?;
                println!("✓ Comment deleted: {}", id);
            }

            CommentsCommands::Resolve { id } => {
                let comment = client.resolve_comment(id.clone()).await?;
                println!("✅ Comment thread resolved: {}", comment.id);
            }

            CommentsCommands::Unresolve { id } => {
                let comment = client.unresolve_comment(id.clone()).await?;
                println!("🔓 Comment thread unresolved: {}", comment.id);
            }
        }

        Ok(())
    }
}
