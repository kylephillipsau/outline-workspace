use anyhow::{Result, Context};
use clap::Subcommand;
use std::collections::HashMap;
use std::fs;

use outline_api::{
    OutlineClient,
    Document,
    ExportFormat,
    ListDocumentsRequest,
    CreateDocumentRequest,
    UpdateDocumentRequest,
    SearchDocumentsRequest,
    MoveDocumentRequest,
    RestoreDocumentRequest,
    ViewedDocumentsRequest,
    DraftsRequest,
    TemplatesRequest,
    ExportDocumentRequest,
    auth
};
use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum DocumentsCommands {
    /// List all documents
    List {
        /// Filter by backlink document ID
        #[arg(long)]
        backlink_document_id: Option<String>,

        /// Filter by collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Sort direction (ASC or DESC)
        #[arg(long)]
        direction: Option<String>,

        /// Limit number of results (default: fetch all)
        #[arg(long)]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long)]
        offset: Option<u32>,

        /// Filter by parent document ID
        #[arg(long)]
        parent_document_id: Option<String>,

        /// Sort field (e.g., "title", "updatedAt")
        #[arg(long)]
        sort: Option<String>,

        /// Filter to only templates
        #[arg(long)]
        template: bool,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<String>,
    },

    /// Get a document by ID
    Get {
        /// Document ID
        id: String,

        /// Show only the document text
        #[arg(long)]
        text_only: bool,
    },

    /// Create a new document
    Create {
        /// Document title
        #[arg(long)]
        title: String,

        /// Document text/content
        #[arg(long)]
        text: String,

        /// Collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Parent document ID
        #[arg(long)]
        parent_id: Option<String>,

        /// Emoji icon
        #[arg(long)]
        emoji: Option<String>,

        /// Publish immediately
        #[arg(long)]
        publish: bool,
    },

    /// Update an existing document
    Update {
        /// Document ID
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New text/content
        #[arg(long)]
        text: Option<String>,

        /// New emoji
        #[arg(long)]
        emoji: Option<String>,

        /// Publish the document
        #[arg(long)]
        publish: Option<bool>,
    },

    /// Delete a document
    Delete {
        /// Document ID
        id: String,

        /// Permanently delete (otherwise archive)
        #[arg(long)]
        permanent: bool,
    },

    /// Search documents
    Search {
        /// Search query
        query: String,

        /// Filter by collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Limit number of results (default: fetch all)
        #[arg(long)]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long)]
        offset: Option<u32>,
    },

    /// Archive a document
    Archive {
        /// Document ID
        id: String,
    },

    /// Unarchive a document
    Unarchive {
        /// Document ID
        id: String,
    },

    /// Star a document
    Star {
        /// Document ID
        id: String,
    },

    /// Unstar a document
    Unstar {
        /// Document ID
        id: String,
    },

    /// Unpublish a document (convert to draft)
    Unpublish {
        /// Document ID
        id: String,
    },

    /// Convert a document into a template
    Templatize {
        /// Document ID
        id: String,
    },

    /// Move a document to a different collection or parent
    Move {
        /// Document ID
        id: String,

        /// Target collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Parent document ID
        #[arg(long)]
        parent_id: Option<String>,

        /// Index position
        #[arg(long)]
        index: Option<u32>,
    },

    /// Restore a document from trash
    Restore {
        /// Document ID
        id: String,

        /// Revision ID to restore to
        #[arg(long)]
        revision_id: Option<String>,

        /// Collection ID to restore to
        #[arg(long)]
        collection_id: Option<String>,
    },

    /// List recently viewed documents
    Viewed {
        /// Limit number of results
        #[arg(long)]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long)]
        offset: Option<u32>,
    },

    /// List draft documents
    Drafts {
        /// Filter by collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Limit number of results
        #[arg(long)]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long)]
        offset: Option<u32>,
    },

    /// List template documents
    Templates {
        /// Filter by collection ID
        #[arg(long)]
        collection_id: Option<String>,

        /// Limit number of results
        #[arg(long)]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long)]
        offset: Option<u32>,
    },

    /// Export a document
    Export {
        /// Document ID
        id: String,

        /// Output file path
        #[arg(long, short)]
        output: String,

        /// Export format (markdown, html, pdf)
        #[arg(long, default_value = "markdown")]
        format: String,
    },
}

