//! appended objects/attractions, the placed collision entries the next step tests against, the per-run RNG
//! 🪣️ Puzzle 3d play app — the fill planner (`FillBuilder`: base scene, the growing plan sequence and its
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
use semio_s_artifact_puzzle_3d_precompute_brush::brush_fill_candidate_at;
use semio_s_artifact_puzzle_3d_precompute_brush::brush_object_id;
use semio_s_artifact_puzzle_3d_precompute_brush::brush_preview_from_candidate;
use semio_s_artifact_puzzle_3d_precompute_brush::fill_rng;
use semio_s_artifact_puzzle_3d_precompute_brush::resolve_object_kind_mesh_url;
use semio_s_artifact_puzzle_3d_precompute_brush::resolve_placed_object_mesh_url;
use semio_s_artifact_puzzle_3d_precompute_brush::vortex_world_from_object;
use semio_s_artifact_puzzle_3d_precompute_brush::AttractionVortexContext;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushCatalogView;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushFillVortexTarget;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSceneView;
use semio_s_artifact_puzzle_3d_precompute_brush::TargetVortexWorld;
use semio_s_artifact_puzzle_3d_precompute_geometry::pose_isometry;
use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work;
use semio_s_artifact_puzzle_3d_precompute_geometry::world_bounds;
use semio_s_artifact_puzzle_3d_precompute_geometry::world_volumes_contain_aabb;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionAabb;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionBody;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexMutation;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepContext;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexOwner;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexRejectedOwner;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexRemoval;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionMutationStep;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionPenetrationState;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionQueryCursor;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionQueryStep;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionSpatialIndex;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepResult;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerMap;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerMapInsert;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerSet;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerSetInsert;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerVec;
use semio_s_artifact_puzzle_3d_precompute_geometry::Pose3d;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_ATTRACTION_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_CANDIDATE_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_KIND_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_OBJECT_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_VOLUME_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_VORTEX_SLOTS;
use {semio_s_artifact_puzzle_3d_editor_snapshot::empty_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject};
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::AttractionProps;
use semio_s_artifact_puzzle_3d_model::BrushCompatibleCandidate;
use semio_s_artifact_puzzle_3d_model::BrushHostRules;
use semio_s_artifact_puzzle_3d_model::BrushPlacePayload;
use semio_s_artifact_puzzle_3d_model::BrushPreviewState;
use semio_s_artifact_puzzle_3d_model::CableKindCatalog;
use semio_s_artifact_puzzle_3d_model::EngineSceneObject;
use semio_s_artifact_puzzle_3d_model::FillRunCheckpoint;
use semio_s_artifact_puzzle_3d_model::FillRunCounter;
use semio_s_artifact_puzzle_3d_model::FillRunReason;
use semio_s_artifact_puzzle_3d_model::FillRunStage;
use semio_s_artifact_puzzle_3d_model::KindCompatEntry;
use semio_s_artifact_puzzle_3d_model::KindCatalogBundle;
use semio_s_artifact_puzzle_3d_model::ObjectKind;
use semio_s_artifact_puzzle_3d_model::SceneConfig;
use semio_s_artifact_puzzle_3d_model::VortexKindCatalog;
use semio_s_artifact_puzzle_3d_model::VortexProps;
use semio_s_artifact_puzzle_3d_model::WorldVolumeProps;
use semio_s_plugin_puzzle_jobs::JobTurn;
use semio_framework_job::Generation;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::Operation;
use semio_framework_job::OperationId;
use semio_framework_job::StepContext;
use semio_framework_tool_run::ToolRunCounter;
use semio_framework_tool_run::ToolRunIdentity;
use semio_framework_tool_run::ToolRunProgress;
use semio_framework_tool_run::ToolRunState;
use semio_framework_tool_run::ToolRunStep;
use semio_framework_tool_run::ToolRunStepArg;
use semio_framework_tool_run::ToolRunStepKind;
use semio_framework_tool_run::ToolRunStepRing;
use semio_framework_tool_run::ToolRunTick;
use semio_framework_tool_run::ToolRunTickWriter;
use semio_framework_tool_run::ToolRunTraceOp;
use semio_framework_tool_run::ToolRunTracePage;
use semio_framework_tool_run::ToolRunTraceSubject;
use semio_framework_tool_run::ToolRunVerdict;
use semio_framework_tool_run::TOOL_RUN_PROVISIONAL_OPS_MAX;
use semio_framework_tool_run::TOOL_RUN_REASON_CONFLICT;
use semio_framework_tool_run::TOOL_RUN_REASON_PROVISIONAL_CAP;
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
