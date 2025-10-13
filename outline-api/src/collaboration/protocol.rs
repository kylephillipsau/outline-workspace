/// Hocuspocus/Yjs WebSocket protocol implementation
///
/// This module implements the binary protocol used by Hocuspocus and Yjs for
/// collaborative editing over WebSockets.
///
/// Protocol message types:
/// - 0: Sync Step 1 (send state vector)
/// - 1: Sync Step 2 (send missing updates)
/// - 2: Update (incremental changes)
/// - 3+: Awareness and other extensions

use anyhow::{anyhow, Result};

/// Message types in the Yjs sync protocol
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    /// Sync Step 1: Send state vector to request missing updates
    SyncStep1 = 0,
    /// Sync Step 2: Send updates that the other peer is missing
    SyncStep2 = 1,
    /// Update: Send incremental document updates
    Update = 2,
    /// Awareness: Presence and cursor information
    Awareness = 3,
    /// Auth: Authentication message
    Auth = 4,
    /// Query Awareness: Request awareness state
    QueryAwareness = 5,
}

impl MessageType {
    /// Convert a u8 to a MessageType
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(MessageType::SyncStep1),
            1 => Ok(MessageType::SyncStep2),
            2 => Ok(MessageType::Update),
            3 => Ok(MessageType::Awareness),
            4 => Ok(MessageType::Auth),
            5 => Ok(MessageType::QueryAwareness),
            _ => Err(anyhow!("Unknown message type: {}", value)),
        }
    }
}

/// A message in the Hocuspocus/Yjs protocol
#[derive(Debug, Clone)]
pub struct Message {
    /// Type of message
    pub message_type: MessageType,
    /// Message payload
    pub payload: Vec<u8>,
}

impl Message {
    /// Create a new message
    pub fn new(message_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload,
        }
    }

    /// Encode message to binary format
    /// Format: [message_type: u8][payload: Vec<u8>]
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(1 + self.payload.len());
        encoded.push(self.message_type as u8);
        encoded.extend_from_slice(&self.payload);
        encoded
    }

    /// Decode message from binary format
    pub fn decode(data: &[u8]) -> Result<Self> {
        if data.is_empty() {
            return Err(anyhow!("Empty message data"));
        }

        let message_type = MessageType::from_u8(data[0])?;
        let payload = data[1..].to_vec();

        Ok(Self {
            message_type,
            payload,
        })
    }

    /// Create a Sync Step 1 message with a state vector
    pub fn sync_step1(state_vector: Vec<u8>) -> Self {
        Self::new(MessageType::SyncStep1, state_vector)
    }

    /// Create a Sync Step 2 message with updates
    pub fn sync_step2(update: Vec<u8>) -> Self {
        Self::new(MessageType::SyncStep2, update)
    }

    /// Create an Update message
    pub fn update(update: Vec<u8>) -> Self {
        Self::new(MessageType::Update, update)
    }

    /// Create an Awareness message
    pub fn awareness(awareness_update: Vec<u8>) -> Self {
        Self::new(MessageType::Awareness, awareness_update)
    }

    /// Create an Auth message
    pub fn auth(token: String) -> Self {
        // Auth payload is JSON: {"token": "..."}
        let json = serde_json::json!({
            "token": token
        });
        let payload = serde_json::to_vec(&json).unwrap_or_default();
        Self::new(MessageType::Auth, payload)
    }

    /// Check if this is a sync message (Step 1 or Step 2)
    pub fn is_sync(&self) -> bool {
        matches!(
            self.message_type,
            MessageType::SyncStep1 | MessageType::SyncStep2
        )
    }

    /// Check if this is an update message
    pub fn is_update(&self) -> bool {
        self.message_type == MessageType::Update
    }

    /// Check if this is an awareness message
    pub fn is_awareness(&self) -> bool {
        self.message_type == MessageType::Awareness
    }
}

/// Encode a variable-length integer (used in Yjs encoding)
pub fn encode_var_uint(mut num: u64) -> Vec<u8> {
    let mut result = Vec::new();
    while num > 0x7F {
        result.push((num & 0x7F) as u8 | 0x80);
        num >>= 7;
    }
    result.push(num as u8);
    result
}

/// Decode a variable-length integer (used in Yjs encoding)
pub fn decode_var_uint(data: &[u8]) -> Result<(u64, usize)> {
    let mut num: u64 = 0;
    let mut shift = 0;
    let mut bytes_read = 0;

    for &byte in data.iter() {
        bytes_read += 1;
        num |= ((byte & 0x7F) as u64) << shift;

        if byte & 0x80 == 0 {
            return Ok((num, bytes_read));
        }

        shift += 7;
        if shift > 63 {
            return Err(anyhow!("Variable integer overflow"));
        }
    }

    Err(anyhow!("Incomplete variable integer"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_from_u8() {
        assert_eq!(MessageType::from_u8(0).unwrap(), MessageType::SyncStep1);
        assert_eq!(MessageType::from_u8(1).unwrap(), MessageType::SyncStep2);
        assert_eq!(MessageType::from_u8(2).unwrap(), MessageType::Update);
        assert_eq!(MessageType::from_u8(3).unwrap(), MessageType::Awareness);
        assert!(MessageType::from_u8(99).is_err());
    }

    #[test]
    fn test_message_encode_decode() {
        let original = Message::new(MessageType::Update, vec![1, 2, 3, 4]);
        let encoded = original.encode();
        let decoded = Message::decode(&encoded).unwrap();

        assert_eq!(decoded.message_type, MessageType::Update);
        assert_eq!(decoded.payload, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sync_step1_message() {
        let msg = Message::sync_step1(vec![1, 2, 3]);
        assert_eq!(msg.message_type, MessageType::SyncStep1);
        assert!(msg.is_sync());
        assert!(!msg.is_update());
    }

    #[test]
    fn test_update_message() {
        let msg = Message::update(vec![4, 5, 6]);
        assert_eq!(msg.message_type, MessageType::Update);
        assert!(msg.is_update());
        assert!(!msg.is_sync());
    }

    #[test]
    fn test_var_uint_encoding() {
        // Test small numbers
        assert_eq!(encode_var_uint(0), vec![0]);
        assert_eq!(encode_var_uint(127), vec![127]);

        // Test larger numbers
        let encoded = encode_var_uint(300);
        let (decoded, bytes_read) = decode_var_uint(&encoded).unwrap();
        assert_eq!(decoded, 300);
        assert_eq!(bytes_read, encoded.len());
    }

    #[test]
    fn test_var_uint_roundtrip() {
        let test_values = vec![0, 1, 127, 128, 255, 256, 1000, 10000, 1000000];

        for value in test_values {
            let encoded = encode_var_uint(value);
            let (decoded, _) = decode_var_uint(&encoded).unwrap();
            assert_eq!(decoded, value, "Failed roundtrip for value {}", value);
        }
    }
}
