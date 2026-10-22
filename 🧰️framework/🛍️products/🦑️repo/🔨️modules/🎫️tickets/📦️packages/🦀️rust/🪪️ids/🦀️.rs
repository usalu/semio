//! 🪪️ The dated ticket id and path scheme and the path helpers.

use super::*;

//#region 🪪️Ids

/// 🎆️ The year directory prefix.
pub const EMOJI_YEAR: &str = "🎆️";
/// 🌙️ The month directory prefix.
pub const EMOJI_MONTH: &str = "🌙️";
/// ☀️ The day directory prefix.
pub const EMOJI_DAY: &str = "☀️";
/// 🎫️ The tickets collection directory, under the repository meta directory.
pub const TICKETS_DIR_NAME: &str = "🎫️tickets";
/// 🎫️ The persisted ticket document.
pub const TICKET_DOCUMENT_NAME: &str = "🎫️ticket.json";
/// 📌️ The directory holding the important document of a ticket.
pub const IMPORTANT_DIR_NAME: &str = "📌️important";
/// 📝️ The important document itself. It must be empty for the whole life of an open ticket.
pub const IMPORTANT_DOCUMENT_NAME: &str = "📝️.md";
/// 🚷️ Directory names a ticket walk never descends into.
pub const SKIPPED_WALK_DIRS: [&str; 5] = ["node_modules", "dist", "build", "target", "__pycache__"];

/// 🔢️ Left-pads a number with zeroes to a width.
pub fn pad_number(value: i64, width: usize) -> String {
    let negative = value < 0;
    let digits = value.unsigned_abs().to_string();
    let padding = width.saturating_sub(digits.len() + usize::from(negative));
    let mut text = String::with_capacity(width.max(digits.len() + 1));
    if negative {
        text.push('-');
    }
    for _ in 0..padding {
        text.push('0');
    }
    text.push_str(&digits);
    text
}

/// 🎆️ The emoji-prefixed year directory segment.
pub fn format_year_dir(year: i64) -> String {
    format!("{EMOJI_YEAR}{}", pad_number(year, 2))
}

/// 🌙️ The emoji-prefixed month directory segment.
pub fn format_month_dir(month: i64) -> String {
    format!("{EMOJI_MONTH}{}", pad_number(month, 2))
}

/// ☀️ The emoji-prefixed day directory segment.
pub fn format_day_dir(day: i64) -> String {
    format!("{EMOJI_DAY}{}", pad_number(day, 2))
}

/// 🧭️ Parses one canonical emoji-prefixed date directory segment.
pub fn parse_dated_dir(segment: &str, prefix: &str) -> TicketResult<i64> {
    let Some(value) = segment.strip_prefix(prefix) else {
        return Err(TicketError::invalid(format!("date directory {segment:?} is missing prefix {prefix:?}")));
    };
    value.parse::<i64>().map_err(|_| TicketError::invalid(format!("invalid date directory {segment:?}")))
}

/// 🎫️ The dated identity of a ticket: `YY/MM/DD/SLUG` logically, `🎆️YY/🌙️MM/☀️DD/SLUG` on disk.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketId {
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub slug: String,
}

impl TicketId {
    /// 🆕️ Builds an id from its parts.
    pub fn new(year: i64, month: i64, day: i64, slug: impl Into<String>) -> TicketId {
        TicketId { year, month, day, slug: slug.into() }
    }

    /// 🪪️ The logical id `YY/MM/DD/SLUG`, which is what a dev types and what `repo://ticket/` carries.
    pub fn id(&self) -> String {
        format!("{}/{}/{}/{}", pad_number(self.year, 2), pad_number(self.month, 2), pad_number(self.day, 2), self.slug)
    }

    /// 🗂️ The relative folder path `🎆️YY/🌙️MM/☀️DD/SLUG` under the tickets directory.
    pub fn rel_path(&self) -> String {
        format!("{}/{}/{}/{}", format_year_dir(self.year), format_month_dir(self.month), format_day_dir(self.day), self.slug)
    }

    /// 🌐️ The `repo://ticket/` URI of the ticket.
    pub fn uri(&self) -> String {
        format!("repo://ticket/{}", self.rel_path())
    }

    /// 🧭️ Reads either spelling back. A slug may itself carry `/` for a child ticket.
    pub fn parse(value: &str) -> TicketResult<TicketId> {
        let trimmed = value.trim().trim_matches('/');
        let segments: Vec<&str> = trimmed.split('/').collect();
        if segments.len() < 4 {
            return Err(TicketError::invalid(format!("ticket id {value:?} must be YY/MM/DD/SLUG")));
        }
        let read = |segment: &str, prefix: &str| -> TicketResult<i64> {
            if segment.starts_with(prefix) {
                parse_dated_dir(segment, prefix)
            } else {
                segment.parse::<i64>().map_err(|_| TicketError::invalid(format!("invalid date segment {segment:?}")))
            }
        };
        let year = read(segments[0], EMOJI_YEAR)?;
        let month = read(segments[1], EMOJI_MONTH)?;
        let day = read(segments[2], EMOJI_DAY)?;
        let slug = segments[3..].join("/");
        if slug.is_empty() {
            return Err(TicketError::invalid(format!("ticket id {value:?} carries no slug")));
        }
        Ok(TicketId { year, month, day, slug })
    }

    /// 👪️ The slug of the parent ticket when this one is nested, otherwise nothing.
    pub fn parent_slug(&self) -> Option<String> {
        self.slug.rsplit_once('/').map(|(parent, _)| parent.to_string())
    }
}

