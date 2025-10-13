use phf::phf_map;
use std::sync::OnceLock;

/// Nerd Font detection result cached at runtime
static NERD_FONT_DETECTED: OnceLock<bool> = OnceLock::new();

/// Font Awesome icon mappings (Nerd Fonts code points)
static FONT_AWESOME_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    // Font Awesome Brand icons
    "apple" => "\u{f179}",        //
    "android" => "\u{f17b}",      //
    "windows" => "\u{f17a}",      //
    "linux" => "\u{f17c}",        //
    "github" => "\u{f09b}",       //
    "gitlab" => "\u{f296}",       //
    "docker" => "\u{f395}",       //
    "python" => "\u{f81f}",       //
    "java" => "\u{e738}",         //
    "javascript" => "\u{f81d}",   //
    "rust" => "\u{e7a8}",         //
    "golang" => "\u{e627}",       //
    "php" => "\u{f81e}",          //
    "ruby" => "\u{e739}",         //
    "swift" => "\u{e755}",        //
    "codepen" => "\u{f1cb}",      //
    "google" => "\u{f1a0}",       //
    "slack" => "\u{f198}",        //
    "discord" => "\u{f392}",      //
    "twitter" => "\u{f099}",      //
    "youtube" => "\u{f167}",      //
    "reddit" => "\u{f281}",       //

    // Font Awesome Solid icons
    "database" => "\u{f1c0}",     //
    "terminal" => "\u{f120}",     //
    "server" => "\u{f233}",       //
    "laptop" => "\u{f109}",       //
    "code" => "\u{f121}",         //
    "book" => "\u{f02d}",         //
    "bookmark" => "\u{f02e}",     //
    "star" => "\u{f005}",         //
    "heart" => "\u{f004}",        //
    "flag" => "\u{f024}",         //
    "tag" => "\u{f02b}",          //
    "bell" => "\u{f0f3}",         //
    "key" => "\u{f084}",          //
    "shield" => "\u{f132}",       //
    "wrench" => "\u{f0ad}",       //
    "hammer" => "\u{f6e3}",       //
    "cog" => "\u{f013}",          //
    "settings" => "\u{f013}",     //
    "lock" => "\u{f023}",         //
    "unlock" => "\u{f09c}",       //
    "folder" => "\u{f07b}",       //
    "folder-open" => "\u{f07c}",  //
    "folder-closed" => "\u{f07b}",//
    "file" => "\u{f15b}",         //
    "document" => "\u{f15b}",     //
    "home" => "\u{f015}",         //
    "building" => "\u{f1ad}",     //
    "office" => "\u{f1ad}",       //
    "briefcase" => "\u{f0b1}",    //
    "clipboard" => "\u{f0ea}",    //
    "rocket" => "\u{f135}",       //
    "lightbulb" => "\u{f0eb}",    //
    "camera" => "\u{f030}",       //
    "image" => "\u{f03e}",        //
    "email" => "\u{f0e0}",        //
    "letter" => "\u{f0e0}",       //
    "chat" => "\u{f075}",         //
    "feedback" => "\u{f075}",     //
    "clock" => "\u{f017}",        //
    "calendar" => "\u{f073}",     //
    "cloud" => "\u{f0c2}",        //
    "globe" => "\u{f0ac}",        //
    "internet" => "\u{f0ac}",     //
    "browser" => "\u{f0ac}",      //
    "graph" => "\u{f080}",        //
    "promote" => "\u{f201}",      //
    "target" => "\u{f140}",       //
    "warning" => "\u{f071}",      //
    "info" => "\u{f05a}",         //
    "question" => "\u{f059}",     //
    "done" => "\u{f00c}",         //
    "eye" => "\u{f06e}",          //
    "sun" => "\u{f185}",          //
    "moon" => "\u{f186}",         //
    "plane" => "\u{f072}",        //
    "airplane" => "\u{f072}",     //
    "car" => "\u{f1b9}",          //
    "vehicle" => "\u{f1b9}",      //
    "truck" => "\u{f0d1}",        //
    "bicycle" => "\u{f206}",      //
    "bus" => "\u{f207}",          //
    "train" => "\u{f238}",        //
    "ship" => "\u{f21a}",         //
    "robot" => "\u{f544}",        //
    "cake" => "\u{f1fd}",         //
    "tools" => "\u{f0ad}",        //
    "pencil" => "\u{f303}",       //
    "notepad" => "\u{f15c}",      //
    "note" => "\u{f15c}",         //
    "padlock" => "\u{f023}",      //
    "palette" => "\u{f53f}",      //
    "bag" => "\u{f290}",          //
    "team" => "\u{f0c0}",         //
};

/// Emoji fallback mappings
static EMOJI_MAP: phf::Map<&'static str, &'static str> = phf_map! {
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
    "codepen" => "✏️",
    "docker" => "🐳",
    "linux" => "🐧",
    "python" => "🐍",
    "java" => "☕",
    "javascript" => "📜",
    "rust" => "🦀",
    "golang" => "🐹",
    "php" => "🐘",
    "ruby" => "💎",
    "swift" => "🦅",

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
    "home" => "🏠",
    "building" => "🏢",
    "car" => "🚗",
    "bus" => "🚌",
    "train" => "🚆",
    "airplane" => "✈️",
    "ship" => "🚢",
    "calendar" => "📅",
};

