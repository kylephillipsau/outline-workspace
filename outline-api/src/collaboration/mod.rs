/// Collaboration module for real-time document editing using CRDT (Yjs)
///
/// This module provides WebSocket-based collaboration functionality that integrates
/// with Outline's Hocuspocus backend for real-time, conflict-free document editing.
///
/// This module is only available when the `collaboration` feature is enabled.

#[cfg(feature = "collaboration")]
mod websocket;
#[cfg(feature = "collaboration")]
mod sync;
#[cfg(feature = "collaboration")]
mod protocol;

#[cfg(feature = "collaboration")]
pub use websocket::{CollaborationClient, ConnectionStatus};
#[cfg(feature = "collaboration")]
pub use sync::{DocumentSync, SyncState};
#[cfg(feature = "collaboration")]
pub use protocol::{Message, MessageType};

#[cfg(feature = "collaboration")]
use anyhow::Result;
#[cfg(feature = "collaboration")]
use tokio::sync::mpsc;

/// Message types for collaboration events
#[cfg(feature = "collaboration")]
#[derive(Debug, Clone)]
pub enum CollaborationEvent {
    /// Document content has been updated
    DocumentUpdated(String),
    /// Connection status changed
    StatusChanged(ConnectionStatus),
    /// Error occurred
    Error(String),
    /// User joined the document
    UserJoined(String),
    /// User left the document
    UserLeft(String),
}

/// Initialize a collaboration session for a document
#[cfg(feature = "collaboration")]
pub async fn start_collaboration(
    api_base_url: String,
    api_token: String,
    document_id: String,
) -> Result<(CollaborationClient, mpsc::Receiver<CollaborationEvent>)> {
    use std::sync::Arc;

    let (tx, rx) = mpsc::channel(100);

    // Create a shared DocumentSync instance
    let doc_sync = Arc::new(DocumentSync::new());

    let client = CollaborationClient::new(
        api_base_url,
        api_token,
        document_id,
        tx,
        doc_sync,
    )?;

    Ok((client, rx))
}