impl DocumentsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;

        // Use automatic authentication (OAuth2 or API token)
        let client = OutlineClient::with_auto_auth(api_base_url)?;

        match self {
            DocumentsCommands::List {
                backlink_document_id,
                collection_id,
                direction: _,
                limit,
                offset,
                parent_document_id,
                sort: _,
                template,
                user_id,
            } => {
                let template_filter = if *template { Some(true) } else { None };

                let mut all_documents = Vec::new();
                let page_size = limit.unwrap_or(100); // Use 100 as page size for fetching
                let mut current_offset = offset.unwrap_or(0);
                let fetch_all = limit.is_none(); // Fetch all if no limit specified

                loop {
                    let request = ListDocumentsRequest {
                        backlink_document_id: backlink_document_id.clone(),
                        collection_id: collection_id.clone(),
                        direction: None, // TODO: parse direction string to enum
                        limit: Some(page_size),
                        offset: Some(current_offset),
                        parent_document_id: parent_document_id.clone(),
                        sort: None, // TODO: parse sort string to enum
                        template: template_filter,
                        user_id: user_id.clone(),
                    };

                    let response = client.list_documents(request).await?;

                    let count = response.data.len();
                    all_documents.extend(response.data);

                    // If limiting results or no more pages, break
                    if !fetch_all || count < page_size as usize {
                        break;
                    }

                    // Check if there's a next page
                    if let Some(pagination) = &response.pagination {
                        if pagination.next_path.is_none() {
                            break;
                        }
                        current_offset += page_size;
                    } else {
                        break;
                    }
                }

                println!("Documents (showing {} results):", all_documents.len());
                println!();

                // Build a tree structure
                display_document_tree(&all_documents);
            }

            DocumentsCommands::Get { id, text_only } => {
                let doc = client.get_document(id.clone()).await?;

                if *text_only {
                    println!("{}", doc.text);
                } else {
                    let emoji = doc.emoji.unwrap_or_else(|| "📄".to_string());
                    println!("{} {}", emoji, doc.title);
                    println!("ID: {}", doc.id);
                    println!("URL ID: {}", doc.url_id);
                    println!("Collection: {}", doc.collection_id.unwrap_or_else(|| "None".to_string()));
                    println!("Created: {}", doc.created_at);
                    println!("Updated: {}", doc.updated_at);
                    println!("\n--- Content ---\n");
                    println!("{}", doc.text);
                }
            }

            DocumentsCommands::Create {
                title,
                text,
                collection_id,
                parent_id,
                emoji,
                publish,
            } => {
                let request = CreateDocumentRequest {
                    title: title.clone(),
                    text: text.clone(),
                    collection_id: collection_id.clone(),
                    parent_document_id: parent_id.clone(),
                    template_id: None,
                    template: None,
                    emoji: emoji.clone(),
                    publish: Some(*publish),
                };

                let doc = client.create_document(request).await?;

                println!("Document created successfully!");
                println!("ID: {}", doc.id);
                println!("Title: {}", doc.title);
            }

            DocumentsCommands::Update {
                id,
                title,
                text,
                emoji,
                publish,
            } => {
                let request = UpdateDocumentRequest {
                    id: id.clone(),
                    title: title.clone(),
                    text: text.clone(),
                    emoji: emoji.clone(),
                    append: None,
                    publish: *publish,
                    done: None,
                };

                let doc = client.update_document(request).await?;

                println!("Document updated successfully!");
                println!("ID: {}", doc.id);
                println!("Title: {}", doc.title);
            }

            DocumentsCommands::Delete { id, permanent } => {
                client.delete_document(id.clone(), *permanent).await?;

                if *permanent {
                    println!("Document permanently deleted: {}", id);
                } else {
                    println!("Document archived: {}", id);
                }
            }

            DocumentsCommands::Search {
                query,
                collection_id,
                limit,
                offset,
            } => {
                let mut all_documents = Vec::new();
                let page_size = limit.unwrap_or(100);
                let mut current_offset = offset.unwrap_or(0);
                let fetch_all = limit.is_none();

                loop {
                    let request = SearchDocumentsRequest {
                        query: query.clone(),
                        collection_id: collection_id.clone(),
                        user_id: None,
                        date_filter: None,
                        include_archived: None,
                        include_drafts: None,
                        offset: Some(current_offset),
                        limit: Some(page_size),
                    };

                    let response = client.search_documents(request).await?;

                    let count = response.data.len();
                    all_documents.extend(response.data);

                    // If limiting results or no more pages, break
                    if !fetch_all || count < page_size as usize {
                        break;
                    }

                    // Check if there's a next page
                    if let Some(pagination) = &response.pagination {
                        if pagination.next_path.is_none() {
                            break;
                        }
                        current_offset += page_size;
                    } else {
                        break;
                    }
                }

                println!("Search results for '{}' ({} found):", query, all_documents.len());
                println!();

                for result in all_documents {
                    let doc = &result.document;
                    let emoji = doc.emoji.as_deref().unwrap_or("📄");
                    println!("{} {} ({})", emoji, doc.title, doc.id);
                    println!("  Relevance: {:.2}", result.ranking);
                    println!("  Collection: {}", doc.collection_id.as_deref().unwrap_or("None"));
                    if !doc.updated_at.is_empty() {
                        println!("  Updated: {}", doc.updated_at);
                    }
                    println!();
                }
            }

            DocumentsCommands::Archive { id } => {
                let doc = client.archive_document(id.clone()).await?;
                println!("✓ Document archived: {}", doc.title);
            }

            DocumentsCommands::Unarchive { id } => {
                let doc = client.unarchive_document(id.clone()).await?;
                println!("✓ Document unarchived: {}", doc.title);
            }

            DocumentsCommands::Star { id } => {
                let doc = client.star_document(id.clone()).await?;
                println!("⭐ Document starred: {}", doc.title);
            }

            DocumentsCommands::Unstar { id } => {
                let doc = client.unstar_document(id.clone()).await?;
                println!("✓ Document unstarred: {}", doc.title);
            }

            DocumentsCommands::Unpublish { id } => {
                let doc = client.unpublish_document(id.clone()).await?;
                println!("✓ Document unpublished (converted to draft): {}", doc.title);
            }

            DocumentsCommands::Templatize { id } => {
                let doc = client.templatize_document(id.clone()).await?;
                println!("✓ Document converted to template: {}", doc.title);
            }

            DocumentsCommands::Move { id, collection_id, parent_id, index } => {
                let mut request = MoveDocumentRequest::new(id.clone());
                request.collection_id = collection_id.clone();
                request.parent_document_id = parent_id.clone();
                request.index = index.clone();
                let doc = client.move_document(request).await?;
                println!("✓ Document moved: {}", doc.title);
            }

            DocumentsCommands::Restore { id, revision_id, collection_id } => {
                let mut request = RestoreDocumentRequest::new(id.clone());
                request.revision_id = revision_id.clone();
                request.collection_id = collection_id.clone();
                let doc = client.restore_document(request).await?;
                println!("✓ Document restored: {}", doc.title);
            }

            DocumentsCommands::Viewed { limit, offset } => {
                let mut request = ViewedDocumentsRequest::new();
                request.limit = Some(limit.unwrap_or(25));
                request.offset = Some(offset.unwrap_or(0));
                let response = client.list_viewed_documents(request).await?;

                println!("Recently viewed documents ({} results):", response.data.len());
                println!();

                for doc in response.data {
                    let emoji = doc.emoji.as_deref().unwrap_or("📄");
                    println!("{} {} ({})", emoji, doc.title, doc.id);
                    if let Some(viewed) = doc.last_viewed_at {
                        println!("  Last viewed: {}", viewed);
                    }
                    println!();
                }
            }

            DocumentsCommands::Drafts { collection_id, limit, offset } => {
                let mut request = DraftsRequest::new();
                request.collection_id = collection_id.clone();
                request.limit = Some(limit.unwrap_or(25));
                request.offset = Some(offset.unwrap_or(0));
                let response = client.list_drafts(request).await?;

                println!("Draft documents ({} results):", response.data.len());
                println!();

                display_document_tree(&response.data);
            }

            DocumentsCommands::Templates { collection_id, limit, offset } => {
                let mut request = TemplatesRequest::new();
                request.collection_id = collection_id.clone();
                request.limit = Some(limit.unwrap_or(25));
                request.offset = Some(offset.unwrap_or(0));
                let response = client.list_templates(request).await?;

                println!("Template documents ({} results):", response.data.len());
                println!();

                display_document_tree(&response.data);
            }

            DocumentsCommands::Export { id, output, format } => {
                let export_format = match format.to_lowercase().as_str() {
                    "markdown" | "md" => ExportFormat::Markdown,
                    "html" => ExportFormat::Html,
                    "pdf" => ExportFormat::Pdf,
                    _ => {
                        anyhow::bail!("Invalid format. Supported formats: markdown, html, pdf");
                    }
                };

                let request = ExportDocumentRequest::new(id.clone(), export_format);
                let data = client.export_document(request).await?;

                fs::write(&output, data)
                    .context(format!("Failed to write to file: {}", output))?;

                println!("✓ Document exported to: {}", output);
            }
        }

        Ok(())
    }
}

