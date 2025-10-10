use anyhow::{Context, Result};
use keyring::Entry;

const SERVICE_NAME: &str = "outline-cli";
const API_KEY_USERNAME: &str = "api-token";

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
