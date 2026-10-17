#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::request_drawing_mutation_arena_pool;
use semio_s_artifact_draw_drawing_style::set_layer_fill_rule::mutation::SetLayerFillRule;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_LAYER_DEPTH;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_MAXIMUM_NESTED_BYTES;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_CONTAINER_SLOT_CAPACITY;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_OVERLAY_PAGE_CAPACITY;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_OWNED_FIELD_BYTES;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingDecodedFieldRetirement;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationRetirementFactory;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingAssetBoundsCursor;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingCloneWorkTotals;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_ARENA_POOL;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DRAWING_MUTATION_ARENA_POOL_CAPACITY;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolBootstrap;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapAdmission;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaProcessState;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPoolAvailability;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneAuthority;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingLayerCloneWorkAuthority;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapJob;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBootstrapStep;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaBorrowError;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaOwner;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingMutationArenaPool;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingNativeLayers;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingNativeText;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::DrawingSnapshotRetirementFactory;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::borrow_drawing_mutation_arena;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::initial_snapshot_clone_grant;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_digest::DrawingContainerRebuildAuthority;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_digest::DrawingMutationDigestAuthority;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_digest::DrawingMutationWorksetPlan;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::{DrawingImageAsset, DrawingLayerNode, FillStyle, PathSegment, StrokeStyle};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[allow(unused_imports)] #[cfg(test)] use semio_s_artifact_draw_drawing_model::GradientStop;
#[allow(unused_imports)] use protocol::{Mutation, OpBinary};
#[allow(unused_imports)] use store::ErasedSnapshotRetirement as _;
#[allow(unused_imports)] use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::native_retirement::DrawingRetirementOwner;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host_arena::initialization_catalog_close::{close_initialization_catalog, initialization_catalog_close_demands};
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host::owned::mutation_candidate::*;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host::owned::initialization_turn::*;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_host::owned::*;
use semio_s_artifact_draw_drawing_structure::create_layer::mutation::CreateLayer;
use semio_s_artifact_draw_drawing_structure::delete_layer::mutation::DeleteLayer;
use semio_s_artifact_draw_drawing_structure::duplicate_layer::mutation::DuplicateLayer;
use semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer;
use semio_s_artifact_draw_drawing_structure::reorder_layer::mutation::ReorderLayer;
use semio_s_artifact_draw_drawing_style::replace_layer_fill::mutation::ReplaceLayerFill;
use semio_s_artifact_draw_drawing_style::replace_layer_stroke::mutation::ReplaceLayerStroke;
use semio_s_artifact_draw_drawing_style::set_layer_blend_mode::mutation::SetLayerBlendMode;
use semio_s_artifact_draw_drawing_transform::set_layer_boolean_operation::mutation::SetLayerBooleanOperation;
use semio_s_artifact_draw_drawing_metadata::set_layer_locked::mutation::SetLayerLocked;
use semio_s_artifact_draw_drawing_style::set_layer_opacity::mutation::SetLayerOpacity;
use semio_s_artifact_draw_drawing_metadata::set_layer_visible::mutation::SetLayerVisible;
use semio_s_artifact_draw_drawing_transform::update_layer_trace_params::mutation::UpdateLayerTraceParams;
use semio_s_artifact_draw_drawing_transform::update_layer_transform::mutation::UpdateLayerTransform;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

#[path = "🧬️snapshot-laws/🦀️.rs"]
mod snapshot_laws;
use snapshot_laws::*;

#[path = "🔎️lookup-laws/🦀️.rs"]
mod lookup_laws;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
use kit::*;

#[path = "🏊️arena-laws/🦀️.rs"]
mod arena_laws;

#[path = "🧩️candidate-laws/🦀️.rs"]
mod candidate_laws;

#[path = "🔏️digest-laws/🦀️.rs"]
mod digest_laws;

#[path = "🪞️duplicate-laws/🦀️.rs"]
mod duplicate_laws;

#[path = "✏️edit-laws/🦀️.rs"]
mod edit_laws;
