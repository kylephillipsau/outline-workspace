use anyhow::Result;
use clap::Subcommand;
use std::collections::HashMap;

use crate::api::types::Document;
use crate::api::OutlineClient;
use crate::auth;
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
}

impl DocumentsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;
        let api_token = auth::get_api_token()?;

        let client = OutlineClient::new(api_base_url)?.with_token(api_token);

        match self {
            DocumentsCommands::List {
                backlink_document_id,
                collection_id,
                direction,
                limit,
                offset,
                parent_document_id,
                sort,
                template,
                user_id,
            } => {
                let template_filter = if *template { Some(true) } else { None };

                let mut all_documents = Vec::new();
                let page_size = limit.unwrap_or(100); // Use 100 as page size for fetching
                let mut current_offset = offset.unwrap_or(0);
                let fetch_all = limit.is_none(); // Fetch all if no limit specified

                loop {
                    let response = client
                        .list_documents(
                            backlink_document_id.clone(),
                            collection_id.clone(),
                            direction.clone(),
                            Some(page_size),
                            Some(current_offset),
                            parent_document_id.clone(),
                            sort.clone(),
                            template_filter,
                            user_id.clone(),
                        )
                        .await?;

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
                let doc = client
                    .create_document(
                        title.clone(),
                        text.clone(),
                        collection_id.clone(),
                        parent_id.clone(),
                        emoji.clone(),
                        Some(*publish),
                    )
                    .await?;

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
                let doc = client
                    .update_document(
                        id.clone(),
                        title.clone(),
                        text.clone(),
                        emoji.clone(),
                        *publish,
                    )
                    .await?;

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
                    let response = client
                        .search_documents(
                            query.clone(),
                            collection_id.clone(),
                            Some(current_offset),
                            Some(page_size),
                        )
                        .await?;

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
