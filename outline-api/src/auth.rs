use anyhow::{Context, Result, anyhow};
use keyring::Entry;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenResponse as OAuth2TokenResponse,
    TokenUrl, basic::BasicClient, reqwest::async_http_client,
};
use oauth2::RefreshToken as OAuth2RefreshToken;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

const SERVICE_NAME: &str = "outline-cli";
const API_KEY_USERNAME: &str = "api-token";
const OAUTH2_TOKENS_USERNAME: &str = "oauth2-tokens";
const OAUTH2_CONFIG_USERNAME: &str = "oauth2-config";

/// OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            auth_url: "https://app.getoutline.com/oauth/authorize".to_string(),
            token_url: "https://app.getoutline.com/oauth/token".to_string(),
            redirect_url: "http://localhost:8080/callback".to_string(),
        }
    }
}

/// OAuth2 tokens with expiry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

impl OAuth2Tokens {
    /// Check if the access token is expired or about to expire (within 5 minutes)
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = Utc::now();
            let buffer = Duration::minutes(5);
            expires_at - buffer < now
        } else {
            false
        }
    }
}

/// Authentication method being used
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    /// Using API token (manual)
    ApiToken,
    /// Using OAuth2 (automatic refresh)
    OAuth2,
    /// No authentication configured
    None,
}

/// Get the API token from the system keyring
pub fn get_api_token() -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, API_KEY_USERNAME)
        .context("Failed to create keyring entry")?;

    entry
        .get_password()
        .context(format!(
            "API token not found in keyring (service: '{}', user: '{}'). Please run 'outline-cli auth set-token' first.",
            SERVICE_NAME, API_KEY_USERNAME
        ))
}

/// Store the API token in the system keyring
pub fn set_api_token(token: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, API_KEY_USERNAME)
        .context("Failed to create keyring entry")?;

    entry
        .set_password(token)
        .context(format!(
            "Failed to store API token in keyring (service: '{}', user: '{}')",
            SERVICE_NAME, API_KEY_USERNAME
        ))?;

    // Verify the token was stored by trying to retrieve it
    let retrieved = entry
        .get_password()
        .context("Failed to verify stored token")?;

    if retrieved != token {
        anyhow::bail!("Token verification failed: stored token doesn't match");
    }

    Ok(())
}

/// Delete the API token from the system keyring
pub fn delete_api_token() -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, API_KEY_USERNAME)
        .context("Failed to create keyring entry")?;

    entry
        .delete_credential()
        .context("Failed to delete API token from keyring")
}

/// Check if an API token exists
pub fn has_api_token() -> bool {
    get_api_token().is_ok()
}

// ============================================================================
// OAuth2 Functions
// ============================================================================

/// Store OAuth2 configuration
pub fn set_oauth2_config(config: &OAuth2Config) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, OAUTH2_CONFIG_USERNAME)
        .context("Failed to create keyring entry for OAuth2 config")?;

    let config_json = serde_json::to_string(config)
        .context("Failed to serialize OAuth2 config")?;

    entry
        .set_password(&config_json)
        .context("Failed to store OAuth2 config in keyring")?;

    Ok(())
}

/// Get OAuth2 configuration
pub fn get_oauth2_config() -> Result<OAuth2Config> {
    let entry = Entry::new(SERVICE_NAME, OAUTH2_CONFIG_USERNAME)
        .context("Failed to create keyring entry for OAuth2 config")?;

    let config_json = entry
        .get_password()
        .context("OAuth2 config not found. Please configure OAuth2 credentials first.")?;

    let config: OAuth2Config = serde_json::from_str(&config_json)
        .context("Failed to parse OAuth2 config")?;

    Ok(config)
}

/// Check if OAuth2 config exists
pub fn has_oauth2_config() -> bool {
    get_oauth2_config().is_ok()
}

