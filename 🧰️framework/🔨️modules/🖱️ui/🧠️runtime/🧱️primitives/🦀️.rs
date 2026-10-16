//! 🧱️ The reconcile primitives — bounded containers, admission limits and censuses, record-diff cursors and the tree-retirement cursor every surface reconciliation is built from. Pure bounded-state building blocks: no registry, no job, no renderer.

#[path = "🗃️containers/🦀️.rs"]
mod containers;
#[path = "🚧️limits/🦀️.rs"]
mod limits;
#[path = "📊️census/🦀️.rs"]
mod census;
#[path = "🧩️records/🦀️.rs"]
mod records;
#[path = "../♻️retirement/🌲️tree/🦀️.rs"]
mod tree_retirement;
#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
pub mod testkit;

pub use containers::{SURFACE_RECONCILE_FIXED_NODES, NodeIdentity, SurfaceFixedVec, SurfaceLinearMap, SurfaceLinearSet};
#[cfg(any(test, feature = "testkit"))]
pub use containers::{SURFACE_RECONCILE_FIXED_OPS, assert_unique_child_keys, identity_of};
pub use limits::{SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_FLOOR_BYTES, SURFACE_RECONCILE_FLOOR_ITEMS, release_surface_reconcile, release_surface_reconcile_one, reserve_surface_reconcile_floor, surface_reconcile_limits_admissible, SURFACE_RECONCILE_MAX_ITEMS, SURFACE_RECONCILE_PAGE_BYTES, SURFACE_RECONCILE_SEMANTIC_COPIES, SURFACE_RECONCILE_SURFACE_BYTES, SURFACE_RECONCILE_VALUE_DEPTH, SurfaceReconcileFault, SurfaceReconcileLimits, SurfaceReconcileUsage};
#[cfg(any(test, feature = "testkit"))]
pub use limits::admit_vec_backing;
pub use census::{SURFACE_RECONCILE_ALLOCATION_ITEMS_PER_CENSUS_ITEM, SurfaceSemanticCensusCursor, SurfaceSemanticCensusStep, SurfaceSemanticMapPage, SurfaceSemanticUsage, SurfaceSemanticValueFrame, surface_subtree_items};
pub use records::{ExistingComponentComparison, FlatPresentedNode, FreshRecordClone, PresentationFrame, RecordDiffCursor, RecordOwnedCopy, RecordSource, RemovalFrame, build_record_owned, diff_record_field, retire_fresh_record_one, retire_record_one};
#[cfg(any(test, feature = "testkit"))]
pub use records::build_record;
pub use tree_retirement::SurfaceTreeRetireCursor;

#[cfg(test)]
use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️primitives-unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"]
mod tree_retirement_tests;
//#endregion 🧪️Tests
