//! 🎫️ The ticket domain of the semio repository: the dated id and path scheme, the
//! `🎫️ticket.json` document codec, the open/close/reopen/change lifecycle, the important-document
//! transaction, artifact purging, search, plan/spec source resolution and issue synchronisation.
//!
//! Every filesystem touch goes through [`TicketStore`], every issue mutation through the
//! `IssueTracker` port of `🧩️providers`, every clock read through [`Clock`] and every emission
//! through [`EventSink`], so the whole lifecycle runs in memory with nothing on disk and no `gh`
//! on the machine — see `🧪️tests/🔓️open-close-reopen-lifecycle`.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/AGENTS.md (§5.6, §8.10 — the ticket algebra and id rule)

use semio_framework_repo_model::{Interaction, InteractionFile, Ticket, TicketAgent, TicketManagementData, TicketPlan, TicketStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔁️Reexports

/// 🔁️ The provider shapes a client of this crate needs in order to call it.
///
/// A client may not name `semio-framework-repo-providers` itself — a generated test host links only
/// the crate under test — so everything the ticket API takes or returns is re-exported here
/// explicitly, per the repository's "reexport explicitly if the client needs it" rule.
pub use semio_framework_repo_providers::{IssueTracker, ManagementIssue, ManagementLabel, ManagementMilestone, ManagementProviders, McpClientKind, NullManagementProvider, ProviderError, ProviderResult};

//#endregion 🔁️Reexports

//#region ❌️Errors

/// ⚠️ Every ticket failure, carrying a class for machine consumers and the Go message verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketError {
    pub class: String,
    pub message: String,
}

impl TicketError {
    /// 🆕️ Builds an error of a class.
    pub fn new(class: &str, message: impl Into<String>) -> TicketError {
        TicketError { class: class.to_string(), message: message.into() }
    }

    /// 🚫️ An input the caller may correct.
    pub fn invalid(message: impl Into<String>) -> TicketError {
        TicketError::new("invalid", message)
    }

    /// 🔍️ A ticket, document or node that is not there.
    pub fn not_found(message: impl Into<String>) -> TicketError {
        TicketError::new("not-found", message)
    }

    /// 💥️ A store operation that failed.
    pub fn store(message: impl Into<String>) -> TicketError {
        TicketError::new("store", message)
    }

    /// 🔀️ A state that forbids the requested transition.
    pub fn conflict(message: impl Into<String>) -> TicketError {
        TicketError::new("conflict", message)
    }
}

impl std::fmt::Display for TicketError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for TicketError {}

/// 🎯️ The result of every operation in this crate.
pub type TicketResult<T> = Result<T, TicketError>;

//#endregion ❌️Errors

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
    let mut normalized = String::with_capacity(replaced.len());
    let mut previous_separator = false;
    for character in replaced.chars() {
        if character == '/' {
            if previous_separator {
                continue;
            }
            previous_separator = true;
        } else {
            previous_separator = false;
        }
        normalized.push(character);
    }
    if normalized.len() > 1 && normalized.ends_with('/') {
        normalized.pop();
    }
    normalized
}

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
    match normalized.rsplit_once('/') {
        Some(("", _)) => "/".to_string(),
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

//#region 🔤️GoJson

/// 🔤️ Encodes one string the way Go's `encoding/json` does, HTML escaping included.
///
/// Divergence note: `<`, `>` and `&` become `\u003c`, `\u003e` and `\u0026` because Go's encoder
/// sets `SetEscapeHTML(true)` by default, which is observable in the committed documents — see
/// `🎆️25/🌙️11/☀️24/LOG-SYSTEM/🎫️ticket.json`, whose title is stored as `"\u003ee-"`.
pub fn go_json_string(value: &str) -> String {
    let mut text = String::with_capacity(value.len() + 2);
    text.push('"');
    for character in value.chars() {
        match character {
            '"' => text.push_str("\\\""),
            '\\' => text.push_str("\\\\"),
            '\n' => text.push_str("\\n"),
            '\r' => text.push_str("\\r"),
            '\t' => text.push_str("\\t"),
            '<' => text.push_str("\\u003c"),
            '>' => text.push_str("\\u003e"),
            '&' => text.push_str("\\u0026"),
            '\u{2028}' => text.push_str("\\u2028"),
            '\u{2029}' => text.push_str("\\u2029"),
            control if (control as u32) < 0x20 => text.push_str(&format!("\\u{:04x}", control as u32)),
            other => text.push(other),
        }
    }
    text.push('"');
    text
}

/// 🧱️ One member of a Go-shaped JSON object, in declaration order.
enum GoMember {
    Text(&'static str, String),
    Object(&'static str, Vec<GoMember>),
    Strings(&'static str, Vec<String>),
}

/// 🖨️ Renders members the way `json.MarshalIndent(value, "", "  ")` renders a struct.
fn render_members(members: &[GoMember], depth: usize) -> String {
    if members.is_empty() {
        return "{}".to_string();
    }
    let indent = "  ".repeat(depth + 1);
    let closing = "  ".repeat(depth);
    let mut lines: Vec<String> = Vec::with_capacity(members.len());
    for member in members {
        match member {
            GoMember::Text(name, value) => lines.push(format!("{indent}{}: {}", go_json_string(name), go_json_string(value))),
            GoMember::Object(name, nested) => lines.push(format!("{indent}{}: {}", go_json_string(name), render_members(nested, depth + 1))),
            GoMember::Strings(name, values) => {
                if values.is_empty() {
                    lines.push(format!("{indent}{}: []", go_json_string(name)));
                } else {
                    let element_indent = "  ".repeat(depth + 2);
                    let elements: Vec<String> = values.iter().map(|value| format!("{element_indent}{}", go_json_string(value))).collect();
                    lines.push(format!("{indent}{}: [\n{}\n{indent}]", go_json_string(name), elements.join(",\n")));
                }
            }
        }
    }
    format!("{{\n{}\n{closing}}}", lines.join(",\n"))
}

//#endregion 🔤️GoJson

//#region 📄️Codec

/// 🧺️ Decodes every element of an array member, skipping anything that is not an object.
fn decode_array<T>(value: Option<&Value>, decode: fn(&serde_json::Map<String, Value>) -> T) -> Vec<T> {
    let Some(Value::Array(entries)) = value else { return Vec::new() };
    entries.iter().filter_map(Value::as_object).map(decode).collect()
}

/// 🔤️ A text member, or the empty string when it is absent or not a string. This is Go's zero value.
fn text_of(members: &serde_json::Map<String, Value>, name: &str) -> String {
    members.get(name).and_then(Value::as_str).unwrap_or_default().to_string()
}

/// 🟫️ Decodes one interaction leniently: every absent member takes its zero value, as in Go.
fn decode_interaction(members: &serde_json::Map<String, Value>) -> Interaction {
    Interaction {
        kind: text_of(members, "kind"),
        date: text_of(members, "date"),
        author: text_of(members, "author"),
        system: text_of(members, "system"),
        client: text_of(members, "client"),
        checkpoint: text_of(members, "checkpoint"),
        prompt: text_of(members, "prompt"),
        summary: text_of(members, "summary"),
        llm: text_of(members, "llm"),
        effort: text_of(members, "effort"),
        files: decode_array(members.get("files"), |file| InteractionFile { path: text_of(file, "path"), id: text_of(file, "id"), uri: text_of(file, "uri") }),
    }
}

/// 🔳️ Decodes one agent record leniently, plan included only when it decodes cleanly.
fn decode_agent(members: &serde_json::Map<String, Value>) -> TicketAgent {
    TicketAgent {
        session: text_of(members, "session"),
        contributor: text_of(members, "contributor"),
        system: text_of(members, "system"),
        client: text_of(members, "client"),
        llm: text_of(members, "llm"),
        effort: text_of(members, "effort"),
        transcript: text_of(members, "transcript"),
        plan: members.get("plan").and_then(|value| serde_json::from_value(value.clone()).ok()),
    }
}

/// 📖️ Decodes one `🎫️ticket.json` document.
///
/// The status member is mandatory and closed: a document without one, or with any spelling other
/// than `open`/`closed`, is refused rather than defaulted. Three pre-schema spellings are absorbed:
/// a `prompt` member fills an absent `description`, a `sessions` array of agent objects contributes
/// its session ids, and a closing interaction's summary fills an absent `summary`.
///
/// Members the shape does not declare (`note`, `closedAt`, `files`, `contributors`, …) are read and
/// dropped; re-encoding a document therefore loses them. That is the Go behaviour verbatim and it is
/// exercised by `🧪️tests/📄️ticket-document-codec`.
pub fn decode_ticket_document(text: &str) -> TicketResult<Ticket> {
    let value: Value = serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))?;
    let Value::Object(members) = value else {
        return Err(TicketError::invalid("ticket document must be a JSON object"));
    };
    let text_member = |name: &str| -> String { members.get(name).and_then(Value::as_str).unwrap_or_default().to_string() };
    let status = match members.get("status").and_then(Value::as_str) {
        Some("open") => TicketStatus::Open,
        Some("closed") => TicketStatus::Closed,
        _ => return Err(TicketError::invalid("ticket status must be explicitly \"open\" or \"closed\"")),
    };
    let management = match members.get("github") {
        Some(Value::Object(_)) => Some(TicketManagementData { issue: members["github"].get("issue").and_then(Value::as_str).unwrap_or_default().to_string() }),
        _ => None,
    };
    let plan = match members.get("plan") {
        Some(value @ Value::Object(_)) => Some(TicketPlan {
            client: value.get("client").and_then(Value::as_str).unwrap_or_default().to_string(),
            id: value.get("id").and_then(Value::as_str).unwrap_or_default().to_string(),
            source: value.get("source").and_then(Value::as_str).unwrap_or_default().to_string(),
            local: value.get("local").and_then(Value::as_str).unwrap_or_default().to_string(),
        }),
        _ => None,
    };
    let interactions: Vec<Interaction> = decode_array(members.get("interactions"), decode_interaction);
    let mut agents: Vec<TicketAgent> = decode_array(members.get("agents"), decode_agent);

    let mut ticket = Ticket {
        year: 0,
        month: 0,
        day: 0,
        slug: String::new(),
        title: text_member("title"),
        emoji: text_member("emoji"),
        status,
        description: text_member("description"),
        summary: text_member("summary"),
        management,
        goal: text_member("goal"),
        parent: text_member("parent"),
        plan,
        sessions: Vec::new(),
        interactions,
        agents: Vec::new(),
        folder_path: String::new(),
        json_path: String::new(),
        important_path: String::new(),
    };
    if ticket.description.is_empty() {
        ticket.description = text_member("prompt");
    }
    match members.get("sessions") {
        Some(Value::Array(entries)) if entries.iter().all(Value::is_string) => {
            for entry in entries {
                append_session_id(&mut ticket, entry.as_str().unwrap_or_default());
            }
        }
        Some(value @ Value::Array(_)) => {
            let legacy = decode_array(Some(value), decode_agent);
            for agent in &legacy {
                append_session_id(&mut ticket, &agent.session);
            }
            if agents.is_empty() {
                agents = legacy;
            }
        }
        _ => {}
    }
    ticket.agents = agents;
    if ticket.summary.is_empty() {
        for interaction in ticket.interactions.iter().rev() {
            if is_ticket_interaction_kind(&interaction.kind, "ticket.close") && !interaction.summary.is_empty() {
                ticket.summary = interaction.summary.clone();
                break;
            }
        }
    }
    let agent_sessions: Vec<String> = ticket.agents.iter().map(|agent| agent.session.clone()).collect();
    for session in agent_sessions {
        append_session_id(&mut ticket, &session);
    }
    Ok(ticket)
}

/// 🖨️ Encodes one `🎫️ticket.json` document, member order and omission rules included.
///
/// Divergence note, faithfully reproduced: `parent` is READ by the decoder but never written, because
/// the Go struct tags it `json:"-"` while the decoding alias declares `parent,omitempty`. A ticket
/// whose parent is set therefore loses it on the next save.
pub fn encode_ticket_document(ticket: &Ticket) -> String {
    let mut members: Vec<GoMember> = Vec::with_capacity(9);
    members.push(GoMember::Text("title", ticket.title.clone()));
    if !ticket.emoji.is_empty() {
        members.push(GoMember::Text("emoji", ticket.emoji.clone()));
    }
    members.push(GoMember::Text("status", ticket.status.as_str().to_string()));
    if !ticket.description.is_empty() {
        members.push(GoMember::Text("description", ticket.description.clone()));
    }
    if !ticket.summary.is_empty() {
        members.push(GoMember::Text("summary", ticket.summary.clone()));
    }
    if let Some(management) = &ticket.management {
        let mut nested: Vec<GoMember> = Vec::with_capacity(1);
        if !management.issue.is_empty() {
            nested.push(GoMember::Text("issue", management.issue.clone()));
        }
        members.push(GoMember::Object("github", nested));
    }
    if !ticket.goal.is_empty() {
        members.push(GoMember::Text("goal", ticket.goal.clone()));
    }
    if let Some(plan) = &ticket.plan {
        let mut nested: Vec<GoMember> = Vec::with_capacity(4);
        if !plan.client.is_empty() {
            nested.push(GoMember::Text("client", plan.client.clone()));
        }
        if !plan.id.is_empty() {
            nested.push(GoMember::Text("id", plan.id.clone()));
        }
        if !plan.source.is_empty() {
            nested.push(GoMember::Text("source", plan.source.clone()));
        }
        if !plan.local.is_empty() {
            nested.push(GoMember::Text("local", plan.local.clone()));
        }
        members.push(GoMember::Object("plan", nested));
    }
    if !ticket.sessions.is_empty() {
        members.push(GoMember::Strings("sessions", ticket.sessions.clone()));
    }
    render_members(&members, 0)
}

/// ➕️ Appends a session id once, ignoring blanks and duplicates.
pub fn append_session_id(ticket: &mut Ticket, session_id: &str) {
    let session_id = session_id.trim();
    if session_id.is_empty() || ticket.sessions.iter().any(|existing| existing == session_id) {
        return;
    }
    ticket.sessions.push(session_id.to_string());
}

/// 🔍️ Whether an interaction kind names an operation, with or without its `.ended` suffix.
pub fn is_ticket_interaction_kind(kind: &str, expected: &str) -> bool {
    let kind = kind.trim();
    let expected = expected.trim();
    kind == expected || kind.strip_suffix(".ended").is_some_and(|stem| stem == expected)
}

//#endregion 📄️Codec

//#region 🗄️Store

/// 🧱️ What a node in the store is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeKind {
    File,
    Directory,
    Symlink,
}

/// 📏️ What the store knows about one node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub kind: NodeKind,
    pub size: u64,
    pub mode: u32,
}

