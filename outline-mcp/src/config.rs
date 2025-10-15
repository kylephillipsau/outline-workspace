use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration for the Outline MCP server
/// Reuses the CLI's config file for consistency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Base URL of the Outline instance (e.g., https://outline.yourdomain.com)
    pub instance_url: Option<String>,

    /// Default output format (not used by MCP, but kept for compatibility)
    #[serde(default = "default_output_format")]
    pub output_format: String,
}

fn default_output_format() -> String {
    "text".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            instance_url: None,
            output_format: default_output_format(),
        }
    }
}

impl Config {
    /// Get the configuration directory path (~/.outline-cli)
    /// Note: We reuse the CLI's config directory for consistency
    pub fn config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Unable to determine home directory")?;
        Ok(home.join(".outline-cli"))
    }

    /// Get the configuration file path
    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_file = Self::config_file()?;

        if !config_file.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_file)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// Get the instance URL, or return an error if not configured
    pub fn get_instance_url(&self) -> Result<String> {
        self.instance_url
            .clone()
            .context("Instance URL not configured. Run 'outline-cli config set-instance <url>' first.")
    }

    /// Get the API base URL (instance_url + /api)
    pub fn get_api_base_url(&self) -> Result<String> {
        let instance_url = self.get_instance_url()?;
        Ok(format!("{}/api", instance_url.trim_end_matches('/')))
    }
}
