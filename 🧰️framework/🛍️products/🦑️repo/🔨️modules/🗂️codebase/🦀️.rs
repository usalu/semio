//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🗂️ Repository codebase: the bundle, folder and file walk over a repository root, the technology
//! and bundle detection that gives every path its owning bundle, and the artifact id and uri
//! builders every other module addresses artifacts with.
//!
//! Behaviour twin of `github.com/usalu/semio/repo/codebase`. The Go original keeps its root
//! directory and its technology, folder-kind and gitignore caches in package-level globals; this
//! crate binds them to an explicit [`Codebase`] handle instead, so a walk can be pointed at any
//! root without a process-wide mutation.
//!
//! Everything above this module in the dependency DAG — statutes, tickets, goals, contributors —
//! contributes its own aggregates; this crate never reaches up for them. Breaches enter as already
//! decoded [`Breach`] records through [`CodebaseContext::with_breachs`].
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/🧬️schema/🔣️.json

//#endregion 🧲️Header

use semio_framework_repo_identity::{emoji_text, flat, normalize_path};
use semio_framework_repo_languages::{language_for_path, parse_definitions, parse_sections};
use semio_framework_repo_model::{
    bundle_kind_emoji, definition_kind_emoji, derive_technology_kind, file_kind_emoji, folder_kind_emoji, technology_kind_emoji, Breach, Bundle, BundleKind, BundleMetricsInternal, CbTreeNode, CbTreeNodeKind, Codebase as CodebaseSnapshot, CodebaseBreach,
    CodebaseBundle, CodebaseDefinition, CodebaseFile, CodebaseFolder, CodebaseSection, Definition, BreachFile, BreachFolder, BreachPriority, DefinitionMetricsInternal, FileBreachRef, FileMetricsInternal,
    FileRange, FolderKind, FolderMetricsInternal, Package, RangePosition, Section, SectionMetricsInternal, Technology, TechnologyKind,
};
use semio_framework_repo_workspace::glob_match;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

//#region 🛤️Paths

/// 🧹️ Reexported so a walk never reaches into `🪪️identity` for the `filepath` twins it keys on.
pub use semio_framework_repo_identity::{base_of, dir_of, ext_of};

//#endregion 🛤️Paths

#[path = "🙈️generated-folders-and-bundle-labels/🦀️.rs"]
mod generated_folders_and_bundle_labels;
pub use generated_folders_and_bundle_labels::*;

#[path = "🏠️codebase-handle/🦀️.rs"]
mod codebase_handle;
pub use codebase_handle::*;

#[path = "🏗️technology-detection/🦀️.rs"]
mod technology_detection;

#[path = "🚫️ignore-rules/🦀️.rs"]
mod ignore_rules;

#[path = "🚶️walk/🦀️.rs"]
mod walk;

#[path = "🪪️artifact-ids/🦀️.rs"]
mod artifact_ids;

#[path = "🧭️scope-and-helpers/🦀️.rs"]
mod scope_and_helpers;
pub use scope_and_helpers::*;

#[path = "📚️codebase-context/🦀️.rs"]
mod codebase_context;
pub use codebase_context::*;

#[path = "🧱️bundle-folder-and-file-aggregates/🦀️.rs"]
mod bundle_folder_and_file_aggregates;

#[path = "🌳️section-definition-breach-and-tree-aggregates/🦀️.rs"]
mod section_definition_breach_and_tree_aggregates;
use section_definition_breach_and_tree_aggregates::*;

//#region 🪪️Section Ids

/// 🔖️ Reexported: the section, definition and file-kind derivations live in `📐️model`, next to the
/// `GetArtifactID` grammar they feed, exactly as the Go twin has them.
pub use semio_framework_repo_model::{build_definition_id, build_section_id, derive_file_kind, is_test_function_name};

//#endregion 🪪️Section Ids

//#region 🔁️Reexports

pub use semio_framework_repo_model::{Bundle as ModelBundle, DefinitionKind as ModelDefinitionKind, FileKind as ModelFileKind, FolderKind as ModelFolderKind};

/// 🏷️ Reexported so a caller never has to reach into `model` for the derivation this crate keys on.
pub use semio_framework_repo_model::derive_definition_kind as definition_kind_of;

//#endregion 🔁️Reexports

//#region 🧪️Tests

#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
