//! 🧰️ Drawing owned envelope decoder, recursive retirement, and retained store initializer.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_s_artifact_draw_drawing_host_arena::DRAWING_DUPLICATE_ID_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_LAYER_DEPTH;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_ITEMS;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_RETAINED_PAGE_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DRAWING_OWNED_FIELD_BYTES;
use semio_s_artifact_draw_drawing_host_arena::DrawingDecodedFieldRetirement;
use semio_s_artifact_draw_drawing_host_arena::DrawingFillCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingFixedOwnerCensus;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerAddress;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneWorkAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingLayerLocator;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapJob;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapStep;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBorrowError;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaOwner;
use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPool;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeLayers;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeSegments;
use semio_s_artifact_draw_drawing_host_arena::DrawingNativeText;
use semio_s_artifact_draw_drawing_host_arena::DrawingRebuiltLayerOwners;
use semio_s_artifact_draw_drawing_host_arena::DrawingSegmentsCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotBoundsAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotOwnerTotals;
use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotRetirementFactory;
use semio_s_artifact_draw_drawing_host_arena::DrawingStrokeCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingTextCloneAuthority;
use semio_s_artifact_draw_drawing_host_arena::DrawingTraversalFrame;
use semio_s_artifact_draw_drawing_host_arena::borrow_drawing_mutation_arena;
use semio_s_artifact_draw_drawing_host_arena::borrow_drawing_mutation_arena_from;
use semio_s_artifact_draw_drawing_host_arena::clone_drawing_string;
use semio_s_artifact_draw_drawing_host_arena::drawing_fill_clone_work_totals;
use semio_s_artifact_draw_drawing_host_arena::drawing_stroke_clone_work_totals;
use semio_s_artifact_draw_drawing_host_arena::initial_snapshot_clone_grant;
use semio_s_artifact_draw_drawing_host_digest::DrawingContainerRebuildAuthority;
use semio_s_artifact_draw_drawing_host_digest::DrawingDuplicateRewriteAuthority;
use semio_s_artifact_draw_drawing_host_digest::DrawingMutationDigestAuthority;
use semio_s_artifact_draw_drawing_host_digest::DrawingMutationWorksetPlan;
use semio_s_artifact_draw_drawing_host_digest::drawing_mutation_target_owner;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::{DrawingImageAsset, DrawingLayerNode, FillStyle, PathSegment, StrokeStyle};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[cfg(test)] use semio_s_artifact_draw_drawing_model::GradientStop;
use protocol::{Mutation, OpBinary};
use store::ErasedSnapshotRetirement as _;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};

use semio_s_artifact_draw_drawing_host_arena::native_retirement::DrawingRetirementOwner;

use semio_s_artifact_draw_drawing_host_arena::initialization_catalog_close::{close_initialization_catalog, initialization_catalog_close_demands};


//#region 🔖️RetainedStoreInitialization

#[path = "🧩️mutation-candidate/🦀️.rs"]
pub mod mutation_candidate;
use mutation_candidate::*;

#[path = "🪜️candidate-step/🦀️.rs"]
mod candidate_step;

#[path = "🚀️store-initialization/🦀️.rs"]
pub mod store_initialization;
pub use store_initialization::*;

#[path = "🔄️initialization-turn/🦀️.rs"]
pub mod initialization_turn;
use initialization_turn::*;

//#endregion 🔖️RetainedStoreInitialization


/// 🧷️ Allocation-free stand-in kept in the history ledger while its original forward is in sealed custody.
fn drawing_forward_placeholder() -> DrawingMutation {
    DrawingMutation::DeleteLayer(semio_s_artifact_draw_drawing_structure::delete_layer::mutation::DeleteLayer { layer_id: Default::default() })
}
