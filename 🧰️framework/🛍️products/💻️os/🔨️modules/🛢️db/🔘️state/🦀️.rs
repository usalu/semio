//! 🗄️ `db_state` — hand-rolled persistent (immutable, structurally-shared) diff-state overlays for
//! the `db` crate family: a 32-way HAMT `PMap`, a bitmapped-trie `PVec`, a rope `PText`, a
//! weight-balanced-by-height `PTree`, and an adjacency-map `PGraph`, plus the content-addressing
//! (`blake3`) primitives and `TouchedRegion` descriptors the rest of the family builds document
//! overlays and conflict detection on top of. No `im`/`im-rc`/`rpds`/`imbl` dependency — every
//! `PMap` uses `Arc`-based path-copying so document authority state can move between process-pool
//! turns; the other single-owner structures remain `Rc`-based. All follow the frozen contract at
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_state` row).
//!
//! 🎯️ Design choice: every structure clones in O(1) (an `Arc` or `Rc` bump) regardless of whether its
//! element types implement `Clone` — `Clone` is implemented by hand rather than derived, since a
//! derived impl would wrongly require `K: Clone, V: Clone` just to bump a reference count. Mutating
//! operations (`insert`/`push_back`/`insert` text/…) are the ones that need `Clone` element bounds,
//! because a path-copy has to clone the *values* sitting along the rewritten path, not the whole
//! structure.
//!
//! 🎯️ Design choice: internal hash-bucket routing (the HAMT's `PMap` and the graph's adjacency
//! maps) uses `std::collections::hash_map::DefaultHasher` (deterministic, unseeded) rather than a
//! cryptographic hash — it only needs to be a stable *router*, not collision-resistant. Content
//! addressing (dedup/identity across snapshots) is the separate, deliberately blake3-based
//! `ContentAddressed`/`PageStore` mechanism in the `📇️Pages` region below.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_ids::DbError;
use std::rc::Rc;
use std::sync::Arc;

#[path = "🧾️canonical-pages-and-retained-state/🦀️.rs"]
mod canonical_pages_and_retained_state;
pub use canonical_pages_and_retained_state::{
    CanonicalEncode, Page, RetainedStateMap, StateCursorControl, StateEntry, StateEntryRejected,
};
use canonical_pages_and_retained_state::hash_bytes;
#[cfg(test)]
use canonical_pages_and_retained_state::RETAINED_STATE_ENTRIES;
#[cfg(test)]
pub use canonical_pages_and_retained_state::PageStore;

#[path = "🗂️persistent-map/🦀️.rs"]
mod persistent_map;
pub use persistent_map::PMap;

#[path = "📃️persistent-vector-and-text/🦀️.rs"]
mod persistent_vector_and_text;
pub use persistent_vector_and_text::{PText, PVec};

#[path = "🌳️persistent-tree-and-graph/🦀️.rs"]
mod persistent_tree_and_graph;
pub use persistent_tree_and_graph::{PGraph, PTree};

#[path = "🪟️overlay-and-touched-regions/🦀️.rs"]
mod overlay_and_touched_regions;
pub use overlay_and_touched_regions::{
    BaseSource, EmptyBase, OverlayRoot, TouchKind, TouchedRegion, TouchedSet,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
