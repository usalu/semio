//#region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
//#endregion 🧲️Header

//! 📡️ Repo events: kinds, envelope, payloads, coordinator emit and the append-only event store.
//!
//! Twin of `📦️packages/🐹️go/🐹️.go`. Kind strings are declared once in
//! `../../🧬️schema/🔣️event-kinds.json` and pulled in here with [`include_str!`]; the constants
//! below mirror them and a test asserts the two agree. Nothing outside `serde`/`serde_json`
//! is used — SHA-256 and the HTTP/1.1 request are hand-rolled.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// 🧩 Explicit re-export: `Event::payload`, `Input::data` and `ExportEntity::value` are
/// `serde_json::Value`, so a client cannot use this crate's public API without naming that type.
/// Re-exporting it here is what keeps the client from declaring the dependency itself.
pub use serde_json;

//#region 📋️EventKind

/// 📡️ A changing interaction. The CLI emits; the coordinator subscribes and notifies.
pub type EventKind = String;

/// 🔣️ One row of `🔣️event-kinds.json`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KindCatalogEntry {
    pub constant: String,
    pub kind: String,
}

/// 📇️ The parsed schema-side kind list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindCatalog {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub kinds: Vec<KindCatalogEntry>,
}

const KIND_CATALOG_SOURCE: &str = include_str!("../../🧬️schema/🔣️event-kinds.json");

static KIND_CATALOG: OnceLock<KindCatalog> = OnceLock::new();

/// 📥️ The kind catalog, parsed once from the schema file compiled into this crate.
pub fn kind_catalog() -> &'static KindCatalog {
    KIND_CATALOG.get_or_init(|| {
        serde_json::from_str(KIND_CATALOG_SOURCE).expect("🔣️event-kinds.json is not valid JSON")
    })
}

pub const TICKET_OPEN_STARTING: &str = "ticket.open.starting";
pub const TICKET_OPEN_ENDED: &str = "ticket.open.ended";
pub const TICKET_CLOSE_STARTING: &str = "ticket.close.starting";
pub const TICKET_CLOSE_ENDED: &str = "ticket.close.ended";
pub const TICKET_REOPEN_STARTING: &str = "ticket.reopen.starting";
pub const TICKET_REOPEN_ENDED: &str = "ticket.reopen.ended";
pub const TICKET_CHANGE_STARTING: &str = "ticket.change.starting";
pub const TICKET_CHANGE_ENDED: &str = "ticket.change.ended";
pub const TICKET_READ_STARTING: &str = "ticket.read.starting";
pub const TICKET_READ_ENDED: &str = "ticket.read.ended";
pub const GOAL_OPEN_STARTING: &str = "goal.open.starting";
pub const GOAL_OPEN_ENDED: &str = "goal.open.ended";
pub const GOAL_CLOSE_STARTING: &str = "goal.close.starting";
pub const GOAL_CLOSE_ENDED: &str = "goal.close.ended";
pub const GOAL_REOPEN_STARTING: &str = "goal.reopen.starting";
pub const GOAL_REOPEN_ENDED: &str = "goal.reopen.ended";
pub const GOAL_CHANGE_STARTING: &str = "goal.change.starting";
pub const GOAL_CHANGE_ENDED: &str = "goal.change.ended";
pub const CONTRIBUTOR_ADD_STARTING: &str = "contributor.add.starting";
pub const CONTRIBUTOR_ADD_ENDED: &str = "contributor.add.ended";
pub const CONTRIBUTOR_REMOVE_STARTING: &str = "contributor.remove.starting";
pub const CONTRIBUTOR_REMOVE_ENDED: &str = "contributor.remove.ended";
pub const CHECKPOINT_STARTING: &str = "checkpoint.starting";
pub const CHECKPOINT_ENDED: &str = "checkpoint.ended";
pub const TODO_CREATE_STARTING: &str = "todo.create.starting";
pub const TODO_CREATE_ENDED: &str = "todo.create.ended";
pub const TODO_CHANGE_STARTING: &str = "todo.change.starting";
pub const TODO_CHANGE_ENDED: &str = "todo.change.ended";
pub const TODO_DELETE_STARTING: &str = "todo.delete.starting";
pub const TODO_DELETE_ENDED: &str = "todo.delete.ended";
pub const DRAFT_CREATE_STARTING: &str = "draft.create.starting";
pub const DRAFT_CREATE_ENDED: &str = "draft.create.ended";
pub const DRAFT_DELETE_STARTING: &str = "draft.delete.starting";
pub const DRAFT_DELETE_ENDED: &str = "draft.delete.ended";
pub const FILE_CREATE_STARTING: &str = "file.create.starting";
pub const FILE_CREATE_ENDED: &str = "file.create.ended";
pub const FILE_MOVE_STARTING: &str = "file.move.starting";
pub const FILE_MOVE_ENDED: &str = "file.move.ended";
pub const FILE_DELETE_STARTING: &str = "file.delete.starting";
pub const FILE_DELETE_ENDED: &str = "file.delete.ended";
pub const FOLDER_CREATE_STARTING: &str = "folder.create.starting";
pub const FOLDER_CREATE_ENDED: &str = "folder.create.ended";
pub const FOLDER_MOVE_STARTING: &str = "folder.move.starting";
pub const FOLDER_MOVE_ENDED: &str = "folder.move.ended";
pub const FOLDER_DELETE_STARTING: &str = "folder.delete.starting";
pub const FOLDER_DELETE_ENDED: &str = "folder.delete.ended";
pub const SECTION_CREATE_STARTING: &str = "section.create.starting";
pub const SECTION_CREATE_ENDED: &str = "section.create.ended";
pub const SECTION_MOVE_STARTING: &str = "section.move.starting";
pub const SECTION_MOVE_ENDED: &str = "section.move.ended";
pub const SECTION_DELETE_STARTING: &str = "section.delete.starting";
pub const SECTION_DELETE_ENDED: &str = "section.delete.ended";
pub const INTEGRATE_STARTING: &str = "integrate.starting";
pub const INTEGRATE_ENDED: &str = "integrate.ended";
pub const EXTRACT_STARTING: &str = "extract.starting";
pub const EXTRACT_ENDED: &str = "extract.ended";
pub const EXPORT_STARTING: &str = "export.starting";
pub const EXPORT_ENDED: &str = "export.ended";
pub const ANALYZE_STARTING: &str = "analyze.starting";
pub const ANALYZE_ENDED: &str = "analyze.ended";
pub const FIX_STARTING: &str = "fix.starting";
pub const FIX_ENDED: &str = "fix.ended";
pub const TREE_STARTING: &str = "tree.starting";
pub const TREE_ENDED: &str = "tree.ended";
pub const GRAPHQL_STARTING: &str = "graphql.starting";
pub const GRAPHQL_ENDED: &str = "graphql.ended";
pub const MOVE_STARTING: &str = "move.starting";
pub const MOVE_ENDED: &str = "move.ended";
pub const POLICY_CHECK_STARTING: &str = "policy.check.starting";
pub const POLICY_CHECK_ENDED: &str = "policy.check.ended";

