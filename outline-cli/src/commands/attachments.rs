use anyhow::{Result, Context};
use clap::Subcommand;
use std::fs;

use outline_api::{
    OutlineClient,
    auth,
    CreateAttachmentRequest,
    ListAttachmentsRequest,
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum AttachmentsCommands {
    /// Upload a file attachment
    Create {
        /// Document ID
        document_id: String,

        /// File path to upload
        file: String,

        /// Attachment name
        #[arg(long)]
        name: String,

        /// Content type (e.g., image/png)
        #[arg(long)]
        content_type: Option<String>,
    },

    /// Delete an attachment
    Delete {
        /// Attachment ID
        id: String,
    },

    /// Get attachment download URL
    Redirect {
        /// Attachment ID
        id: String,
    },

    /// List attachments
    List {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },
}

impl AttachmentsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;
        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            AttachmentsCommands::Create { document_id, file, name, content_type } => {
                // Read file data
                let file_data = fs::read(&file)
                    .context(format!("Failed to read file: {}", file))?;

                let ct = content_type.clone().unwrap_or_else(|| "application/octet-stream".to_string());

                let request = CreateAttachmentRequest::new(name.clone(), ct, file_data)
                    .document_id(document_id.clone());

                let attachment = client.create_attachment(request).await?;
                println!("✓ Attachment uploaded: {} ({})", attachment.name, attachment.id);
                println!("  Size: {} bytes", attachment.size);
            }

            AttachmentsCommands::Delete { id } => {
                client.delete_attachment(id.clone()).await?;
                println!("✓ Attachment deleted: {}", id);
            }

            AttachmentsCommands::Redirect { id } => {
                let url = client.redirect_attachment(id.clone()).await?;
                println!("📎 Attachment download URL:");
                println!("{}", url);
            }

            AttachmentsCommands::List { offset, limit } => {
                let mut request = ListAttachmentsRequest::new();
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_attachments(request).await?;

                println!("Attachments (showing {} results):", response.data.len());
                println!();

                for attachment in response.data {
                    println!("📎 {} ({})", attachment.name, attachment.id);
                    println!("   Size: {} bytes", attachment.size);
                    println!("   Type: {}", attachment.content_type);
                    println!();
                }
            }
        }

        Ok(())
    }
}
