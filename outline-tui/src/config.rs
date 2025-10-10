use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_path)
            .context(format!("Failed to read config file: {:?}", config_path))?;

        toml::from_str(&contents)
            .context("Failed to parse config file")
    }

    /// Get the configuration file path
    pub fn config_file_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Failed to determine home directory")?;
        let config_dir = home.join(".outline-cli");

        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        Ok(config_dir.join("config.toml"))
    }

    /// Get the instance URL
    pub fn get_instance_url(&self) -> Result<String> {
        self.instance_url
            .clone()
            .or_else(|| std::env::var("OUTLINE_INSTANCE_URL").ok())
            .context("Instance URL not configured. Run 'outline-cli config set-instance <url>' first.")
    }

    /// Get API base URL (instance_url + /api)
    pub fn get_api_base_url(&self) -> Result<String> {
        let instance_url = self.get_instance_url()?;
        Ok(format!("{}/api", instance_url.trim_end_matches('/')))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            instance_url: None,
            output_format: None,
        }
    }
}