/// 🗂️ The ordered catalog of every declared kind.
pub const ALL_EVENT_KINDS: &[&str] = &[
    TICKET_OPEN_STARTING,
    TICKET_OPEN_ENDED,
    TICKET_CLOSE_STARTING,
    TICKET_CLOSE_ENDED,
    TICKET_REOPEN_STARTING,
    TICKET_REOPEN_ENDED,
    TICKET_CHANGE_STARTING,
    TICKET_CHANGE_ENDED,
    TICKET_READ_STARTING,
    TICKET_READ_ENDED,
    GOAL_OPEN_STARTING,
    GOAL_OPEN_ENDED,
    GOAL_CLOSE_STARTING,
    GOAL_CLOSE_ENDED,
    GOAL_REOPEN_STARTING,
    GOAL_REOPEN_ENDED,
    GOAL_CHANGE_STARTING,
    GOAL_CHANGE_ENDED,
    CONTRIBUTOR_ADD_STARTING,
    CONTRIBUTOR_ADD_ENDED,
    CONTRIBUTOR_REMOVE_STARTING,
    CONTRIBUTOR_REMOVE_ENDED,
    CHECKPOINT_STARTING,
    CHECKPOINT_ENDED,
    TODO_CREATE_STARTING,
    TODO_CREATE_ENDED,
    TODO_CHANGE_STARTING,
    TODO_CHANGE_ENDED,
    TODO_DELETE_STARTING,
    TODO_DELETE_ENDED,
    DRAFT_CREATE_STARTING,
    DRAFT_CREATE_ENDED,
    DRAFT_DELETE_STARTING,
    DRAFT_DELETE_ENDED,
    FILE_CREATE_STARTING,
    FILE_CREATE_ENDED,
    FILE_MOVE_STARTING,
    FILE_MOVE_ENDED,
    FILE_DELETE_STARTING,
    FILE_DELETE_ENDED,
    FOLDER_CREATE_STARTING,
    FOLDER_CREATE_ENDED,
    FOLDER_MOVE_STARTING,
    FOLDER_MOVE_ENDED,
    FOLDER_DELETE_STARTING,
    FOLDER_DELETE_ENDED,
    SECTION_CREATE_STARTING,
    SECTION_CREATE_ENDED,
    SECTION_MOVE_STARTING,
    SECTION_MOVE_ENDED,
    SECTION_DELETE_STARTING,
    SECTION_DELETE_ENDED,
    INTEGRATE_STARTING,
    INTEGRATE_ENDED,
    EXTRACT_STARTING,
    EXTRACT_ENDED,
    EXPORT_STARTING,
    EXPORT_ENDED,
    ANALYZE_STARTING,
    ANALYZE_ENDED,
    FIX_STARTING,
    FIX_ENDED,
    TREE_STARTING,
    TREE_ENDED,
    GRAPHQL_STARTING,
    GRAPHQL_ENDED,
    MOVE_STARTING,
    MOVE_ENDED,
    POLICY_CHECK_STARTING,
    POLICY_CHECK_ENDED,
];

//#endregion 📋️EventKind

//#region ✉️Envelope

/// ✉️ The canonical envelope for a changing interaction sent from CLI to coordinator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub kind: EventKind,
    pub source: String,
    pub payload: serde_json::Value,
}

//#endregion ✉️Envelope

//#region 🌨️Payloads

fn is_zero(value: &i64) -> bool {
    *value == 0
}

/// 📦️ Common ticket identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
}

/// 🎫️ Payload for `ticket.open`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketOpenPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    pub title: String,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    pub client: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
    pub goal: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
}

/// 📪️ Payload for `ticket.close`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketClosePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    pub summary: String,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🔓️ Payload for `ticket.reopen`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketReopenPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    pub client: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// ♻️ Payload for `ticket.change`. A field absent from the wire is a field left untouched.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TicketChangePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub year: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub month: i64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub day: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// ⛳️ Common goal identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalPayload {
    pub id: String,
}

/// 🎯️ Payload for `goal.open`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalOpenPayload {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🏁️ Payload for `goal.close`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalClosePayload {
    pub id: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🔄️ Payload for `goal.reopen`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalReopenPayload {
    pub id: String,
    pub prompt: String,
    pub client: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📐️ Payload for `goal.change`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GoalChangePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 👥️ Contributor identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContributorPayload {
    pub github: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 💾️ Payload for `checkpoint` (a GitHub push).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheckpointPayload {
    pub author: String,
    pub github: String,
    pub sha: String,
    pub message: String,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub technologies: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files_changed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
}

/// ✅️ Todo identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TodoPayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🆕️ Payload for `todo.create`.
pub type TodoCreatePayload = TodoPayload;

/// 🗑️ Payload for `todo.delete`.
pub type TodoDeletePayload = TodoPayload;

/// ✏️ Payload for `todo.change`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TodoChangePayload {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 💼️ A single item a contributor is working on.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkItem {
    pub kind: String,
    pub id: String,
}

/// 🤝️ Every work item of one contributor.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContributorWork {
    pub github: String,
    #[serde(default)]
    pub tickets: Option<Vec<String>>,
    #[serde(default)]
    pub goals: Option<Vec<String>>,
    #[serde(default)]
    pub todos: Option<Vec<String>>,
    #[serde(default)]
    pub technologies: Option<Vec<String>>,
    #[serde(default)]
    pub bundles: Option<Vec<String>>,
    #[serde(default)]
    pub folders: Option<Vec<String>>,
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[serde(default)]
    pub sections: Option<Vec<String>>,
    #[serde(default)]
    pub definitions: Option<Vec<String>>,
}

/// 📝️ Draft identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DraftPayload {
    pub slug: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📄️ File operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FilePayload {
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📁️ Folder operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FolderPayload {
    pub path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub from: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 📑️ Section operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SectionPayload {
    pub file: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub old_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🧬️ Integrate operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IntegratePayload {
    pub source: String,
    pub target_file: String,
    pub target_section: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// 🧲️ Extract operation identifiers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExtractPayload {
    pub source_file: String,
    pub source_section: String,
    pub target_file: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

//#endregion 🌨️Payloads

//#region 📤️Emit

/// 🛰️ Delivers an envelope to the coordinator.
pub trait Emitter {
    /// 📤️ Sends one envelope. Delivery failures are swallowed, exactly as in the Go twin.
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value);
}

/// 🌐️ Posts envelopes to `COMPOSE_SERVER_ADDR` over a hand-rolled HTTP/1.1 request.
#[derive(Debug, Clone, Default)]
pub struct HttpEmitter;

/// 🔗️ The coordinator endpoint for an address, empty when the address is empty.
pub fn emit_url(addr: &str) -> String {
    let addr = addr.trim();
    if addr.is_empty() {
        return String::new();
    }
    let url = if addr.starts_with("http://") || addr.starts_with("https://") {
        addr.to_string()
    } else {
        format!("http://{addr}")
    };
    format!("{}/api/v1/events", url.strip_suffix('/').unwrap_or(&url))
}

/// 🧭️ The host, port and path a request target decomposes into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestTarget {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub secure: bool,
}

/// 🔍️ Splits an absolute URL into its connection target.
pub fn parse_target(url: &str) -> Option<RequestTarget> {
    let (secure, rest) = match url.strip_prefix("http://") {
        Some(rest) => (false, rest),
        None => (true, url.strip_prefix("https://")?),
    };
    let (authority, path) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };
    if authority.is_empty() {
        return None;
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host, port.parse().ok()?),
        None => (authority, if secure { 443 } else { 80 }),
    };
    Some(RequestTarget { host: host.to_string(), port, path: path.to_string(), secure })
}

