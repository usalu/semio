//! 📋️ Event kinds and the envelope: the kind constants and catalog, the event record and its builders.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

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

const KIND_CATALOG_SOURCE: &str = include_str!("../../../🧬️schema/🔣️event-kinds.json");

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
