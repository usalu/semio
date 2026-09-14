//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🪪️ Repository identity: platform-entropy identifiers, relative time rendering, and the
//! entity-emoji codec that turns repository artifacts into semantic ids and back.
//! Behaviour twin of `github.com/usalu/semio/repo/identity`; the emoji vocabulary is the same
//! `🧬️schema/🔣️entity-emojis.json` document both implementations read.

//#endregion 🧲️Header

use std::collections::{BTreeMap, HashSet};
use std::sync::OnceLock;

//#region 🆔️Identifier

/// 🆔️ A 128-bit random identifier rendered in the canonical dashed hexadecimal form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(pub [u8; 16]);

/// 🎲️ Supplies the raw bytes an identifier is built from so tests can inject a seed.
pub trait Entropy {
    /// 🎰️ Fills the buffer with entropy.
    fn fill(&mut self, buffer: &mut [u8]) -> Result<(), String>;
}

/// 🌱️ A deterministic, reproducible entropy source for tests and fixtures.
#[derive(Debug, Clone)]
pub struct SeededEntropy {
    state: u64,
}

impl SeededEntropy {
    /// 🎯️ Returns a deterministic entropy source for the given seed.
    pub fn new(seed: u64) -> Self {
        SeededEntropy { state: seed }
    }
}

impl Entropy for SeededEntropy {
    fn fill(&mut self, buffer: &mut [u8]) -> Result<(), String> {
        for slot in buffer.iter_mut() {
            self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut value = self.state;
            value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            value ^= value >> 31;
            *slot = (value >> 56) as u8;
        }
        Ok(())
    }
}

impl Id {
    /// 🎲️ Returns a version 4 identifier drawn from an explicit entropy source.
    pub fn new_from(entropy: &mut dyn Entropy) -> Result<Self, String> {
        let mut bytes = [0u8; 16];
        entropy.fill(&mut bytes)?;
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Ok(Id(bytes))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex: Vec<String> = self.0.iter().map(|byte| format!("{byte:02x}")).collect();
        write!(
            formatter,
            "{}-{}-{}-{}-{}",
            hex[0..4].concat(),
            hex[4..6].concat(),
            hex[6..8].concat(),
            hex[8..10].concat(),
            hex[10..16].concat()
        )
    }
}

//#endregion 🆔️Identifier

//#region ⏳️RelativeTime

/// ⏳️ One second expressed in whole seconds, the unit relative time is measured in.
const SECOND: i64 = 1;
/// ⏳️ One minute expressed in whole seconds.
const MINUTE: i64 = 60;
/// ⏳️ One hour expressed in whole seconds.
const HOUR: i64 = 60 * MINUTE;
/// ⏳️ One day expressed in whole seconds.
const DAY: i64 = 24 * HOUR;
/// ⏳️ One nominal month expressed in whole seconds.
const MONTH: i64 = 30 * DAY;
/// ⏳️ One nominal year expressed in whole seconds.
const YEAR: i64 = 365 * DAY;

/// 🧮️ Renders an elapsed number of seconds as a concise, pluralised relative phrase.
pub fn humanize_seconds(delta_seconds: i64) -> String {
    let future = delta_seconds < 0;
    let delta = delta_seconds.abs();
    let (amount, unit) = if delta < MINUTE {
        ((delta / SECOND).max(1), "second")
    } else if delta < HOUR {
        (delta / MINUTE, "minute")
    } else if delta < DAY {
        (delta / HOUR, "hour")
    } else if delta < MONTH {
        (delta / DAY, "day")
    } else if delta < YEAR {
        (delta / MONTH, "month")
    } else {
        (delta / YEAR, "year")
    };
    let plural = if amount == 1 { unit.to_string() } else { format!("{unit}s") };
    if future {
        format!("{amount} {plural} from now")
    } else {
        format!("{amount} {plural} ago")
    }
}

//#endregion ⏳️RelativeTime

