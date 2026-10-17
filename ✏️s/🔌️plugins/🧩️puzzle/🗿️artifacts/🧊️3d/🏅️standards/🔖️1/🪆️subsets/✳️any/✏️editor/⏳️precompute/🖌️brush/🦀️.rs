//! 🖌️ Puzzle 3d play app — the precompute brush rulebook: which object kind may dock onto which vortex
//! (port shape, single-letter family, the manifest's explicit `kindCompatibility` rows, the
//! host-specific tambour/capsule rules), how the surviving candidates are ranked and weighted, how
//! the fill lane's targets and candidates are sampled without replacement, and how one accepted
//! candidate becomes a concrete `BrushPreviewState`/placement spliced into a `EngineSceneSnapshot`. Rehomed from
//! the former `⚙️engine/🖌️brush` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this
//! is the interactive brush tool's own decision logic, so it lives with the app, not the artifact.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::{AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushKindWeights, BrushPlacePayload, BrushPreviewState, CableKindCatalog, EngineSceneSnapshot, EngineSceneObject, KindCatalogBundle, KindCompatEntry, ObjectKind, ObjectKindVortexTemplate, Quat, Vec3, VortexKindCatalog, VortexProps};
use semio_s_artifact_puzzle_3d_precompute_geometry::{collision_body_from_buffers, compute_brush_placement_pose, normalize_vec3, pose_isometry, precompute_work, quat_rotate_vec, vec3_add, CollisionAabb, CollisionBody, CollisionPenetrationState, CollisionStepContext, CollisionStepResult, Pose3d};
use semio_s_artifact_puzzle_3d_model::{BrushSuggestionsRunCounter, BrushSuggestionsRunReason, BrushSuggestionsRunStage, SceneConfig};
use semio_s_plugin_puzzle_jobs::JobTurn;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::{ArtifactInstanceOperationOwnerHandle, ToolRunJobPort};
use semio_framework_tool_run::{ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

pub const DEFAULT_CABLE_KIND_ID: &str = "cable.link";

//#region 🔖️VortexContext

#[path = "🌀️vortex-compatibility/🦀️.rs"]
mod vortex_compatibility;
pub use vortex_compatibility::*;

#[path = "🎯️candidates-and-placement/🦀️.rs"]
mod candidates_and_placement;
pub use candidates_and_placement::*;

#[path = "⏯️suggestions-run/🦀️.rs"]
mod suggestions_run;
pub use suggestions_run::*;

//#endregion 🔖️VortexContext

//#region 🔖️Compatibility

//#endregion 🔖️Compatibility

//#region 🔖️StackPairs

//#endregion 🔖️StackPairs

//#region 🔖️Candidates

//#endregion 🔖️Candidates

//#region 🔖️Weights

//#endregion 🔖️Weights

//#region 🔖️Placement

//#endregion 🔖️Placement

//#region ⏯️BrushSuggestionsRun

//#endregion ⏯️BrushSuggestionsRun

//#region 🧪️Tests
//#endregion 🧪️Tests