/// 📇️ One child of a directory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub kind: NodeKind,
}

/// 🗄️ Every filesystem touch the ticket domain performs.
///
/// Paths are forward-slash strings; an implementation converts to the platform spelling itself.
pub trait TicketStore {
    /// 🔍️ What is at a path, without following a symlink.
    fn metadata(&self, path: &str) -> Option<NodeMetadata>;
    /// 📖️ Reads a document as text.
    fn read(&self, path: &str) -> TicketResult<String>;
    /// ✍️ Writes a document, creating parent directories.
    fn write(&self, path: &str, content: &str) -> TicketResult<()>;
    /// ✨️ Creates an empty file, failing when anything already occupies the path.
    fn create_exclusive(&self, path: &str, mode: u32) -> TicketResult<()>;
    /// 🗂️ Creates a directory and every missing ancestor.
    fn create_directory(&self, path: &str) -> TicketResult<()>;
    /// 🗑️ Removes one file.
    fn remove_file(&self, path: &str) -> TicketResult<()>;
    /// 🗑️ Removes one directory, which must be empty.
    fn remove_directory(&self, path: &str) -> TicketResult<()>;
    /// 🧨️ Removes a whole subtree.
    fn remove_tree(&self, path: &str) -> TicketResult<()>;
    /// 🚚️ Moves a node.
    fn rename(&self, from: &str, to: &str) -> TicketResult<()>;
    /// 📇️ Lists the children of a directory, sorted by name.
    fn entries(&self, path: &str) -> TicketResult<Vec<DirectoryEntry>>;

    /// ✔️ Whether anything is at a path.
    fn exists(&self, path: &str) -> bool {
        self.metadata(path).is_some()
    }

    /// 🗂️ Whether a directory is at a path.
    fn is_directory(&self, path: &str) -> bool {
        matches!(self.metadata(path), Some(NodeMetadata { kind: NodeKind::Directory, .. }))
    }

    /// 📄️ Whether a regular file is at a path.
    fn is_file(&self, path: &str) -> bool {
        matches!(self.metadata(path), Some(NodeMetadata { kind: NodeKind::File, .. }))
    }
}

/// 🧠️ One node of the in-memory store.
#[derive(Debug, Clone)]
struct MemoryNode {
    kind: NodeKind,
    content: String,
    mode: u32,
}

/// 🧠️ A whole ticket tree in memory, with failure injection, so a test never touches `.🧬semio`.
#[derive(Debug, Default)]
pub struct MemoryTicketStore {
    nodes: RefCell<BTreeMap<String, MemoryNode>>,
    write_failures: RefCell<BTreeMap<String, String>>,
}

impl MemoryTicketStore {
    /// 🆕️ An empty store.
    pub fn new() -> MemoryTicketStore {
        MemoryTicketStore::default()
    }

    /// 💣️ Makes the next and every following write to a path fail with a message.
    pub fn fail_write(&self, path: &str, message: &str) {
        self.write_failures.borrow_mut().insert(normalize_separators(path), message.to_string());
    }

    /// 🧹️ Lifts an injected failure.
    pub fn clear_failure(&self, path: &str) {
        self.write_failures.borrow_mut().remove(&normalize_separators(path));
    }

    /// 📸️ Every path in the store with its content, for a projection.
    pub fn snapshot(&self) -> BTreeMap<String, String> {
        self.nodes
            .borrow()
            .iter()
            .filter(|(_, node)| node.kind == NodeKind::File)
            .map(|(path, node)| (path.clone(), node.content.clone()))
            .collect()
    }

    /// 📸️ Every path in the store, files and directories alike.
    pub fn paths(&self) -> Vec<String> {
        self.nodes.borrow().keys().cloned().collect()
    }

    /// 🌱️ Seeds one file, creating its ancestors.
    pub fn seed_file(&self, path: &str, content: &str) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::File, content: content.to_string(), mode: 0o644 });
    }

    /// 🌱️ Seeds one file of a size, without materialising its bytes beyond the recorded length.
    pub fn seed_sized_file(&self, path: &str, size: u64) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path.clone(), MemoryNode { kind: NodeKind::File, content: String::new(), mode: 0o644 });
        if let Some(node) = self.nodes.borrow_mut().get_mut(&path) {
            node.content = "\0".repeat(usize::try_from(size).unwrap_or(usize::MAX));
        }
    }

    /// 🌱️ Seeds one directory.
    pub fn seed_directory(&self, path: &str) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::Directory, content: String::new(), mode: 0o755 });
    }

    /// 🔗️ Seeds a symlink, which every transaction guard must refuse.
    pub fn seed_symlink(&self, path: &str) {
        let path = normalize_separators(path);
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::Symlink, content: String::new(), mode: 0o777 });
    }

    fn ensure_ancestors(&self, path: &str) {
        let mut current = dir_name(path);
        let mut pending: Vec<String> = Vec::new();
        while current != "." && current != "/" && !current.is_empty() {
            if self.nodes.borrow().contains_key(&current) {
                break;
            }
            pending.push(current.clone());
            let parent = dir_name(&current);
            if parent == current {
                break;
            }
            current = parent;
        }
        for directory in pending.into_iter().rev() {
            self.nodes.borrow_mut().insert(directory, MemoryNode { kind: NodeKind::Directory, content: String::new(), mode: 0o755 });
        }
    }

    fn injected(&self, path: &str) -> Option<String> {
        self.write_failures.borrow().get(path).cloned()
    }
}

impl TicketStore for MemoryTicketStore {
    fn metadata(&self, path: &str) -> Option<NodeMetadata> {
        let path = normalize_separators(path);
        self.nodes.borrow().get(&path).map(|node| NodeMetadata { kind: node.kind, size: node.content.len() as u64, mode: node.mode })
    }

    fn read(&self, path: &str) -> TicketResult<String> {
        let path = normalize_separators(path);
        match self.nodes.borrow().get(&path) {
            Some(node) if node.kind == NodeKind::File => Ok(node.content.clone()),
            Some(_) => Err(TicketError::store(format!("{path} is not a regular file"))),
            None => Err(TicketError::not_found(format!("{path} does not exist"))),
        }
    }

    fn write(&self, path: &str, content: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        if let Some(message) = self.injected(&path) {
            return Err(TicketError::store(message));
        }
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::File, content: content.to_string(), mode: 0o644 });
        Ok(())
    }

    fn create_exclusive(&self, path: &str, mode: u32) -> TicketResult<()> {
        let path = normalize_separators(path);
        if let Some(message) = self.injected(&path) {
            return Err(TicketError::store(message));
        }
        if self.nodes.borrow().contains_key(&path) {
            return Err(TicketError::conflict(format!("{path} already exists")));
        }
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::File, content: String::new(), mode });
        Ok(())
    }

    fn create_directory(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        if let Some(message) = self.injected(&path) {
            return Err(TicketError::store(message));
        }
        if let Some(existing) = self.nodes.borrow().get(&path) {
            return if existing.kind == NodeKind::Directory { Ok(()) } else { Err(TicketError::conflict(format!("{path} is not a directory"))) };
        }
        self.ensure_ancestors(&path);
        self.nodes.borrow_mut().insert(path, MemoryNode { kind: NodeKind::Directory, content: String::new(), mode: 0o755 });
        Ok(())
    }

    fn remove_file(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        match self.nodes.borrow_mut().remove(&path) {
            Some(_) => Ok(()),
            None => Err(TicketError::not_found(format!("{path} does not exist"))),
        }
    }

    fn remove_directory(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        let prefix = format!("{path}/");
        if self.nodes.borrow().keys().any(|candidate| candidate.starts_with(&prefix)) {
            return Err(TicketError::conflict(format!("{path} is not empty")));
        }
        match self.nodes.borrow_mut().remove(&path) {
            Some(_) => Ok(()),
            None => Err(TicketError::not_found(format!("{path} does not exist"))),
        }
    }

    fn remove_tree(&self, path: &str) -> TicketResult<()> {
        let path = normalize_separators(path);
        let prefix = format!("{path}/");
        let doomed: Vec<String> = self.nodes.borrow().keys().filter(|candidate| **candidate == path || candidate.starts_with(&prefix)).cloned().collect();
        let mut nodes = self.nodes.borrow_mut();
        for candidate in doomed {
            nodes.remove(&candidate);
        }
        Ok(())
    }

    fn rename(&self, from: &str, to: &str) -> TicketResult<()> {
        let from = normalize_separators(from);
        let to = normalize_separators(to);
        if let Some(message) = self.injected(&to) {
            return Err(TicketError::store(message));
        }
        if !self.nodes.borrow().contains_key(&from) {
            return Err(TicketError::not_found(format!("{from} does not exist")));
        }
        if self.nodes.borrow().contains_key(&to) {
            return Err(TicketError::conflict(format!("{to} already exists")));
        }
        let prefix = format!("{from}/");
        let moved: Vec<String> = self.nodes.borrow().keys().filter(|candidate| **candidate == from || candidate.starts_with(&prefix)).cloned().collect();
        self.ensure_ancestors(&to);
        let mut nodes = self.nodes.borrow_mut();
        for candidate in moved {
            let Some(node) = nodes.remove(&candidate) else { continue };
            let target = if candidate == from { to.clone() } else { format!("{to}/{}", &candidate[prefix.len()..]) };
            nodes.insert(target, node);
        }
        Ok(())
    }

    fn entries(&self, path: &str) -> TicketResult<Vec<DirectoryEntry>> {
        let path = normalize_separators(path);
        if !self.is_directory(&path) {
            return Err(TicketError::not_found(format!("{path} is not a directory")));
        }
        let prefix = format!("{path}/");
        let mut listed: BTreeSet<DirectoryEntry> = BTreeSet::new();
        for (candidate, node) in self.nodes.borrow().iter() {
            let Some(rest) = candidate.strip_prefix(&prefix) else { continue };
            if rest.contains('/') {
                continue;
            }
            listed.insert(DirectoryEntry { name: rest.to_string(), kind: node.kind });
        }
        Ok(listed.into_iter().collect())
    }
}

/// 💽️ The real filesystem.
#[derive(Debug, Clone, Default)]
pub struct FileTicketStore;

impl FileTicketStore {
    /// 🆕️ The store rooted at the machine's filesystem.
    pub fn new() -> FileTicketStore {
        FileTicketStore
    }
}

impl TicketStore for FileTicketStore {
    fn metadata(&self, path: &str) -> Option<NodeMetadata> {
        let metadata = std::fs::symlink_metadata(path).ok()?;
        let kind = if metadata.file_type().is_symlink() {
            NodeKind::Symlink
        } else if metadata.is_dir() {
            NodeKind::Directory
        } else {
            NodeKind::File
        };
        Some(NodeMetadata { kind, size: metadata.len(), mode: file_mode(&metadata) })
    }

    fn read(&self, path: &str) -> TicketResult<String> {
        std::fs::read_to_string(path).map_err(|error| TicketError::store(format!("read {path}: {error}")))
    }

