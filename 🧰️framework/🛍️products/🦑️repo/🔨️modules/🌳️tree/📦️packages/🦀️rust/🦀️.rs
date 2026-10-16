//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//! 🌳️ Repository tree projection: the monorepo tree assembled from already-loaded records, the
//! goal/ticket tree, the statute and territory trees, parent-id propagation, filtering,
//! deterministic sorting, in-memory search, text/markdown/Mermaid rendering and the
//! content-digest tree cache.
//!
//! Behaviour twin of `github.com/usalu/semio/repo/tree`. Nothing in this crate touches the
//! filesystem, git or a process: every door to the outside world is a trait in
//! [`ports`](#ports) — [`TreeSource`] supplies the records, [`ArtifactIdentifier`] mints the
//! artifact ids parent-id propagation needs, [`EntityRenderer`] renders one entity line and
//! [`StatuteCatalog`] resolves statute metadata. The CLI satisfies them with the loader,
//! identity and statutes implementations; the language-agnostic tests satisfy them with the
//! committed fixtures, so no test needs a repository on disk.
//!
//! Ordering is deterministic everywhere. The Go original leans on Go map iteration in the
//! codebase and bundle assembly and then repairs the order with `sortTreeChildren`; this crate
//! keeps the same repair but builds through ordered maps, so the intermediate order is stable as
//! well.
//!
//! See `🧬️schema/🔣️.json` for the wire shapes and `🧪️tests/` for the language-agnostic cases.

//#endregion 🧲️Header

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// 🔣️ The JSON value a tree node's data map carries. Re-exported explicitly so a client never
/// has to name the encoder crate itself.
pub use serde_json::Value;

/// 🔏️ The digest and compression codec of the cache envelope, owned by 📜️statutes and shared
/// with this crate so one hand-rolled SHA-256 and one hand-rolled gzip serve both caches.
pub use semio_framework_repo_statutes::{gzip_decode, gzip_encode, sha256_hex};

pub use semio_framework_repo_model::{
    BreachPriority, BundleKind, FileKind, GoalNode, Statute, StatuteMeta, Technology, TechnologyKind, Territory, TicketNode, TreeFilter, TreeNode, TreeNodeKind,
};

//#region 🔖️Children

#[path = "🧱️tree-model/🦀️.rs"]
mod tree_model;
pub use tree_model::*;

#[path = "🩻️monorepo-tree/🦀️.rs"]
mod monorepo_tree;
pub use monorepo_tree::*;

#[path = "🎯️goal-tree/🦀️.rs"]
mod goal_tree;
pub use goal_tree::*;

#[path = "⚖️statute-tree/🦀️.rs"]
mod statute_tree;
pub use statute_tree::*;

#[path = "🧹️sort-filter-search/🦀️.rs"]
mod sort_filter_search;
pub use sort_filter_search::*;

#[path = "🎨️rendering/🦀️.rs"]
mod rendering;
pub use rendering::*;

#[path = "📌️cache/🦀️.rs"]
mod cache;
pub use cache::*;

//#endregion 🔖️Children

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
