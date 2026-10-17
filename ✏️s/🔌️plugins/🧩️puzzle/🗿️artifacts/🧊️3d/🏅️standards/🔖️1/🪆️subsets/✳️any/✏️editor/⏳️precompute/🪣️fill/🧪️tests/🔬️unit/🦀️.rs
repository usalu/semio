//! 

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_precompute_fill::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::VORTEX_MARKER_MESH_KIND;
use semio_s_artifact_puzzle_3d_precompute_brush::{brush_fill_candidate_at, brush_object_id, brush_preview_from_candidate, fill_rng, resolve_object_kind_mesh_url, resolve_placed_object_mesh_url, vortex_world_from_object, AttractionVortexContext, BrushCatalogView, BrushFillVortexTarget, BrushSceneView, TargetVortexWorld};
use semio_s_artifact_puzzle_3d_precompute_geometry::{pose_isometry, precompute_work, world_bounds, world_volumes_contain_aabb, CollisionAabb, CollisionBody, CollisionIndexMutation, CollisionStepContext, CollisionIndexOwner, CollisionIndexRejectedOwner, CollisionIndexRemoval, CollisionMutationStep, CollisionPenetrationState, CollisionQueryCursor, CollisionQueryStep, CollisionSpatialIndex, CollisionStepResult, FixedOwnerMap, FixedOwnerMapInsert, FixedOwnerSet, FixedOwnerSetInsert, FixedOwnerVec, Pose3d, DOCUMENT_ATTRACTION_SLOTS, DOCUMENT_CANDIDATE_SLOTS, DOCUMENT_KIND_SLOTS, DOCUMENT_OBJECT_SLOTS, DOCUMENT_VOLUME_SLOTS, DOCUMENT_VORTEX_SLOTS};
use {semio_s_artifact_puzzle_3d_editor_snapshot::empty_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject};
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::{AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushPlacePayload, BrushPreviewState, CableKindCatalog, EngineSceneObject, FillRunCheckpoint, FillRunCounter, FillRunReason, FillRunStage, KindCompatEntry, KindCatalogBundle, ObjectKind, SceneConfig, VortexKindCatalog, VortexProps, WorldVolumeProps};
use semio_s_plugin_puzzle_jobs::JobTurn;
use semio_framework_job::{Generation, InteractiveJob, Operation, OperationId, StepContext};
use semio_framework_tool_run::{
    ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_PROVISIONAL_OPS_MAX,
    TOOL_RUN_REASON_CONFLICT, TOOL_RUN_REASON_PROVISIONAL_CAP,
};
use std::collections::HashMap;
use std::sync::Arc;

fn faulted(turn: JobTurn) -> bool {
    matches!(turn, JobTurn::Fault(_))
}

use semio_s_artifact_puzzle_3d_precompute_geometry::{collision_body_from_buffers, precompute_work_clock, precompute_work_done, OwnerReservationLimit, DOCUMENT_CELL_MEMBER_SLOTS, DOCUMENT_CELL_SLOTS, DOCUMENT_OWNER_PAGE_BYTES, FIXED_OWNER_PAGE_BYTES, FIXED_OWNER_SLOTS};
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use semio_s_artifact_puzzle_3d_model::{BrushKindWeights, EngineSceneSnapshot, KindCatalogBundle, ObjectKind, ObjectKindRepresentation, ObjectKindVortexTemplate, VortexProps};
use semio_framework_job::{root_cancel_token, Generation, OperationId, RevisionId, StepBudget};

/// 🧮️ Primitive work one fill builder step may cost ([`precompute_work_done`]): one transition of the cursorized planner —
/// one preparation item, one candidate, one broad-phase lookup or one collision probe. The measured maximum is 177 units
/// (the adversarial broad-phase plan; an empty scene's steps cost 1).
const FILL_STEP_WORK_CEILING: u64 = 512;

/// 🧮️ Primitive work the adversarial broad-phase plan may spend before its first candidate is on screen. It measures
/// 1490 units.
const FILL_FIRST_CANDIDATE_WORK_CEILING: u64 = 4_096;

/// 🎯️ What a fresh editor asks for — the product default, never a planner ceiling.
const TEST_REQUESTED_COUNT: usize = 100;

fn empty_builder() -> FillBuilder {
    let scene = Arc::new(SceneConfig {
        scene_snapshot: EngineSceneSnapshot::default(),
        kind_catalogs: Some(KindCatalogBundle::default()),
        kind_compatibility: Vec::new(),
        contact_tolerance: 0.0,
        seed: 17,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(HashMap::new())), Operation::new(OperationId(1), RevisionId(1), Generation(1), 17), TEST_REQUESTED_COUNT)
}

fn test_context<'a>(builder: &FillBuilder, cancel: semio_framework_job::CancelToken, sequence: &'a mut u64) -> StepContext<'a> {
    fn now() -> Option<u64> {
        Some(0)
    }
    StepContext::new(builder.operation.operation, builder.operation.generation, StepBudget::new(100, 10, semio_s_plugin_puzzle_jobs::testing::unbounded_grant()), cancel, now, sequence, semio_s_plugin_puzzle_jobs::testing::test_progress())
}

include!("🧮️planner-laws/🦀️.rs");

include!("⏯️fill-run-laws/🦀️.rs");

include!("🛑️run-end-laws/🦀️.rs");