/// 🗺️ Every path the ticket domain owns, derived from one repository meta directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TicketLayout {
    repo_meta_dir: String,
}

impl TicketLayout {
    /// 🆕️ Anchors the layout at a repository meta directory (`<root>/.🧬semio/🦑️repo`).
    pub fn new(repo_meta_dir: impl Into<String>) -> TicketLayout {
        TicketLayout { repo_meta_dir: normalize_separators(&repo_meta_dir.into()) }
    }

    /// 🏠️ The meta directory the layout is anchored at.
    pub fn repo_meta_dir(&self) -> &str {
        &self.repo_meta_dir
    }

    /// 🎫️ The tickets collection directory.
    pub fn tickets_dir(&self) -> String {
        join_path(&self.repo_meta_dir, TICKETS_DIR_NAME)
    }

    /// 🎆️ The directory of one year.
    pub fn year_dir(&self, year: i64) -> String {
        join_path(&self.tickets_dir(), &format_year_dir(year))
    }

    /// 🌙️ The directory of one month.
    pub fn month_dir(&self, year: i64, month: i64) -> String {
        join_path(&self.year_dir(year), &format_month_dir(month))
    }

    /// ☀️ The directory of one day.
    pub fn day_dir(&self, year: i64, month: i64, day: i64) -> String {
        join_path(&self.month_dir(year, month), &format_day_dir(day))
    }

    /// 🗂️ The folder of one ticket.
    pub fn ticket_dir(&self, id: &TicketId) -> String {
        join_path(&self.tickets_dir(), &id.rel_path())
    }

    /// 🎫️ The persisted document of one ticket.
    pub fn document_path(&self, id: &TicketId) -> String {
        join_path(&self.ticket_dir(id), TICKET_DOCUMENT_NAME)
    }

    /// 📌️ The important directory of one ticket.
    pub fn important_dir(&self, id: &TicketId) -> String {
        join_path(&self.ticket_dir(id), IMPORTANT_DIR_NAME)
    }

    /// 📝️ The important document of one ticket.
    pub fn important_path(&self, id: &TicketId) -> String {
        join_path(&self.important_dir(id), IMPORTANT_DOCUMENT_NAME)
    }
}

/// ➗️ Replaces every backslash with a forward slash and collapses repeated separators.
pub fn normalize_separators(path: &str) -> String {
    let replaced = path.replace('\\', "/");
    let unc = replaced.starts_with("//") && !replaced.starts_with("///");
    let mut normalized = String::with_capacity(replaced.len());
    let mut previous_separator = false;
    for character in replaced.chars() {
        if character == '/' { if previous_separator { continue; } previous_separator = true; } else { previous_separator = false; }
        normalized.push(character);
    }
    if unc { normalized.insert(0, '/'); }
    if normalized.len() > 1 && normalized.ends_with('/') && normalized != "//" && !drive_root(&normalized) { normalized.pop(); }
    normalized
}

fn drive_root(path: &str) -> bool { let bytes = path.as_bytes(); bytes.len() == 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/' }

/// 🔗️ Joins two path fragments with a single forward slash.
pub fn join_path(base: &str, child: &str) -> String {
    let base = normalize_separators(base);
    let child = normalize_separators(child);
    if base.is_empty() {
        return child;
    }
    if child.is_empty() {
        return base;
    }
    format!("{}/{}", base.trim_end_matches('/'), child.trim_start_matches('/'))
}

/// 📛️ The last segment of a path.
pub fn base_name(path: &str) -> String {
    let normalized = normalize_separators(path);
    normalized.rsplit('/').next().unwrap_or_default().to_string()
}

/// 🗂️ Everything before the last segment of a path.
pub fn dir_name(path: &str) -> String {
    let normalized = normalize_separators(path);
    if drive_root(&normalized) || normalized == "/" || normalized.strip_prefix("//").is_some_and(|value| value.split('/').count() <= 2) { return normalized; }
    match normalized.rsplit_once('/') {
        Some(("", _)) => "/".to_string(),
        Some((parent, _)) if parent.len() == 2 && parent.as_bytes()[0].is_ascii_alphabetic() && parent.ends_with(':') => format!("{parent}/"),
        Some((parent, _)) => parent.to_string(),
        None => ".".to_string(),
    }
}

/// 🔤️ The ticket slug of a title: the repository's camel-case-aware upper-kebab rule.
pub fn ticket_slug_from_title(title: &str) -> TicketResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(TicketError::invalid("ticket title is required"));
    }
    let slug = semio_framework_repo_model::slugify(trimmed);
    if slug.is_empty() {
        return Err(TicketError::invalid("ticket title must contain at least one alphanumeric character"));
    }
    Ok(slug)
}

/// 🎫️ Validates the emoji and title pair a ticket is opened with and returns the derived slug.
pub fn validate_ticket_emoji_title(emoji: &str, title: &str) -> TicketResult<String> {
    let emoji = emoji.trim();
    if emoji.is_empty() {
        return Err(TicketError::invalid("ticket emoji is required"));
    }
    let (extracted, remaining) = semio_framework_repo_identity::extract_entity_emoji(emoji);
    if extracted.is_empty() || !remaining.trim().is_empty() {
        return Err(TicketError::invalid("ticket emoji must be a single emoji character"));
    }
    ticket_slug_from_title(title)
}

//#endregion 🪪️Ids
