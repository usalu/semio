//! ♻️ The reconcile authority — the thin [`SurfaceReconciler`] owner that composes a retained shadow with its handback reservation, the credit reservation, the process-wide handback registry, the resumable job, the generation-qualified patch and document owners, the terminal close authority and the surface output pool. The shadow and cursor it drives live in `semio_framework_ui_runtime_reconcile_cursor`; the bounded primitives in `semio_framework_ui_runtime_reconcile_primitives`.

use std::mem::{size_of, take};
use std::sync::{LazyLock, Mutex, MutexGuard, OnceLock};

use semio_framework_ui_runtime_presentation::{ComponentTree, UiNativeResourceLoans, UiNativeResources};
use semio_framework_ui_runtime_reconcile_primitives::{SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_PAGE_BYTES, SurfaceReconcileFault, SurfaceReconcileLimits, SurfaceReconcileUsage, SurfaceTreeRetireCursor, release_surface_reconcile, release_surface_reconcile_one, reserve_surface_reconcile_floor, surface_reconcile_limits_admissible};
use semio_framework_ui_runtime_reconcile_cursor::{SurfaceShadow, SurfaceReconcileCursor, SurfaceReconcileStep};
#[cfg(test)]
use semio_framework_ui_runtime_presentation::TreeNode;

#[path = "../📤️output/🦀️.rs"]
mod output;
pub use output::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputs, SurfaceReconcileOutputTransfer};

#[path = "🧮️surface-reconciler/🦀️.rs"]
mod surface_reconciler;
pub use surface_reconciler::SurfaceReconciler;

#[path = "🎫️reservations-and-registry/🦀️.rs"]
mod reservations_and_registry;
pub use reservations_and_registry::{
    surface_reconcile_registry_census, SurfaceReconcileRegistryCensus, SurfaceReconcileReservation,
    SurfaceReconcileReservationRefusal, SURFACE_RECONCILE_ADMISSION_SLOTS,
    SURFACE_RECONCILE_AGGREGATE_BYTES, SURFACE_RECONCILE_AGGREGATE_ITEMS,
};
use reservations_and_registry::{
    register_surface_reconcile_backing, reserve_surface_reconcile, SurfaceReconcileJobPhase,
};

#[path = "🤝️handback-registry/🦀️.rs"]
mod handback_registry;
pub use handback_registry::{SurfaceReconcileHandbackKey, SURFACE_RECONCILE_HANDBACK_SLOTS};
use handback_registry::{
    acquire_reserved_surface_reconcile_handback, acquire_surface_reconcile_handback,
    release_surface_reconcile_handback, release_surface_reconcile_handback_in,
    release_surface_reconcile_handback_one, reserve_surface_reconcile_handback,
    try_reserve_surface_reconcile_handback, SurfaceReconcileHandbackRegistry,
    SurfaceReconcileHandbackReservation, SurfaceReconcileHandbackSlot, SURFACE_RECONCILE_HANDBACKS,
};

#[path = "🎟️retained-authority/🦀️.rs"]
mod retained_authority;
use retained_authority::{handback_surface_reconcile, SurfaceReconcileRetained};

#[path = "📄️document-producer-and-patches/🦀️.rs"]
mod document_producer_and_patches;
pub use document_producer_and_patches::{
    SurfaceDocumentFault, SurfaceDocumentOutcome, SurfaceDocumentProducer,
    SurfaceDocumentProducerStep, SurfaceReconcileJobStep, SurfaceReconcilePublishedAck,
    SurfaceReconcilePublishedPatch, SurfaceReconcileReadyPatch,
};
#[cfg(test)]
use document_producer_and_patches::pending_surface_patch;

#[path = "🏃️reconcile-job/🦀️.rs"]
mod reconcile_job;
pub use reconcile_job::SurfaceReconcileJob;

#[path = "🏁️rejected-and-terminal/🦀️.rs"]
mod rejected_and_terminal;
pub use rejected_and_terminal::{
    close_surface_reconcile_handback_one, take_surface_reconcile_terminal, SurfaceReconcileRejected,
    SurfaceReconcileTerminal,
};

#[path = "🧪️registry-test-support-and-drain/🦀️.rs"]
mod registry_test_support_and_drain;
pub use registry_test_support_and_drain::{
    surface_reconcile_registry_test_guard, SurfaceReconcileRegistryTestGuard,
};
#[cfg(test)]
use registry_test_support_and_drain::drain_surface_reconcile_registry_until_idle;

//#region ⏭️ResumableReconcile

//#region 🎟️RetainedAuthority

#[cfg(test)]
#[path = "../🩹️patch/🧪️tests/🩹️patch/🦀️.rs"]
mod patch_handoff_tests;

#[cfg(test)]
#[path = "../🚪️handback/🧪️tests/🚪️handback/🦀️.rs"]
mod handback_entry_tests;

//#endregion 🎟️RetainedAuthority

//#endregion ⏭️ResumableReconcile

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️authority-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