/// Detect if terminal supports Nerd Fonts
fn detect_nerd_fonts() -> bool {
    // Check environment variables that might indicate Nerd Font support
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        // Known terminals that commonly use Nerd Fonts
        match term_program.as_str() {
            "WezTerm" | "Alacritty" | "iTerm.app" | "Hyper" => return true,
            _ => {}
        }
    }

    // Check for explicit Nerd Font indicator
    if std::env::var("NERD_FONTS").is_ok() {
        return true;
    }

    // Check terminal emulator
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("kitty") || term.contains("alacritty") {
            return true;
        }
    }

    // Windows Terminal supports Nerd Fonts when configured
    if let Ok(wt_session) = std::env::var("WT_SESSION") {
        if !wt_session.is_empty() {
            return true;
        }
    }

    // Default to false (emoji fallback)
    false
}

/// Get whether Nerd Fonts are detected (cached)
pub fn has_nerd_fonts() -> bool {
    *NERD_FONT_DETECTED.get_or_init(detect_nerd_fonts)
}

/// Convert Outline icon name to the appropriate icon (Font Awesome or emoji)
///
/// # Arguments
/// * `icon_name` - The icon name from Outline (e.g., "codepen", "windows", "database")
///                 Can also be an actual emoji character (e.g., "🔥", "💻")
///
/// # Returns
/// The corresponding icon as a string (Font Awesome glyph if supported, emoji otherwise)
///
/// # Examples
/// ```
/// use outline_api::icon_to_string;
///
/// // Returns Font Awesome glyph or emoji depending on terminal
/// let icon = icon_to_string("database");
/// // Or pass an actual emoji - it will be returned as-is
/// let emoji_icon = icon_to_string("🔥");
/// ```
pub fn icon_to_string(icon_name: &str) -> &str {
    // If the input is already an emoji character (not an ASCII icon name), return it as-is
    // Check if the first character is outside the ASCII range (likely an emoji/Unicode)
    if let Some(first_char) = icon_name.chars().next() {
        if first_char as u32 > 127 {
            // Non-ASCII character, likely an emoji - return as-is
            return icon_name;
        }
    }

    if has_nerd_fonts() {
        // Try Font Awesome first, fallback to emoji
        FONT_AWESOME_MAP
            .get(icon_name)
            .or_else(|| EMOJI_MAP.get(icon_name))
            .copied()
            .unwrap_or("📄")
    } else {
        // Use emoji
        EMOJI_MAP.get(icon_name).copied().unwrap_or("📄")
    }
}

/// Convert Outline icon name to emoji (always returns emoji, never Font Awesome)
///
/// # Arguments
/// * `icon_name` - The icon name from Outline
///
/// # Returns
/// The corresponding emoji as a string
pub fn icon_to_emoji(icon_name: &str) -> &'static str {
    EMOJI_MAP.get(icon_name).copied().unwrap_or("📄")
}

/// Convert Outline icon name to Font Awesome glyph (always returns Font Awesome if available)
///
/// # Arguments
/// * `icon_name` - The icon name from Outline
///
/// # Returns
/// The corresponding Font Awesome glyph, or falls back to emoji if not found
pub fn icon_to_nerd_font(icon_name: &str) -> &'static str {
    FONT_AWESOME_MAP
        .get(icon_name)
        .or_else(|| EMOJI_MAP.get(icon_name))
        .copied()
        .unwrap_or("📄")
}

/// Convert Outline collection icon to appropriate icon
pub fn collection_icon_to_string(icon_name: &str) -> &'static str {
    if has_nerd_fonts() {
        FONT_AWESOME_MAP
            .get(icon_name)
            .or_else(|| EMOJI_MAP.get(icon_name))
            .copied()
            .unwrap_or("\u{f07b}") // Folder icon
    } else {
        EMOJI_MAP.get(icon_name).copied().unwrap_or("📁")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_fallback() {
        assert_eq!(icon_to_emoji("database"), "🗄️");
        assert_eq!(icon_to_emoji("windows"), "🪟");
        assert_eq!(icon_to_emoji("terminal"), "⌨️");
        assert_eq!(icon_to_emoji("codepen"), "✏️");
        assert_eq!(icon_to_emoji("internet"), "🌐");
    }

    #[test]
    fn test_nerd_font_mapping() {
        assert_eq!(icon_to_nerd_font("database"), "\u{f1c0}");
        assert_eq!(icon_to_nerd_font("windows"), "\u{f17a}");
        assert_eq!(icon_to_nerd_font("terminal"), "\u{f120}");
    }

    #[test]
    fn test_unknown_icon() {
        assert_eq!(icon_to_emoji("unknown_icon"), "📄");
        assert_eq!(icon_to_nerd_font("unknown_icon"), "📄");
    }

    #[test]
    fn test_detection() {
        // Just ensure it doesn't panic
        let _ = has_nerd_fonts();
    }
}
