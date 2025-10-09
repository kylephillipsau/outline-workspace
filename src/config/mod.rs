use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Base URL of the Outline instance (e.g., https://outline.yourdomain.com)
    pub instance_url: Option<String>,

    /// Default output format
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

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        let config_file = Self::config_file()?;

        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_file, contents)
            .context("Failed to write config file")?;

        Ok(())
    }

    /// Set the instance URL
    pub fn set_instance_url(&mut self, url: String) {
        self.instance_url = Some(url);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.output_format, "text");
        assert!(config.instance_url.is_none());
    }

    #[test]
    fn test_api_base_url() {
        let mut config = Config::default();
        config.set_instance_url("https://outline.example.com".to_string());

        let api_url = config.get_api_base_url().unwrap();
        assert_eq!(api_url, "https://outline.example.com/api");
    }

    #[test]
    fn test_api_base_url_trailing_slash() {
        let mut config = Config::default();
        config.set_instance_url("https://outline.example.com/".to_string());

        let api_url = config.get_api_base_url().unwrap();
        assert_eq!(api_url, "https://outline.example.com/api");
    }
}