impl Emitter for HttpEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value) {
        let url = emit_url(&std::env::var("COMPOSE_SERVER_ADDR").unwrap_or_default());
        if url.is_empty() {
            return;
        }
        let Some(target) = parse_target(&url) else { return };
        if target.secure {
            return;
        }
        let envelope = Event { kind: kind.to_string(), source: source.to_string(), payload: payload.clone() };
        let Ok(body) = serde_json::to_vec(&envelope) else { return };
        let token = std::env::var("COMPOSE_SERVER_TOKEN").unwrap_or_default();
        let token = token.trim();
        let mut request = format!(
            "POST {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n",
            target.path,
            target.host,
            target.port,
            body.len()
        );
        if !token.is_empty() {
            request.push_str(&format!("Authorization: Bearer {token}\r\n"));
        }
        request.push_str("\r\n");
        let Ok(mut stream) = TcpStream::connect((target.host.as_str(), target.port)) else { return };
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(5)));
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
        if stream.write_all(request.as_bytes()).is_err() {
            return;
        }
        if stream.write_all(&body).is_err() {
            return;
        }
        let _ = stream.flush();
        let mut discard = Vec::new();
        let _ = stream.read_to_end(&mut discard);
    }
}

/// 📤️ Posts an event to the repo coordinator. No-operation when `COMPOSE_SERVER_ADDR` is unset.
pub fn emit<T: Serialize>(kind: &str, source: &str, payload: &T) {
    let Ok(value) = serde_json::to_value(payload) else { return };
    HttpEmitter.emit(kind, source, &value);
}

//#endregion 📤️Emit

//#region 🔏️Sha256

