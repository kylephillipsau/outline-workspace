use anyhow::Result;
use clap::Subcommand;

use crate::config::Config;

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Set the Outline instance URL
    SetInstance {
        /// The base URL of your Outline instance (e.g., https://outline.yourdomain.com)
        url: String,
    },

    /// Show current configuration
    Show,
}

impl ConfigCommands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            ConfigCommands::SetInstance { url } => {
                let mut config = Config::load()?;
                config.set_instance_url(url.clone());
                config.save()?;

                println!("Instance URL set to: {}", url);
                println!("API base URL: {}", config.get_api_base_url()?);
            }

            ConfigCommands::Show => {
                let config = Config::load()?;

                println!("Current configuration:");
                println!();
                println!("Instance URL: {}", config.instance_url.as_ref().map(|s| s.as_str()).unwrap_or("Not set"));
                println!("API Base URL: {}", config.get_api_base_url().unwrap_or_else(|_| "Not set".to_string()));
                println!("Output format: {}", config.output_format);

                let config_file = Config::config_file()?;
                println!();
                println!("Config file: {}", config_file.display());
            }
        }

        Ok(())
    }
}
