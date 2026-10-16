//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 📐️ Domain model of the `🦑️repo` product: the pure data shapes every other repo module
//! exchanges, plus the slug vocabularies and the derivations that are defined by the shape
//! alone. Nothing here touches the filesystem, a process or a clock.
//!
//! The wire contract is `🧬️schema/🔣️.json` (JSON Schema 2020-12) and the allowed LLM /
//! effort / client tables are `🧬️schema/🔣️allowed-values.json`, embedded by `📋️allowed-values` with
//! [`include_str!`] and embedded by the Go twin with `go:embed` — one source of truth.
//!
//! Serialisation is byte-compatible with Go `encoding/json` over the same structs:
//! `omitempty` maps to `skip_serializing_if`, a Go `nil` slice or map maps to `Option::None`
//! (which encodes as `null`, exactly as Go does) while an `omitempty` slice maps to a plain
//! `Vec` (both `nil` and empty vanish), and every map is a [`BTreeMap`] so key order matches
//! Go's sorted map encoding.
//!
//! See also `🧰️framework/🛍️products/🦑️repo/AGENTS.md` §🪪 Identification.

//#region 🔌️Adapters
use std::collections::BTreeMap;
use std::fmt;
use std::sync::OnceLock;

use semio_framework_repo_identity::{emoji_text, entity as entity_emoji, ext_of, extract_entity_emoji, flat};
use serde::{Deserialize, Serialize};
use serde_json::Value;
//#endregion 🔌️Adapters

//#region 🪶️Encoding Helpers

/// 🚫️ Serde predicate for a Go `bool` carrying `omitempty`.
fn is_false(value: &bool) -> bool {
    !*value
}

/// 0️⃣ Serde predicate for a Go `int` carrying `omitempty`.
fn is_zero(value: &i64) -> bool {
    *value == 0
}

//#endregion 🪶️Encoding Helpers

//#region ❌️Errors

/// ❌️ Every failure this module can produce. One variant per error class so a test can assert
/// the class without matching a rendered message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelError {
    /// 🚫️ A value did not resolve against one of the allowed vocabularies.
    NotAllowed {
        /// 🏷️ The vocabulary that rejected the value: `llm`, `effort` or `client`.
        field: &'static str,
        /// 🔤️ The normalised slug that was looked up.
        slug: String,
        /// 📋️ The full allowed vocabulary, in declaration order.
        allowed: Vec<String>,
    },
}

impl ModelError {
    /// 🏷️ The stable error class, for language-agnostic assertions.
    pub fn class(&self) -> &'static str {
        match self {
            ModelError::NotAllowed { .. } => "not-allowed",
        }
    }
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::NotAllowed { field, slug, allowed } => write!(
                f,
                "{field} '{slug}' is not allowed. Please use one of: {}",
                allowed.join(", ")
            ),
        }
    }
}

impl std::error::Error for ModelError {}

//#endregion ❌️Errors

#[path = "📋️allowed-values/🦀️.rs"]
mod allowed_values;
pub use allowed_values::*;

#[path = "🏷️kinds/🦀️.rs"]
mod kinds;
pub use kinds::*;

#[path = "🌐️repo-aggregates/🦀️.rs"]
mod repo_aggregates;
pub use repo_aggregates::*;

#[path = "🎫️work-aggregates/🦀️.rs"]
mod work_aggregates;
pub use work_aggregates::*;

#[path = "🖥️codebase-aggregate/🦀️.rs"]
mod codebase_aggregate;
pub use codebase_aggregate::*;

#[path = "🌳️tree-aggregate/🦀️.rs"]
mod tree_aggregate;
pub use tree_aggregate::*;

#[path = "🎨️ansi/🦀️.rs"]
pub mod ansi;

#[path = "🪪️entity/🦀️.rs"]
pub mod entity;

//#region 🔁️Wire Round Trip

macro_rules! wire_types {
    ($($name:literal => $ty:ty),+ $(,)?) => {
        /// 📦️ Every domain type this crate can decode and re-encode by name, in declaration order.
        pub fn wire_type_names() -> &'static [&'static str] {
            &[$($name),+]
        }

        /// 🔁️ Decodes one JSON document into the named domain type and encodes it again.
        ///
        /// The wire contract is what other modules depend on, so it is exercised through this
        /// owned entry point rather than by re-exporting a serialisation crate: nothing outside
        /// this codebase appears in the signature.
        pub fn round_trip_json(type_name: &str, text: &str) -> Result<String, String> {
            match type_name {
                $($name => {
                    let value: $ty = serde_json::from_str(text).map_err(|error| error.to_string())?;
                    serde_json::to_string(&value).map_err(|error| error.to_string())
                })+
                other => Err(format!("unknown domain type {other}")),
            }
        }
    };
}

wire_types! {
    "Statute" => Statute,
    "Range" => Range,
    "LineMetrics" => LineMetrics,
    "CountMetrics" => CountMetrics,
    "PriorityCount" => PriorityCount,
    "AnalyzeMetrics" => AnalyzeMetrics,
    "Package" => Package,
    "Bundle" => Bundle,
    "Technology" => Technology,
    "Repo" => Repo,
    "Folder" => Folder,
    "File" => File,
    "Definition" => Definition,
    "Section" => Section,
    "ContributorIcons" => ContributorIcons,
    "ContributorLink" => ContributorLink,
    "ContributorTicket" => ContributorTicket,
    "ContributorCheckpoint" => ContributorCheckpoint,
    "ContributorContributionsStorage" => ContributorContributionsStorage,
    "Contributor" => Contributor,
    "Checkpoint" => Checkpoint,
    "CheckpointDiffRename" => CheckpointDiffRename,
    "CheckpointDiffStats" => CheckpointDiffStats,
    "InteractionFile" => InteractionFile,
    "TicketPlan" => TicketPlan,
    "TicketAgentPlanStep" => TicketAgentPlanStep,
    "TicketAgent" => TicketAgent,
    "TicketManagementData" => TicketManagementData,
    "Ticket" => Ticket,
    "TicketData" => TicketData,
    "TicketFile" => TicketFile,
    "TicketFileRenamed" => TicketFileRenamed,
    "TicketDiffSet" => TicketDiffSet,
    "TicketSectionMetrics" => TicketSectionMetrics,
    "TicketFileMetricsEntry" => TicketFileMetricsEntry,
    "GoalDates" => GoalDates,
    "GoalManagementData" => GoalManagementData,
    "Draft" => Draft,
    "Location" => Location,
    "Todo" => Todo,
    "Territory" => Territory,
    "StatuteMeta" => StatuteMeta,
    "Breach" => Breach,
    "RangePosition" => RangePosition,
    "FileRange" => FileRange,
    "BreachFile" => BreachFile,
    "BreachFolder" => BreachFolder,
    "CodebaseBreach" => CodebaseBreach,
    "BundleMetricsInternal" => BundleMetricsInternal,
    "CodebaseFile" => CodebaseFile,
    "CbTreeNode" => CbTreeNode,
    "Codebase" => Codebase,
    "TreeNode" => TreeNode,
    "TicketNode" => TicketNode,
    "GoalNode" => GoalNode,
    "SemanticChange" => SemanticChange,
    "DiffLines" => DiffLines,
    "FileListInput" => FileListInput,
    "TicketOpenInput" => TicketOpenInput,
    "TicketCloseInput" => TicketCloseInput,
    "FilterInput" => FilterInput,
}

//#endregion 🔁️Wire Round Trip
