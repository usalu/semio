//! 🩹️ Retained fixed-admission reconciliation for mounted plugin surfaces.

use semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey;
use semio_framework_job::StepContext;
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_ui_runtime::{ComponentTreeProducer, ComponentTreeProducerStep, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconcilePublishedAck, SurfaceReconcileReadyPatch, SurfaceReconcileRejected, SurfaceReconcileReservation, SurfaceReconcileTerminal, SurfaceReconciler, SURFACE_RECONCILE_ADMISSION_SLOTS};
use semio_framework_ui_runtime::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputTransfer, SurfaceReconcileOutputs};
use std::cell::RefCell;

#[path = "🎰️surface-slots-and-reconcile-grant/🦀️.rs"]
mod surface_slots_and_reconcile_grant;
#[cfg(test)]
use surface_slots_and_reconcile_grant::READY_PATCH_CAPACITY;
use surface_slots_and_reconcile_grant::{
    refused_slot_index, ClosingInstance, MountedReconcileOwner, MountedTreeTerminal,
    PatchTrackerState, ReadySlot, RejectedSlot, SurfaceSlot, TerminalSlot, UnadmittedSlot,
};
pub use surface_slots_and_reconcile_grant::{MountedReconcileGrant, PatchTracker};

#[path = "🚪️tracker-admission-and-reservation/🦀️.rs"]
mod tracker_admission_and_reservation;

#[path = "🏃️tracker-drive-and-work-predicates/🦀️.rs"]
mod tracker_drive_and_work_predicates;
use tracker_drive_and_work_predicates::{
    commit_generation, deferred_surface_ready, pending_render_ready, surface_instance,
};
pub use tracker_drive_and_work_predicates::{has_work, next_generation};

#[path = "📬️ready-patches-and-publication-acks/🦀️.rs"]
mod ready_patches_and_publication_acks;
pub use ready_patches_and_publication_acks::ready_output_is_admissible;

#[path = "🔚️instance-close-and-terminals/🦀️.rs"]
mod instance_close_and_terminals;
use instance_close_and_terminals::close_output;

#[cfg(any(test,feature="original-actor-context-testing"))]
#[path="🔁️retry/🧪️tests/🦀️.rs"]
pub mod original_retry_laws;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
#[cfg(test)]
extern crate semio_framework_os_kernel as dsl;

semio_framework_plugin_abi::component_persistent_local! {
    /// 🩹️ One `PatchTracker` shared by every instance this actor hosts (surfaces are already
    /// namespaced by their own `surface` string, which today embeds the instance — see
    /// `render_surface`'s key).
    // 🌉️ `thread_local!` initializer expressions run in a plain (non-const, non-async) context —
    // bridged via `resolve_ready` since every `::new()` here is a pure `Self::default()`.
    pub static PATCHES: PatchTracker = PatchTracker::new();
}
