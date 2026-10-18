//! 🎨️ The ANSI escapes the human renderer paints with.
//! 🧊️ The ANSI escapes the human renderer paints with, byte-identical to the Go constants.

//#region 🎨️Ansi

/// 🔄️ Restores the default graphic rendition.
pub const RESET: &str = "\u{1b}[0m";
/// 🟥️ Red foreground.
pub const RED: &str = "\u{1b}[31m";
/// 🟩️ Green foreground.
pub const GREEN: &str = "\u{1b}[32m";
/// 🟨️ Yellow foreground.
pub const YELLOW: &str = "\u{1b}[33m";
/// 🟦️ Blue foreground.
pub const BLUE: &str = "\u{1b}[34m";
/// 🌫️ Dim intensity.
pub const DIM: &str = "\u{1b}[2m";
/// 🔆️ Bold intensity.
pub const BOLD: &str = "\u{1b}[1m";

/// 🎨️ Resolves a template colour name to its escape, empty for an unknown name.
pub fn code(name: &str) -> &'static str {
    match name {
        "red" => RED,
        "green" => GREEN,
        "yellow" => YELLOW,
        "blue" => BLUE,
        "dim" => DIM,
        "bold" => BOLD,
        _ => "",
    }
}

/// 🖌️ Wraps `text` in `colour` when `enabled`, otherwise returns it verbatim.
pub fn colorize(text: &str, colour: &str, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    format!("{}{text}{RESET}", code(colour))
}

/// 🌈️ The colour the entity template paints the property at `index` with.
pub fn prop_color(index: usize) -> &'static str {
    match index {
        0 => "yellow",
        1 => "blue",
        2 => "green",
        _ => "dim",
    }
}

/// 📏️ The column budget the human renderer truncates entity lines to.
pub fn terminal_width() -> usize {
    match std::env::var("COLUMNS").ok().and_then(|value| value.parse::<usize>().ok()) {
        Some(width) if width > 0 => width,
        _ => 120,
    }
}

/// ✂️ Truncates to `maximum` visible characters, counting escape sequences as zero width.
pub fn truncate(text: &str, maximum: usize) -> String {
    if maximum == 0 {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let mut visible = 0usize;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == 0x1b {
            let mut end = index + 1;
            while end < bytes.len() && bytes[end] != b'm' {
                end += 1;
            }
            index = if end < bytes.len() { end + 1 } else { end };
            continue;
        }
        let width = utf8_width(bytes[index]);
        index += width;
        visible += 1;
        if visible > maximum {
            break;
        }
    }
    if visible <= maximum {
        return text.to_string();
    }
    let limit = maximum.max(3) - 3;
    let mut out = String::new();
    let mut visible = 0usize;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == 0x1b {
            let mut end = index + 1;
            while end < bytes.len() && bytes[end] != b'm' {
                end += 1;
            }
            let end = if end < bytes.len() { end + 1 } else { end };
            out.push_str(&text[index..end]);
            index = end;
            continue;
        }
        if visible >= limit {
            break;
        }
        let width = utf8_width(bytes[index]);
        out.push_str(&text[index..index + width]);
        index += width;
        visible += 1;
    }
    out.push_str("...");
    out
}

/// 📐️ The byte length of the UTF-8 sequence a lead byte opens.
fn utf8_width(lead: u8) -> usize {
    match lead {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        _ => 4,
    }
}

//#endregion 🎨️Ansi