/// Display documents in a tree hierarchy
fn display_document_tree(documents: &[Document]) {
    // Group documents by collection
    let mut by_collection: HashMap<Option<String>, Vec<&Document>> = HashMap::new();

    for doc in documents {
        by_collection
            .entry(doc.collection_id.clone())
            .or_insert_with(Vec::new)
            .push(doc);
    }

    // For each collection, build and display the tree
    for (collection_id, docs) in by_collection.iter() {
        if let Some(cid) = collection_id {
            println!("📁 Collection: {}", cid);
        } else {
            println!("📁 No Collection");
        }
        println!();

        // Build parent-child relationships
        let mut children: HashMap<Option<String>, Vec<&Document>> = HashMap::new();
        let mut all_doc_ids: HashMap<String, &Document> = HashMap::new();

        for doc in docs {
            all_doc_ids.insert(doc.id.clone(), doc);
            children
                .entry(doc.parent_document_id.clone())
                .or_insert_with(Vec::new)
                .push(doc);
        }

        // Find root documents (those with no parent or parent not in current list)
        let roots: Vec<&Document> = docs
            .iter()
            .filter(|doc| {
                doc.parent_document_id.is_none() ||
                !all_doc_ids.contains_key(doc.parent_document_id.as_ref().unwrap())
            })
            .copied()
            .collect();

        // Display the tree starting from roots
        for root in roots {
            display_document_node(root, &children, 0);
        }

        println!();
    }
}

/// Recursively display a document and its children
fn display_document_node(
    doc: &Document,
    children: &HashMap<Option<String>, Vec<&Document>>,
    depth: usize,
) {
    let indent = "  ".repeat(depth);

    // Note: The documents.list API doesn't return emoji field for performance reasons
    // Emojis are only available when fetching individual documents via documents.info
    let emoji = doc.emoji.as_deref().unwrap_or("📄");

    // Use tree characters for better visualization
    let prefix = if depth == 0 {
        ""
    } else {
        "└─ "
    };

    println!("{}{}{} {}", indent, prefix, emoji, doc.title);

    // Display children
    if let Some(child_docs) = children.get(&Some(doc.id.clone())) {
        for child in child_docs {
            display_document_node(child, children, depth + 1);
        }
    }
}
