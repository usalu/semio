//! ⏭️ The retained surface shadow and the resumable reconcile cursor that diffs a presented tree against it — presentation discovery, identity allocation, postorder record diffing and stale-tree removal each advance one node at a time, and the shadow is read-only until the cursor completes. The authority crate composes the shadow with the process-wide handback registry; this crate knows no registry, no job and no renderer.
//!
//! The one property that makes the emitted patches worth anything: identity comes from **`(parent, key)`, never from
//! position**. A node keeps its [`ui_contract::UiNodeId`] across reorders, insertions and removals of its siblings,
//! which is exactly what lets renderer-side state (scroll offset, focus, a DOM node, a GPU cache entry) survive a
//! re-present instead of being torn down and rebuilt every frame.

use std::mem::{size_of, take};
#[cfg(any(test, feature = "testkit"))]
use std::collections::HashSet;

use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode, UiNativeResourceLoans, UiNativeResources};
use semio_framework_ui_runtime_reconcile_primitives::{ExistingComponentComparison, FlatPresentedNode, FreshRecordClone, NodeIdentity, PresentationFrame, RecordDiffCursor, RecordOwnedCopy, RemovalFrame, SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_FIXED_NODES, SURFACE_RECONCILE_FLOOR_BYTES, SURFACE_RECONCILE_FLOOR_ITEMS, SURFACE_RECONCILE_PAGE_BYTES, SURFACE_RECONCILE_SURFACE_BYTES, SURFACE_RECONCILE_VALUE_DEPTH, SurfaceFixedVec, SurfaceLinearMap, SurfaceLinearSet, SurfaceReconcileFault, SurfaceReconcileLimits, SurfaceReconcileUsage, SurfaceSemanticCensusCursor, SurfaceSemanticCensusStep, SurfaceSemanticUsage, SurfaceTreeRetireCursor, build_record_owned, diff_record_field, release_surface_reconcile_one, reserve_surface_reconcile_floor, retire_fresh_record_one, retire_record_one, surface_reconcile_limits_admissible};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_ui_runtime_reconcile_primitives::{assert_unique_child_keys, build_record, identity_of};

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
pub mod testkit;

#[path = "🪞️surface-shadow-and-record-read/🦀️.rs"]
mod surface_shadow_and_record_read;
pub use surface_shadow_and_record_read::{SurfaceRecordRead, SurfaceShadow};

#[path = "🧭️cursor-state-and-construction/🦀️.rs"]
mod cursor_state_and_construction;
pub use cursor_state_and_construction::{
    SurfaceReconcileCursor, SurfaceReconcileStage, SurfaceReconcileStep,
};

#[path = "🚶️cursor-step/🦀️.rs"]
mod cursor_step;

#[path = "🧩️component-advance-and-retirement/🦀️.rs"]
mod component_advance_and_retirement;

#[path = "🔮️reconcile-oracle/🦀️.rs"]
mod reconcile_oracle;
#[cfg(any(test,feature="testkit"))]
use reconcile_oracle::SurfaceReconcileOracle;

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️cursor-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
