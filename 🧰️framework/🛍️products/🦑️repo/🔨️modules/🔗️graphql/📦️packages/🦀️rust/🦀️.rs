//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🔗️ The repository's owned GraphQL document grammar: lexer, AST, recursive-descent parser,
//! validation and variable coercion. Byte-for-byte behavioural twin of
//! `🔗️graphql/📦️packages/🐹️go/🔤️lexer/🐹️.go`, `🌳️ast/🐹️.go` and `🧩️parser/🐹️.go`, down to the diagnostic text and the byte offsets, so the
//! executor can be ported against this AST without re-deciding a single grammar question.

//#endregion 🧲️Header

use semio_framework_repo_graphql_schema::{NamedType, ObjectType, TypeRef};
use semio_framework_repo_identity::{emoji_text, entity, flat};
use semio_framework_repo_model::{
    derive_technology_kind, AnalyzeMetrics, AnalyzeResult, Breach, BreachPriority, Bundle, BundleKind, Checkpoint, Contributor, ContributorAddInput, ContributorContributionsStorage, ContributorLink,
    Definition, DefinitionKind, Draft, DraftCreateInput, File, FilterInput, Folder, FolderKind, Goal, GoalChangeInput, GoalCloseInput, GoalCreateInput, GoalDates, GoalDeleteInput,
    GoalManagementData, GoalReopenInput, GoalStatus, Interaction, InteractionFile, InteractionResource, Policy, PriorityCount, Range, Section, Statute, StatuteMeta, Technology, Territory, Ticket,
    TicketAgent, TicketChangeInput, TicketCloseInput, TicketDeleteInput, TicketManagementData, TicketOpenInput, TicketReopenInput, TicketStatus, Todo, TodoChangeInput, TodoCreateInput,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value as Json};
use std::collections::BTreeMap;
use std::fmt;

/// 🔌️ Explicit re-export: the projection and the coercion helpers speak `serde_json`, so a client of
/// this crate reaches that vocabulary through this crate and never declares the dependency itself.
pub use serde_json;

/// 🏗️ Explicit re-export: the executor serves the schema its sibling crate builds, so a client reaches the schema,
/// its builder and its inventory through this crate.
pub use semio_framework_repo_graphql_schema::{build_schema, schema_inventory, Schema};

//#region 🔖️Children

#[path = "🔤️grammar/🦀️.rs"]
mod grammar;
pub use grammar::*;

#[path = "🎥️context-port/🦀️.rs"]
mod context_port;
pub use context_port::*;

#[path = "🖼️sources/🦀️.rs"]
mod sources;
pub use sources::*;

#[path = "⚡️execution/🦀️.rs"]
mod execution;
pub use execution::*;

#[path = "🩻️recording-context/🦀️.rs"]
mod recording_context;
pub use recording_context::*;

//#endregion 🔖️Children

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests