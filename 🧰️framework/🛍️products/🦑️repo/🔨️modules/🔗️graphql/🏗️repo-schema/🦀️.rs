//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🏗️ The repo GraphQL schema: the type vocabulary, the repo schema builders and the SDL rendering. A leaf of
//! `semio-framework-repo-graphql`, which executes documents against it. The committed SDL
//! `🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql` is the oracle shared with the Go twin in
//! `🔗️graphql/📦️packages/🐹️go/🐹️.go`.

//#endregion 🧲️Header

use serde_json::{json, Value as Json};
use std::collections::BTreeMap;

/// 🔌️ Explicit re-export: the argument defaults and the schema inventory speak `serde_json`, so a client of
/// this crate reaches that vocabulary through this crate and never declares the dependency itself.
pub use serde_json;

#[path = "🗺️type-vocabulary/🦀️.rs"]
mod type_vocabulary;
pub use type_vocabulary::*;

#[path = "🧱️common-schemas/🦀️.rs"]
mod common_schemas;
use common_schemas::*;

#[path = "🗃️repo-and-codebase-schemas/🦀️.rs"]
mod repo_and_codebase_schemas;
use repo_and_codebase_schemas::*;

#[path = "📜️section-and-statute-schemas/🦀️.rs"]
mod section_and_statute_schemas;
use section_and_statute_schemas::*;

#[path = "🤝️interaction-goal-and-todo-schemas/🦀️.rs"]
mod interaction_goal_and_todo_schemas;
use interaction_goal_and_todo_schemas::*;

#[path = "🎫️ticket-and-contributor-schemas/🦀️.rs"]
mod ticket_and_contributor_schemas;
use ticket_and_contributor_schemas::*;

#[path = "⌨️inputs-and-operations/🦀️.rs"]
mod inputs_and_operations;
use inputs_and_operations::*;

#[path = "🧾️build-and-sdl/🦀️.rs"]
mod build_and_sdl;
pub use build_and_sdl::*;

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