    fn write(&self, path: &str, content: &str) -> TicketResult<()> {
        let parent = dir_name(path);
        if parent != "." && parent != "/" {
            std::fs::create_dir_all(&parent).map_err(|error| TicketError::store(format!("create {parent}: {error}")))?;
        }
        std::fs::write(path, content).map_err(|error| TicketError::store(format!("write {path}: {error}")))
    }

    fn create_exclusive(&self, path: &str, _mode: u32) -> TicketResult<()> {
        std::fs::OpenOptions::new().write(true).create_new(true).open(path).map(|_| ()).map_err(|error| TicketError::store(format!("create {path}: {error}")))
    }

    fn create_directory(&self, path: &str) -> TicketResult<()> {
        std::fs::create_dir_all(path).map_err(|error| TicketError::store(format!("create {path}: {error}")))
    }

    fn remove_file(&self, path: &str) -> TicketResult<()> {
        std::fs::remove_file(path).map_err(|error| TicketError::store(format!("remove {path}: {error}")))
    }

    fn remove_directory(&self, path: &str) -> TicketResult<()> {
        std::fs::remove_dir(path).map_err(|error| TicketError::store(format!("remove {path}: {error}")))
    }

    fn remove_tree(&self, path: &str) -> TicketResult<()> {
        match std::fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(TicketError::store(format!("remove {path}: {error}"))),
        }
    }

    fn rename(&self, from: &str, to: &str) -> TicketResult<()> {
        std::fs::rename(from, to).map_err(|error| TicketError::store(format!("rename {from} -> {to}: {error}")))
    }

    fn entries(&self, path: &str) -> TicketResult<Vec<DirectoryEntry>> {
        let reader = std::fs::read_dir(path).map_err(|error| TicketError::store(format!("read directory {path}: {error}")))?;
        let mut listed: Vec<DirectoryEntry> = Vec::new();
        for entry in reader {
            let entry = entry.map_err(|error| TicketError::store(format!("read directory {path}: {error}")))?;
            let file_type = entry.file_type().map_err(|error| TicketError::store(format!("read directory {path}: {error}")))?;
            let kind = if file_type.is_symlink() {
                NodeKind::Symlink
            } else if file_type.is_dir() {
                NodeKind::Directory
            } else {
                NodeKind::File
            };
            listed.push(DirectoryEntry { name: entry.file_name().to_string_lossy().to_string(), kind });
        }
        listed.sort();
        Ok(listed)
    }
}

#[cfg(unix)]
fn file_mode(metadata: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    metadata.mode()
}

#[cfg(not(unix))]
fn file_mode(metadata: &std::fs::Metadata) -> u32 {
    if metadata.permissions().readonly() {
        0o444
    } else {
        0o644
    }
}

//#endregion 🗄️Store

//#region 💾️Important

/// 📓️ One recorded step of an important-document transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalStep {
    pub op: String,
    pub path: String,
    pub outcome: String,
}

/// 📓️ The ordered record of every step a transaction attempted, in the order it attempted them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionJournal {
    pub steps: Vec<JournalStep>,
}

impl TransactionJournal {
    /// ➕️ Records one step.
    pub fn record(&mut self, op: &str, path: &str, outcome: &str) {
        self.steps.push(JournalStep { op: op.to_string(), path: path.to_string(), outcome: outcome.to_string() });
    }

    /// 🔤️ The step sequence as `op:outcome` pairs, for a compact projection.
    pub fn trace(&self) -> Vec<String> {
        self.steps.iter().map(|step| format!("{}:{}", step.op, step.outcome)).collect()
    }
}

/// 🖼️ The exact empty-document bundle a close must find and may restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportantPreimage {
    pub path: String,
    pub dir: String,
    pub mode: u32,
}

/// ✨️ What a reopen created, so a failed save removes exactly that and nothing else.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportantCreation {
    pub path: String,
    pub dir: String,
    pub file_created: bool,
    pub dir_created: bool,
}

/// 🔎️ Requires the canonical important document to be an exact empty regular-file bundle.
pub fn inspect_important_document<S: TicketStore>(store: &S, path: &str, journal: &mut TransactionJournal) -> TicketResult<ImportantPreimage> {
    let dir = dir_name(path);
    let Some(metadata) = store.metadata(path) else {
        journal.record("inspect", path, "failed");
        return Err(TicketError::not_found(format!("cannot finish ticket: required important document {path} is unavailable")));
    };
    if metadata.kind != NodeKind::File {
        journal.record("inspect", path, "failed");
        return Err(TicketError::invalid(format!("cannot finish ticket: important document {path} is not a regular file")));
    }
    let content = store.read(path)?;
    if !content.is_empty() {
        journal.record("inspect", path, "failed");
        return Err(TicketError::invalid(format!("cannot finish ticket: important document {path} is not empty")));
    }
    let entries = store.entries(&dir)?;
    let expected = base_name(path);
    if entries.len() != 1 || entries[0].name != expected {
        journal.record("inspect", &dir, "failed");
        return Err(TicketError::invalid(format!("cannot finish ticket: important directory {dir} must contain only {expected}")));
    }
    journal.record("inspect", path, "ok");
    Ok(ImportantPreimage { path: path.to_string(), dir, mode: metadata.mode })
}

/// 🔄️ Restores an empty document without overwriting an existing node.
pub fn restore_important_document<S: TicketStore>(store: &S, preimage: &ImportantPreimage, journal: &mut TransactionJournal) -> TicketResult<()> {
    store.create_directory(&preimage.dir)?;
    store.create_exclusive(&preimage.path, preimage.mode & 0o777)?;
    journal.record("restore", &preimage.path, "ok");
    Ok(())
}

/// 🗑️ Removes the exact empty document bundle, restoring it when the directory will not go.
pub fn remove_important_document<S: TicketStore>(store: &S, preimage: &ImportantPreimage, journal: &mut TransactionJournal) -> TicketResult<()> {
    store.remove_file(&preimage.path)?;
    journal.record("remove-file", &preimage.path, "ok");
    if let Err(error) = store.remove_directory(&preimage.dir) {
        journal.record("remove-dir", &preimage.dir, "failed");
        return match restore_important_document(store, preimage, journal) {
            Ok(()) => Err(TicketError::store(format!("remove important directory: {error}"))),
            Err(restore) => Err(TicketError::store(format!("remove important directory: {error}; restore important document: {restore}"))),
        };
    }
    journal.record("remove-dir", &preimage.dir, "ok");
    Ok(())
}

/// ✨️ Preserves an existing regular document or exclusively creates an empty one.
pub fn ensure_important_document<S: TicketStore>(store: &S, path: &str, journal: &mut TransactionJournal) -> TicketResult<ImportantCreation> {
    let dir = dir_name(path);
    let mut creation = ImportantCreation { path: path.to_string(), dir: dir.clone(), file_created: false, dir_created: false };
    if let Some(metadata) = store.metadata(path) {
        if metadata.kind != NodeKind::File {
            journal.record("ensure", path, "failed");
            return Err(TicketError::invalid(format!("important document {path} is not a regular file")));
        }
        store.read(path)?;
        journal.record("ensure", path, "preserved");
        return Ok(creation);
    }
    match store.metadata(&dir) {
        Some(metadata) if metadata.kind == NodeKind::Directory => {}
        Some(_) => {
            journal.record("ensure-dir", &dir, "failed");
            return Err(TicketError::invalid(format!("important directory {dir} is not a physical directory")));
        }
        None => {
            store.create_directory(&dir)?;
            creation.dir_created = true;
            journal.record("ensure-dir", &dir, "created");
        }
    }
    if let Err(error) = store.create_exclusive(path, 0o644) {
        if creation.dir_created {
            let _ = store.remove_directory(&dir);
        }
        journal.record("ensure", path, "failed");
        return Err(error);
    }
    creation.file_created = true;
    journal.record("ensure", path, "created");
    Ok(creation)
}

/// ↩️ Removes only the nodes the current reopen created.
pub fn rollback_important_creation<S: TicketStore>(store: &S, creation: &ImportantCreation, journal: &mut TransactionJournal) -> TicketResult<()> {
    if creation.file_created && store.exists(&creation.path) {
        store.remove_file(&creation.path)?;
        journal.record("rollback-file", &creation.path, "ok");
    }
    if creation.dir_created && store.exists(&creation.dir) {
        store.remove_directory(&creation.dir)?;
        journal.record("rollback-dir", &creation.dir, "ok");
    }
    Ok(())
}

//#endregion 💾️Important

//#region 🗺️PlanSource

/// 🗺️ Where a plan or spec id resolves to, and whether it names a directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanSource {
    pub path: String,
    pub is_directory: bool,
}

/// 🏠️ The two roots plan resolution reads: the repository and the contributor's home directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanRoots {
    pub repo_root: String,
    pub home_dir: String,
}

/// 🏷️ The value stored in `ticket.json` for the plan attachment of a client.
pub fn plan_client_tag(kind: McpClientKind) -> &'static str {
    match kind {
        McpClientKind::Cursor => "cursor",
        McpClientKind::Kiro => "kiro",
        McpClientKind::Copilot => "copilot",
        McpClientKind::Claude => "claude",
        McpClientKind::Codex => "codex",
        McpClientKind::Generic => "generic",
    }
}

/// 🗺️ Resolves a plan or spec id to a store path, per IDE.
///
/// Cursor globs `<root>/.cursor/plans/*_<id>.plan.md` and refuses an ambiguous match; Kiro requires
/// the directory `<root>/.kiro/specs/<id>`; Copilot, Claude and Codex each name one file under the
/// contributor's home directory. The generic surface carries no plan attachment at all.
pub fn resolve_plan_source<S: TicketStore>(store: &S, roots: &PlanRoots, kind: McpClientKind, id: &str) -> TicketResult<PlanSource> {
    let id = id.trim();
    if id.is_empty() {
        return Err(TicketError::invalid("plan or spec id is empty"));
    }
    let root = roots.repo_root.trim();
    if root.is_empty() {
        return Err(TicketError::invalid("repository root is not set"));
    }
    match kind {
        McpClientKind::Cursor => {
            let directory = join_path(root, ".cursor/plans");
            let suffix = format!("_{id}.plan.md");
            let matches: Vec<String> = store
                .entries(&directory)
                .unwrap_or_default()
                .into_iter()
                .filter(|entry| entry.kind == NodeKind::File && entry.name.ends_with(&suffix))
                .map(|entry| join_path(&directory, &entry.name))
                .collect();
            match matches.len() {
                0 => Err(TicketError::not_found(format!("no Cursor plan matches id {id:?} (glob {directory}/*{suffix})"))),
                1 => Ok(PlanSource { path: matches[0].clone(), is_directory: false }),
                count => Err(TicketError::conflict(format!("ambiguous Cursor plan id {id:?}: {count} matches"))),
            }
        }
        McpClientKind::Kiro => {
            let directory = join_path(root, &format!(".kiro/specs/{id}"));
            match store.metadata(&directory) {
                Some(metadata) if metadata.kind == NodeKind::Directory => Ok(PlanSource { path: directory, is_directory: true }),
                Some(_) => Err(TicketError::invalid(format!("Kiro spec {id:?} is not a directory"))),
                None => Err(TicketError::not_found(format!("Kiro spec {id:?}: no such directory"))),
            }
        }
        McpClientKind::Copilot | McpClientKind::Claude | McpClientKind::Codex => {
            let home = roots.home_dir.trim();
            if home.is_empty() {
                return Err(TicketError::invalid("home directory is not set"));
            }
            let repo_base = base_name(root);
            let path = match kind {
                McpClientKind::Copilot => join_path(home, &format!(".copilot/projects/{repo_base}/memory/{id}.md")),
                McpClientKind::Claude => join_path(home, &format!(".claude/plans/{id}.md")),
                _ => join_path(home, &format!(".codex/memory/{repo_base}/{id}.md")),
            };
            match store.metadata(&path) {
                Some(metadata) if metadata.kind == NodeKind::Directory => Err(TicketError::invalid(format!("expected file at {path}"))),
                Some(_) => Ok(PlanSource { path, is_directory: false }),
                None => Err(TicketError::not_found(format!("plan file for id {id:?}: no such file"))),
            }
        }
        McpClientKind::Generic => Err(TicketError::invalid(format!("plan or spec attachment is not supported for mcp kind {:?}", kind.as_str()))),
    }
}

/// 📎️ Resolves `plan_id` or `spec_id` and attaches it to a ticket before it is saved.
pub fn apply_ticket_plan_from_ids<S: TicketStore>(store: &S, roots: &PlanRoots, ticket: &mut Ticket, kind: McpClientKind, plan_id: &str, spec_id: &str) -> TicketResult<()> {
    let plan_id = plan_id.trim();
    let spec_id = spec_id.trim();
    if plan_id.is_empty() && spec_id.is_empty() {
        return Ok(());
    }
    if !plan_id.is_empty() && !spec_id.is_empty() {
        return Err(TicketError::invalid("pass only one of plan_id or spec_id"));
    }
    let id = if kind == McpClientKind::Kiro {
        if !plan_id.is_empty() {
            return Err(TicketError::invalid("use spec_id for Kiro, not plan_id"));
        }
        spec_id
    } else {
        if !spec_id.is_empty() {
            return Err(TicketError::invalid("use plan_id for this client, not spec_id"));
        }
        plan_id
    };
    let source = resolve_plan_source(store, roots, kind, id)?;
    ticket.plan = Some(TicketPlan { client: plan_client_tag(kind).to_string(), id: id.to_string(), source: source.path, local: String::new() });
    Ok(())
}

