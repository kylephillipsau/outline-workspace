use anyhow::{Result, Context};
use clap::Subcommand;
use std::fs;

use outline_api::{
    OutlineClient,
    auth,
    ExportFormat,
    ImportFormat,
    Permission,
    ListCollectionsRequest,
    CreateCollectionRequest,
    UpdateCollectionRequest,
    MoveCollectionRequest,
    ExportCollectionRequest,
    ExportAllCollectionsRequest,
    ImportFileToCollectionRequest,
    AddUserToCollectionRequest,
    RemoveUserFromCollectionRequest,
    AddGroupToCollectionRequest,
    RemoveGroupFromCollectionRequest,
    CollectionMembershipsRequest,
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum CollectionsCommands {
    /// List all collections
    List {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },

    /// Get a specific collection
    Get {
        /// Collection ID
        id: String,
    },

    /// Create a new collection
    Create {
        /// Collection name
        name: String,

        /// Optional description
        #[arg(long)]
        description: Option<String>,

        /// Optional color (hex code)
        #[arg(long)]
        color: Option<String>,

        /// Make collection private
        #[arg(long)]
        private: bool,
    },

    /// Update a collection
    Update {
        /// Collection ID
        id: String,

        /// New name
        #[arg(long)]
        name: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New color (hex code)
        #[arg(long)]
        color: Option<String>,
    },

    /// Delete a collection
    Delete {
        /// Collection ID
        id: String,
    },

    /// Move a collection to a new index
    Move {
        /// Collection ID
        id: String,

        /// New index position
        index: u32,
    },

    /// Export a collection
    Export {
        /// Collection ID
        id: String,

        /// Output file path
        output: String,

        /// Export format (markdown, html, pdf)
        #[arg(long, default_value = "markdown")]
        format: String,
    },

    /// Export all collections
    ExportAll {
        /// Output file path
        output: String,

        /// Export format (markdown, html, pdf)
        #[arg(long, default_value = "markdown")]
        format: String,
    },

    /// Import a file into a collection
    ImportFile {
        /// Collection ID
        id: String,

        /// File path to import
        file: String,

        /// Import format (markdown, html, docx, notion, confluence)
        #[arg(long)]
        format: String,
    },

    /// Add a user to a collection
    AddUser {
        /// Collection ID
        id: String,

        /// User ID
        user_id: String,

        /// Permission (read or read_write)
        #[arg(long, default_value = "read")]
        permission: String,
    },

    /// Remove a user from a collection
    RemoveUser {
        /// Collection ID
        id: String,

        /// User ID
        user_id: String,
    },

    /// Add a group to a collection
    AddGroup {
        /// Collection ID
        id: String,

        /// Group ID
        group_id: String,

        /// Permission (read or read_write)
        #[arg(long, default_value = "read")]
        permission: String,
    },

    /// Remove a group from a collection
    RemoveGroup {
        /// Collection ID
        id: String,

        /// Group ID
        group_id: String,
    },

    /// List collection memberships
    Memberships {
        /// Collection ID
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

impl CollectionsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;

        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            CollectionsCommands::List { offset, limit } => {
                let request = ListCollectionsRequest::with_pagination(*offset, *limit);
                let response = client.list_collections(request).await?;

                println!("Collections (showing {} results):", response.data.len());
                println!();

                for collection in response.data {
                    let icon = collection.icon();
                    println!("{} {} ({})", icon, collection.name, collection.id);

                    if let Some(desc) = collection.description {
                        println!("  Description: {}", desc);
                    }
                    if let Some(color) = collection.color {
                        println!("  Color: {}", color);
                    }
                    println!("  Updated: {}", collection.updated_at);
                    println!();
                }
            }

            CollectionsCommands::Get { id } => {
                let collection = client.get_collection(id.clone()).await?;

                let icon = collection.icon();

                println!("{} {}", icon, collection.name);
                println!("ID: {}", collection.id);

                if let Some(desc) = collection.description {
                    println!("Description: {}", desc);
                }
                if let Some(color) = collection.color {
                    println!("Color: {}", color);
                }
                println!("Created: {}", collection.created_at);
                println!("Updated: {}", collection.updated_at);
            }

            CollectionsCommands::Create { name, description, color, private } => {
                let mut request = CreateCollectionRequest::builder(name.clone());

                if let Some(desc) = description {
                    request = request.description(desc.clone());
                }
                if let Some(col) = color {
                    request = request.color(col.clone());
                }
                if *private {
                    request = request.private(true);
                }

                let collection = client.create_collection(request.build()).await?;
                println!("✓ Collection created: {} ({})", collection.name, collection.id);
            }

            CollectionsCommands::Update { id, name, description, color } => {
                let mut request = UpdateCollectionRequest::builder(id.clone());

                if let Some(n) = name {
                    request = request.name(n.clone());
                }
                if let Some(desc) = description {
                    request = request.description(desc.clone());
                }
                if let Some(col) = color {
                    request = request.color(col.clone());
                }

                let collection = client.update_collection(request.build()).await?;
                println!("✓ Collection updated: {}", collection.name);
            }

            CollectionsCommands::Delete { id } => {
                client.delete_collection(id.clone()).await?;
                println!("✓ Collection deleted: {}", id);
            }

            CollectionsCommands::Move { id, index } => {
                let request = MoveCollectionRequest::new(id.clone(), *index);
                let collection = client.move_collection(request).await?;
                println!("✓ Collection moved: {} to index {}", collection.name, index);
            }

            CollectionsCommands::Export { id, output, format } => {
                let export_format = match format.to_lowercase().as_str() {
                    "markdown" | "md" => ExportFormat::Markdown,
                    "html" => ExportFormat::Html,
                    "pdf" => ExportFormat::Pdf,
                    _ => anyhow::bail!("Invalid format '{}'. Use: markdown, html, or pdf", format),
                };

                let request = ExportCollectionRequest::new(id.clone(), export_format);
                let data = client.export_collection(request).await?;

                fs::write(&output, data)
                    .context(format!("Failed to write export to {}", output))?;

                println!("✓ Collection exported to: {}", output);
            }

            CollectionsCommands::ExportAll { output, format } => {
                let export_format = match format.to_lowercase().as_str() {
                    "markdown" | "md" => ExportFormat::Markdown,
                    "html" => ExportFormat::Html,
                    "pdf" => ExportFormat::Pdf,
                    _ => anyhow::bail!("Invalid format '{}'. Use: markdown, html, or pdf", format),
                };

                let request = ExportAllCollectionsRequest::new(export_format);
                let data = client.export_all_collections(request).await?;

                fs::write(&output, data)
                    .context(format!("Failed to write export to {}", output))?;

                println!("✓ All collections exported to: {}", output);
            }

            CollectionsCommands::ImportFile { id, file, format } => {
                let import_format = match format.to_lowercase().as_str() {
                    "markdown" | "md" => ImportFormat::Markdown,
                    "html" => ImportFormat::Html,
                    "docx" => ImportFormat::Docx,
                    "notion" => ImportFormat::Notion,
                    "confluence" => ImportFormat::Confluence,
                    _ => anyhow::bail!("Invalid format '{}'. Use: markdown, html, docx, notion, or confluence", format),
                };

                let file_data = fs::read(&file)
                    .context(format!("Failed to read file: {}", file))?;

                let request = ImportFileToCollectionRequest::new(
                    id.clone(),
                    file_data,
                    import_format,
                );

                client.import_file_to_collection(request).await?;
                println!("✓ File {} imported to collection", file);
            }

            CollectionsCommands::AddUser { id, user_id, permission } => {
                let perm = match permission.to_lowercase().as_str() {
                    "read" => Permission::Read,
                    "read_write" | "readwrite" | "write" => Permission::ReadWrite,
                    _ => anyhow::bail!("Invalid permission '{}'. Use: read or read_write", permission),
                };

                let request = AddUserToCollectionRequest::new(id.clone(), user_id.clone(), perm);
                client.add_user_to_collection(request).await?;
                println!("✓ User {} added to collection with {} permission", user_id, permission);
            }

            CollectionsCommands::RemoveUser { id, user_id } => {
                let request = RemoveUserFromCollectionRequest::new(id.clone(), user_id.clone());
                client.remove_user_from_collection(request).await?;
                println!("✓ User {} removed from collection", user_id);
            }

            CollectionsCommands::AddGroup { id, group_id, permission } => {
                let perm = match permission.to_lowercase().as_str() {
                    "read" => Permission::Read,
                    "read_write" | "readwrite" | "write" => Permission::ReadWrite,
                    _ => anyhow::bail!("Invalid permission '{}'. Use: read or read_write", permission),
                };

                let request = AddGroupToCollectionRequest::new(id.clone(), group_id.clone(), perm);
                client.add_group_to_collection(request).await?;
                println!("✓ Group {} added to collection with {} permission", group_id, permission);
            }

            CollectionsCommands::RemoveGroup { id, group_id } => {
                let request = RemoveGroupFromCollectionRequest::new(id.clone(), group_id.clone());
                client.remove_group_from_collection(request).await?;
                println!("✓ Group {} removed from collection", group_id);
            }

            CollectionsCommands::Memberships { id, query, offset, limit } => {
                let mut request = CollectionMembershipsRequest::new(id.clone());

                if let Some(q) = query {
                    request.query = Some(q.clone());
                }
                request.offset = Some(*offset);
                request.limit = Some(*limit);

                let response = client.list_collection_memberships(request).await?;

                println!("Collection memberships:");
                println!("{}", serde_json::to_string_pretty(&response)?);
            }
        }

        Ok(())
    }
}
