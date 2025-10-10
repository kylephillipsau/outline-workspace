use anyhow::Result;
use clap::Subcommand;
use phf::phf_map;

use outline_api::{OutlineClient, auth};
use crate::config::Config;

/// Map Outline icon names to emoji characters
/// Based on Outline's IconLibrary.tsx:
/// https://github.com/outline/outline/blob/main/shared/utils/IconLibrary.tsx
static ICON_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    // Internal Outline icons
    "academicCap" => "🎓",
    "bicycle" => "🚲",
    "beaker" => "🧪",
    "buildingBlocks" => "🧱",
    "bookmark" => "🔖",
    "browser" => "🌐",
    "collection" => "📚",
    "coins" => "🪙",
    "camera" => "📷",
    "carrot" => "🥕",
    "clock" => "⏰",
    "cloud" => "☁️",
    "code" => "💻",
    "database" => "🗄️",
    "done" => "✅",
    "email" => "📧",
    "eye" => "👁️",
    "feedback" => "💬",
    "flame" => "🔥",
    "graph" => "📊",
    "globe" => "🌍",
    "hashtag" => "#️⃣",
    "info" => "ℹ️",
    "icecream" => "🍦",
    "image" => "🖼️",
    "internet" => "🌐",
    "leaf" => "🍃",
    "library" => "📚",
    "lightbulb" => "💡",
    "lightning" => "⚡",
    "letter" => "✉️",
    "math" => "🔢",
    "moon" => "🌙",
    "notepad" => "📝",
    "padlock" => "🔒",
    "palette" => "🎨",
    "pencil" => "✏️",
    "plane" => "✈️",
    "promote" => "📈",
    "ramen" => "🍜",
    "question" => "❓",
    "server" => "🖥️",
    "sun" => "☀️",
    "shapes" => "🔷",
    "sport" => "⚽",
    "smiley" => "😊",
    "target" => "🎯",
    "team" => "👥",
    "terminal" => "⌨️",
    "thumbsup" => "👍",
    "truck" => "🚚",
    "tools" => "🔧",
    "vehicle" => "🚗",
    "warning" => "⚠️",

    // Common folder/file icons
    "folder" => "📁",
    "folder-open" => "📂",
    "folder-closed" => "📁",
    "document" => "📄",
    "file" => "📄",

    // Font Awesome brand icons (common ones)
    "apple" => "🍎",
    "android" => "🤖",
    "windows" => "🪟",
    "github" => "🐙",
    "gitlab" => "🦊",
    "google" => "🔍",
    "slack" => "💬",
    "discord" => "💬",
    "twitter" => "🐦",
    "youtube" => "📺",
    "reddit" => "🤖",

    // Font Awesome common icons
    "bag" => "👜",
    "book" => "📖",
    "cake" => "🎂",
    "robot" => "🤖",
    "rocket" => "🚀",
    "star" => "⭐",
    "heart" => "❤️",
    "flag" => "🚩",
    "tag" => "🏷️",
    "bell" => "🔔",
    "key" => "🔑",
    "shield" => "🛡️",
    "wrench" => "🔧",
    "hammer" => "🔨",
    "cog" => "⚙️",
    "settings" => "⚙️",
    "lock" => "🔒",
    "unlock" => "🔓",
    "chat" => "💬",
    "note" => "📝",
    "briefcase" => "💼",
    "clipboard" => "📋",
    "office" => "🏢",
};

/// Convert Outline icon name to emoji
fn icon_name_to_emoji(icon_name: &str) -> String {
    ICON_MAP
        .get(icon_name)
        .unwrap_or(&"📁")
        .to_string()
}

#[derive(Debug, Subcommand)]
pub enum CollectionsCommands {
    /// List all collections
    List {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit number of results
        #[arg(long, default_value = "25")]
        limit: u32,
    },
}

impl CollectionsCommands {
    pub async fn execute(&self) -> Result<()> {
        let config = Config::load()?;
        let api_base_url = config.get_api_base_url()?;
        let api_token = auth::get_api_token()?;

        let client = OutlineClient::new(api_base_url)?.with_token(api_token);

        match self {
            CollectionsCommands::List { offset, limit } => {
                let response = client.list_collections(Some(*offset), Some(*limit)).await?;

                println!("Collections (showing {} results):", response.data.len());
                println!();

                for collection in response.data {
                    let icon = collection.icon
                        .as_ref()
                        .map(|i| icon_name_to_emoji(i))
                        .unwrap_or_else(|| "📁".to_string());
                    println!("{} {} ({})", icon, collection.name, collection.id);

                    if let Some(desc) = collection.description {
                        println!("  Description: {}", desc);
                    }
                    if let Some(color) = collection.color {
                        println!("  Color: {}", color);
                    }
                    println!("  Updated: {}", collection.updated_at);
                    println!();
                }
            }
        }

        Ok(())
    }
}
