use serde::{Deserialize, Serialize};
use super::user::User;
use super::common::SortDirection;

/// An audit trail event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Unique identifier for the event
    pub id: String,
    /// The name of the event (e.g., "documents.create", "users.delete")
    pub name: String,
    /// The user who triggered the event (actor)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<User>,
    /// The ID of the actor (user who triggered the event)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    /// The ID of the document related to this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// The ID of the collection related to this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// The ID of the team related to this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Additional event data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// IP address of the actor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// When the event occurred
    pub created_at: String,
}

/// Sort order for events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventSort {
    CreatedAt,
    Name,
}

/// Request to list events (audit log)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListEventsRequest {
    /// Filter by event name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by actor ID (user who triggered the event)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<String>,
    /// Filter by document ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// Filter by collection ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// Pagination offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Pagination limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<EventSort>,
    /// Sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SortDirection>,
}

impl ListEventsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn actor_id(mut self, actor_id: String) -> Self {
        self.actor_id = Some(actor_id);
        self
    }

    pub fn document_id(mut self, document_id: String) -> Self {
        self.document_id = Some(document_id);
        self
    }

    pub fn collection_id(mut self, collection_id: String) -> Self {
        self.collection_id = Some(collection_id);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort(mut self, sort: EventSort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn direction(mut self, direction: SortDirection) -> Self {
        self.direction = Some(direction);
        self
    }
}

/// Response from listing events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEventsResponse {
    pub data: Vec<Event>,
    pub pagination: super::common::Pagination,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_events_request_builder() {
        let request = ListEventsRequest::new()
            .name("documents.create".to_string())
            .actor_id("user-123".to_string())
            .document_id("doc-456".to_string())
            .collection_id("coll-789".to_string())
            .offset(20)
            .limit(50)
            .sort(EventSort::CreatedAt)
            .direction(SortDirection::Desc);

        assert_eq!(request.name, Some("documents.create".to_string()));
        assert_eq!(request.actor_id, Some("user-123".to_string()));
        assert_eq!(request.document_id, Some("doc-456".to_string()));
        assert_eq!(request.collection_id, Some("coll-789".to_string()));
        assert_eq!(request.offset, Some(20));
        assert_eq!(request.limit, Some(50));
        assert!(matches!(request.sort, Some(EventSort::CreatedAt)));
        assert!(matches!(request.direction, Some(SortDirection::Desc)));
    }

    #[test]
    fn test_event_sort_serialization() {
        let sort_created = EventSort::CreatedAt;
        let sort_name = EventSort::Name;

        let json_created = serde_json::to_string(&sort_created).unwrap();
        let json_name = serde_json::to_string(&sort_name).unwrap();

        assert_eq!(json_created, "\"createdat\"");
        assert_eq!(json_name, "\"name\"");
    }

    #[test]
    fn test_sort_direction_serialization() {
        let asc = SortDirection::Asc;
        let desc = SortDirection::Desc;

        let json_asc = serde_json::to_string(&asc).unwrap();
        let json_desc = serde_json::to_string(&desc).unwrap();

        assert_eq!(json_asc, "\"asc\"");
        assert_eq!(json_desc, "\"desc\"");
    }
}