/// 🔏️ A hand-rolled streaming SHA-256 (FIPS 180-4).
///
/// Consolidation note: `🔌️mcp` carries its own copy; a later ticket moves one shared
/// implementation into `🪪️identity` and both crates depend on that instead.
#[derive(Debug, Clone)]
pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffered: usize,
    length: u64,
}

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// 🆕️ A fresh hasher primed with the FIPS 180-4 initial state.
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            buffer: [0u8; 64],
            buffered: 0,
            length: 0,
        }
    }

    /// ➕️ Absorbs more bytes.
    pub fn update(&mut self, data: &[u8]) {
        self.length = self.length.wrapping_add(data.len() as u64);
        let mut offset = 0;
        if self.buffered > 0 {
            let take = (64 - self.buffered).min(data.len());
            self.buffer[self.buffered..self.buffered + take].copy_from_slice(&data[..take]);
            self.buffered += take;
            offset = take;
            if self.buffered == 64 {
                let block = self.buffer;
                self.compress(&block);
                self.buffered = 0;
            }
        }
        while offset + 64 <= data.len() {
            let mut block = [0u8; 64];
            block.copy_from_slice(&data[offset..offset + 64]);
            self.compress(&block);
            offset += 64;
        }
        if offset < data.len() {
            let remaining = data.len() - offset;
            self.buffer[..remaining].copy_from_slice(&data[offset..]);
            self.buffered = remaining;
        }
    }

    /// 🏁️ Finishes the digest and renders it as lowercase hex.
    pub fn finish(mut self) -> String {
        let bits = self.length.wrapping_mul(8);
        self.update(&[0x80]);
        while self.buffered != 56 {
            self.update(&[0x00]);
        }
        let mut tail = [0u8; 8];
        tail.copy_from_slice(&bits.to_be_bytes());
        self.update(&tail);
        let mut output = String::with_capacity(64);
        for word in self.state {
            output.push_str(&format!("{word:08x}"));
        }
        output
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut schedule = [0u32; 64];
        for index in 0..16 {
            schedule[index] = u32::from_be_bytes([
                block[index * 4],
                block[index * 4 + 1],
                block[index * 4 + 2],
                block[index * 4 + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = schedule[index - 15].rotate_right(7)
                ^ schedule[index - 15].rotate_right(18)
                ^ (schedule[index - 15] >> 3);
            let s1 = schedule[index - 2].rotate_right(17)
                ^ schedule[index - 2].rotate_right(19)
                ^ (schedule[index - 2] >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.state;
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(choice)
                .wrapping_add(SHA256_K[index])
                .wrapping_add(schedule[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, value) in self.state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
}

/// 🧮️ The hex SHA-256 of raw bytes.
pub fn digest(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finish()
}

//#endregion 🔏️Sha256

//#region 🗄️Store

pub const SCHEMA: &str = "semio.event/1";
const STAGE_SCHEMA: &str = "semio.event.stage/1";
pub const MAX_EVENT_SIZE: usize = 1 << 20;
pub const MAX_BATCH_SIZE: usize = 64 << 20;
pub const MAX_BATCH_EVENTS: usize = 100_000;

/// ⚠️ Every way an append or a replay can refuse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Duplicate(String),
    Corrupt(String),
    TooLarge(String),
    Cancelled,
    Invalid(String),
    Io(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(detail) => write!(formatter, "duplicate event: {detail}"),
            Self::Corrupt(detail) => write!(formatter, "corrupt event log: {detail}"),
            Self::TooLarge(detail) => write!(formatter, "event exceeds maximum size: {detail}"),
            Self::Cancelled => write!(formatter, "cancelled"),
            Self::Invalid(detail) => write!(formatter, "{detail}"),
            Self::Io(detail) => write!(formatter, "{detail}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

/// 🧾️ A JSON document carried as the exact text its owner encoded, the twin of Go's
/// `json.RawMessage`.
///
/// A record's member order is part of its wire shape: a domain struct marshals in declaration order
/// while a generic document marshals sorted, and `serde_json::Value` cannot tell the two apart
/// because it sorts everything. Carrying the encoded text is what lets a payload keep the order the
/// owning module wrote it in, so a log written by one implementation checksums identically in the
/// other.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Payload(Box<serde_json::value::RawValue>);

impl Payload {
    /// 🧾️ Encodes one value into its payload, in the member order its own `Serialize` writes.
    pub fn of<T: Serialize + ?Sized>(value: &T) -> Result<Payload, StoreError> {
        serde_json::value::to_raw_value(value).map(Payload).map_err(|error| StoreError::Invalid(error.to_string()))
    }

    /// 📄️ The encoded text, byte for byte as it reaches the log.
    pub fn text(&self) -> &str {
        self.0.get()
    }
}

impl PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl Eq for Payload {}

/// 🧾️ One committed record of the append-only log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StoreEvent {
    pub schema: String,
    pub sequence: u64,
    pub id: String,
    pub kind: String,
    pub data: Payload,
    pub checksum: String,
}

/// 📨️ An uncommitted record offered to the log.
///
/// The reference declares `Data interface{}` here and `json.RawMessage` on the committed record: an
/// offered document is re-encoded on the way in — compact, and sorted when it is a generic
/// document — while a committed one is carried verbatim. `normalized_payload` is that difference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Input {
    pub id: String,
    pub kind: String,
    #[serde(deserialize_with = "normalized_payload")]
    pub data: Payload,
}

/// 🧾️ Decodes an offered document and re-encodes it the way the reference's `interface{}` does.
fn normalized_payload<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Payload, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    Payload::of(&value).map_err(serde::de::Error::custom)
}

/// ⏳️ One step of an append or a replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    pub current: usize,
    pub total: usize,
    pub step: String,
}

/// ⏸️ Cooperative cancellation and progress reporting, the Rust shape of Go's `context` + callback.
pub trait Interrupt {
    /// 🛑️ True once the caller has asked to stop.
    fn cancelled(&self) -> bool {
        false
    }
    /// 📣️ Receives one progress step.
    fn report(&self, _progress: Progress) {}
}

/// ▶️ An interrupt that never cancels and ignores progress.
#[derive(Debug, Clone, Copy, Default)]
pub struct Uninterrupted;

impl Interrupt for Uninterrupted {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Stage {
    schema: String,
    #[serde(rename = "priorExists")]
    prior_exists: bool,
    #[serde(rename = "priorSize")]
    prior_size: u64,
    #[serde(rename = "priorChecksum")]
    prior_checksum: String,
    #[serde(rename = "batchSize")]
    batch_size: usize,
    #[serde(rename = "batchChecksum")]
    batch_checksum: String,
}

/// 🔏️ The SHA-256 identity of a record over schema, sequence, id, kind and data.
pub fn checksum(event: &StoreEvent) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        format!("{}\u{0}{}\u{0}{}\u{0}{}\u{0}", event.schema, event.sequence, event.id, event.kind)
            .as_bytes(),
    );
    hasher.update(event.data.text().as_bytes());
    hasher.finish()
}

/// 🗄️ A recoverable append-only JSONL log with deterministic replay.
#[derive(Debug, Clone)]
pub struct Store {
    pub path: PathBuf,
}

impl Store {
    /// 🆕️ A store over one log file.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    fn stage_path(&self) -> PathBuf {
        let mut value = self.path.clone().into_os_string();
        value.push(".stage");
        PathBuf::from(value)
    }

    fn next_path(&self) -> PathBuf {
        let mut value = self.stage_path().into_os_string();
        value.push(".next");
        PathBuf::from(value)
    }

    /// ➕️ Stages, writes and syncs a batch, leaving the prior log untouched on any failure.
    pub fn append(
        &self,
        inputs: &[Input],
        interrupt: &dyn Interrupt,
    ) -> Result<Vec<StoreEvent>, StoreError> {
        if self.path.as_os_str().is_empty() {
            return Err(StoreError::Invalid("event log path is required".into()));
        }
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        self.recover()?;
        let existing =
            if self.path.exists() { self.replay_locked(&Uninterrupted)? } else { Vec::new() };
        if inputs.len() > MAX_BATCH_EVENTS {
            return Err(StoreError::TooLarge(format!(
                "{} events > {MAX_BATCH_EVENTS}",
                inputs.len()
            )));
        }
        let mut seen: Vec<String> = existing.iter().map(|event| event.id.clone()).collect();
        let mut created = Vec::with_capacity(inputs.len());
        let mut batch = String::new();
        for (index, input) in inputs.iter().enumerate() {
            if interrupt.cancelled() {
                return Err(StoreError::Cancelled);
            }
            if input.id.is_empty() || input.kind.is_empty() {
                return Err(StoreError::Invalid("event id and kind are required".into()));
            }
            if seen.iter().any(|id| id == &input.id) {
                return Err(StoreError::Duplicate(input.id.clone()));
            }
            let data = input.data.text();
            if data.len() > MAX_EVENT_SIZE {
                return Err(StoreError::TooLarge(format!("{} > {MAX_EVENT_SIZE}", data.len())));
            }
            let mut event = StoreEvent {
                schema: SCHEMA.to_string(),
                sequence: (existing.len() + index + 1) as u64,
                id: input.id.clone(),
                kind: input.kind.clone(),
                data: input.data.clone(),
                checksum: String::new(),
            };
            event.checksum = checksum(&event);
            batch.push_str(
                &serde_json::to_string(&event)
                    .map_err(|error| StoreError::Invalid(error.to_string()))?,
            );
            batch.push('\n');
            if batch.len() > MAX_BATCH_SIZE {
                return Err(StoreError::TooLarge(format!("batch > {MAX_BATCH_SIZE}")));
            }
            seen.push(input.id.clone());
            created.push(event);
            interrupt.report(Progress {
                current: index + 1,
                total: inputs.len(),
                step: "encoded".into(),
            });
        }
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        if created.is_empty() {
            return Ok(created);
        }
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let prior = self.inspect_prior()?;
        let staged = Stage {
            schema: STAGE_SCHEMA.to_string(),
            prior_exists: prior.0,
            prior_size: prior.1,
            prior_checksum: prior.2,
            batch_size: batch.len(),
            batch_checksum: digest(batch.as_bytes()),
        };
        self.write_stage(&staged)?;
        interrupt.report(Progress { current: 0, total: created.len(), step: "staged".into() });
        if interrupt.cancelled() {
            self.rollback(&staged)?;
            return Err(StoreError::Cancelled);
        }
        let write = fs::OpenOptions::new().create(true).append(true).open(&self.path);
        let mut file = match write {
            Ok(file) => file,
            Err(error) => {
                self.rollback(&staged)?;
                return Err(error.into());
            }
        };
        let mut failure = file.write_all(batch.as_bytes()).err().map(StoreError::from);
        interrupt.report(Progress {
            current: batch.len(),
            total: batch.len(),
            step: "appended".into(),
        });
        if failure.is_none() && interrupt.cancelled() {
            failure = Some(StoreError::Cancelled);
        }
        if failure.is_none() {
            failure = file.sync_all().err().map(StoreError::from);
        }
        if failure.is_none() {
            interrupt.report(Progress {
                current: created.len(),
                total: created.len(),
                step: "synced".into(),
            });
            if interrupt.cancelled() {
                failure = Some(StoreError::Cancelled);
            }
        }
        drop(file);
        if let Some(error) = failure {
            self.rollback(&staged)?;
            return Err(error);
        }
        let _ = fs::remove_file(self.stage_path());
        interrupt.report(Progress {
            current: created.len(),
            total: created.len(),
            step: "committed".into(),
        });
        Ok(created)
    }

    /// ⏪️ Recovers any interrupted append, then reads and validates the whole log.
    pub fn replay(&self, interrupt: &dyn Interrupt) -> Result<Vec<StoreEvent>, StoreError> {
        if self.path.as_os_str().is_empty() {
            return Err(StoreError::Invalid("event log path is required".into()));
        }
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        self.recover()?;
        self.replay_locked(interrupt)
    }

    fn replay_locked(&self, interrupt: &dyn Interrupt) -> Result<Vec<StoreEvent>, StoreError> {
        let content = fs::read(&self.path)?;
        let mut events: Vec<StoreEvent> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        let mut start = 0usize;
        while start < content.len() {
            if interrupt.cancelled() {
                return Err(StoreError::Cancelled);
            }
            let end = match content[start..].iter().position(|byte| *byte == b'\n') {
                Some(offset) => start + offset,
                None => {
                    return Err(StoreError::Corrupt(format!(
                        "incomplete event at sequence {}",
                        events.len() + 1
                    )))
                }
            };
            let line = &content[start..end];
            if line.len() > MAX_EVENT_SIZE * 2 {
                return Err(StoreError::Corrupt("encoded event too large".into()));
            }
            let expected = (events.len() + 1) as u64;
            let event: StoreEvent = serde_json::from_slice(line)
                .map_err(|error| StoreError::Corrupt(format!("at sequence {expected}: {error}")))?;
            if event.schema != SCHEMA
                || event.sequence != expected
                || event.id.is_empty()
                || event.kind.is_empty()
                || event.checksum != checksum(&event)
            {
                return Err(StoreError::Corrupt(format!("at sequence {expected}")));
            }
            if seen.iter().any(|id| id == &event.id) {
                return Err(StoreError::Duplicate(event.id));
            }
            seen.push(event.id.clone());
            events.push(event);
            interrupt.report(Progress {
                current: events.len(),
                total: 0,
                step: "replayed".into(),
            });
            start = end + 1;
        }
        Ok(events)
    }

    fn inspect_prior(&self) -> Result<(bool, u64, String), StoreError> {
        match fs::read(&self.path) {
            Ok(content) => Ok((true, content.len() as u64, digest(&content))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok((false, 0, digest(&[])))
            }
            Err(error) => Err(error.into()),
        }
    }

    fn write_stage(&self, staged: &Stage) -> Result<(), StoreError> {
        let next = self.next_path();
        let _ = fs::remove_file(&next);
        let mut encoded =
            serde_json::to_vec(staged).map_err(|error| StoreError::Invalid(error.to_string()))?;
        encoded.push(b'\n');
        let mut file = fs::File::create(&next)?;
        file.write_all(&encoded)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&next, self.stage_path())?;
        Ok(())
    }

    fn recover(&self) -> Result<(), StoreError> {
        let _ = fs::remove_file(self.next_path());
        let data = match fs::read(self.stage_path()) {
            Ok(data) => data,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error.into()),
        };
        let staged: Stage = serde_json::from_slice(&data)
            .map_err(|_| StoreError::Corrupt("invalid append stage".into()))?;
        if staged.schema != STAGE_SCHEMA || staged.batch_size == 0 {
            return Err(StoreError::Corrupt("invalid append stage".into()));
        }
        let content = match fs::read(&self.path) {
            Ok(content) => content,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if !staged.prior_exists {
                    fs::remove_file(self.stage_path())?;
                    return Ok(());
                }
                return Err(StoreError::Corrupt(format!("staged log missing: {error}")));
            }
            Err(error) => return Err(error.into()),
        };
        let size = content.len() as u64;
        if size < staged.prior_size || size > staged.prior_size + staged.batch_size as u64 {
            return Err(StoreError::Corrupt("staged append size".into()));
        }
        let prior_size = staged.prior_size as usize;
        if digest(&content[..prior_size]) != staged.prior_checksum {
            return Err(StoreError::Corrupt("staged append prefix".into()));
        }
        let committed = size == staged.prior_size + staged.batch_size as u64
            && digest(&content[prior_size..]) == staged.batch_checksum;
        if !committed && size != staged.prior_size {
            let file = fs::OpenOptions::new().write(true).open(&self.path)?;
            file.set_len(staged.prior_size)?;
            file.sync_all()?;
        }
        if !staged.prior_exists && staged.prior_size == 0 && !committed {
            match fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        fs::remove_file(self.stage_path())?;
        Ok(())
    }

    fn rollback(&self, staged: &Stage) -> Result<(), StoreError> {
        match fs::OpenOptions::new().write(true).open(&self.path) {
            Ok(file) => {
                file.set_len(staged.prior_size)?;
                file.sync_all()?;
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if !staged.prior_exists {
                    let _ = fs::remove_file(self.stage_path());
                    return Ok(());
                }
                return Err(error.into());
            }
            Err(error) => return Err(error.into()),
        }
        if !staged.prior_exists {
            match fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        match fs::remove_file(self.stage_path()) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        Ok(())
    }
}

//#endregion 🗄️Store

//#region 📦️Export

/// 🏷️ One entity of a repository snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportEntity {
    pub kind: String,
    pub id: String,
    pub value: Payload,
}

