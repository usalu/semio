//! 🧭️ The `repo_cli` facade: mounts the command-line children and re-exports their items, so every `repo_cli::name` path
//! the binaries and the language-agnostic test adapters use stays valid.

//#region 🧭️RepoCli

use semio_framework_repo_codebase::{Codebase, Scope as CodebaseScope};
use semio_framework_repo_graphql::{RecordingContext, RepoContext};
use semio_framework_repo_graphql_fs::FsRepoContext;
use semio_framework_repo_identity::{emoji_text, entity as identity_entity, flat};
use semio_framework_repo_model::{entity, DraftCreateInput, TicketStatus, TreeFilter, TreeNode, TreeNodeKind};
use serde_json::{json, Map, Value as Json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

pub use crate::event_stream::*;

#[path = "🎮️command-framework/🦀️.rs"]
mod command_framework;
pub use command_framework::*;

#[path = "🌳️command-tree/🦀️.rs"]
mod command_tree;
pub use command_tree::*;

#[path = "🗂️tree-source-and-search/🦀️.rs"]
mod tree_source_and_search;
pub use tree_source_and_search::*;

//#region 🚦️Verbs
/// 🚦️ One handler per verb, each a thin front for its domain owner.
#[path = "🚦️verbs/🦀️.rs"]
//#endregion 🚦️Verbs
pub mod verbs;

#[path = "🚀️run-and-projection/🦀️.rs"]
mod run_and_projection;
pub use run_and_projection::*;

//#endregion 🧭️RepoCli
