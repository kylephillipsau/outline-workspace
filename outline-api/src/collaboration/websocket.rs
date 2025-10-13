use anyhow::{Context, Result};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::{Message as WsMessage, http::Request}};
use url::Url;
use std::sync::Arc;

use super::{CollaborationEvent, DocumentSync, protocol::{Message, MessageType}};

/// Connection status for the WebSocket client
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Synced,
    Error(String),
}

/// WebSocket client for Outline collaboration
pub struct CollaborationClient {
    api_base_url: String,
    api_token: String,
    document_id: String,
    event_tx: mpsc::Sender<CollaborationEvent>,
    doc_sync: Arc<DocumentSync>,
    message_tx: Option<mpsc::Sender<Message>>,
}

impl CollaborationClient {
    /// Create a new collaboration client with an existing DocumentSync
    pub fn new(
        api_base_url: String,
        api_token: String,
        document_id: String,
        event_tx: mpsc::Sender<CollaborationEvent>,
        doc_sync: Arc<DocumentSync>,
    ) -> Result<Self> {
        Ok(Self {
            api_base_url,
            api_token,
            document_id,
            event_tx,
            doc_sync,
            message_tx: None,
        })
    }

    /// Build the WebSocket URL for Outline's collaboration endpoint
    fn build_ws_url(&self) -> Result<Url> {
        // Convert https:// to wss:// or http:// to ws://
        let base_url = self.api_base_url
            .replace("https://", "wss://")
            .replace("http://", "ws://");

        // Outline's collaboration endpoint format
        let ws_url = format!("{}/collaboration/document.{}", base_url, self.document_id);

        Url::parse(&ws_url).context("Failed to parse WebSocket URL")
    }

