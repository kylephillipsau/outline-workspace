use serde::{Deserialize, Serialize};

/// Team settings and information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// Unique identifier for the team
    pub id: String,
    /// Team name
    pub name: String,
    /// URL to team avatar image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Whether sharing is enabled
    #[serde(default)]
    pub sharing: bool,
    /// Whether collaborative editing is enabled
    #[serde(default)]
    pub collaborative_editing: bool,
    /// Default collection ID for new documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_collection_id: Option<String>,
    /// Team domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// List of allowed domains for sign-up
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    /// Default language for the team
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
    /// Whether sign-up is enabled
    #[serde(default)]
    pub signup_enabled: bool,
    /// Whether guest sign-in is enabled
    #[serde(default)]
    pub guest_signin: bool,
    /// Maximum number of imports per day
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_imports_per_day: Option<u32>,
    /// When the team was created
    pub created_at: String,
    /// When the team was last updated
    pub updated_at: String,
}

/// Request to get team information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamInfoRequest {}

impl TeamInfoRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Request to update team settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeamRequest {
    /// New team name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL to new avatar image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Enable/disable sharing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<bool>,
    /// Enable/disable collaborative editing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborative_editing: Option<bool>,
    /// Set default collection ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_collection_id: Option<String>,
    /// List of allowed domains
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
    /// Default language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language: Option<String>,
    /// Enable/disable sign-up
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_enabled: Option<bool>,
    /// Enable/disable guest sign-in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_signin: Option<bool>,
}

impl UpdateTeamRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn avatar_url(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn sharing(mut self, sharing: bool) -> Self {
        self.sharing = Some(sharing);
        self
    }

    pub fn collaborative_editing(mut self, collaborative_editing: bool) -> Self {
        self.collaborative_editing = Some(collaborative_editing);
        self
    }

    pub fn default_collection_id(mut self, default_collection_id: String) -> Self {
        self.default_collection_id = Some(default_collection_id);
        self
    }

    pub fn allowed_domains(mut self, allowed_domains: Vec<String>) -> Self {
        self.allowed_domains = Some(allowed_domains);
        self
    }

    pub fn default_language(mut self, default_language: String) -> Self {
        self.default_language = Some(default_language);
        self
    }

    pub fn signup_enabled(mut self, signup_enabled: bool) -> Self {
        self.signup_enabled = Some(signup_enabled);
        self
    }

    pub fn guest_signin(mut self, guest_signin: bool) -> Self {
        self.guest_signin = Some(guest_signin);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_info_request() {
        let _request = TeamInfoRequest::new();
        // Just ensure it constructs
    }

    #[test]
    fn test_update_team_request_builder() {
        let request = UpdateTeamRequest::new()
            .name("My Awesome Team".to_string())
            .avatar_url("https://example.com/avatar.png".to_string())
            .sharing(true)
            .collaborative_editing(true)
            .default_collection_id("coll-123".to_string())
            .allowed_domains(vec!["example.com".to_string(), "test.com".to_string()])
            .default_language("en-US".to_string())
            .signup_enabled(true)
            .guest_signin(false);

        assert_eq!(request.name, Some("My Awesome Team".to_string()));
        assert_eq!(request.avatar_url, Some("https://example.com/avatar.png".to_string()));
        assert_eq!(request.sharing, Some(true));
        assert_eq!(request.collaborative_editing, Some(true));
        assert_eq!(request.default_collection_id, Some("coll-123".to_string()));
        assert_eq!(request.allowed_domains, Some(vec!["example.com".to_string(), "test.com".to_string()]));
        assert_eq!(request.default_language, Some("en-US".to_string()));
        assert_eq!(request.signup_enabled, Some(true));
        assert_eq!(request.guest_signin, Some(false));
    }
}
