use anyhow::Result;
use yrs::{Doc, Text, Transact, ReadTxn, GetString, UpdateEvent, StateVector};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use std::sync::{Arc, Mutex};
use super::protocol::Message;

/// Synchronization state for the document
#[derive(Debug, Clone, PartialEq)]
pub enum SyncState {
    /// Not yet synchronized
    NotSynced,
    /// Currently synchronizing
    Syncing,
    /// Fully synchronized
    Synced,
    /// Sync error occurred
    Error(String),
}

/// Document synchronization handler using Yjs CRDT
pub struct DocumentSync {
    /// The Yjs document
    doc: Arc<Doc>,
    /// Current sync state
    state: Arc<Mutex<SyncState>>,
}

impl DocumentSync {
    /// Create a new document sync handler
    pub fn new() -> Self {
        let doc = Doc::new();

        Self {
            doc: Arc::new(doc),
            state: Arc::new(Mutex::new(SyncState::NotSynced)),
        }
    }

    /// Get the Yjs document
    pub fn doc(&self) -> Arc<Doc> {
        Arc::clone(&self.doc)
    }

    /// Get the current sync state
    pub fn state(&self) -> SyncState {
        self.state.lock().unwrap().clone()
    }

    /// Set the sync state
    pub fn set_state(&self, state: SyncState) {
        *self.state.lock().unwrap() = state;
    }

    /// Get the document text content
    pub fn get_text(&self) -> Result<String> {
        let txn = self.doc.transact();
        let text = self.doc.get_or_insert_text("content");
        Ok(text.get_string(&txn))
    }

    /// Set the document text content
    pub fn set_text(&self, content: &str) -> Result<()> {
        let mut txn = self.doc.transact_mut();
        let text = self.doc.get_or_insert_text("content");

        // Clear existing content
        let len = text.len(&txn);
        if len > 0 {
            text.remove_range(&mut txn, 0, len);
        }

        // Insert new content
        text.insert(&mut txn, 0, content);

        Ok(())
    }

    /// Apply a binary update from the server
    pub fn apply_update(&self, update: &[u8]) -> Result<()> {
        self.set_state(SyncState::Syncing);

        let mut txn = self.doc.transact_mut();
        let decoded_update = yrs::Update::decode_v1(update)
            .map_err(|e| anyhow::anyhow!("Failed to decode update: {:?}", e))?;
        txn.apply_update(decoded_update)
            .map_err(|e| anyhow::anyhow!("Failed to apply update: {:?}", e))?;

        self.set_state(SyncState::Synced);
        Ok(())
    }

    /// Generate an update to send to the server
    pub fn create_update(&self) -> Result<Vec<u8>> {
        let txn = self.doc.transact();
        let state_vector = txn.state_vector();
        let update = txn.encode_diff_v1(&state_vector);
        Ok(update)
    }

    /// Get the state vector for sync
    pub fn get_state_vector(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.state_vector().encode_v1()
    }

    /// Create a Sync Step 1 message with the current state vector
    pub fn create_sync_step1(&self) -> Message {
        let state_vector = self.get_state_vector();
        Message::sync_step1(state_vector)
    }

    /// Create a Sync Step 2 message with updates for a given state vector
    pub fn create_sync_step2(&self, remote_state_vector: &[u8]) -> Result<Message> {
        let txn = self.doc.transact();
        let sv = StateVector::decode_v1(remote_state_vector)
            .map_err(|e| anyhow::anyhow!("Failed to decode state vector: {:?}", e))?;
        let update = txn.encode_diff_v1(&sv);
        Ok(Message::sync_step2(update))
    }

    /// Create an Update message from recent changes
    pub fn create_update_message(&self) -> Result<Message> {
        let update = self.create_update()?;
        Ok(Message::update(update))
    }

    /// Subscribe to document changes
    pub fn subscribe<F>(&self, callback: F) -> yrs::Subscription
    where
        F: Fn(&yrs::TransactionMut, &UpdateEvent) + 'static,
    {
        self.doc.observe_update_v1(callback).unwrap()
    }
}

impl Default for DocumentSync {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_sync_new() {
        let sync = DocumentSync::new();
        assert_eq!(sync.state(), SyncState::NotSynced);
    }

    #[test]
    fn test_set_and_get_text() {
        let sync = DocumentSync::new();
        sync.set_text("Hello, World!").unwrap();
        assert_eq!(sync.get_text().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_state_management() {
        let sync = DocumentSync::new();
        sync.set_state(SyncState::Syncing);
        assert_eq!(sync.state(), SyncState::Syncing);
        sync.set_state(SyncState::Synced);
        assert_eq!(sync.state(), SyncState::Synced);
    }
}