/// 📦️ Moves an attached plan or spec into the ticket folder as the ticket closes.
pub fn move_ticket_plan_into_folder<S: TicketStore>(store: &S, ticket: &mut Ticket) -> TicketResult<()> {
    let Some(plan) = ticket.plan.clone() else { return Ok(()) };
    if plan.source.trim().is_empty() {
        return Ok(());
    }
    if ticket.folder_path.is_empty() {
        return Err(TicketError::invalid("ticket folder path is empty"));
    }
    let source = normalize_separators(&plan.source);
    let destination_name = base_name(&source);
    let destination = join_path(&ticket.folder_path, &destination_name);
    if !store.exists(&source) {
        if store.exists(&destination) {
            if let Some(plan) = ticket.plan.as_mut() {
                plan.local = destination_name;
                plan.source = String::new();
            }
            return Ok(());
        }
        return Err(TicketError::not_found(format!("plan source {source:?} missing and destination {destination:?} not found")));
    }
    store.rename(&source, &destination)?;
    if let Some(plan) = ticket.plan.as_mut() {
        plan.local = destination_name;
        plan.source = String::new();
    }
    Ok(())
}

/// 📝️ Strips YAML frontmatter from plan markdown.
pub fn strip_plan_frontmatter(content: &str) -> String {
    if !content.starts_with("---") {
        return content.trim().to_string();
    }
    let Some(end) = content[3..].find("\n---") else { return content.trim().to_string() };
    let mut rest = &content[3 + end + 4..];
    if let Some(stripped) = rest.strip_prefix("\r\n") {
        rest = stripped;
    } else if let Some(stripped) = rest.strip_prefix('\n') {
        rest = stripped;
    }
    rest.trim().to_string()
}

/// 📝️ Formats one plan file as a collapsible GitHub markdown section.
pub fn format_plan_file_section(name: &str, raw: &str) -> String {
    let body = strip_plan_frontmatter(raw);
    if body.trim().is_empty() {
        return String::new();
    }
    format!("<details>\n<summary>{name}</summary>\n\n{body}\n\n</details>")
}

/// 📝️ Builds the issue comment body of a bound plan or spec.
pub fn format_plan_comment<S: TicketStore>(store: &S, source: &str) -> TicketResult<String> {
    let source = source.trim();
    if source.is_empty() {
        return Ok(String::new());
    }
    let Some(metadata) = store.metadata(source) else {
        return Err(TicketError::not_found(format!("plan source {source} does not exist")));
    };
    let mut sections: Vec<String> = Vec::new();
    if metadata.kind == NodeKind::Directory {
        let mut names: Vec<String> = store.entries(source)?.into_iter().filter(|entry| entry.kind == NodeKind::File && entry.name.ends_with(".md")).map(|entry| entry.name).collect();
        names.sort();
        for name in names {
            let section = format_plan_file_section(&name, &store.read(&join_path(source, &name))?);
            if !section.is_empty() {
                sections.push(section);
            }
        }
    } else {
        let section = format_plan_file_section(&base_name(source), &store.read(source)?);
        if !section.is_empty() {
            sections.push(section);
        }
    }
    if sections.is_empty() {
        return Ok(String::new());
    }
    Ok(format!("# 📋️ Plan\n\n{}", sections.join("\n\n")))
}

//#endregion 🗺️PlanSource

//#region 📐️FileScope

/// 📈️ The lines a diff added and removed in one file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffLines {
    pub added: Vec<i64>,
    pub removed: Vec<i64>,
}

/// 🌳️ The git ref of the staging index. Ticket close diffs the index against the working tree only.
pub const GIT_INDEX_REF: &str = ":0";

/// 🔒️ Everything the ticket domain asks a version-control system.
pub trait VersionControl {
    /// 📨️ The unified diff of a path set, produced with zero context lines.
    fn unified_diff(&self, base: &str, head: &str, paths: &[String]) -> TicketResult<String>;
    /// 🏷️ The name-status diff of a path set.
    fn name_status(&self, base: &str, head: &str, paths: &[String]) -> TicketResult<String>;
    /// 🚫️ Whether a path is excluded by the ignore rules.
    fn is_ignored(&self, path: &str) -> bool;
}

/// 🎞️ A recorded version-control transcript, so a ticket case never needs a git checkout.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordedVersionControl {
    #[serde(default)]
    pub unified_diff: String,
    #[serde(default)]
    pub name_status: String,
    #[serde(default)]
    pub ignored: Vec<String>,
}

impl VersionControl for RecordedVersionControl {
    fn unified_diff(&self, _base: &str, _head: &str, _paths: &[String]) -> TicketResult<String> {
        Ok(self.unified_diff.clone())
    }

    fn name_status(&self, _base: &str, _head: &str, _paths: &[String]) -> TicketResult<String> {
        Ok(self.name_status.clone())
    }

    fn is_ignored(&self, path: &str) -> bool {
        self.ignored.iter().any(|candidate| candidate == path)
    }
}

/// 📨️ Parses a `git diff -U0` hunk stream into the added and removed line numbers per file.
pub fn parse_diff_lines(stdout: &str) -> BTreeMap<String, DiffLines> {
    let mut result: BTreeMap<String, DiffLines> = BTreeMap::new();
    let mut current = String::new();
    for line in stdout.split('\n') {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                current = parts[1].strip_prefix("b/").unwrap_or(parts[1]).to_string();
                result.entry(current.clone()).or_default();
            }
        } else if let Some(rest) = line.strip_prefix("+++ b/") {
            current = rest.to_string();
            result.entry(current.clone()).or_default();
        } else if line.starts_with("@@") && !current.is_empty() {
            let Some(hunk) = parse_hunk_header(line) else { continue };
            let entry = result.entry(current.clone()).or_default();
            for offset in 0..hunk.old_count {
                entry.removed.push(hunk.old_start + offset);
            }
            for offset in 0..hunk.new_count {
                entry.added.push(hunk.new_start + offset);
            }
        }
    }
    result
}

struct HunkHeader {
    old_start: i64,
    old_count: i64,
    new_start: i64,
    new_count: i64,
}

fn parse_hunk_header(line: &str) -> Option<HunkHeader> {
    let rest = line.strip_prefix("@@")?.trim_start();
    let mut parts = rest.split_whitespace();
    let old = parts.next()?.strip_prefix('-')?;
    let new = parts.next()?.strip_prefix('+')?;
    let (old_start, old_count) = parse_range(old)?;
    let (new_start, new_count) = parse_range(new)?;
    Some(HunkHeader { old_start, old_count, new_start, new_count })
}

fn parse_range(value: &str) -> Option<(i64, i64)> {
    match value.split_once(',') {
        Some((start, count)) => Some((start.parse().ok()?, count.parse().ok()?)),
        None => Some((value.parse().ok()?, 1)),
    }
}

/// 🧹️ Whether a path is excluded from every ticket file scope regardless of ignore rules.
pub fn is_repo_excluded_path(path: &str) -> bool {
    let normalized = normalize_separators(path.trim()).trim_start_matches("./").to_string();
    if normalized.is_empty() {
        return false;
    }
    let under = |root: &str| normalized == root || normalized.starts_with(&format!("{root}/"));
    if under(".🧬semio") || under("assets/repo") || normalized.contains("/asset/repo/") {
        return true;
    }
    if under("node_modules") || normalized.contains("/node_modules/") {
        return true;
    }
    if under(".git") || normalized.contains("/.git/") {
        return true;
    }
    for segment in ["/dist/", "/build/", "/target/", "/__pycache__/", "/.next/", "/coverage/"] {
        if normalized.contains(segment) {
            return true;
        }
    }
    base_name(&normalized).ends_with(".Designer.cs") || normalized.contains("/codegen/")
}

/// 📝️ Normalises one file identifier a close request carries into a repository-relative path.
///
/// The Go twin additionally resolves a bare file artifact id through the codebase index. That
/// lookup belongs to `🗂️codebase`; here a value with no separator and no extension is passed
/// through unchanged, which is what the Go fallback does when the index holds no match.
pub fn normalize_ticket_file_input(file_path: &str) -> String {
    let normalized = file_path.trim();
    if normalized.is_empty() {
        return String::new();
    }
    if let Some(rest) = normalized.strip_prefix("file://") {
        return normalize_separators(rest).trim_start_matches("./").to_string();
    }
    normalize_separators(normalized).trim_start_matches("./").to_string()
}

/// 💿️ Normalises and de-duplicates the file identifiers of a close request, preserving order.
pub fn normalize_ticket_file_inputs(files: &[String]) -> Vec<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut filtered: Vec<String> = Vec::with_capacity(files.len());
    for file_path in files {
        let normalized = normalize_ticket_file_input(file_path);
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        filtered.push(normalized);
    }
    filtered
}

/// 🗺️ Drops the files that live inside the ticket's own folder — a ticket never contributes itself.
pub fn filter_ticket_workspace_files(folder_path: &str, files: &[String]) -> Vec<String> {
    let relative = normalize_separators(folder_path).trim_start_matches("./").to_string();
    if relative.is_empty() {
        return files.to_vec();
    }
    files
        .iter()
        .filter(|file_path| {
            let normalized = normalize_separators(file_path).trim_start_matches("./").to_string();
            normalized != relative && !normalized.starts_with(&format!("{relative}/"))
        })
        .cloned()
        .collect()
}

/// 📐️ The file scope a ticket closes with, together with the diff of every file in it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketFileScope {
    pub files: Vec<String>,
    pub excluded: Vec<String>,
    pub diff_lines: BTreeMap<String, DiffLines>,
    pub statuses: Vec<String>,
}

/// 🎫️ Computes the file scope of a ticket close through the version-control port.
pub fn compute_ticket_file_scope<V: VersionControl>(version_control: &V, folder_path: &str, files: &[String]) -> TicketResult<TicketFileScope> {
    let normalized = normalize_ticket_file_inputs(files);
    let workspace = filter_ticket_workspace_files(folder_path, &normalized);
    let mut kept: Vec<String> = Vec::with_capacity(workspace.len());
    let mut excluded: Vec<String> = Vec::new();
    for file_path in workspace {
        if is_repo_excluded_path(&file_path) || version_control.is_ignored(&file_path) {
            excluded.push(file_path);
        } else {
            kept.push(file_path);
        }
    }
    if kept.is_empty() {
        return Err(TicketError::invalid("at least one file is required"));
    }
    let statuses: Vec<String> = version_control.name_status(GIT_INDEX_REF, "", &kept)?.lines().filter(|line| !line.trim().is_empty()).map(str::to_string).collect();
    let diff_lines = parse_diff_lines(&version_control.unified_diff(GIT_INDEX_REF, "", &kept)?);
    Ok(TicketFileScope { files: kept, excluded, diff_lines, statuses })
}

/// 📪️ Whether a ticket may be closed, with the reasons it may not.
pub fn can_close_ticket(ticket: Option<&Ticket>) -> (bool, Vec<String>) {
    match ticket {
        None => (false, vec!["Ticket data is nil".to_string()]),
        Some(_) => (true, Vec::new()),
    }
}

//#endregion 📐️FileScope

//#region 🐙️IssueSync

/// 🤖️ The prompt section heading of an issue body or comment.
pub fn format_prompt_heading(body: &str) -> String {
    if body.is_empty() {
        "# 🤖️ Prompt".to_string()
    } else {
        format!("# 🤖️ Prompt\n\n{body}")
    }
}

/// 🔍️ The summary section heading of a closing comment.
pub fn format_summary_heading(body: &str) -> String {
    if body.is_empty() {
        "# 🔍️ Summary".to_string()
    } else {
        format!("# 🔍️ Summary\n\n{body}")
    }
}

