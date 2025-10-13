use anyhow::Result;
use outline_api::{
    OutlineClient, CreateDocumentRequest, UpdateDocumentRequest, SearchDocumentsRequest,
    CreateCollectionRequest, UpdateCollectionRequest, ExportFormat, ExportDocumentRequest,
    MoveDocumentRequest, InviteUserRequest, CreateCommentRequest, CommentData,
    CreateGroupRequest, CreateShareRequest, UserInfoRequest, ListUsersRequest,
};
use crate::actions::Action;
use crate::app::App;
use crate::modals::InputField;

/// Execute an action with the API client
pub async fn execute_action(
    action: Action,
    app: &mut App,
    client: &OutlineClient,
    input_values: Vec<String>,
) -> Result<String> {
    match action {
        // Document operations - Phase 1
        Action::CreateDocument => {
            if input_values.len() < 2 {
                return Err(anyhow::anyhow!("Need title and text"));
            }
            let title = &input_values[0];
            let text = &input_values[1];
            let collection_id = if input_values.len() > 2 && !input_values[2].is_empty() {
                Some(input_values[2].clone())
            } else {
                None
            };

            let request = CreateDocumentRequest {
                title: title.clone(),
                text: text.clone(),
                collection_id,
                parent_document_id: None,
                template_id: None,
                template: None,
                emoji: None,
                publish: Some(true),
            };

            let doc = client.create_document(request).await?;
            Ok(format!("Created document: {} ({})", doc.title, doc.id))
        }

        Action::UpdateDocument => {
            if let Some(doc) = &app.current_document {
                if input_values.is_empty() {
                    return Err(anyhow::anyhow!("Need new text"));
                }
                let new_text = &input_values[0];

                let request = UpdateDocumentRequest {
                    id: doc.id.clone(),
                    title: None,
                    text: Some(new_text.clone()),
                    emoji: None,
                    append: None,
                    publish: None,
                    done: None,
                };

                let updated = client.update_document(request).await?;
                app.document_text = updated.text.clone();
                app.current_document = Some(updated.clone());
                Ok(format!("Updated document: {}", updated.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::DeleteDocument => {
            if let Some(doc) = &app.current_document {
                let permanent = input_values.first().map(|s| s == "yes").unwrap_or(false);
                let doc_id = doc.id.clone();
                let doc_title = doc.title.clone();
                client.delete_document(doc_id, permanent).await?;
                app.current_document = None;
                app.document_text.clear();
                Ok(format!("Deleted document: {}", doc_title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::SearchDocuments => {
            if input_values.is_empty() {
                return Err(anyhow::anyhow!("Need search query"));
            }
            let query = &input_values[0];

            let request = SearchDocumentsRequest {
                query: query.clone(),
                collection_id: None,
                user_id: None,
                date_filter: None,
                include_archived: None,
                include_drafts: None,
                offset: Some(0),
                limit: Some(25),
            };

            let response = client.search_documents(request).await?;
            let results: Vec<String> = response
                .data
                .iter()
                .map(|r| format!("{} - {}", r.document.title, r.document.id))
                .collect();

            Ok(format!("Found {} results:\n{}", results.len(), results.join("\n")))
        }

        // Document operations - Phase 2
        Action::ArchiveDocument => {
            if let Some(doc) = &app.current_document {
                let archived = client.archive_document(doc.id.clone()).await?;
                Ok(format!("Archived: {}", archived.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::UnarchiveDocument => {
            if let Some(doc) = &app.current_document {
                let unarchived = client.unarchive_document(doc.id.clone()).await?;
                Ok(format!("Unarchived: {}", unarchived.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::StarDocument => {
            if let Some(doc) = &app.current_document {
                let starred = client.star_document(doc.id.clone()).await?;
                Ok(format!("Starred: {}", starred.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::UnstarDocument => {
            if let Some(doc) = &app.current_document {
                let unstarred = client.unstar_document(doc.id.clone()).await?;
                Ok(format!("Unstarred: {}", unstarred.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::ExportDocument => {
            if let Some(doc) = &app.current_document {
                let format = input_values.first()
                    .and_then(|s| match s.to_lowercase().as_str() {
                        "html" => Some(ExportFormat::Html),
                        "pdf" => Some(ExportFormat::Pdf),
                        _ => Some(ExportFormat::Markdown),
                    })
                    .unwrap_or(ExportFormat::Markdown);

                let request = ExportDocumentRequest::new(doc.id.clone(), format);
                let _data = client.export_document(request).await?;
                Ok(format!("Exported document: {}", doc.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::MoveDocument => {
            if let Some(doc) = &app.current_document {
                if input_values.is_empty() {
                    return Err(anyhow::anyhow!("Need collection ID or parent ID"));
                }

                let collection_id = if !input_values[0].is_empty() {
                    Some(input_values[0].clone())
                } else {
                    None
                };

                let parent_id = if input_values.len() > 1 && !input_values[1].is_empty() {
                    Some(input_values[1].clone())
                } else {
                    None
                };

                let mut request = MoveDocumentRequest::new(doc.id.clone());
                request.collection_id = collection_id;
                request.parent_document_id = parent_id;

                let moved = client.move_document(request).await?;
                Ok(format!("Moved document: {}", moved.title))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        // Collection operations - Phase 1
        Action::CreateCollection => {
            if input_values.is_empty() {
                return Err(anyhow::anyhow!("Need collection name"));
            }
            let name = &input_values[0];
            let description = input_values.get(1).cloned();

            let mut request = CreateCollectionRequest::new(name.clone());
            if let Some(desc) = description {
                request.description = Some(desc);
            }

            let collection = client.create_collection(request).await?;
            Ok(format!("Created collection: {} ({})", collection.name, collection.id))
        }

        Action::UpdateCollection => {
            if input_values.len() < 2 {
                return Err(anyhow::anyhow!("Need collection ID and new name"));
            }
            let id = &input_values[0];
            let name = &input_values[1];

            let mut request = UpdateCollectionRequest::new(id.clone());
            request.name = Some(name.clone());

            let collection = client.update_collection(request).await?;
            Ok(format!("Updated collection: {}", collection.name))
        }

        Action::DeleteCollection => {
            if input_values.is_empty() {
                return Err(anyhow::anyhow!("Need collection ID"));
            }
            let id = &input_values[0];
            client.delete_collection(id.clone()).await?;
            Ok(format!("Deleted collection: {}", id))
        }

        // User operations - Phase 3
        Action::ViewCurrentUser => {
            let request = UserInfoRequest { id: None };
            let user = client.get_user(request).await?;
            Ok(format!("Current user: {} ({})", user.name, user.id))
        }

        Action::ListUsers => {
            let request = ListUsersRequest::new();
            let response = client.list_users(request).await?;
            let users: Vec<String> = response
                .data
                .iter()
                .map(|u| format!("{} - {}", u.name, u.email.as_deref().unwrap_or("no email")))
                .collect();

            Ok(format!("Users ({}):\n{}", users.len(), users.join("\n")))
        }

        Action::InviteUser => {
            if input_values.len() < 2 {
                return Err(anyhow::anyhow!("Need email and name"));
            }
            let email = &input_values[0];
            let name = &input_values[1];

            let request = InviteUserRequest::new(email.clone(), name.clone());
            client.invite_user(request).await?;
            Ok(format!("Invited user: {}", email))
        }

        // Comment operations - Phase 3
        Action::CreateComment => {
            if let Some(doc) = &app.current_document {
                if input_values.is_empty() {
                    return Err(anyhow::anyhow!("Need comment text"));
                }
                let comment_text = &input_values[0];

                let data = CommentData::new(comment_text.clone());
                let request = CreateCommentRequest::new(doc.id.clone(), data);
                let comment = client.create_comment(request).await?;
                Ok(format!("Created comment: {}", comment.id))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::ViewComments => {
            if let Some(doc) = &app.current_document {
                use outline_api::ListCommentsRequest;
                let request = ListCommentsRequest::new(doc.id.clone());
                let response = client.list_comments(request).await?;
                let comments: Vec<String> = response
                    .data
                    .iter()
                    .map(|c| format!("{}: {}", c.created_by.name, c.data.text))
                    .collect();

                Ok(format!("Comments ({}):\n{}", comments.len(), comments.join("\n")))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        // Group operations - Phase 3
        Action::CreateGroup => {
            if input_values.is_empty() {
                return Err(anyhow::anyhow!("Need group name"));
            }
            let name = &input_values[0];

            let request = CreateGroupRequest::new(name.clone());
            let group = client.create_group(request).await?;
            Ok(format!("Created group: {} ({})", group.name, group.id))
        }

        Action::ListGroups => {
            let response = client.list_groups(Default::default()).await?;
            let groups: Vec<String> = response
                .data
                .iter()
                .map(|g| format!("{} ({})", g.name, g.id))
                .collect();

            Ok(format!("Groups ({}):\n{}", groups.len(), groups.join("\n")))
        }

        // Share operations - Phase 3
        Action::CreateShare => {
            if let Some(doc) = &app.current_document {
                let request = CreateShareRequest::new(doc.id.clone());
                let share = client.create_share(request).await?;
                Ok(format!("Created share: {}\nURL: {}", share.id, share.url))
            } else {
                Err(anyhow::anyhow!("No document selected"))
            }
        }

        Action::ListShares => {
            let response = client.list_shares(Default::default()).await?;
            let shares: Vec<String> = response
                .data
                .iter()
                .map(|s| format!("{} - {}", s.id, s.url))
                .collect();

            Ok(format!("Shares ({}):\n{}", shares.len(), shares.join("\n")))
        }

        // Attachment operations - Phase 3
        Action::ListAttachments => {
            let response = client.list_attachments(Default::default()).await?;
            let attachments: Vec<String> = response
                .data
                .iter()
                .map(|a| format!("{} ({} bytes)", a.name, a.size))
                .collect();

            Ok(format!("Attachments ({}):\n{}", attachments.len(), attachments.join("\n")))
        }

        // Not yet implemented actions
        _ => Ok(format!("Action {:?} not yet implemented in TUI", action)),
    }
}

/// Get input fields for an action
pub fn get_input_fields_for_action(action: &Action, _app: &App) -> Vec<InputField> {
    match action {
        Action::CreateDocument => vec![
            InputField::new("Title", "Document title"),
            InputField::new("Text", "Document content"),
            InputField::new("Collection ID (optional)", ""),
        ],
        Action::UpdateDocument => vec![
            InputField::new("New Text", "Updated document content"),
        ],
        Action::DeleteDocument => vec![
            InputField::new("Permanent? (yes/no)", "no"),
        ],
        Action::SearchDocuments => vec![
            InputField::new("Search Query", "Enter search terms"),
        ],
        Action::CreateCollection => vec![
            InputField::new("Name", "Collection name"),
            InputField::new("Description (optional)", ""),
        ],
        Action::UpdateCollection => vec![
            InputField::new("Collection ID", ""),
            InputField::new("New Name", ""),
        ],
        Action::DeleteCollection => vec![
            InputField::new("Collection ID", ""),
        ],
        Action::ExportDocument => vec![
            InputField::new("Format", "markdown, html, or pdf").with_value("markdown"),
        ],
        Action::MoveDocument => vec![
            InputField::new("Collection ID (optional)", ""),
            InputField::new("Parent Document ID (optional)", ""),
        ],
        Action::InviteUser => vec![
            InputField::new("Email", "user@example.com"),
            InputField::new("Name", "User's full name"),
        ],
        Action::CreateComment => vec![
            InputField::new("Comment", "Your comment text"),
        ],
        Action::CreateGroup => vec![
            InputField::new("Group Name", ""),
        ],
        _ => Vec::new(),
    }
}

/// Check if an action requires input
pub fn action_requires_input(action: &Action) -> bool {
    !matches!(action,
        Action::Quit | Action::ToggleFocus | Action::Refresh | Action::ShowHelp |
        Action::ShowMenu | Action::EnterCommandMode | Action::ArchiveDocument |
        Action::UnarchiveDocument | Action::StarDocument | Action::UnstarDocument |
        Action::ViewCurrentUser | Action::ListUsers | Action::ViewComments |
        Action::ListGroups | Action::CreateShare | Action::ListShares |
        Action::ListAttachments | Action::ViewDrafts | Action::ViewTemplates |
        Action::ViewRecent
    )
}
