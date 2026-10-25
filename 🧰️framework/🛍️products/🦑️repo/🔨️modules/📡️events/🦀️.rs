//#region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
//#endregion 🧲️Header

//! 📡️ Repo events: kinds, envelope, payloads, coordinator emit and the append-only event store.
//!
//! Twin of `📦️packages/🐹️go/🐹️.go`. Kind strings are declared once in
//! `./🧬️schema/🔣️event-kinds.json` and pulled in here with [`include_str!`]; the constants
//! below mirror them and a test asserts the two agree. Nothing outside `serde`/`serde_json`
//! is used — SHA-256 and the HTTP/1.1 request are hand-rolled.

#[cfg(test)]
use serde::Deserialize;
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use std::path::{Path, PathBuf};

/// 🧩 Explicit re-export: `Event::payload`, `Input::data` and `ExportEntity::value` are
/// `serde_json::Value`, so a client cannot use this crate's public API without naming that type.
/// Re-exporting it here is what keeps the client from declaring the dependency itself.
pub use serde_json;

#[path = "📋️event-kinds/🦀️.rs"]
mod event_kinds;
pub use event_kinds::{
    kind_catalog, Event, EventKind, KindCatalog, KindCatalogEntry, ALL_EVENT_KINDS, ANALYZE_ENDED, ANALYZE_STARTING, CHECKPOINT_ENDED, CHECKPOINT_STARTING, CONTRIBUTOR_ADD_ENDED, CONTRIBUTOR_ADD_STARTING, CONTRIBUTOR_REMOVE_ENDED,
    CONTRIBUTOR_REMOVE_STARTING, DRAFT_CREATE_ENDED, DRAFT_CREATE_STARTING, DRAFT_DELETE_ENDED, DRAFT_DELETE_STARTING, EXPORT_ENDED, EXPORT_STARTING, EXTRACT_ENDED, EXTRACT_STARTING, FILE_CREATE_ENDED, FILE_CREATE_STARTING, FILE_DELETE_ENDED,
    FILE_DELETE_STARTING, FILE_MOVE_ENDED, FILE_MOVE_STARTING, FIX_ENDED, FIX_STARTING, FOLDER_CREATE_ENDED, FOLDER_CREATE_STARTING, FOLDER_DELETE_ENDED, FOLDER_DELETE_STARTING, FOLDER_MOVE_ENDED, FOLDER_MOVE_STARTING, GOAL_CHANGE_ENDED,
    GOAL_CHANGE_STARTING, GOAL_CLOSE_ENDED, GOAL_CLOSE_STARTING, GOAL_OPEN_ENDED, GOAL_OPEN_STARTING, GOAL_REOPEN_ENDED, GOAL_REOPEN_STARTING, GRAPHQL_ENDED, GRAPHQL_STARTING, INTEGRATE_ENDED, INTEGRATE_STARTING, MOVE_ENDED, MOVE_STARTING,
    POLICY_CHECK_ENDED, POLICY_CHECK_STARTING, SECTION_CREATE_ENDED, SECTION_CREATE_STARTING, SECTION_DELETE_ENDED, SECTION_DELETE_STARTING, SECTION_MOVE_ENDED, SECTION_MOVE_STARTING, TICKET_CHANGE_ENDED, TICKET_CHANGE_STARTING, TICKET_CLOSE_ENDED,
    TICKET_CLOSE_STARTING, TICKET_OPEN_ENDED, TICKET_OPEN_STARTING, TICKET_READ_ENDED, TICKET_READ_STARTING, TICKET_REOPEN_ENDED, TICKET_REOPEN_STARTING, TODO_CHANGE_ENDED, TODO_CHANGE_STARTING, TODO_CREATE_ENDED, TODO_CREATE_STARTING,
    TODO_DELETE_ENDED, TODO_DELETE_STARTING, TREE_ENDED, TREE_STARTING,
};
#[path = "🌨️payloads/🦀️.rs"]
mod payloads;
pub use payloads::{
    CheckpointPayload, ContributorPayload, ContributorWork, DraftPayload, ExtractPayload, FilePayload, FolderPayload, GoalChangePayload, GoalClosePayload, GoalOpenPayload, GoalPayload, GoalReopenPayload, IntegratePayload, SectionPayload,
    TicketChangePayload, TicketClosePayload, TicketOpenPayload, TicketPayload, TicketReopenPayload, TodoChangePayload, TodoCreatePayload, TodoDeletePayload, TodoPayload, WorkItem,
};
#[path = "📤️emit/🦀️.rs"]
mod emit;
pub use emit::{emit, emit_url, parse_target, Emitter, HttpEmitter, RequestTarget};
#[path = "🔏️sha256/🦀️.rs"]
mod sha256;
pub use sha256::{digest, Sha256};
#[path = "🗄️store/🦀️.rs"]
mod store;
#[cfg(test)]
use store::{Stage, STAGE_SCHEMA};
pub use store::{Input, Interrupt, MAX_BATCH_EVENTS, MAX_BATCH_SIZE, MAX_EVENT_SIZE, Payload, Progress, SCHEMA, Store, StoreError, StoreEvent, Uninterrupted, checksum};
#[path = "📦️export/🦀️.rs"]
mod export;
pub use export::{build_export_snapshot, ExportEntity, ExportSnapshot, ExportSource};
#[path = "🔐️server-client/🦀️.rs"]
mod server_client;
pub use server_client::{server_addr, server_token, server_whoami};
//#region 🧪️Tests

#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