/// Store OAuth2 tokens
pub fn set_oauth2_tokens(tokens: &OAuth2Tokens) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, OAUTH2_TOKENS_USERNAME)
        .context("Failed to create keyring entry for OAuth2 tokens")?;

    let tokens_json = serde_json::to_string(tokens)
        .context("Failed to serialize OAuth2 tokens")?;

    entry
        .set_password(&tokens_json)
        .context("Failed to store OAuth2 tokens in keyring")?;

    Ok(())
}

/// Get OAuth2 tokens
pub fn get_oauth2_tokens() -> Result<OAuth2Tokens> {
    let entry = Entry::new(SERVICE_NAME, OAUTH2_TOKENS_USERNAME)
        .context("Failed to create keyring entry for OAuth2 tokens")?;

    let tokens_json = entry
        .get_password()
        .context("OAuth2 tokens not found. Please authenticate with OAuth2 first.")?;

    let tokens: OAuth2Tokens = serde_json::from_str(&tokens_json)
        .context("Failed to parse OAuth2 tokens")?;

    Ok(tokens)
}

/// Check if OAuth2 tokens exist
pub fn has_oauth2_tokens() -> bool {
    get_oauth2_tokens().is_ok()
}

/// Delete OAuth2 tokens
pub fn delete_oauth2_tokens() -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, OAUTH2_TOKENS_USERNAME)
        .context("Failed to create keyring entry for OAuth2 tokens")?;

    entry
        .delete_credential()
        .context("Failed to delete OAuth2 tokens from keyring")
}

/// Delete OAuth2 config
pub fn delete_oauth2_config() -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, OAUTH2_CONFIG_USERNAME)
        .context("Failed to create keyring entry for OAuth2 config")?;

    entry
        .delete_credential()
        .context("Failed to delete OAuth2 config from keyring")
}

/// Determine which authentication method is available
pub fn get_auth_method() -> AuthMethod {
    if has_oauth2_tokens() {
        AuthMethod::OAuth2
    } else if has_api_token() {
        AuthMethod::ApiToken
    } else {
        AuthMethod::None
    }
}

/// Get the current access token (from OAuth2 or API token)
pub async fn get_access_token() -> Result<String> {
    match get_auth_method() {
        AuthMethod::OAuth2 => {
            let mut tokens = get_oauth2_tokens()?;

            // Refresh if expired
            if tokens.is_expired() {
                tokens = refresh_oauth2_tokens().await?;
            }

            Ok(tokens.access_token)
        }
        AuthMethod::ApiToken => {
            get_api_token()
        }
        AuthMethod::None => {
            Err(anyhow!("No authentication configured. Please run 'outline-cli auth login' or set an API token."))
        }
    }
}

