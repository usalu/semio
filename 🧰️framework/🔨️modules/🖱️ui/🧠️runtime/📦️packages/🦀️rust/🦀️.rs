//! 🧠️ The headless UI runtime — presentation state and the reconciler that turns it into
//! [`ui_contract`] patches. It renders nothing and knows about no renderer.
//!
//! ```text
//! projections → Entity<T> state → Present → ComponentTree → keyed reconcile → UiPatch
//! ```
//!
//! Two properties make this crate what it is:
//!
//! 1. **Production frame authority is mounted by the product host.** This headless crate retains the
//!    entity, presentation, and reconciliation primitives; its former standalone transaction is a
//!    test oracle only so production cannot select an unmounted alternate frame authority.
//! 2. **It runs on both sides of the plugin boundary.** A guest plugin embeds it to produce patches;
//!    a host embeds it to drive its own screens. Hence no `Send` bounds and no dependency beyond the
//!    contract and shared job protocol — it compiles for `wasm32-wasip2` and
//!    `wasm32-unknown-unknown` by construction.
//!
//! The facade owns no logic: it re-exports, as explicit items, what its consumers use from the producer crates
//! `semio_framework_ui_runtime_exchange` (command gateway, projection inbox, presence), `..._presentation`
//! (entities, tracking, intent routing, component tree), `..._reconcile_primitives` (bounded containers, limits,
//! census, record diff, tree retirement), `..._reconcile_cursor` (retained shadow and resumable cursor) and
//! `..._reconcile` (handback registry, job, patches, terminal, output pool). Only the test oracle of the former
//! standalone transaction is mounted here.

pub use semio_framework_ui_runtime_exchange::PresenceHub;
pub use semio_framework_ui_runtime_presentation::{COMPONENT_TREE_PRODUCER_DEPTH, ComponentTree, ComponentTreeProducer, ComponentTreeProducerStep, DEFAULT_REVISION_TOLERANCE, TreeNode, UI_NATIVE_MESH_MESSAGE_BYTES, UiNativeMeshChannel, UiNativeMeshMessage, UiNativeMeshResource, UiNativeResourceLoans, UiNativeResources, is_stale_intent};
pub use semio_framework_ui_runtime_reconcile::{SURFACE_RECONCILE_ADMISSION_SLOTS, SURFACE_RECONCILE_AGGREGATE_BYTES, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconcileOutputReservation, SurfaceReconcileOutputTransfer, SurfaceReconcileOutputs, SurfaceReconcilePublishedAck, SurfaceReconcilePublishedPatch, SurfaceReconcileReadyPatch, SurfaceReconcileRejected, SurfaceReconcileReservation, SurfaceReconcileTerminal, SurfaceReconciler, close_surface_reconcile_handback_one, surface_reconcile_registry_census, surface_reconcile_registry_test_guard};
pub use semio_framework_ui_runtime_reconcile_primitives::{SURFACE_RECONCILE_FLOOR_BYTES, SURFACE_RECONCILE_FLOOR_ITEMS, SURFACE_RECONCILE_MAX_ITEMS, SURFACE_RECONCILE_PAGE_BYTES, SurfaceReconcileLimits, surface_subtree_items};

#[cfg(test)]
use semio_framework_ui_runtime_exchange::{Command, CommandGateway, CommandId, CommandSink, CorrelationId, SinkFull, InboxOverflow, OptimisticStatus, ProjectionDelta, ProjectionInbox};
#[cfg(test)]
use semio_framework_ui_runtime_presentation::{Context, DeferredKey, DeferredOp, DependencyTracker, DispatchOutcome, Entity, EntityId, EntityStore, HandleIntent, Present, PresentCx};

#[cfg(test)]
#[path = "../../🔄️transaction/🦀️.rs"]
mod transaction;
#[cfg(test)]
pub use transaction::*;