    /// Connect to the WebSocket server and start the message loop
    pub async fn connect(&mut self) -> Result<()> {
        let ws_url = self.build_ws_url()?;

        // Send connecting status
        self.event_tx
            .send(CollaborationEvent::StatusChanged(ConnectionStatus::Connecting))
            .await
            .ok();

        // Build WebSocket request with authentication header
        let request = Request::builder()
            .uri(ws_url.as_str())
            .header("Authorization", format!("Bearer {}", self.api_token))
            .body(())
            .context("Failed to build WebSocket request")?;

        // Connect to WebSocket with timeout
        let ws_stream = match tokio::time::timeout(
            std::time::Duration::from_secs(10),
            connect_async(request)
        ).await {
            Ok(Ok((stream, _))) => stream,
            Ok(Err(e)) => {
                let err_msg = format!("Connection failed: {}", e);
                self.event_tx
                    .send(CollaborationEvent::StatusChanged(
                        ConnectionStatus::Error(err_msg.clone())
                    ))
                    .await
                    .ok();
                return Err(anyhow::anyhow!(err_msg));
            }
            Err(_) => {
                let err_msg = "Connection timeout".to_string();
                self.event_tx
                    .send(CollaborationEvent::StatusChanged(
                        ConnectionStatus::Error(err_msg.clone())
                    ))
                    .await
                    .ok();
                return Err(anyhow::anyhow!(err_msg));
            }
        };

        // Send connected status
        self.event_tx
            .send(CollaborationEvent::StatusChanged(ConnectionStatus::Connected))
            .await
            .ok();

        let (mut ws_write, mut ws_read) = ws_stream.split();

        // Create channel for sending messages
        let (msg_tx, mut msg_rx) = mpsc::channel::<Message>(100);
        self.message_tx = Some(msg_tx);

        // Clone references for async tasks
        let event_tx = self.event_tx.clone();
        let doc_sync = Arc::clone(&self.doc_sync);
        let event_tx_send = self.event_tx.clone();

        // Spawn task to handle outgoing messages
        tokio::spawn(async move {
            while let Some(message) = msg_rx.recv().await {
                let encoded = message.encode();
                if let Err(e) = ws_write.send(WsMessage::Binary(encoded)).await {
                    let _ = event_tx_send
                        .send(CollaborationEvent::Error(format!(
                            "Failed to send message: {}",
                            e
                        )))
                        .await;
                    break;
                }
            }
        });

        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(msg_result) = ws_read.next().await {
                match msg_result {
                    Ok(WsMessage::Binary(data)) => {
                        if let Err(e) = Self::handle_binary_message(
                            data,
                            &doc_sync,
                            &event_tx
                        ).await {
                            let _ = event_tx
                                .send(CollaborationEvent::Error(format!(
                                    "Error handling binary message: {}",
                                    e
                                )))
                                .await;
                        }
                    }
                    Ok(WsMessage::Text(text)) => {
                        if let Err(e) = Self::handle_text_message(text, &event_tx).await {
                            let _ = event_tx
                                .send(CollaborationEvent::Error(format!(
                                    "Error handling text message: {}",
                                    e
                                )))
                                .await;
                        }
                    }
                    Ok(WsMessage::Close(_)) => {
                        let _ = event_tx
                            .send(CollaborationEvent::StatusChanged(
                                ConnectionStatus::Disconnected,
                            ))
                            .await;
                        break;
                    }
                    Ok(WsMessage::Ping(_)) | Ok(WsMessage::Pong(_)) | Ok(WsMessage::Frame(_)) => {
                        // Ignore ping/pong frames
                    }
                    Err(e) => {
                        let _ = event_tx
                            .send(CollaborationEvent::Error(format!("WebSocket error: {}", e)))
                            .await;
                        break;
                    }
                }
            }
        });

        // Send initial Sync Step 1 to request document state
        let sync_step1 = self.doc_sync.create_sync_step1();
        self.send_message(sync_step1).await?;

        Ok(())
    }

    /// Handle binary messages (Yjs protocol)
    async fn handle_binary_message(
        data: Vec<u8>,
        doc_sync: &Arc<DocumentSync>,
        event_tx: &mpsc::Sender<CollaborationEvent>,
    ) -> Result<()> {
        // Decode the protocol message
        let message = Message::decode(&data)?;

        match message.message_type {
            MessageType::SyncStep1 => {
                // Server is requesting our state - send Sync Step 2 with updates
                // This usually doesn't happen in client-server model, but handle it
                let _sync_step2 = doc_sync.create_sync_step2(&message.payload)?;
                // We would need to send this back, but we don't have access to message_tx here
                // This is handled by the server sending SyncStep2 to us instead
                tracing::debug!("Received SyncStep1 from server (unusual)");
            }
            MessageType::SyncStep2 => {
                // Server is sending us the document state
                doc_sync.apply_update(&message.payload)?;
                event_tx
                    .send(CollaborationEvent::StatusChanged(ConnectionStatus::Synced))
                    .await
                    .ok();
                event_tx
                    .send(CollaborationEvent::DocumentUpdated(
                        doc_sync.get_text().unwrap_or_default()
                    ))
                    .await
                    .ok();
            }
            MessageType::Update => {
                // Server is sending us incremental updates
                doc_sync.apply_update(&message.payload)?;
                event_tx
                    .send(CollaborationEvent::DocumentUpdated(
                        doc_sync.get_text().unwrap_or_default()
                    ))
                    .await
                    .ok();
            }
            MessageType::Awareness => {
                // Handle awareness (presence) updates
                // For now, just log it
                tracing::debug!("Received awareness update");
            }
            MessageType::Auth => {
                // Authentication response
                tracing::debug!("Received auth response");
            }
            MessageType::QueryAwareness => {
                // Server is querying awareness state
                tracing::debug!("Received query awareness");
            }
        }

        Ok(())
    }

    /// Handle text messages (JSON metadata, presence, etc.)
    async fn handle_text_message(
        text: String,
        event_tx: &mpsc::Sender<CollaborationEvent>,
    ) -> Result<()> {
        // Parse JSON message
        let msg: serde_json::Value = serde_json::from_str(&text)?;

        // Handle different message types
        if let Some(msg_type) = msg.get("type").and_then(|v| v.as_str()) {
            match msg_type {
                "user.join" => {
                    if let Some(user) = msg.get("user").and_then(|v| v.as_str()) {
                        event_tx
                            .send(CollaborationEvent::UserJoined(user.to_string()))
                            .await
                            .ok();
                    }
                }
                "user.leave" => {
                    if let Some(user) = msg.get("user").and_then(|v| v.as_str()) {
                        event_tx
                            .send(CollaborationEvent::UserLeft(user.to_string()))
                            .await
                            .ok();
                    }
                }
                _ => {
                    tracing::debug!("Unknown message type: {}", msg_type);
                }
            }
        }

        Ok(())
    }

    /// Send a message to the WebSocket server
    pub async fn send_message(&self, message: Message) -> Result<()> {
        if let Some(tx) = &self.message_tx {
            tx.send(message)
                .await
                .context("Failed to send message to WebSocket")?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("WebSocket not connected"))
        }
    }

    /// Send a document update to the server
    pub async fn send_update(&self, update: Vec<u8>) -> Result<()> {
        let message = Message::update(update);
        self.send_message(message).await
    }

    /// Get the document sync instance
    pub fn doc_sync(&self) -> Arc<DocumentSync> {
        Arc::clone(&self.doc_sync)
    }

    /// Disconnect from the WebSocket server
    pub async fn disconnect(&self) -> Result<()> {
        self.event_tx
            .send(CollaborationEvent::StatusChanged(
                ConnectionStatus::Disconnected,
            ))
            .await
            .ok();
        Ok(())
    }
}