/// 🐙️ Creates, links or reopens the issue of a ticket, returning the warnings it swallowed.
///
/// A management failure is never fatal to the ticket: the Go twin logs and continues, and so does
/// this, which is why the warnings come back as data instead of as an error.
pub fn ensure_ticket_issue<T: IssueTracker>(tracker: &T, ticket: &mut Ticket, title: &str, prompt: &str, issue: &str, milestone: Option<i64>, reopen_if_closed: bool) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();
    let issue = issue.trim();
    if !issue.is_empty() {
        ticket.management = Some(TicketManagementData { issue: issue.to_string() });
        return warnings;
    }
    if let Some(existing) = ticket.management.clone().filter(|management| !management.issue.is_empty()) {
        if reopen_if_closed {
            match tracker.get_issue_details(&existing.issue) {
                Ok(Some(remote)) if remote.state.eq_ignore_ascii_case("closed") => {
                    if let Err(error) = tracker.reopen_issue(&existing.issue) {
                        warnings.push(format!("reopen github issue: {}", error.message));
                    }
                }
                Ok(_) => {}
                Err(error) => warnings.push(format!("read github issue: {}", error.message)),
            }
        }
        return warnings;
    }
    match tracker.create_issue(title, &format_prompt_heading(prompt), milestone) {
        Ok(url) if url.trim().is_empty() => warnings.push("github issue create returned empty url".to_string()),
        Ok(url) => ticket.management = Some(TicketManagementData { issue: url }),
        Err(error) => warnings.push(format!("Failed to create GitHub issue: {}", error.message)),
    }
    warnings
}

/// 📪️ Comments the summary onto the issue, labels it with the touched bundles and closes it.
pub fn close_ticket_issue<T: IssueTracker>(tracker: &T, ticket: &Ticket, summary: &str, labels: &[String], bulk: bool) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();
    let Some(management) = ticket.management.as_ref().filter(|management| !management.issue.is_empty()) else { return warnings };
    let issue_url = management.issue.as_str();
    if !bulk {
        if !labels.is_empty() {
            if let Err(error) = tracker.add_labels(issue_url, labels) {
                warnings.push(format!("Failed to add labels to GitHub issue: {}", error.message));
            }
        }
        if let Err(error) = tracker.add_comment(issue_url, &format_summary_heading(summary)) {
            warnings.push(format!("Failed to add summary and metrics comment to GitHub issue: {}", error.message));
        }
    }
    if let Err(error) = tracker.close_issue(issue_url) {
        warnings.push(format!("Failed to close GitHub issue: {}", error.message));
    }
    warnings
}

/// 🎯️ Looks a milestone number up by title, swallowing a lookup failure the way the Go twin does.
pub fn milestone_number_for_title<T: IssueTracker>(tracker: &T, title: &str) -> Option<i64> {
    tracker.find_milestone_by_title(title).ok().flatten().map(|milestone| milestone.number)
}


/// 🎞️ What a recorded issue tracker answers, so an issue-synchronisation case needs no `gh`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueTrackerScript {
    /// 🔗️ The URL `create_issue` returns. An empty string reproduces the empty-url defect.
    #[serde(default)]
    pub created_issue_url: String,
    /// 🔎️ The state `get_issue_details` reports, or empty for an issue that cannot be read.
    #[serde(default)]
    pub issue_state: String,
    /// 🎯️ The milestone number `find_milestone_by_title` answers for the ticket's goal.
    #[serde(default)]
    pub milestone: Option<i64>,
    /// 🎯️ The title that milestone carries.
    #[serde(default)]
    pub milestone_title: String,
    /// 💣️ Method names that fail instead of answering.
    #[serde(default)]
    pub failures: Vec<String>,
}

/// 🎞️ A fixture-driven issue tracker that records every interaction in the order it happened.
///
/// It is deliberately a port-level recording rather than a process transcript: the GitHub provider
/// writes its comment bodies to a temporary file whose name carries the process id, so a recorded
/// argv could never match twice. What a ticket owes its issue tracker is WHICH calls it makes, in
/// which order, with which arguments — and that is exactly what this records.
#[derive(Debug, Default)]
pub struct RecordedIssueTracker {
    script: IssueTrackerScript,
    calls: RefCell<Vec<String>>,
}

impl RecordedIssueTracker {
    /// 🆕️ Binds a tracker to a script.
    pub fn new(script: IssueTrackerScript) -> RecordedIssueTracker {
        RecordedIssueTracker { script, calls: RefCell::new(Vec::new()) }
    }

    /// 📥️ Reads a script from JSON text.
    pub fn from_json(text: &str) -> TicketResult<RecordedIssueTracker> {
        serde_json::from_str(text).map(RecordedIssueTracker::new).map_err(|error| TicketError::invalid(error.to_string()))
    }

    /// 📼️ Every interaction so far, in order, as `method|argument|argument`.
    pub fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    fn record(&self, call: String) {
        self.calls.borrow_mut().push(call);
    }

    fn fails(&self, method: &str) -> Option<ProviderError> {
        self.script.failures.iter().any(|candidate| candidate == method).then(|| ProviderError::new(format!("{method} failed")))
    }
}

impl IssueTracker for RecordedIssueTracker {
    fn create_issue(&self, title: &str, body: &str, milestone: Option<i64>) -> Result<String, ProviderError> {
        self.record(format!("create_issue|{title}|{body}|{}", milestone.map(|number| number.to_string()).unwrap_or_default()));
        match self.fails("create_issue") {
            Some(error) => Err(error),
            None => Ok(self.script.created_issue_url.clone()),
        }
    }

    fn close_issue(&self, issue_url: &str) -> Result<(), ProviderError> {
        self.record(format!("close_issue|{issue_url}"));
        self.fails("close_issue").map_or(Ok(()), Err)
    }

    fn reopen_issue(&self, issue_url: &str) -> Result<(), ProviderError> {
        self.record(format!("reopen_issue|{issue_url}"));
        self.fails("reopen_issue").map_or(Ok(()), Err)
    }

    fn get_issue_details(&self, issue_url: &str) -> Result<Option<ManagementIssue>, ProviderError> {
        self.record(format!("get_issue_details|{issue_url}"));
        if let Some(error) = self.fails("get_issue_details") {
            return Err(error);
        }
        if self.script.issue_state.is_empty() {
            return Ok(None);
        }
        Ok(Some(ManagementIssue { url: issue_url.to_string(), state: self.script.issue_state.clone(), ..ManagementIssue::default() }))
    }

    fn add_comment(&self, issue_url: &str, comment: &str) -> Result<(), ProviderError> {
        self.record(format!("add_comment|{issue_url}|{comment}"));
        self.fails("add_comment").map_or(Ok(()), Err)
    }

    fn add_labels(&self, issue_url: &str, labels: &[String]) -> Result<(), ProviderError> {
        self.record(format!("add_labels|{issue_url}|{}", labels.join(",")));
        self.fails("add_labels").map_or(Ok(()), Err)
    }

    fn remove_labels(&self, issue_url: &str, labels: &[String]) -> Result<(), ProviderError> {
        self.record(format!("remove_labels|{issue_url}|{}", labels.join(",")));
        self.fails("remove_labels").map_or(Ok(()), Err)
    }

    fn find_milestone_by_title(&self, title: &str) -> Result<Option<ManagementMilestone>, ProviderError> {
        self.record(format!("find_milestone_by_title|{title}"));
        if let Some(error) = self.fails("find_milestone_by_title") {
            return Err(error);
        }
        Ok(self.script.milestone.map(|number| ManagementMilestone { number, title: self.script.milestone_title.clone(), ..ManagementMilestone::default() }))
    }

    fn list_repo_labels(&self) -> Result<Vec<ManagementLabel>, ProviderError> {
        self.record("list_repo_labels".to_string());
        self.fails("list_repo_labels").map_or(Ok(Vec::new()), Err)
    }
}


/// 🐙️ What one issue synchronisation left behind: the link the ticket now carries and the warnings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueSyncOutcome {
    pub issue: String,
    pub warnings: Vec<String>,
}

/// 🐙️ Synchronises the issue of a ticket that is being opened or reopened, without a ticket in hand.
///
/// The ticket-shaped entry point is [`ensure_ticket_issue`]; this one takes the two members it
/// actually reads, so a caller that has only the link — a test host, a CLI verb — needs no model type.
pub fn sync_open_issue<T: IssueTracker>(tracker: &T, existing_issue: &str, title: &str, prompt: &str, issue: &str, milestone: Option<i64>, reopen_if_closed: bool) -> IssueSyncOutcome {
    let mut ticket = Ticket {
        year: 0,
        month: 0,
        day: 0,
        slug: String::new(),
        title: title.to_string(),
        emoji: String::new(),
        status: TicketStatus::Open,
        description: String::new(),
        summary: String::new(),
        management: (!existing_issue.is_empty()).then(|| TicketManagementData { issue: existing_issue.to_string() }),
        goal: String::new(),
        parent: String::new(),
        plan: None,
        sessions: Vec::new(),
        interactions: Vec::new(),
        agents: Vec::new(),
        folder_path: String::new(),
        json_path: String::new(),
        important_path: String::new(),
    };
    let warnings = ensure_ticket_issue(tracker, &mut ticket, title, prompt, issue, milestone, reopen_if_closed);
    IssueSyncOutcome { issue: ticket.management.map(|management| management.issue).unwrap_or_default(), warnings }
}

/// 📪️ Synchronises the issue of a ticket that is being closed, without a ticket in hand.
pub fn sync_close_issue<T: IssueTracker>(tracker: &T, issue_url: &str, summary: &str, labels: &[String], bulk: bool) -> Vec<String> {
    let ticket = Ticket {
        year: 0,
        month: 0,
        day: 0,
        slug: String::new(),
        title: String::new(),
        emoji: String::new(),
        status: TicketStatus::Closed,
        description: String::new(),
        summary: String::new(),
        management: (!issue_url.is_empty()).then(|| TicketManagementData { issue: issue_url.to_string() }),
        goal: String::new(),
        parent: String::new(),
        plan: None,
        sessions: Vec::new(),
        interactions: Vec::new(),
        agents: Vec::new(),
        folder_path: String::new(),
        json_path: String::new(),
        important_path: String::new(),
    };
    close_ticket_issue(tracker, &ticket, summary, labels, bulk)
}

//#endregion 🐙️IssueSync

//#region ⏰️Ports

/// ⏰️ The clock the lifecycle reads, so a case pins every timestamp.
pub trait Clock {
    /// 📅️ Today as `(YY, MM, DD)`, the two-digit year the folder scheme uses.
    fn today(&self) -> (i64, i64, i64);
    /// 🕰️ The interaction timestamp, formatted `YYYY-MM-DD HH:MM:SS`.
    fn stamp(&self) -> String;
}

/// 📌️ A clock frozen at one instant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedClock {
    pub date: (i64, i64, i64),
    pub timestamp: String,
}

impl FixedClock {
    /// 🆕️ Freezes a clock at a date and a timestamp.
    pub fn new(year: i64, month: i64, day: i64, timestamp: impl Into<String>) -> FixedClock {
        FixedClock { date: (year, month, day), timestamp: timestamp.into() }
    }
}

impl Clock for FixedClock {
    fn today(&self) -> (i64, i64, i64) {
        self.date
    }

    fn stamp(&self) -> String {
        self.timestamp.clone()
    }
}

/// 📡️ One emitted event: the kind, the source and the payload as JSON text.
///
/// The payload is carried as text on purpose — no serialisation type from outside this codebase
/// appears anywhere in the public surface of this crate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedEvent {
    pub kind: String,
    pub source: String,
    pub payload: String,
}

/// 📡️ Where the lifecycle sends its events.
pub trait EventSink {
    /// 📤️ Emits one event, its payload already encoded. Delivery failures are never fatal.
    fn emit(&self, kind: &str, source: &str, payload: &str);
}

/// 📼️ An in-memory sink that keeps every event in emission order.
#[derive(Debug, Default)]
pub struct RecordingEventSink {
    events: RefCell<Vec<RecordedEvent>>,
}

impl RecordingEventSink {
    /// 🆕️ An empty sink.
    pub fn new() -> RecordingEventSink {
        RecordingEventSink::default()
    }

    /// 📼️ Everything emitted so far, in order.
    pub fn recorded(&self) -> Vec<RecordedEvent> {
        self.events.borrow().clone()
    }
}

impl EventSink for RecordingEventSink {
    fn emit(&self, kind: &str, source: &str, payload: &str) {
        self.events.borrow_mut().push(RecordedEvent { kind: kind.to_string(), source: source.to_string(), payload: payload.to_string() });
    }
}

/// 🌐️ The sink that posts to the repository coordinator through `📡️events`.
#[derive(Debug, Clone, Default)]
pub struct CoordinatorEventSink;

impl EventSink for CoordinatorEventSink {
    fn emit(&self, kind: &str, source: &str, payload: &str) {
        let Ok(value) = serde_json::from_str::<Value>(payload) else { return };
        semio_framework_repo_events::emit(kind, source, &value);
    }
}

/// 🕰️ The host wall clock, the production [`Clock`] of the ticket lifecycle.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn today(&self) -> (i64, i64, i64) {
        let (year, month, day) = civil_from_days(unix_seconds() / 86_400);
        (year % 100, month, day)
    }

    fn stamp(&self) -> String {
        let seconds = unix_seconds();
        let (year, month, day) = civil_from_days(seconds / 86_400);
        let rest = seconds.rem_euclid(86_400);
        format!("{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}", rest / 3600, (rest % 3600) / 60, rest % 60)
    }
}

