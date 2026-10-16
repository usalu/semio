//! 🗣️ Repo language table: section parsing, definition parsing, scope identifiers and file headers.
//!
//! Every parsing rule comes from `🧬️schema/🔣️languages.json`, embedded here and read at init by the
//! Go twin, so the two implementations cannot drift. Patterns are the declarative token language of
//! `🧬️schema/🔣️.json`, matched by the hand-rolled backtracking matcher below — this crate depends on
//! no regular-expression engine.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

//#region 🔖️Children

#[path = "🪪️identity-pending/🦀️.rs"]
mod identity_pending;
pub use identity_pending::*;

#[path = "🗂️language-table/🦀️.rs"]
mod language_table;
pub use language_table::*;

#[path = "🧩️pattern-matcher/🦀️.rs"]
mod pattern_matcher;
pub use pattern_matcher::*;

#[path = "📑️section-parsing/🦀️.rs"]
mod section_parsing;
pub use section_parsing::*;

#[path = "🧷️definition-parsing/🦀️.rs"]
mod definition_parsing;
pub use definition_parsing::*;

#[path = "🧭️scopes-and-headers/🦀️.rs"]
mod scopes_and_headers;
pub use scopes_and_headers::*;

//#endregion 🔖️Children

// #region 🔖️Records

// 📐️ `Section`, `Definition` and `DefinitionKind` are owned by `🔨️modules/📐️model` and re-exported
// here so a consumer of this crate never has to name the model crate to hold a parse result.
pub use semio_framework_repo_model::{Definition, DefinitionKind, Section};

/// 📐️ A definition before it is given a canonical kind and file identity. The raw keyword survives
/// here — `Definition` only carries the canonical kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionRange {
    pub name: String,
    pub kind: String,
    pub start: i64,
    pub end: i64,
    pub excerpt: String,
}

// #endregion 🔖️Records

// #region 🔖️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
