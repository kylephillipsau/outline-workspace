use serde::{Deserialize, Serialize};
use super::user::User;

/// A notification for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// Unique identifier for the notification
    pub id: String,
    /// The event that triggered this notification
    pub event: String,
    /// The user who triggered the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<User>,
    /// The ID of the document related to this notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// The ID of the collection related to this notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// The ID of the comment related to this notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// When the notification was created
    pub created_at: String,
    /// When the notification was viewed by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed_at: Option<String>,
    /// When the notification was archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    /// Whether the user has viewed this notification
    #[serde(default)]
    pub viewed: bool,
    /// Whether the notification has been archived
    #[serde(default)]
    pub archived: bool,
}

/// Request to list notifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationsRequest {
    /// Include archived notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Pagination offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Pagination limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl ListNotificationsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
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
}

/// Response from listing notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNotificationsResponse {
    pub data: Vec<Notification>,
    pub pagination: super::common::Pagination,
}

/// Request to update a notification (mark as read)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationRequest {
    /// Notification ID
    pub id: String,
    /// When the notification was viewed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed_at: Option<String>,
}

impl UpdateNotificationRequest {
    pub fn new(id: String) -> Self {
        Self {
            id,
            viewed_at: None,
        }
    }

    pub fn viewed_at(mut self, viewed_at: String) -> Self {
        self.viewed_at = Some(viewed_at);
        self
    }
}

/// Request to archive a notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveNotificationRequest {
    /// Notification ID
    pub id: String,
}

impl ArchiveNotificationRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to unarchive a notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnarchiveNotificationRequest {
    /// Notification ID
    pub id: String,
}

impl UnarchiveNotificationRequest {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Request to archive all notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveAllNotificationsRequest {}

impl Default for ArchiveAllNotificationsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchiveAllNotificationsRequest {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_notifications_request_builder() {
        let request = ListNotificationsRequest::new()
            .archived(false)
            .offset(10)
            .limit(25);

        assert_eq!(request.archived, Some(false));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.limit, Some(25));
    }

    #[test]
    fn test_update_notification_request() {
        let request = UpdateNotificationRequest::new("notif-123".to_string())
            .viewed_at("2025-10-13T12:00:00Z".to_string());

        assert_eq!(request.id, "notif-123");
        assert_eq!(request.viewed_at, Some("2025-10-13T12:00:00Z".to_string()));
    }

    #[test]
    fn test_archive_notification_request() {
        let request = ArchiveNotificationRequest::new("notif-456".to_string());
        assert_eq!(request.id, "notif-456");
    }

    #[test]
    fn test_archive_all_notifications_request() {
        let _request = ArchiveAllNotificationsRequest::new();
        // Just ensure it constructs
    }
}