/// 🧭️ The port an export reads its entities through.
pub trait ExportSource {
    /// 📋️ Enumerates every snapshot entity in taxonomy order.
    fn export_entities(&self) -> Vec<ExportEntity>;
}

impl ExportSource for Vec<ExportEntity> {
    fn export_entities(&self) -> Vec<ExportEntity> {
        self.clone()
    }
}

/// 🧾️ The deterministic identity, per-kind counts and namespaced inputs of one export batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSnapshot {
    pub snapshot: String,
    pub counts: Vec<(String, usize)>,
    pub inputs: Vec<Input>,
}

impl ExportSnapshot {
    /// 🔢️ The count recorded for one entity kind.
    pub fn count(&self, kind: &str) -> usize {
        self.counts
            .iter()
            .find(|(name, _)| name == kind)
            .map(|(_, count)| *count)
            .unwrap_or_default()
    }
}

/// 🧮️ Sorts entities by id, hashes them and namespaces every input id with the snapshot.
pub fn build_export_snapshot(
    source: &dyn ExportSource,
    interrupt: &dyn Interrupt,
) -> Result<ExportSnapshot, StoreError> {
    let entities = source.export_entities();
    let mut counts: Vec<(String, usize)> = Vec::new();
    let mut inputs: Vec<Input> = Vec::with_capacity(entities.len());
    for entity in entities {
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        match counts.iter_mut().find(|(name, _)| name == &entity.kind) {
            Some(slot) => slot.1 += 1,
            None => counts.push((entity.kind.clone(), 1)),
        }
        inputs.push(Input {
            id: format!("{}:{}", entity.kind, entity.id),
            kind: format!("{}.recorded", entity.kind),
            data: entity.value,
        });
    }
    inputs.sort_by(|left, right| left.id.cmp(&right.id));
    let mut hasher = Sha256::new();
    for input in &inputs {
        if interrupt.cancelled() {
            return Err(StoreError::Cancelled);
        }
        hasher.update(format!("{}\u{0}{}\u{0}", input.id, input.kind).as_bytes());
        hasher.update(input.data.text().as_bytes());
    }
    let snapshot = hasher.finish();
    for input in inputs.iter_mut() {
        input.id = format!("snapshot:{snapshot}:{}", input.id);
    }
    Ok(ExportSnapshot { snapshot, counts, inputs })
}