//#region 🔣️EmojiTable

/// 🔣️ The shared entity-emoji vocabulary both implementations load.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityEmojiTable {
    /// 🏷️ The vocabulary schema identifier.
    pub schema: String,
    /// 📝️ Code points whose default presentation is text and therefore need U+FE0F re-added.
    #[serde(rename = "textDefaultEmojis")]
    pub text_default_emojis: Vec<String>,
    /// 🧱️ Per entity kind emoji.
    pub entities: BTreeMap<String, String>,
    /// 🗃️ Per collection kind emoji.
    pub collections: BTreeMap<String, String>,
    /// 📜️ The order `all_entity_emojis` reports the vocabulary in.
    #[serde(rename = "allEntityOrder")]
    pub all_entity_order: Vec<String>,
    /// 🔖️ Prefixes that mark a section reference.
    #[serde(rename = "sectionRefEmojis")]
    pub section_ref_emojis: Vec<String>,
    /// 🏷️ Prefixes that mark a definition reference.
    #[serde(rename = "definitionRefEmojis")]
    pub definition_ref_emojis: Vec<String>,
    /// 📁️ Prefixes that mark a folder reference.
    #[serde(rename = "folderRefEmojis")]
    pub folder_ref_emojis: Vec<String>,
    /// 📄️ Prefixes that mark a file reference.
    #[serde(rename = "fileRefEmojis")]
    pub file_ref_emojis: Vec<String>,
}

/// 📥️ The embedded vocabulary document, shared verbatim with the Go implementation.
const ENTITY_EMOJIS_JSON: &str = include_str!("../../🧬️schema/🔣️entity-emojis.json");

/// 🗄️ The parsed vocabulary, decoded once per process.
static TABLE: OnceLock<EntityEmojiTable> = OnceLock::new();

/// 📥️ Returns the shared emoji table, decoding it on first use.
pub fn entity_emoji_table() -> &'static EntityEmojiTable {
    TABLE.get_or_init(|| serde_json::from_str(ENTITY_EMOJIS_JSON).expect("entity emoji table must parse"))
}

/// 🏷️ Returns the emoji registered for an entity kind, or the empty string.
pub fn entity(name: &str) -> &'static str {
    entity_emoji_table().entities.get(name).map_or("", String::as_str)
}

/// 🗃️ Returns the emoji registered for a collection kind, or the empty string.
pub fn collection(name: &str) -> &'static str {
    entity_emoji_table().collections.get(name).map_or("", String::as_str)
}

/// 😀️ Returns the deduplicated, presentation-normalised entity emoji vocabulary.
pub fn all_entity_emojis() -> Vec<String> {
    let table = entity_emoji_table();
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for name in &table.all_entity_order {
        let raw = table.entities.get(name).or_else(|| table.collections.get(name));
        let Some(raw) = raw else { continue };
        let normalized = emoji_text(raw);
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        result.push(normalized);
    }
    result
}

//#endregion 🔣️EmojiTable

//#region 😀️EmojiCodec

/// 🔤️ The text presentation selector.
const VS15: char = '\u{FE0E}';
/// 🔤️ The emoji presentation selector.
const VS16: char = '\u{FE0F}';

/// 😀️ Normalises an emoji to its presentation form for the text-default code points.
pub fn emoji_text(emoji: &str) -> String {
    let base: String = emoji.chars().filter(|value| *value != VS15 && *value != VS16).collect();
    for text_default in &entity_emoji_table().text_default_emojis {
        if base.contains(text_default.as_str()) {
            return base.replace(text_default.as_str(), &format!("{text_default}{VS16}"));
        }
    }
    base
}