/// Start OAuth2 authorization flow
pub async fn oauth2_authorize(config: OAuth2Config, scopes: Vec<String>) -> Result<OAuth2Tokens> {
    // Create OAuth2 client
    let client = BasicClient::new(
        ClientId::new(config.client_id.clone()),
        Some(ClientSecret::new(config.client_secret.clone())),
        AuthUrl::new(config.auth_url.clone())
            .context("Invalid authorization URL")?,
        Some(TokenUrl::new(config.token_url.clone())
            .context("Invalid token URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(config.redirect_url.clone())
            .context("Invalid redirect URL")?,
    );

    // Generate the authorization URL
    let mut auth_request = client.authorize_url(CsrfToken::new_random);

    for scope in &scopes {
        auth_request = auth_request.add_scope(Scope::new(scope.clone()));
    }

    let (authorize_url, csrf_state) = auth_request.url();

    // Start local HTTP server to receive the callback
    let redirect_url = Url::parse(&config.redirect_url)?;
    let port = redirect_url.port().unwrap_or(8080);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .context("Failed to bind to local port for OAuth callback")?;

    println!("Opening browser for authorization...");
    println!("If the browser doesn't open, visit this URL:");
    println!("{}", authorize_url);
    println!();
    println!("Waiting for authorization callback on http://localhost:{}...", port);

    // Open the browser
    if let Err(e) = webbrowser::open(authorize_url.as_str()) {
        eprintln!("Failed to open browser: {}", e);
        eprintln!("Please manually open the URL above.");
    }

    // Wait for the callback
    let (code, state) = receive_callback(&listener)?;

    // Verify CSRF token
    if state.secret() != csrf_state.secret() {
        return Err(anyhow!("CSRF token mismatch"));
    }

    // Exchange the code for an access token
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .context("Failed to exchange authorization code for access token")?;

    // Calculate expiry time
    let expires_at = token_result.expires_in().map(|duration| {
        Utc::now() + Duration::seconds(duration.as_secs() as i64)
    });

    // Create tokens structure
    let tokens = OAuth2Tokens {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result.refresh_token().map(|t| t.secret().clone()),
        expires_at,
        scopes: scopes.clone(),
    };

    // Store the tokens
    set_oauth2_tokens(&tokens)?;

    Ok(tokens)
}

/// Refresh OAuth2 access token using refresh token
pub async fn refresh_oauth2_tokens() -> Result<OAuth2Tokens> {
    let config = get_oauth2_config()?;
    let current_tokens = get_oauth2_tokens()?;

    let refresh_token = current_tokens.refresh_token.as_ref()
        .ok_or_else(|| anyhow!("No refresh token available. Please re-authenticate."))?
        .clone();

    // Create OAuth2 client
    let client = BasicClient::new(
        ClientId::new(config.client_id.clone()),
        Some(ClientSecret::new(config.client_secret.clone())),
        AuthUrl::new(config.auth_url.clone())
            .context("Invalid authorization URL")?,
        Some(TokenUrl::new(config.token_url.clone())
            .context("Invalid token URL")?),
    );

    // Exchange refresh token for new access token
    let token_result = client
        .exchange_refresh_token(&OAuth2RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await
        .context("Failed to refresh access token")?;

    // Calculate expiry time
    let expires_at = token_result.expires_in().map(|duration| {
        Utc::now() + Duration::seconds(duration.as_secs() as i64)
    });

    // Create new tokens structure (keep refresh token if not provided)
    let new_tokens = OAuth2Tokens {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result.refresh_token()
            .map(|t| t.secret().clone())
            .or(current_tokens.refresh_token),
        expires_at,
        scopes: current_tokens.scopes.clone(),
    };

    // Store the new tokens
    set_oauth2_tokens(&new_tokens)?;

    Ok(new_tokens)
}

/// Receive OAuth2 callback from local HTTP server
fn receive_callback(listener: &TcpListener) -> Result<(String, CsrfToken)> {
    // Accept a single connection
    let (mut stream, _) = listener.accept()
        .context("Failed to accept callback connection")?;

    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)
        .context("Failed to read callback request")?;

    // Parse the request line
    let redirect_url = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow!("Invalid callback request"))?;

    let url = Url::parse(&format!("http://localhost{}", redirect_url))
        .context("Failed to parse callback URL")?;

    // Extract code and state from query parameters
    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .ok_or_else(|| anyhow!("Authorization code not found in callback"))?;

    let state = url
        .query_pairs()
        .find(|(key, _)| key == "state")
        .map(|(_, value)| CsrfToken::new(value.into_owned()))
        .ok_or_else(|| anyhow!("State not found in callback"))?;

    // Send success response
    let response = "HTTP/1.1 200 OK\r\n\r\n\
        <html><body>\
        <h1>Authorization Successful!</h1>\
        <p>You can close this window and return to the terminal.</p>\
        </body></html>";

    stream.write_all(response.as_bytes())
        .context("Failed to send callback response")?;

    Ok((code, state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Ignore by default as it requires system keyring access
    fn test_token_storage() {
        let test_token = "test-token-123";

        // Set token
        set_api_token(test_token).unwrap();

        // Retrieve token
        let retrieved = get_api_token().unwrap();
        assert_eq!(retrieved, test_token);

        // Delete token
        delete_api_token().unwrap();

        // Verify deletion
        assert!(!has_api_token());
    }
}
