//! 🪣️ Puzzle 3d play app — the fill planner (`FillBuilder`: base scene, the growing plan sequence and its
//! appended objects/attractions, the placed collision entries the next step tests against, the per-run RNG
//! stream) and the fill tool run's two jobs, [`FillRunJob`] and [`FillRevalidateJob`], which the framework
//! tool run ledger owns and steps (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS `📋️tool-run-contract.md`
//! §2.7, §3.7).

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

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

#[path = "🛑️fill-contract/🦀️.rs"]
mod fill_contract;
pub use self::fill_contract::*;

#[path = "🎲️weighted-draw/🦀️.rs"]
mod weighted_draw;
use self::weighted_draw::*;

#[path = "🏗️builder-state/🦀️.rs"]
mod builder_state;
pub use self::builder_state::*;

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
use self::retirement::*;

#[path = "🧱️builder-preparation/🦀️.rs"]
mod builder_preparation;
use self::builder_preparation::*;

//#region 🧵️InteractiveFillJob
#[path = "🧵️interactive-fill/🦀️.rs"]
mod interactive_fill;
use self::interactive_fill::*;

//#endregion 🧵️InteractiveFillJob

//#region ⏯️FillRunJob
#[path = "⏯️fill-run/🦀️.rs"]
mod fill_run;
pub use self::fill_run::*;

#[path = "🔍️fill-revalidation/🦀️.rs"]
mod fill_revalidation;
pub use self::fill_revalidation::*;

//#endregion ⏯️FillRunJob

//#region 🧪️Tests
//#endregion 🧪️Tests
