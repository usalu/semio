//! 🎮️ The canonical command registry: every runnable thing of the workspace is one entry with an
//! identity, a verb, a label, the parameters it accepts and the launch it resolves to. Entries come
//! from build targets, the playground catalog, the `metadata.semio.dashboard` declarations of project
//! manifests, the `🎮️commands.json` of open tickets and the repo domain. Nothing else is a command.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📚️inventory/🦀️.rs
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌳️command-tree/🦀️.rs

use dashboard_core::env_contract::RUNNER_ENV;
use dashboard_core::xxh3_64;
use dashboard_repo_domain::{RepoAction, RepoImplementation};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use ui_locale::Locale;
use ui_tui::tui::text::elide_end;

// #region 🔖️Wire
/// 🏷️ The launch types the registry produces and the daemon wire carries, which the launch contract owns: what a running task is
/// called, the address its output announces when it is ready and the processes a resolved launch starts.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧾️launch/🦀️.rs
use dashboard_launch::{Launch, LaunchProcess, Ready, Stop, TaskLabel};
// #endregion 🔖️Wire

#[path = "🪪️command-identity/🦀️.rs"]
mod command_identity;
pub use command_identity::*;

#[path = "🧾️declarations/🦀️.rs"]
mod declarations;
pub use declarations::*;

#[path = "📋️facts/🦀️.rs"]
mod facts;
pub use facts::*;

#[path = "🧬️model/🦀️.rs"]
mod model;
pub use model::*;

#[path = "⚖️run-policy/🦀️.rs"]
mod run_policy;
pub use run_policy::*;

#[path = "🛂️tokens-and-validation/🦀️.rs"]
mod tokens_and_validation;
pub use tokens_and_validation::*;

#[path = "📚️registry-record/🦀️.rs"]
mod registry_record;
pub use registry_record::*;

#[path = "🧶️build-and-derive/🦀️.rs"]
mod build_and_derive;

#[path = "🔍️lookup-and-search/🦀️.rs"]
mod lookup_and_search;

#[path = "🚀️resolve-and-launch/🦀️.rs"]
mod resolve_and_launch;

#[path = "🩺️check-and-json/🦀️.rs"]
mod check_and_json;

#[path = "🪶️entry-draft/🦀️.rs"]
mod entry_draft;
use entry_draft::*;

// #region 🔖️RunPolicy
#[path = "../../📚️library/🏗️build/🕸️graph/🦀️rust/🦀️.rs"]
mod publication;
pub use publication::graph_file;

// #endregion 🔖️RunPolicy