//#endregion 📦️Export

//#region 🔐️ServerClient

/// 🌐️ The coordinator address `COMPOSE_SERVER_ADDR` names, normalised the way the Go twin does:
/// a bare authority gains `http://` and a trailing slash is dropped. Empty when unset.
pub fn server_addr() -> String {
    let addr = std::env::var("COMPOSE_SERVER_ADDR").unwrap_or_default();
    let addr = addr.trim();
    if addr.is_empty() {
        return String::new();
    }
    let addr = if addr.starts_with("http://") || addr.starts_with("https://") { addr.to_string() } else { format!("http://{addr}") };
    addr.strip_suffix('/').unwrap_or(&addr).to_string()
}

/// 🎟️ The API key `COMPOSE_SERVER_TOKEN` carries, trimmed.
pub fn server_token() -> String {
    std::env::var("COMPOSE_SERVER_TOKEN").unwrap_or_default().trim().to_string()
}

/// 📨️ One authenticated `GET` against the coordinator, answered as `(status line, body)`.
///
/// Only `http://` is reachable: this crate hand-rolls HTTP/1.1 over [`TcpStream`] and owns no TLS.
fn server_get(path: &str) -> Result<(u16, String, String), String> {
    let addr = server_addr();
    if addr.is_empty() {
        return Err("COMPOSE_SERVER_ADDR not set".to_string());
    }
    let target = parse_target(&format!("{addr}{path}")).ok_or_else(|| format!("invalid server address {addr}"))?;
    if target.secure {
        return Err("https coordinator addresses are not supported".to_string());
    }
    let mut request = format!("GET {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nConnection: close\r\n", target.path, target.host, target.port);
    let token = server_token();
    if !token.is_empty() {
        request.push_str(&format!("Authorization: Bearer {token}\r\n"));
    }
    request.push_str("\r\n");
    let mut stream = TcpStream::connect((target.host.as_str(), target.port)).map_err(|error| error.to_string())?;
    let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(10)));
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(10)));
    stream.write_all(request.as_bytes()).map_err(|error| error.to_string())?;
    stream.flush().map_err(|error| error.to_string())?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|error| error.to_string())?;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let (head, body) = text.split_once("\r\n\r\n").ok_or_else(|| "malformed response".to_string())?;
    let status_line = head.lines().next().unwrap_or_default().trim().to_string();
    let mut fields = status_line.splitn(3, ' ');
    let _ = fields.next();
    let code: u16 = fields.next().unwrap_or_default().parse().map_err(|_| format!("malformed status line {status_line:?}"))?;
    let reason = fields.next().unwrap_or_default().to_string();
    Ok((code, if reason.is_empty() { code.to_string() } else { format!("{code} {reason}") }, body.to_string()))
}