/// 🧲️ Splits the leading emoji grapheme from the remaining text.
pub fn extract_entity_emoji(value: &str) -> (String, String) {
    if value.is_empty() {
        return (String::new(), String::new());
    }
    let runes: Vec<char> = value.chars().collect();
    let mut index = 0usize;
    let first = runes[0];
    if "0123456789#*".contains(first) {
        let mut end = 1usize;
        if end < runes.len() && runes[end] == '\u{FE0F}' {
            end += 1;
        }
        if end < runes.len() && runes[end] == '\u{20E3}' {
            return (runes[..=end].iter().collect(), runes[end + 1..].iter().collect());
        }
        return (String::new(), value.to_string());
    }
    if !is_emoji_rune(first) {
        return (String::new(), value.to_string());
    }
    index += 1;
    if ('\u{1F1E6}'..='\u{1F1FF}').contains(&first)
        && index < runes.len()
        && ('\u{1F1E6}'..='\u{1F1FF}').contains(&runes[index])
    {
        index += 1;
    }
    while index < runes.len() {
        let current = runes[index];
        if current == VS16 || current == VS15 || current == '\u{20E3}' {
            index += 1;
        } else if current == '\u{200D}' {
            index += 1;
            if index < runes.len() {
                index += 1;
                while index < runes.len() && (runes[index] == VS16 || runes[index] == VS15) {
                    index += 1;
                }
            }
        } else if ('\u{1F3FB}'..='\u{1F3FF}').contains(&current) {
            index += 1;
        } else {
            break;
        }
    }
    (runes[..index].iter().collect(), runes[index..].iter().collect())
}

/// 🗺️ The closed code-point span table both implementations share.
const EMOJI_RANGES: &[(u32, u32)] = &[
    (0x00A9, 0x00A9),
    (0x00AE, 0x00AE),
    (0x200D, 0x200D),
    (0x2139, 0x2139),
    (0x2194, 0x2195),
    (0x2196, 0x2199),
    (0x21A9, 0x21AA),
    (0x231A, 0x231B),
    (0x2300, 0x23FF),
    (0x25AA, 0x25AB),
    (0x25B6, 0x25B6),
    (0x25C0, 0x25C0),
    (0x25FB, 0x25FE),
    (0x2600, 0x26FF),
    (0x2700, 0x27BF),
    (0x2934, 0x2935),
    (0x2B05, 0x2B07),
    (0x2B50, 0x2B55),
    (0x3030, 0x3030),
    (0x303D, 0x303D),
    (0x3297, 0x3297),
    (0x3299, 0x3299),
    (0xFE00, 0xFE0F),
    (0x1F1E6, 0x1F1FF),
    (0x1F300, 0x1F5FF),
    (0x1F600, 0x1F64F),
    (0x1F680, 0x1F6FF),
    (0x1F700, 0x1F77F),
    (0x1F780, 0x1F7FF),
    (0x1F800, 0x1F8FF),
    (0x1F900, 0x1F9FF),
    (0x1FA00, 0x1FA6F),
    (0x1FA70, 0x1FAFF),
];

/// 🔷️ Reports whether a character can start or continue an emoji grapheme.
pub fn is_emoji_rune(value: char) -> bool {
    let code = value as u32;
    EMOJI_RANGES.iter().any(|(low, high)| code >= *low && code <= *high)
}

/// 🔤️ Go `Flat` — keeps `A-Za-z0-9` and EVERY rune above `0x7F`, then lower cases what survived.
///
/// The non-ASCII half of that rule is load bearing: a folder named `🖥️app` flattens to `🖥️app`, not
/// to `app`, so an artifact id carries both its kind emoji and the emoji its name already had.
/// Filtering runs before the case fold, exactly as in Go, and the fold is the simple (non
/// special-casing) mapping `strings.ToLower` applies.
pub fn flat(value: &str) -> String {
    value
        .chars()
        .filter(|current| current.is_ascii_alphanumeric() || (*current as u32) > 0x7F)
        .map(|current| {
            let mut lower = current.to_lowercase();
            match (lower.next(), lower.next()) {
                (Some(single), None) => single,
                _ => current,
            }
        })
        .collect()
}