/// 🕰️ The host wall clock in whole seconds since the Unix epoch.
fn unix_seconds() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|value| value.as_secs() as i64).unwrap_or_default()
}

/// 📅️ Howard Hinnant's days-since-epoch to civil-date conversion.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 { shifted } else { shifted - 146_096 } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = if month_prime < 10 { month_prime + 3 } else { month_prime - 9 };
    (year_of_era + era * 400 + i64::from(month <= 2), month, day)
}

/// 🚫️ The tracker used when the caller asked for no management traffic: every call succeeds and
/// reports nothing, so a lifecycle runs unchanged with the network switched off.
#[derive(Debug, Default)]
pub struct NullIssueTracker;

impl IssueTracker for NullIssueTracker {
    fn create_issue(&self, _title: &str, _body: &str, _milestone: Option<i64>) -> ProviderResult<String> {
        Ok(String::new())
    }

    fn close_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }

    fn reopen_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }

    fn get_issue_details(&self, _issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        Ok(None)
    }

    fn add_comment(&self, _issue_url: &str, _comment: &str) -> ProviderResult<()> {
        Ok(())
    }

    fn add_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }

    fn remove_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }

    fn find_milestone_by_title(&self, _title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        Ok(None)
    }

    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        Ok(Vec::new())
    }
}

/// 🏷️ The event source every ticket operation carries.
pub const EVENT_SOURCE: &str = "repo-cli";

//#endregion ⏰️Ports

//#region 🔓️Lifecycle

/// 📬️ Everything an open request carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketOpenRequest {
    pub emoji: String,
    pub title: String,
    pub prompt: String,
    #[serde(default)]
    pub llm: String,
    #[serde(default)]
    pub effort: String,
    pub client: String,
    pub goal: String,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub no_issue: bool,
    #[serde(default)]
    pub no_management: bool,
    #[serde(default)]
    pub issue: String,
    #[serde(default)]
    pub session: String,
    #[serde(default)]
    pub plan_id: String,
    #[serde(default)]
    pub spec_id: String,
}

/// 📪️ Everything a close request carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketCloseRequest {
    pub id: String,
    pub summary: String,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub no_management: bool,
    #[serde(default)]
    pub bulk: bool,
}

/// 🔓️ Everything a reopen request carries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketReopenRequest {
    pub id: String,
    pub prompt: String,
    #[serde(default)]
    pub llm: String,
    #[serde(default)]
    pub effort: String,
    pub client: String,
    #[serde(default)]
    pub goal: String,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub no_management: bool,
    #[serde(default)]
    pub session: String,
    #[serde(default)]
    pub plan_id: String,
    #[serde(default)]
    pub spec_id: String,
}

/// ♻️ Everything a change request carries. An absent member leaves the ticket's value alone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketChangeRequest {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub goal: Option<String>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub no_management: bool,
}

/// 🎫️ The outcome of one lifecycle operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketOutcome {
    pub id: String,
    pub rel_path: String,
    pub status: String,
    pub document: String,
    #[serde(default, skip_serializing_if = "TransactionJournal::is_empty")]
    pub journal: TransactionJournal,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

impl TransactionJournal {
    /// 🈳️ Whether nothing was recorded.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// 🧹️ What an artifact purge removed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeReport {
    pub removed_directories: Vec<String>,
    pub removed_files: Vec<String>,
}

/// 🧹️ A file above this size is purged from a closed ticket folder.
pub const OVERSIZED_FILE_BYTES: u64 = 5 << 20;
/// 🧹️ A subfolder whose files total above this size is purged from a closed ticket folder.
pub const OVERSIZED_FOLDER_BYTES: u64 = 10 << 20;

/// 🎫️ The ticket domain bound to one store, one issue tracker, one clock and one event sink.
pub struct TicketService<'a, S: TicketStore, T: IssueTracker, C: Clock, E: EventSink> {
    pub layout: TicketLayout,
    pub store: &'a S,
    pub tracker: &'a T,
    pub clock: &'a C,
    pub events: &'a E,
    pub roots: PlanRoots,
    pub mcp_client: McpClientKind,
}

