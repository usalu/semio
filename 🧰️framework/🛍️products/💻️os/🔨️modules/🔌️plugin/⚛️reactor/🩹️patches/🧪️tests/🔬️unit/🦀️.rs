use semio_framework_ui_runtime::{ComponentTree, ComponentTreeProducer, ComponentTreeProducerStep, SURFACE_RECONCILE_ADMISSION_SLOTS, SurfaceReconcileOutputs, SurfaceReconcilePublishedAck, SurfaceReconcileReadyPatch, SurfaceReconcileTerminal, SurfaceReconciler};
use crate::{MountedReconcileGrant, MountedReconcileOwner, MountedTreeTerminal, PatchTracker, PatchTrackerState, READY_PATCH_CAPACITY, SurfaceSlot, TerminalSlot, has_work, next_generation, surface_instance};
use semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey;
use semio_framework_job::StepContext;
use semio_framework_ui_contract as ui_contract;
#[doc(hidden)]
use semio_framework_ui_runtime::TreeNode;

include!("🪤️panic-hooks-and-tracker-fixtures/🦀️.rs");
include!("🚪️mounted-output-admission-laws/🦀️.rs");
include!("🌳️mounted-tree-publication-laws/🦀️.rs");
include!("🔔️obsolete-feedback-acks-and-reservation-cap/🦀️.rs");
include!("🌪️stale-generation-resize-and-terminal-saturation/🦀️.rs");
include!("♾️generation-exhaustion-and-abandoned-jobs/🦀️.rs");
include!("🔓️reservation-release-and-registry-idle/🦀️.rs");
include!("🏋️real-size-surfaces-and-producer-shell-denials/🦀️.rs");