/// 🔠️ Go `strings.ToUpper` — simple (non-special-casing) upper mapping, so `ß` stays `ß`.
fn go_to_upper(text: &str) -> String {
    text.chars()
        .map(|current| {
            let mut upper = current.to_uppercase();
            match (upper.next(), upper.next()) {
                (Some(single), None) => single,
                _ => current,
            }
        })
        .collect()
}

/// 💠️ Splits camel case into `-` boundaries, upper cases, collapses every run outside `[A-Z0-9]`
/// into a single `-` and trims the result. This is the repo's identifier rule: a goal id, a todo
/// id, a ticket slug and a draft id are all this function applied to a human title.
pub fn slugify(text: &str) -> String {
    let runes: Vec<char> = text.chars().collect();
    let mut spaced = String::with_capacity(text.len() + 8);
    for (index, &current) in runes.iter().enumerate() {
        if index > 0 && current.is_ascii_uppercase() {
            let previous = runes[index - 1];
            let leaves_a_word = previous.is_ascii_lowercase();
            let starts_a_word = previous.is_ascii_uppercase() && runes.get(index + 1).is_some_and(char::is_ascii_lowercase);
            if leaves_a_word || starts_a_word {
                spaced.push('-');
            }
        }
        spaced.push(current);
    }
    let upper = go_to_upper(&spaced);
    let mut slug = String::with_capacity(upper.len());
    let mut in_separator = false;
    for character in upper.chars() {
        if character.is_ascii_uppercase() || character.is_ascii_digit() {
            slug.push(character);
            in_separator = false;
        } else if !in_separator {
            slug.push('-');
            in_separator = true;
        }
    }
    slug.trim_matches('-').to_string()
}

//#endregion 😀️EmojiCodec

//#region 🧱️SemanticId

/// 💿️ One emoji-tagged identifier segment.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SemanticId {
    /// 😀️ The segment emoji in presentation form.
    pub emoji: String,
    /// 🔤️ The segment payload.
    pub value: String,
}

impl std::fmt::Display for SemanticId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.is_empty() {
            write!(formatter, "{}", emoji_text(&self.emoji))
        } else {
            write!(formatter, "{}{}", emoji_text(&self.emoji), self.value)
        }
    }
}

/// 🧬️ Splits a compose id into its emoji-tagged segments.
pub fn parse_semantic_ids(compose_id: &str) -> Vec<SemanticId> {
    let mut result: Vec<SemanticId> = Vec::new();
    let mut remaining = compose_id.to_string();
    while !remaining.is_empty() {
        let (emoji, rest) = extract_entity_emoji(&remaining);
        if emoji.is_empty() {
            match result.last_mut() {
                Some(last) => {
                    last.value.push_str(&rest);
                    return result;
                }
                None => return Vec::new(),
            }
        }
        let mut value = String::new();
        let mut cursor = rest;
        while !cursor.is_empty() {
            let (next_emoji, _) = extract_entity_emoji(&cursor);
            if !next_emoji.is_empty() {
                break;
            }
            let mut chars = cursor.chars();
            if let Some(first) = chars.next() {
                value.push(first);
            }
            cursor = chars.collect();
        }
        result.push(SemanticId { emoji: emoji_text(&emoji), value });
        remaining = cursor;
    }
    result
}

/// 🎯️ Converts a filesystem goal path into the emoji compose id.
pub fn goal_path_to_compose_id(goal_path: &str) -> String {
    if goal_path.is_empty() {
        return String::new();
    }
    let prefix = emoji_text(entity("goal"));
    goal_path.split('/').map(|part| format!("{prefix}{}", flat(part))).collect()
}

/// 🛤️ Splits a goal compose id back into its flattened segments.
pub fn compose_id_to_goal_segments(compose_id: &str) -> Vec<String> {
    if compose_id.is_empty() {
        return Vec::new();
    }
    let prefix = emoji_text(entity("goal"));
    let trimmed = compose_id.strip_prefix(&prefix).unwrap_or(compose_id);
    trimmed.split(&prefix).filter(|segment| !segment.is_empty()).map(str::to_string).collect()
}