impl<'a, S: TicketStore, T: IssueTracker, C: Clock, E: EventSink> TicketService<'a, S, T, C, E> {
    /// 🆕️ Binds the domain to its ports.
    pub fn new(layout: TicketLayout, store: &'a S, tracker: &'a T, clock: &'a C, events: &'a E) -> TicketService<'a, S, T, C, E> {
        TicketService { layout, store, tracker, clock, events, roots: PlanRoots { repo_root: String::new(), home_dir: String::new() }, mcp_client: McpClientKind::Generic }
    }

    /// 🗺️ Sets the roots plan resolution reads.
    pub fn with_roots(mut self, roots: PlanRoots) -> Self {
        self.roots = roots;
        self
    }

    /// 🪪️ Sets the MCP surface a plan or spec id is resolved against.
    pub fn with_mcp_client(mut self, mcp_client: McpClientKind) -> Self {
        self.mcp_client = mcp_client;
        self
    }

    /// 📖️ Reads one ticket, filling in every derived path.
    pub fn read(&self, id: &TicketId) -> TicketResult<Ticket> {
        let document_path = self.layout.document_path(id);
        if !self.store.exists(&document_path) {
            return Err(TicketError::not_found(format!("ticket not found: {document_path}")));
        }
        let mut ticket = decode_ticket_document(&self.store.read(&document_path)?)?;
        self.hydrate(&mut ticket, id);
        Ok(ticket)
    }

    /// 💾️ Writes one ticket back, refusing a status the vocabulary does not carry.
    pub fn save(&self, ticket: &Ticket) -> TicketResult<()> {
        if ticket.json_path.is_empty() {
            return Err(TicketError::invalid("ticket json path is empty"));
        }
        self.store.write(&ticket.json_path, &encode_ticket_document(ticket))
    }

    /// ▪️ Every ticket, optionally narrowed to a year, a month and a day.
    pub fn list(&self, year: Option<i64>, month: Option<i64>, day: Option<i64>) -> TicketResult<Vec<Ticket>> {
        let tickets_dir = self.layout.tickets_dir();
        if !self.store.exists(&tickets_dir) {
            return Ok(Vec::new());
        }
        let mut tickets: Vec<Ticket> = Vec::new();
        for year_value in self.dated_children(&tickets_dir, EMOJI_YEAR, year) {
            let year_path = join_path(&tickets_dir, &format_year_dir(year_value));
            for month_value in self.dated_children(&year_path, EMOJI_MONTH, month) {
                let month_path = join_path(&year_path, &format_month_dir(month_value));
                for day_value in self.dated_children(&month_path, EMOJI_DAY, day) {
                    let day_path = join_path(&month_path, &format_day_dir(day_value));
                    for slug in self.slugs_under(&day_path, "") {
                        let id = TicketId::new(year_value, month_value, day_value, slug);
                        if let Ok(ticket) = self.read(&id) {
                            tickets.push(ticket);
                        }
                    }
                }
            }
        }
        Ok(tickets)
    }

    /// 🔎️ The most recently created ticket whose slug matches, searched newest first.
    pub fn find_by_slug(&self, slug: &str) -> TicketResult<Ticket> {
        let tickets = self.list(None, None, None)?;
        for ticket in tickets.iter().rev() {
            if ticket.slug == slug || base_name(&ticket.slug) == slug {
                return Ok(ticket.clone());
            }
        }
        Err(TicketError::not_found(format!("ticket not found: {slug}")))
    }

    /// 🕰️ The newest ticket by date, then by slug.
    pub fn latest(&self) -> TicketResult<Ticket> {
        let mut tickets = self.list(None, None, None)?;
        if tickets.is_empty() {
            return Err(TicketError::not_found("no tickets found"));
        }
        tickets.sort_by(|left, right| (right.year, right.month, right.day, &right.slug).cmp(&(left.year, left.month, left.day, &left.slug)));
        Ok(tickets.remove(0))
    }

    /// 🔍️ Every ticket a query keeps, in listing order.
    pub fn search(&self, query: &TicketQuery) -> TicketResult<Vec<Ticket>> {
        Ok(self.list(None, None, None)?.into_iter().filter(|ticket| query.matches(ticket)).collect())
    }

    /// 📬️ Opens a ticket: validates, materialises the folder, attaches a plan, syncs the issue and emits.
    pub fn open(&self, request: &TicketOpenRequest) -> TicketResult<TicketOutcome> {
        if request.goal.trim().is_empty() {
            return Err(TicketError::invalid("ticket goal is required"));
        }
        let mut slug = validate_ticket_emoji_title(&request.emoji, &request.title)?;
        let llm = resolve_optional(&request.llm, semio_framework_repo_model::resolve_allowed_llm)?;
        let effort = resolve_optional(&request.effort, semio_framework_repo_model::resolve_allowed_effort)?;
        let client = semio_framework_repo_model::resolve_allowed_client(&request.client).map_err(|error| TicketError::invalid(error.to_string()))?;

        let (mut year, mut month, mut day) = self.clock.today();
        if !request.parent.trim().is_empty() {
            let parent = self.find_by_slug(request.parent.trim())?;
            year = parent.year;
            month = parent.month;
            day = parent.day;
            slug = format!("{}/{slug}", parent.slug);
        }
        let id = TicketId::new(year, month, day, slug);
        let ticket_dir = self.layout.ticket_dir(&id);
        if self.store.exists(&ticket_dir) {
            return Err(TicketError::conflict(format!("ticket folder already exists: {ticket_dir}")));
        }
        self.store.create_directory(&ticket_dir)?;
        let mut journal = TransactionJournal::default();
        self.store.write(&self.layout.important_path(&id), "")?;
        journal.record("create-important", &self.layout.important_path(&id), "ok");

        let mut ticket = Ticket {
            year,
            month,
            day,
            slug: id.slug.clone(),
            title: request.title.trim().to_string(),
            emoji: request.emoji.trim().to_string(),
            status: TicketStatus::Open,
            description: request.prompt.clone(),
            summary: String::new(),
            management: None,
            goal: request.goal.clone(),
            parent: request.parent.clone(),
            plan: None,
            sessions: Vec::new(),
            interactions: vec![Interaction {
                kind: "ticket.open".to_string(),
                date: self.clock.stamp(),
                author: String::new(),
                system: String::new(),
                client: client.clone(),
                checkpoint: String::new(),
                prompt: request.prompt.clone(),
                summary: String::new(),
                llm: llm.clone(),
                effort: effort.clone(),
                files: Vec::new(),
            }],
            agents: Vec::new(),
            folder_path: ticket_dir,
            json_path: self.layout.document_path(&id),
            important_path: self.layout.important_path(&id),
        };
        append_session_id(&mut ticket, &request.session);
        apply_ticket_plan_from_ids(self.store, &self.roots, &mut ticket, self.mcp_client, &request.plan_id, &request.spec_id)?;

        let mut warnings: Vec<String> = Vec::new();
        if !request.no_issue && !request.no_management {
            let milestone = milestone_number_for_title(self.tracker, &request.goal);
            let issue_title = ticket.title.clone();
            warnings.extend(ensure_ticket_issue(self.tracker, &mut ticket, &issue_title, &request.prompt, &request.issue, milestone, false));
        }
        self.save(&ticket)?;
        self.emit_ticket_event(
            semio_framework_repo_events::TICKET_OPEN_ENDED,
            &id,
            serde_json::json!({ "title": ticket.title, "prompt": request.prompt, "llm": llm, "effort": effort, "client": client, "goal": request.goal, "parent": request.parent }),
        );
        Ok(self.outcome(&id, &ticket, journal, warnings))
    }

    /// 📪️ Closes a ticket: consumes the important document, moves the plan in, syncs and emits.
    pub fn close(&self, request: &TicketCloseRequest) -> TicketResult<TicketOutcome> {
        let id = TicketId::parse(&request.id)?;
        let mut ticket = self.read(&id)?;
        if ticket.status != TicketStatus::Open {
            return Err(TicketError::conflict("ticket is not open"));
        }
        let files = normalize_ticket_file_inputs(&request.files);
        let mut summary = request.summary.clone();
        if request.bulk {
            if summary.is_empty() {
                summary = "Bulk close".to_string();
            }
        } else {
            if summary.is_empty() {
                return Err(TicketError::invalid("summary is required to finish a ticket"));
            }
            if files.is_empty() {
                return Err(TicketError::invalid("at least one file is required to finish a ticket"));
            }
        }
        let mut journal = TransactionJournal::default();
        let preimage = inspect_important_document(self.store, &ticket.important_path.clone(), &mut journal)?;
        move_ticket_plan_into_folder(self.store, &mut ticket)?;

        let mut warnings: Vec<String> = Vec::new();
        if !request.no_management {
            warnings.extend(close_ticket_issue(self.tracker, &ticket, &summary, &[], request.bulk));
        }
        remove_important_document(self.store, &preimage, &mut journal)?;

        let previous = ticket.clone();
        ticket.summary = summary.clone();
        ticket.status = TicketStatus::Closed;
        ticket.interactions.push(Interaction {
            kind: "ticket.close".to_string(),
            date: self.clock.stamp(),
            author: String::new(),
            system: String::new(),
            client: previous.interactions.last().map(|interaction| interaction.client.clone()).unwrap_or_default(),
            checkpoint: String::new(),
            prompt: String::new(),
            summary: summary.clone(),
            llm: String::new(),
            effort: String::new(),
            files: files.iter().map(|path| InteractionFile { path: path.clone(), id: String::new(), uri: String::new() }).collect(),
        });
        if let Err(error) = self.save(&ticket) {
            journal.record("save", &ticket.json_path, "failed");
            let restore = restore_important_document(self.store, &preimage, &mut journal);
            ticket = previous;
            let _ = ticket;
            return match restore {
                Ok(()) => Err(error),
                Err(restore) => Err(TicketError::store(format!("save ticket: {error}; restore important document: {restore}"))),
            };
        }
        journal.record("save", &ticket.json_path, "ok");
        self.emit_ticket_event(semio_framework_repo_events::TICKET_CLOSE_ENDED, &id, serde_json::json!({ "summary": summary, "files": files }));
        Ok(self.outcome(&id, &ticket, journal, warnings))
    }

    /// 🔓️ Reopens a closed ticket, creating the important document transactionally.
    pub fn reopen(&self, request: &TicketReopenRequest) -> TicketResult<TicketOutcome> {
        let id = TicketId::parse(&request.id)?;
        let mut ticket = self.read(&id)?;
        if ticket.status != TicketStatus::Closed {
            return Err(TicketError::conflict("ticket is already open"));
        }
        if request.client.trim().is_empty() {
            return Err(TicketError::invalid("client is required"));
        }
        let llm = resolve_optional(&request.llm, semio_framework_repo_model::resolve_allowed_llm)?;
        let effort = resolve_optional(&request.effort, semio_framework_repo_model::resolve_allowed_effort)?;
        let client = semio_framework_repo_model::resolve_allowed_client(&request.client).map_err(|error| TicketError::invalid(error.to_string()))?;
        if !request.goal.trim().is_empty() {
            ticket.goal = request.goal.trim().to_string();
        }
        if !request.parent.trim().is_empty() {
            ticket.parent = request.parent.trim().to_string();
        }
        apply_ticket_plan_from_ids(self.store, &self.roots, &mut ticket, self.mcp_client, &request.plan_id, &request.spec_id)?;

        let mut journal = TransactionJournal::default();
        let creation = ensure_important_document(self.store, &ticket.important_path.clone(), &mut journal)?;

        let previous = ticket.clone();
        ticket.status = TicketStatus::Open;
        ticket.interactions.push(Interaction {
            kind: "ticket.reopen".to_string(),
            date: self.clock.stamp(),
            author: String::new(),
            system: String::new(),
            client: client.clone(),
            checkpoint: String::new(),
            prompt: request.prompt.clone(),
            summary: String::new(),
            llm: llm.clone(),
            effort: effort.clone(),
            files: Vec::new(),
        });
        append_session_id(&mut ticket, &request.session);

        let mut warnings: Vec<String> = Vec::new();
        if !request.no_management {
            warnings.extend(ensure_ticket_issue(self.tracker, &mut ticket, &previous.title, &request.prompt, "", None, true));
            if let Some(management) = ticket.management.as_ref().filter(|management| !management.issue.is_empty()) {
                if let Err(error) = self.tracker.add_comment(&management.issue, &format_prompt_heading(&request.prompt)) {
                    warnings.push(format!("Failed to add prompt comment to GitHub issue: {}", error.message));
                }
            }
        }
        if let Err(error) = self.save(&ticket) {
            journal.record("save", &ticket.json_path, "failed");
            let rollback = rollback_important_creation(self.store, &creation, &mut journal);
            return match rollback {
                Ok(()) => Err(error),
                Err(rollback) => Err(TicketError::store(format!("save ticket: {error}; rollback important document: {rollback}"))),
            };
        }
        journal.record("save", &ticket.json_path, "ok");
        self.emit_ticket_event(semio_framework_repo_events::TICKET_REOPEN_ENDED, &id, serde_json::json!({ "prompt": request.prompt, "llm": llm, "effort": effort, "client": client }));
        Ok(self.outcome(&id, &ticket, journal, warnings))
    }

    /// ♻️ Changes a ticket in place, renaming its folder when the title changes its slug.
    pub fn change(&self, request: &TicketChangeRequest) -> TicketResult<TicketOutcome> {
        let id = TicketId::parse(&request.id)?;
        let mut ticket = self.read(&id)?;
        let mut journal = TransactionJournal::default();
        let mut next_id = id.clone();
        if let Some(title) = request.title.as_ref() {
            let title = title.trim();
            if title.is_empty() {
                return Err(TicketError::invalid("ticket title is required"));
            }
            let mut slug = ticket_slug_from_title(title)?;
            if let Some(parent) = id.parent_slug() {
                slug = format!("{parent}/{slug}");
            }
            next_id = TicketId::new(id.year, id.month, id.day, slug);
            if next_id.slug != id.slug {
                let target = self.layout.ticket_dir(&next_id);
                if self.store.exists(&target) {
                    return Err(TicketError::conflict(format!("ticket folder already exists: {target}")));
                }
                self.store.rename(&self.layout.ticket_dir(&id), &target)?;
                journal.record("rename", &target, "ok");
            }
            ticket.title = title.to_string();
        }
        if let Some(description) = request.description.as_ref() {
            ticket.description = description.clone();
        }
        if let Some(goal) = request.goal.as_ref() {
            ticket.goal = goal.clone();
        }
        if let Some(parent) = request.parent.as_ref() {
            ticket.parent = parent.clone();
        }
        self.hydrate(&mut ticket, &next_id);
        self.save(&ticket)?;
        journal.record("save", &ticket.json_path, "ok");
        self.emit_ticket_event(semio_framework_repo_events::TICKET_CHANGE_ENDED, &next_id, serde_json::json!({ "title": ticket.title, "goal": ticket.goal }));
        Ok(self.outcome(&next_id, &ticket, journal, Vec::new()))
    }

    /// 🧹️ Deletes oversized artifacts from one ticket folder, deepest folder first.
    pub fn purge_artifacts(&self, id: &TicketId) -> TicketResult<PurgeReport> {
        let root = self.layout.ticket_dir(id);
        let mut report = PurgeReport::default();
        if !self.store.is_directory(&root) {
            return Ok(report);
        }
        let mut directories: Vec<String> = Vec::new();
        let mut files: Vec<(String, u64)> = Vec::new();
        self.collect_tree(&root, &mut directories, &mut files);
        let mut directory_sizes: BTreeMap<String, u64> = BTreeMap::new();
        for (path, size) in &files {
            let mut parent = dir_name(path);
            while parent.starts_with(&root) {
                *directory_sizes.entry(parent.clone()).or_default() += size;
                if parent == root {
                    break;
                }
                parent = dir_name(&parent);
            }
        }
        directories.sort_by_key(|path| std::cmp::Reverse(path.matches('/').count()));
        let mut deleted: Vec<String> = Vec::new();
        for directory in directories {
            if deleted.iter().any(|removed| directory.starts_with(&format!("{removed}/"))) {
                continue;
            }
            if directory_sizes.get(&directory).copied().unwrap_or_default() <= OVERSIZED_FOLDER_BYTES {
                continue;
            }
            self.store.remove_tree(&directory)?;
            report.removed_directories.push(directory.clone());
            deleted.push(directory);
        }
        for (path, size) in files {
            if deleted.iter().any(|removed| path == *removed || path.starts_with(&format!("{removed}/"))) {
                continue;
            }
            if base_name(&path) == TICKET_DOCUMENT_NAME || size <= OVERSIZED_FILE_BYTES {
                continue;
            }
            self.store.remove_file(&path)?;
            report.removed_files.push(path);
        }
        report.removed_directories.sort();
        report.removed_files.sort();
        Ok(report)
    }

    fn collect_tree(&self, root: &str, directories: &mut Vec<String>, files: &mut Vec<(String, u64)>) {
        let Ok(entries) = self.store.entries(root) else { return };
        for entry in entries {
            let path = join_path(root, &entry.name);
            match entry.kind {
                NodeKind::Directory => {
                    directories.push(path.clone());
                    self.collect_tree(&path, directories, files);
                }
                NodeKind::File => files.push((path.clone(), self.store.metadata(&path).map(|metadata| metadata.size).unwrap_or_default())),
                NodeKind::Symlink => {}
            }
        }
    }

    fn hydrate(&self, ticket: &mut Ticket, id: &TicketId) {
        ticket.year = id.year;
        ticket.month = id.month;
        ticket.day = id.day;
        ticket.slug = id.slug.clone();
        ticket.folder_path = self.layout.ticket_dir(id);
        ticket.json_path = self.layout.document_path(id);
        ticket.important_path = self.layout.important_path(id);
    }

    fn outcome(&self, id: &TicketId, ticket: &Ticket, journal: TransactionJournal, warnings: Vec<String>) -> TicketOutcome {
        TicketOutcome { id: id.id(), rel_path: id.rel_path(), status: ticket.status.as_str().to_string(), document: encode_ticket_document(ticket), journal, warnings }
    }

    fn emit_ticket_event(&self, kind: &str, id: &TicketId, extra: Value) {
        let mut payload = serde_json::json!({ "id": id.rel_path(), "year": id.year, "month": id.month, "day": id.day, "slug": id.slug });
        if let (Some(target), Value::Object(members)) = (payload.as_object_mut(), extra) {
            for (name, value) in members {
                target.insert(name, value);
            }
        }
        self.events.emit(kind, EVENT_SOURCE, &payload.to_string());
    }

    fn dated_children(&self, path: &str, prefix: &str, pinned: Option<i64>) -> Vec<i64> {
        if let Some(value) = pinned {
            let candidate = join_path(path, &format!("{prefix}{}", pad_number(value, 2)));
            return if self.store.exists(&candidate) { vec![value] } else { Vec::new() };
        }
        let Ok(entries) = self.store.entries(path) else { return Vec::new() };
        let mut values: Vec<i64> = entries.into_iter().filter(|entry| entry.kind == NodeKind::Directory).filter_map(|entry| parse_dated_dir(&entry.name, prefix).ok()).collect();
        values.sort_unstable();
        values
    }

    fn slugs_under(&self, day_path: &str, prefix: &str) -> Vec<String> {
        let root = if prefix.is_empty() { day_path.to_string() } else { join_path(day_path, prefix) };
        let Ok(entries) = self.store.entries(&root) else { return Vec::new() };
        let mut slugs: Vec<String> = Vec::new();
        for entry in entries {
            if entry.kind != NodeKind::Directory || entry.name.starts_with('.') || SKIPPED_WALK_DIRS.contains(&entry.name.as_str()) {
                continue;
            }
            let slug = if prefix.is_empty() { entry.name.clone() } else { format!("{prefix}/{}", entry.name) };
            if self.store.exists(&join_path(&join_path(day_path, &slug), TICKET_DOCUMENT_NAME)) {
                slugs.push(slug);
            } else {
                slugs.extend(self.slugs_under(day_path, &slug));
            }
        }
        slugs
    }
}

fn resolve_optional(value: &str, resolve: fn(&str) -> Result<String, semio_framework_repo_model::ModelError>) -> TicketResult<String> {
    if value.trim().is_empty() {
        return Ok(String::new());
    }
    resolve(value).map_err(|error| TicketError::invalid(error.to_string()))
}


/// 📥️ Reads an open request from JSON text. Concrete on purpose: no external type leaves this crate.
pub fn parse_open_request(text: &str) -> TicketResult<TicketOpenRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📥️ Reads a close request from JSON text.
pub fn parse_close_request(text: &str) -> TicketResult<TicketCloseRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📥️ Reads a reopen request from JSON text.
pub fn parse_reopen_request(text: &str) -> TicketResult<TicketReopenRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📥️ Reads a change request from JSON text.
pub fn parse_change_request(text: &str) -> TicketResult<TicketChangeRequest> {
    serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))
}

/// 📤️ Renders an outcome as JSON text.
pub fn encode_outcome(outcome: &TicketOutcome) -> String {
    serde_json::to_string(outcome).unwrap_or_default()
}

//#endregion 🔓️Lifecycle

//#region 🔎️Search

/// 🔍️ What narrows a ticket listing.
///
/// Regex filtering is deliberately absent: it needs a regular-expression engine, which this crate
/// does not own and may not import. A caller that needs it pre-filters and passes the surviving
/// slugs in `slugs`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketQuery {
    #[serde(default)]
    pub filter: String,
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub match_case: bool,
    #[serde(default)]
    pub match_whole_word: bool,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub include_years: Vec<i64>,
    #[serde(default)]
    pub exclude_years: Vec<i64>,
    #[serde(default)]
    pub slugs: Vec<String>,
}

