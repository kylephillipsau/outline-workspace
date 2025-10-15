use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            _ => anyhow::bail!("Invalid output format: '{}'. Supported formats: text, json", s),
        }
    }

    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json)
    }
}

/// Output JSON to stdout
pub fn output_json<T: Serialize + ?Sized>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)
        .context("Failed to serialize data to JSON")?;
    println!("{}", json);
    Ok(())
}
