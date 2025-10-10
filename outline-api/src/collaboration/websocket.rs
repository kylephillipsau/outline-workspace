use anyhow::{Context, Result};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::{Message, http::Request}};
use url::Url;

use super::CollaborationEvent;

/// Connection status for the WebSocket client
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// WebSocket client for Outline collaboration
pub struct CollaborationClient {
    api_base_url: String,
    api_token: String,
    document_id: String,
    event_tx: mpsc::Sender<CollaborationEvent>,
}

impl CollaborationClient {
    /// Create a new collaboration client
    pub fn new(
        api_base_url: String,
        api_token: String,
        document_id: String,
        event_tx: mpsc::Sender<CollaborationEvent>,
    ) -> Result<Self> {
        Ok(Self {
            api_base_url,
            api_token,
            document_id,
            event_tx,
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

    /// Connect to the WebSocket server
    pub async fn connect(&self) -> Result<()> {
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
            std::time::Duration::from_secs(5),
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

        let (_write, mut read) = ws_stream.split();

        // Clone event sender for the read task
        let event_tx = self.event_tx.clone();

        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(Message::Binary(data)) => {
                        // Handle binary Yjs update messages
                        if let Err(e) = Self::handle_binary_message(data, &event_tx).await {
                            let _ = event_tx
                                .send(CollaborationEvent::Error(format!(
                                    "Error handling binary message: {}",
                                    e
                                )))
                                .await;
                        }
                    }
                    Ok(Message::Text(text)) => {
                        // Handle text messages (e.g., presence updates)
                        if let Err(e) = Self::handle_text_message(text, &event_tx).await {
                            let _ = event_tx
                                .send(CollaborationEvent::Error(format!(
                                    "Error handling text message: {}",
                                    e
                                )))
                                .await;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        let _ = event_tx
                            .send(CollaborationEvent::StatusChanged(
                                ConnectionStatus::Disconnected,
                            ))
                            .await;
                        break;
                    }
                    Ok(Message::Ping(_)) | Ok(Message::Pong(_)) | Ok(Message::Frame(_)) => {
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

        Ok(())
    }

    /// Handle binary messages (Yjs updates)
    async fn handle_binary_message(
        _data: Vec<u8>,
        _event_tx: &mpsc::Sender<CollaborationEvent>,
    ) -> Result<()> {
        // TODO: Decode Yjs binary update and apply to local document
        // This will be implemented in the sync module
        Ok(())
    }

    /// Handle text messages (presence, metadata, etc.)
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
                    // Unknown message type, log for debugging
                    event_tx
                        .send(CollaborationEvent::Error(format!(
                            "Unknown message type: {}",
                            msg_type
                        )))
                        .await
                        .ok();
                }
            }
        }

        Ok(())
    }

    /// Send a message to the WebSocket server
    pub async fn send_message(&self, _message: Vec<u8>) -> Result<()> {
        // TODO: Implement message sending
        // This will be used to send Yjs updates to the server
        Ok(())
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