impl TicketQuery {
    /// ✔️ Whether a ticket survives the query.
    pub fn matches(&self, ticket: &Ticket) -> bool {
        if let Some(status) = &self.status {
            if ticket.status.as_str() != status {
                return false;
            }
        }
        if !self.include_years.is_empty() && !self.include_years.contains(&ticket.year) {
            return false;
        }
        if self.exclude_years.contains(&ticket.year) {
            return false;
        }
        if !self.slugs.is_empty() && !self.slugs.contains(&ticket.slug) {
            return false;
        }
        let identity = TicketId::new(ticket.year, ticket.month, ticket.day, ticket.slug.clone()).id();
        if !self.filter.is_empty() && ![identity.as_str(), ticket.slug.as_str(), ticket.title.as_str()].iter().any(|candidate| self.matches_filter(candidate)) {
            return false;
        }
        if !self.query.is_empty() {
            let haystack = format!("{identity} {} {} {} {}", ticket.slug, ticket.title, ticket.description, ticket.status.as_str());
            if !haystack.to_lowercase().contains(&self.query.to_lowercase()) {
                return false;
            }
        }
        true
    }

    fn matches_filter(&self, name: &str) -> bool {
        let (target, pattern) = if self.match_case { (name.to_string(), self.filter.clone()) } else { (name.to_lowercase(), self.filter.to_lowercase()) };
        if self.match_whole_word {
            return target.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_')).any(|word| word == pattern);
        }
        target.contains(&pattern)
    }
}

//#endregion 🔎️Search

//#region 🧹️RealPurge

/// 🧹️ Every slug directory under a dated `🎫️tickets` tree, in `YY/MM/DD/SLUG` walk order.
///
/// Twin of `collectTicketFolderRoots` in `📦️packages/🐹️go/🐹️.go`: four levels deep, directories
/// only, dot-prefixed slugs skipped, an unreadable level silently contributing nothing.
pub fn ticket_folder_roots(tickets_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut roots = Vec::new();
    for year in sorted_directories(tickets_dir) {
        for month in sorted_directories(&year) {
            for day in sorted_directories(&month) {
                for slug in sorted_directories(&day) {
                    if slug.file_name().is_none_or(|name| name.to_string_lossy().starts_with('.')) {
                        continue;
                    }
                    roots.push(slug);
                }
            }
        }
    }
    roots
}

/// 📁️ The child directories of a path, in name order; an unreadable path contributes none.
fn sorted_directories(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    let Ok(entries) = std::fs::read_dir(path) else { return Vec::new() };
    let mut found: Vec<std::path::PathBuf> = entries.flatten().filter(|entry| entry.path().is_dir()).map(|entry| entry.path()).collect();
    found.sort();
    found
}

/// 🧹️ Deletes the oversized artifacts of one real ticket folder, deepest subfolder first.
///
/// Twin of `PurgeOversizedTicketArtifacts` in `📦️packages/🐹️go/🐹️.go`. A missing folder is a
/// no-operation, symlinks are never followed, `🎫️ticket.json` is never removed and a file inside
/// an already-deleted subtree is not visited again.
pub fn purge_oversized_artifacts(ticket_dir: &std::path::Path) -> Result<PurgeReport, String> {
    let mut report = PurgeReport::default();
    let Ok(metadata) = std::fs::symlink_metadata(ticket_dir) else { return Ok(report) };
    if !metadata.is_dir() {
        return Err(format!("ticket path is not a directory: {}", ticket_dir.display()));
    }
    let mut directories: Vec<std::path::PathBuf> = Vec::new();
    let mut files: Vec<(std::path::PathBuf, u64)> = Vec::new();
    collect_real_tree(ticket_dir, &mut directories, &mut files);
    let mut directory_sizes: BTreeMap<std::path::PathBuf, u64> = BTreeMap::new();
    for (path, size) in &files {
        let mut parent = path.parent().map(std::path::Path::to_path_buf);
        while let Some(current) = parent {
            if !current.starts_with(ticket_dir) {
                break;
            }
            *directory_sizes.entry(current.clone()).or_default() += size;
            if current == ticket_dir {
                break;
            }
            parent = current.parent().map(std::path::Path::to_path_buf);
        }
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    let mut deleted: Vec<std::path::PathBuf> = Vec::new();
    for directory in directories {
        if deleted.iter().any(|removed| directory.starts_with(removed)) {
            continue;
        }
        if directory_sizes.get(&directory).copied().unwrap_or_default() <= OVERSIZED_FOLDER_BYTES {
            continue;
        }
        std::fs::remove_dir_all(&directory).map_err(|error| format!("delete oversized ticket folder {}: {error}", directory.display()))?;
        report.removed_directories.push(directory.to_string_lossy().replace('\\', "/"));
        deleted.push(directory);
    }
    for (path, size) in files {
        if deleted.iter().any(|removed| path.starts_with(removed)) {
            continue;
        }
        if path.file_name().is_some_and(|name| name == std::ffi::OsStr::new(TICKET_DOCUMENT_NAME)) || size <= OVERSIZED_FILE_BYTES {
            continue;
        }
        std::fs::remove_file(&path).map_err(|error| format!("delete oversized ticket file {}: {error}", path.display()))?;
        report.removed_files.push(path.to_string_lossy().replace('\\', "/"));
    }
    report.removed_directories.sort();
    report.removed_files.sort();
    Ok(report)
}

/// 🚶️ Every directory below `root` and every regular file with its size, symlinks skipped.
fn collect_real_tree(root: &std::path::Path, directories: &mut Vec<std::path::PathBuf>, files: &mut Vec<(std::path::PathBuf, u64)>) {
    let Ok(entries) = std::fs::read_dir(root) else { return };
    let mut children: Vec<std::path::PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    children.sort();
    for child in children {
        let Ok(metadata) = std::fs::symlink_metadata(&child) else { continue };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            directories.push(child.clone());
            collect_real_tree(&child, directories, files);
        } else {
            files.push((child, metadata.len()));
        }
    }
}

//#endregion 🧹️RealPurge

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_repo_providers::{ManagementProviders, NullManagementProvider};

    fn layout() -> TicketLayout {
        TicketLayout::new("/repo/.🧬semio/🦑️repo")
    }

    fn service<'a>(store: &'a MemoryTicketStore, tracker: &'a ManagementProviders, clock: &'a FixedClock, events: &'a RecordingEventSink) -> TicketService<'a, MemoryTicketStore, ManagementProviders, FixedClock, RecordingEventSink> {
        TicketService::new(layout(), store, tracker, clock, events)
    }

    #[test]
    fn ticket_id_round_trips_through_both_spellings() {
        let id = TicketId::new(26, 9, 6, "SOME-TICKET");
        assert_eq!(id.id(), "26/09/06/SOME-TICKET");
        assert_eq!(id.rel_path(), "🎆️26/🌙️09/☀️06/SOME-TICKET");
        assert_eq!(TicketId::parse(&id.id()).unwrap(), id);
        assert_eq!(TicketId::parse(&id.rel_path()).unwrap(), id);
        assert_eq!(TicketId::parse("26/09/06/PARENT/CHILD").unwrap().slug, "PARENT/CHILD");
    }

    #[test]
    fn a_title_becomes_an_upper_kebab_slug() {
        assert_eq!(ticket_slug_from_title("Tree Text Short IDs").unwrap(), "TREE-TEXT-SHORT-I-DS");
        assert!(ticket_slug_from_title("   ").is_err());
    }

    #[test]
    fn the_encoder_html_escapes_the_way_go_does() {
        assert_eq!(go_json_string(">e-"), "\"\\u003ee-\"");
        assert_eq!(go_json_string("<e-"), "\"\\u003ce-\"");
        assert_eq!(go_json_string("a&b"), "\"a\\u0026b\"");
    }

    #[test]
    fn a_document_without_a_status_is_refused() {
        let error = decode_ticket_document("{\"title\":\"x\"}").unwrap_err();
        assert_eq!(error.message, "ticket status must be explicitly \"open\" or \"closed\"");
    }

    #[test]
    fn unknown_members_are_dropped_on_re_encoding() {
        let source = "{\n  \"title\": \"T\",\n  \"status\": \"open\",\n  \"note\": \"kept nowhere\"\n}";
        let ticket = decode_ticket_document(source).unwrap();
        assert_eq!(encode_ticket_document(&ticket), "{\n  \"title\": \"T\",\n  \"status\": \"open\"\n}");
    }

    #[test]
    fn an_open_close_reopen_round_trip_emits_three_events() {
        let store = MemoryTicketStore::new();
        let tracker = ManagementProviders::Null(NullManagementProvider);
        let clock = FixedClock::new(26, 9, 6, "2026-09-06 12:00:00");
        let events = RecordingEventSink::new();
        let service = service(&store, &tracker, &clock, &events);
        let opened = service
            .open(&TicketOpenRequest { emoji: "🎫️".to_string(), title: "Some Ticket".to_string(), prompt: "do it".to_string(), client: "claude-code".to_string(), goal: "🎯g".to_string(), no_issue: true, no_management: true, ..Default::default() })
            .unwrap();
        assert_eq!(opened.id, "26/09/06/SOME-TICKET");
        let closed = service.close(&TicketCloseRequest { id: opened.id, summary: "done".to_string(), files: vec!["a/b.rs".to_string()], no_management: true, bulk: false }).unwrap();
        assert_eq!(closed.status, "closed");
        assert!(!store.exists(&service.layout.important_path(&TicketId::parse(&closed.id).unwrap())));
        let reopened = service.reopen(&TicketReopenRequest { id: closed.id, prompt: "again".to_string(), client: "claude-code".to_string(), no_management: true, ..Default::default() }).unwrap();
        assert_eq!(reopened.status, "open");
        assert_eq!(events.recorded().iter().map(|event| event.kind.clone()).collect::<Vec<String>>(), vec!["ticket.open.ended", "ticket.close.ended", "ticket.reopen.ended"]);
    }

    #[test]
    fn a_failed_save_rolls_the_important_document_back() {
        let store = MemoryTicketStore::new();
        let tracker = ManagementProviders::Null(NullManagementProvider);
        let clock = FixedClock::new(26, 9, 6, "2026-09-06 12:00:00");
        let events = RecordingEventSink::new();
        let service = service(&store, &tracker, &clock, &events);
        let opened = service
            .open(&TicketOpenRequest { emoji: "🎫️".to_string(), title: "Rollback".to_string(), prompt: "p".to_string(), client: "claude-code".to_string(), goal: "🎯g".to_string(), no_issue: true, no_management: true, ..Default::default() })
            .unwrap();
        let id = TicketId::parse(&opened.id).unwrap();
        service.close(&TicketCloseRequest { id: opened.id.clone(), summary: "s".to_string(), files: vec!["a.rs".to_string()], no_management: true, bulk: false }).unwrap();
        store.fail_write(&service.layout.document_path(&id), "disk is full");
        let error = service.reopen(&TicketReopenRequest { id: opened.id, prompt: "again".to_string(), client: "claude-code".to_string(), no_management: true, ..Default::default() }).unwrap_err();
        assert_eq!(error.message, "disk is full");
        assert!(!store.exists(&service.layout.important_path(&id)));
        assert!(!store.exists(&service.layout.important_dir(&id)));
    }

    #[test]
    fn a_hunk_stream_becomes_line_numbers() {
        let diff = "diff --git a/x.rs b/x.rs\n--- a/x.rs\n+++ b/x.rs\n@@ -1,2 +1,3 @@\n";
        let parsed = parse_diff_lines(diff);
        assert_eq!(parsed["x.rs"].removed, vec![1, 2]);
        assert_eq!(parsed["x.rs"].added, vec![1, 2, 3]);
    }

    #[test]
    fn an_oversized_artifact_is_purged_but_the_document_is_not() {
        let store = MemoryTicketStore::new();
        let tracker = ManagementProviders::Null(NullManagementProvider);
        let clock = FixedClock::new(26, 9, 6, "2026-09-06 12:00:00");
        let events = RecordingEventSink::new();
        let service = service(&store, &tracker, &clock, &events);
        let id = TicketId::new(26, 9, 6, "PURGE");
        store.seed_file(&service.layout.document_path(&id), "{}");
        store.seed_sized_file(&join_path(&service.layout.ticket_dir(&id), "🗑️generated/huge.log"), OVERSIZED_FILE_BYTES + 1);
        store.seed_sized_file(&join_path(&service.layout.ticket_dir(&id), "small.txt"), 10);
        let report = service.purge_artifacts(&id).unwrap();
        assert_eq!(report.removed_files.len(), 1);
        assert!(store.exists(&service.layout.document_path(&id)));
        assert!(store.exists(&join_path(&service.layout.ticket_dir(&id), "small.txt")));
    }
}

//#endregion 🧪️Tests