/// 🤝️ Converts a contributor alias into the emoji compose id.
pub fn contributor_to_compose_id(identifier: &str) -> String {
    if identifier.is_empty() || identifier == "unknown" {
        return identifier.to_string();
    }
    let prefix = emoji_text(entity("contributor"));
    if identifier.starts_with(&prefix) {
        return identifier.to_string();
    }
    format!("{prefix}{}", flat(identifier))
}

/// 🐙️ Converts a contributor compose id back into its flattened alias.
pub fn compose_id_to_contributor_flat(compose_id: &str) -> String {
    if compose_id.is_empty() || compose_id == "unknown" {
        return compose_id.to_string();
    }
    let prefix = emoji_text(entity("contributor"));
    compose_id.strip_prefix(&prefix).unwrap_or(compose_id).to_string()
}

//#endregion 🧱️SemanticId

//#region 🟦️ArtifactRef

/// 🟦️ A parsed reference to a folder, file, section or definition.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArtifactRef {
    /// 🏷️ One of `folder`, `file`, `section` or `definition`.
    pub kind: String,
    /// 🛤️ The normalised artifact path.
    pub path: String,
    /// 🔖️ The section trail, when the reference names one.
    #[serde(rename = "sectionParts", skip_serializing_if = "Vec::is_empty", default)]
    pub section_parts: Vec<String>,
}

/// 🧹️ Rewrites a reference path to forward slashes without a leading `./`.
pub fn normalize_path(value: &str) -> String {
    let mut normalized = value.replace('\\', "/");
    while let Some(stripped) = normalized.strip_prefix("./") {
        normalized = stripped.to_string();
    }
    normalized
}

/// 💾️ Classifies an emoji-prefixed or plain artifact reference.
pub fn parse_artifact_ref(reference: &str) -> ArtifactRef {
    let table = entity_emoji_table();
    let clean: String = reference.chars().filter(|value| *value != VS15 && *value != VS16).collect();
    if let Some(rest) = match_ref_emoji(&clean, &table.section_ref_emojis) {
        let (path, sections) = split_sections(&rest);
        return ArtifactRef { kind: "section".to_string(), path, section_parts: sections };
    }
    if let Some(rest) = match_ref_emoji(&clean, &table.definition_ref_emojis) {
        return ArtifactRef { kind: "definition".to_string(), path: normalize_path(&rest), section_parts: Vec::new() };
    }
    if let Some(rest) = match_ref_emoji(&clean, &table.folder_ref_emojis) {
        let trimmed = rest.strip_suffix('/').unwrap_or(&rest).to_string();
        return ArtifactRef { kind: "folder".to_string(), path: normalize_path(&trimmed), section_parts: Vec::new() };
    }
    if let Some(rest) = match_ref_emoji(&clean, &table.file_ref_emojis) {
        return ArtifactRef { kind: "file".to_string(), path: normalize_path(&rest), section_parts: Vec::new() };
    }
    if reference.contains('#') {
        let (path, sections) = split_sections(reference);
        return ArtifactRef { kind: "section".to_string(), path, section_parts: sections };
    }
    if let Some(trimmed) = reference.strip_suffix('/') {
        return ArtifactRef { kind: "folder".to_string(), path: normalize_path(trimmed), section_parts: Vec::new() };
    }
    ArtifactRef { kind: "file".to_string(), path: normalize_path(reference), section_parts: Vec::new() }
}

/// ✂️ Splits a `path#a#b` reference into its path and section trail.
fn split_sections(value: &str) -> (String, Vec<String>) {
    match value.split_once('#') {
        Some((path, rest)) => (normalize_path(path), rest.split('#').map(str::to_string).collect()),
        None => (normalize_path(value), Vec::new()),
    }
}