/// 🔐️ The authenticated developer the coordinator reports, or the refusal the Go twin words.
///
/// Twin of `ServerWhoami` in `📦️packages/🐹️go/🐹️.go`: `GET /api/v1/auth`, a non-200 becoming
/// `auth failed: <status>`.
pub fn server_whoami() -> Result<serde_json::Map<String, serde_json::Value>, String> {
    let (code, status, body) = server_get("/api/v1/auth")?;
    if code != 200 {
        return Err(format!("auth failed: {status}"));
    }
    match serde_json::from_str(&body) {
        Ok(serde_json::Value::Object(fields)) => Ok(fields),
        Ok(_) => Err("auth response is not an object".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

//#endregion 🔐️ServerClient

//#region 🧫️Fixtures

/// 📍️ The module's fixture directory, resolved relative to this crate's manifest.
pub fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("🧫️fixtures")
}

/// 📥️ Reads and parses one fixture of this module.
pub fn fixture<T: serde::de::DeserializeOwned>(name: &str) -> Result<T, String> {
    let path = fixture_dir().join(name);
    let data = fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&data).map_err(|error| format!("{}: {error}", path.display()))
}

//#endregion 🧫️Fixtures

//#region 🧪️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let unique = COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = std::env::temp_dir()
                .join(format!("semio-repo-events-{}-{unique}", std::process::id()));
            fs::create_dir_all(&path).expect("temp dir");
            Self(path)
        }
        fn join(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    struct PhaseCancel {
        phase: String,
        hit: Cell<bool>,
    }

    impl PhaseCancel {
        fn new(phase: &str) -> Self {
            Self { phase: phase.to_string(), hit: Cell::new(false) }
        }
    }

    impl Interrupt for PhaseCancel {
        fn cancelled(&self) -> bool {
            self.hit.get()
        }
        fn report(&self, progress: Progress) {
            if progress.step == self.phase {
                self.hit.set(true);
            }
        }
    }

    struct CancelAtReplay(Cell<bool>);

    impl Interrupt for CancelAtReplay {
        fn cancelled(&self) -> bool {
            self.0.get()
        }
        fn report(&self, progress: Progress) {
            if progress.current == 1 {
                self.0.set(true);
            }
        }
    }

    struct AlwaysCancelled;

    impl Interrupt for AlwaysCancelled {
        fn cancelled(&self) -> bool {
            true
        }
    }

    //#region 📋️EventKindCatalog

    #[test]
    fn kind_constants_match_schema_catalog() {
        let catalog = kind_catalog();
        assert_eq!(catalog.schema_version, 1);
        assert_eq!(ALL_EVENT_KINDS.len(), catalog.kinds.len());
        for (index, entry) in catalog.kinds.iter().enumerate() {
            assert_eq!(ALL_EVENT_KINDS[index], entry.kind, "kind {index}");
        }
        let mut seen: Vec<&str> = Vec::new();
        for kind in ALL_EVENT_KINDS {
            assert!(!seen.contains(kind), "duplicate kind {kind}");
            assert!(kind.contains('.'), "kind {kind} is not dotted");
            seen.push(kind);
        }
    }

    //#endregion 📋️EventKindCatalog

    //#region ✉️PayloadEncoding

    #[derive(Deserialize)]
    struct PayloadCase {
        id: String,
        #[serde(rename = "type")]
        kind: String,
        input: serde_json::Value,
        encoded: String,
    }

    #[derive(Deserialize)]
    struct PayloadVectors {
        cases: Vec<PayloadCase>,
    }

    fn round_trip(kind: &str, input: &serde_json::Value) -> Result<String, String> {
        macro_rules! decode {
            ($($name:literal => $type:ty),* $(,)?) => {
                match kind {
                    $($name => {
                        let value: $type = serde_json::from_value(input.clone()).map_err(|error| error.to_string())?;
                        serde_json::to_string(&value).map_err(|error| error.to_string())
                    })*
                    other => Err(format!("unknown payload type {other}")),
                }
            };
        }
        decode! {
            "TicketPayload" => TicketPayload,
            "TicketOpenPayload" => TicketOpenPayload,
            "TicketClosePayload" => TicketClosePayload,
            "TicketReopenPayload" => TicketReopenPayload,
            "TicketChangePayload" => TicketChangePayload,
            "GoalPayload" => GoalPayload,
            "GoalOpenPayload" => GoalOpenPayload,
            "GoalClosePayload" => GoalClosePayload,
            "GoalReopenPayload" => GoalReopenPayload,
            "GoalChangePayload" => GoalChangePayload,
            "ContributorPayload" => ContributorPayload,
            "CheckpointPayload" => CheckpointPayload,
            "TodoPayload" => TodoPayload,
            "TodoCreatePayload" => TodoCreatePayload,
            "TodoChangePayload" => TodoChangePayload,
            "TodoDeletePayload" => TodoDeletePayload,
            "WorkItem" => WorkItem,
            "ContributorWork" => ContributorWork,
            "DraftPayload" => DraftPayload,
            "FilePayload" => FilePayload,
            "FolderPayload" => FolderPayload,
            "SectionPayload" => SectionPayload,
            "IntegratePayload" => IntegratePayload,
            "ExtractPayload" => ExtractPayload,
        }
    }

    #[test]
    fn payload_golden_encoding() {
        let vectors: PayloadVectors = fixture("✉️payload-vectors.json").expect("payload vectors");
        assert!(!vectors.cases.is_empty());
        for case in &vectors.cases {
            let encoded = round_trip(&case.kind, &case.input)
                .unwrap_or_else(|error| panic!("{}: {error}", case.id));
            assert_eq!(encoded, case.encoded, "{}", case.id);
        }
    }

    #[test]
    fn envelope_encoding() {
        let envelope = Event {
            kind: TICKET_OPEN_STARTING.to_string(),
            source: "repo-cli".to_string(),
            payload: serde_json::json!({"id": "a"}),
        };
        let expected = concat!(
            "{\"kind\":\"ticket.open.starting\",",
            "\"source\":\"repo-cli\",",
            "\"payload\":{\"id\":\"a\"}}"
        );
        assert_eq!(serde_json::to_string(&envelope).unwrap(), expected);
    }

    #[test]
    fn emit_url_and_target() {
        assert_eq!(emit_url(""), "");
        assert_eq!(emit_url("   "), "");
        assert_eq!(emit_url("127.0.0.1:8787"), "http://127.0.0.1:8787/api/v1/events");
        assert_eq!(emit_url("http://host:1/"), "http://host:1/api/v1/events");
        assert_eq!(emit_url("https://host.example"), "https://host.example/api/v1/events");
        assert_eq!(emit_url("http://host.example///"), "http://host.example///api/v1/events");
        let target = parse_target("http://127.0.0.1:8787/api/v1/events").unwrap();
        assert_eq!(target.host, "127.0.0.1");
        assert_eq!(target.port, 8787);
        assert_eq!(target.path, "/api/v1/events");
        assert!(!target.secure);
        assert_eq!(parse_target("https://host/x").unwrap().port, 443);
        assert!(parse_target("ftp://host/x").is_none());
    }

    //#endregion ✉️PayloadEncoding

    //#region 🗄️StoreAppendSequence

    #[derive(Deserialize)]
    struct StoreVectors {
        inputs: Vec<Input>,
        sequences: Vec<u64>,
        #[serde(rename = "interruptPhases")]
        interrupt_phases: Vec<String>,
    }

    fn store_vectors() -> StoreVectors {
        fixture("🗄️store-vectors.json").expect("store vectors")
    }

    #[test]
    fn store_fixture_deterministic_replay() {
        let vectors = store_vectors();
        let first_dir = TempDir::new();
        let store = Store::new(first_dir.join("events.jsonl"));
        store.append(&vectors.inputs, &Uninterrupted).expect("append");
        let first = fs::read(&store.path).expect("read");
        let replayed = store.replay(&Uninterrupted).expect("replay");
        let sequences: Vec<u64> = replayed.iter().map(|event| event.sequence).collect();
        assert_eq!(sequences, vectors.sequences);
        let second_dir = TempDir::new();
        let second_store = Store::new(second_dir.join("events.jsonl"));
        second_store.append(&vectors.inputs, &Uninterrupted).expect("append");
        assert_eq!(first, fs::read(&second_store.path).expect("read"));
    }

    #[test]
    fn store_duplicate_interrupted_and_corrupt_event() {
        let vectors = store_vectors();
        let dir = TempDir::new();
        let store = Store::new(dir.join("events.jsonl"));
        store.append(&vectors.inputs[..1], &Uninterrupted).expect("append");
        let before = fs::read(&store.path).expect("read");
        assert!(matches!(
            store.append(&vectors.inputs[..1], &Uninterrupted),
            Err(StoreError::Duplicate(_))
        ));
        let cancel = PhaseCancel::new("encoded");
        assert_eq!(store.append(&vectors.inputs[1..], &cancel), Err(StoreError::Cancelled));
        assert_eq!(before, fs::read(&store.path).expect("read"));
        let mut corrupt = before;
        let middle = corrupt.len() / 2;
        corrupt[middle] ^= 1;
        fs::write(&store.path, &corrupt).expect("write");
        assert!(matches!(store.replay(&Uninterrupted), Err(StoreError::Corrupt(_))));
    }

    #[test]
    fn store_cancellation_and_maximum() {
        let dir = TempDir::new();
        let store = Store::new(dir.join("events.jsonl"));
        let one =
            vec![Input { id: "a".into(), kind: "recorded".into(), data: Payload::of(&serde_json::json!("a")).expect("payload") }];
        assert_eq!(store.append(&one, &AlwaysCancelled), Err(StoreError::Cancelled));
        let maximum = "x".repeat(MAX_EVENT_SIZE - 2);
        store
            .append(
                &[Input {
                    id: "max".into(),
                    kind: "recorded".into(),
                    data: Payload::of(&serde_json::Value::String(maximum)).expect("payload"),
                }],
                &Uninterrupted,
            )
            .expect("maximum input");
        let before = fs::read(&store.path).expect("read");
        let plus_one = "x".repeat(MAX_EVENT_SIZE - 1);
        assert!(matches!(
            store.append(
                &[Input {
                    id: "plus-one".into(),
                    kind: "recorded".into(),
                    data: Payload::of(&serde_json::Value::String(plus_one)).expect("payload"),
                }],
                &Uninterrupted
            ),
            Err(StoreError::TooLarge(_))
        ));
        assert_eq!(before, fs::read(&store.path).expect("read"));
    }

    #[test]
    fn store_interruptions_preserve_committed_log() {
        let vectors = store_vectors();
        for phase in &vectors.interrupt_phases {
            let dir = TempDir::new();
            let store = Store::new(dir.join("events.jsonl"));
            store.append(&vectors.inputs[..1], &Uninterrupted).expect("append");
            let before = fs::read(&store.path).expect("read");
            let cancel = PhaseCancel::new(phase);
            assert_eq!(
                store.append(&vectors.inputs[1..], &cancel),
                Err(StoreError::Cancelled),
                "{phase}"
            );
            assert_eq!(before, fs::read(&store.path).expect("read"), "{phase}");
            assert!(!store.stage_path().exists(), "{phase} left a stage");
            let events = store.replay(&Uninterrupted).expect("replay");
            assert_eq!(events.len(), 1, "{phase}");
            assert_eq!(events[0].id, vectors.inputs[0].id, "{phase}");
        }
    }

    #[test]
    fn store_replay_cancellation_preserves_log() {
        let vectors = store_vectors();
        let dir = TempDir::new();
        let store = Store::new(dir.join("events.jsonl"));
        store.append(&vectors.inputs, &Uninterrupted).expect("append");
        let before = fs::read(&store.path).expect("read");
        let cancel = CancelAtReplay(Cell::new(false));
        assert_eq!(store.replay(&cancel), Err(StoreError::Cancelled));
        assert_eq!(before, fs::read(&store.path).expect("read"));
        assert_eq!(store.replay(&Uninterrupted).expect("replay").len(), vectors.inputs.len());
    }

    #[test]
    fn staged_append_recovery() {
        for (name, numerator, denominator, want_events, want_changed) in [
            ("stage only", 0usize, 1usize, 1usize, false),
            ("partial batch", 1, 2, 1, false),
            ("complete batch", 1, 1, 2, true),
        ] {
            let dir = TempDir::new();
            let store = Store::new(dir.join("events.jsonl"));
            store
                .append(
                    &[Input {
                        id: "first".into(),
                        kind: "recorded".into(),
                        data: Payload::of(&serde_json::json!("first")).expect("payload"),
                    }],
                    &Uninterrupted,
                )
                .expect("append");
            let before = fs::read(&store.path).expect("read");
            let mut event = StoreEvent {
                schema: SCHEMA.into(),
                sequence: 2,
                id: "second".into(),
                kind: "recorded".into(),
                data: Payload::of(&serde_json::json!("second")).expect("payload"),
                checksum: String::new(),
            };
            event.checksum = checksum(&event);
            let mut batch = serde_json::to_string(&event).unwrap();
            batch.push('\n');
            let staged = Stage {
                schema: STAGE_SCHEMA.into(),
                prior_exists: true,
                prior_size: before.len() as u64,
                prior_checksum: digest(&before),
                batch_size: batch.len(),
                batch_checksum: digest(batch.as_bytes()),
            };
            fs::write(store.stage_path(), serde_json::to_vec(&staged).unwrap()).expect("stage");
            let written = &batch.as_bytes()[..batch.len() * numerator / denominator];
            if !written.is_empty() {
                let mut file = fs::OpenOptions::new().append(true).open(&store.path).unwrap();
                file.write_all(written).unwrap();
            }
            let events = store.replay(&Uninterrupted).expect("replay");
            assert_eq!(events.len(), want_events, "{name}");
            let after = fs::read(&store.path).expect("read");
            assert_eq!(before != after, want_changed, "{name}");
            assert!(!store.stage_path().exists(), "{name} left a stage");
        }
    }

    //#endregion 🗄️StoreAppendSequence

    //#region 📤️ExportContentHash

    #[derive(Deserialize)]
    struct ExportEntityVector {
        kind: String,
        id: String,
        value: serde_json::Value,
    }

    #[derive(Deserialize)]
    struct ExportVectors {
        entities: Vec<ExportEntityVector>,
        #[serde(rename = "sortedIds")]
        sorted_ids: Vec<String>,
        snapshot: String,
        #[serde(rename = "inputIds")]
        input_ids: Vec<String>,
        counts: std::collections::BTreeMap<String, usize>,
    }

    #[test]
    fn export_snapshot_content_hash() {
        let vectors: ExportVectors = fixture("📤️export-vectors.json").expect("export vectors");
        let entities: Vec<ExportEntity> = vectors
            .entities
            .iter()
            .map(|entity| ExportEntity {
                kind: entity.kind.clone(),
                id: entity.id.clone(),
                value: Payload::of(&entity.value).expect("payload"),
            })
            .collect();
        let snapshot = build_export_snapshot(&entities, &Uninterrupted).expect("snapshot");
        assert_eq!(snapshot.snapshot, vectors.snapshot);
        for (kind, count) in &vectors.counts {
            assert_eq!(snapshot.count(kind), *count, "{kind}");
        }
        let ids: Vec<String> = snapshot.inputs.iter().map(|input| input.id.clone()).collect();
        assert_eq!(ids, vectors.input_ids);
        let prefix = format!("snapshot:{}:", vectors.snapshot);
        let stripped: Vec<String> = ids
            .iter()
            .map(|id| id.trim_start_matches(prefix.as_str()).to_string())
            .collect();
        assert_eq!(stripped, vectors.sorted_ids);
        let dir = TempDir::new();
        let store = Store::new(dir.join("export.events.jsonl"));
        store.append(&snapshot.inputs, &Uninterrupted).expect("append");
        assert!(matches!(
            store.append(&snapshot.inputs, &Uninterrupted),
            Err(StoreError::Duplicate(_))
        ));
        assert_eq!(build_export_snapshot(&entities, &AlwaysCancelled), Err(StoreError::Cancelled));
    }

    #[test]
    fn digest_and_checksum_are_stable() {
        assert_eq!(digest(&[]), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(
            digest(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(digest(&[b'a'; 1000]), digest("a".repeat(1000).as_bytes()));
        let event = StoreEvent {
            schema: SCHEMA.into(),
            sequence: 1,
            id: "a".into(),
            kind: "recorded".into(),
            data: Payload::of(&serde_json::json!("a")).expect("payload"),
            checksum: String::new(),
        };
        let mut other = event.clone();
        other.sequence = 2;
        assert_ne!(checksum(&event), checksum(&other));
    }

    //#endregion 📤️ExportContentHash
}

//#endregion 🧪️Tests