/// 🔎️ Returns the remainder after the first matching prefix emoji.
fn match_ref_emoji(clean: &str, emojis: &[String]) -> Option<String> {
    for emoji in emojis {
        let bare: String = emoji.chars().filter(|value| *value != VS15 && *value != VS16).collect();
        if !bare.is_empty() {
            if let Some(rest) = clean.strip_prefix(&bare) {
                return Some(rest.to_string());
            }
        }
    }
    None
}

//#endregion 🟦️ArtifactRef

//#region 🛤️Paths

/// 🧹️ Lexically cleans a slash-separated path the way Go's `filepath.Clean` does.
pub fn clean_path(path: &str) -> String {
    let rooted = path.starts_with('/');
    let mut parts: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if let Some(last) = parts.last() {
                    if *last != ".." {
                        parts.pop();
                        continue;
                    }
                }
                if !rooted {
                    parts.push("..");
                }
            }
            other => parts.push(other),
        }
    }
    let joined = parts.join("/");
    if rooted {
        return format!("/{joined}");
    }
    if joined.is_empty() {
        return ".".to_string();
    }
    joined
}

/// 📁️ The directory part of a path, matching Go's `filepath.Dir`.
pub fn dir_of(path: &str) -> String {
    let cleaned = clean_path(&normalize_path(path));
    match cleaned.rfind('/') {
        Some(0) => "/".to_string(),
        Some(index) => cleaned[..index].to_string(),
        None => ".".to_string(),
    }
}

/// 📄️ The final element of a path, matching Go's `filepath.Base`.
pub fn base_of(path: &str) -> String {
    let normalized = normalize_path(path);
    if normalized.is_empty() {
        return ".".to_string();
    }
    let cleaned = clean_path(&normalized);
    match cleaned.rfind('/') {
        Some(index) if index + 1 < cleaned.len() => cleaned[index + 1..].to_string(),
        Some(_) => "/".to_string(),
        None => cleaned,
    }
}

/// 🔤️ The extension of a path including its dot, matching Go's `filepath.Ext`.
pub fn ext_of(path: &str) -> String {
    let name = base_of(path);
    match name.rfind('.') {
        Some(index) => name[index..].to_string(),
        None => String::new(),
    }
}

//#endregion 🛤️Paths

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_identifiers_are_reproducible() {
        let mut left = SeededEntropy::new(42);
        let mut right = SeededEntropy::new(42);
        let one = Id::new_from(&mut left).unwrap();
        let two = Id::new_from(&mut right).unwrap();
        assert_eq!(one.to_string(), two.to_string());
        assert_eq!(one.to_string().len(), 36);
        assert_eq!(&one.to_string()[14..15], "4");
    }

    #[test]
    fn relative_time_pluralises() {
        assert_eq!(humanize_seconds(1), "1 second ago");
        assert_eq!(humanize_seconds(120), "2 minutes ago");
        assert_eq!(humanize_seconds(-3600), "1 hour from now");
    }

    #[test]
    fn text_default_emojis_get_the_presentation_selector() {
        assert_eq!(emoji_text("⚙"), "⚙\u{FE0F}");
        assert_eq!(emoji_text("🎯\u{FE0F}"), "🎯");
    }

    #[test]
    fn goal_compose_ids_round_trip() {
        let compose = goal_path_to_compose_id("AI-OPTIMIZED-REPO/REPO-CLI");
        assert_eq!(compose_id_to_goal_segments(&compose), vec!["aioptimizedrepo", "repocli"]);
    }

    #[test]
    fn artifact_refs_are_classified_by_prefix() {
        assert_eq!(parse_artifact_ref("💻️a/b.go").kind, "file");
        assert_eq!(parse_artifact_ref("🔖️a/b.go#Header").section_parts, vec!["Header"]);
        assert_eq!(parse_artifact_ref("a/b/").kind, "folder");
    }
}

//#endregion 🧪️Tests
