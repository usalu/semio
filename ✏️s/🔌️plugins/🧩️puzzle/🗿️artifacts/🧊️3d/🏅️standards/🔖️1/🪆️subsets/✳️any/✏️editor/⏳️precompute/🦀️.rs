//! collision meshes and the background brush-candidate lane, plus `dispatch`, which drives
//! ⏳️ Puzzle 3d play app — the precompute session: the scene the host syncs in, the registered
//! `Puzzle3dEngineCommand`/`Puzzle3dEngineOutcome` (schema types, `semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema`)
//! through the session. The rules the lane consults live in `🖌️brush/🦀️.rs`, the geometry in
//! `📐️geometry/🦀️.rs`, the fill planner and its tool run jobs in `🪣️fill/🦀️.rs` (owned by the framework tool
//! run ledger, never by this session). Rehomed from the former `⚙️engine/⏳️session` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): a puzzle-3d artifact is a schema plus an io
//! system, never an engine — this interactive brush session is the app's own state machine over
//! that schema, not artifact behaviour.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🔖️Reexports
pub use semio_s_artifact_puzzle_3d_precompute_brush::apply_brush_placement_to_snapshot;
//#endregion 🔖️Reexports

//#region 🔖️Constants
/// 🥽️ Collision mesh identities one session holds.
pub const COLLISION_MESH_MAX_MESHES: usize = 64;
/// 🥽️ Values (positions or indices) one collision mesh may carry.
pub const COLLISION_MESH_MAX_VALUES: usize = 196_608;
/// 🥽️ Bytes of one collision mesh identity.
pub const COLLISION_MESH_MAX_URL_BYTES: usize = 4 * 1024;
//#endregion 🔖️Constants

use semio_s_artifact_puzzle_3d_precompute_brush::brush_candidate_suggestion_weight;

use semio_s_artifact_puzzle_3d_precompute_brush::brush_compatible_candidates;

use semio_s_artifact_puzzle_3d_precompute_brush::brush_preview_from_candidate;

use semio_s_artifact_puzzle_3d_precompute_brush::brush_target_vortex_allows_suggestion;

use semio_s_artifact_puzzle_3d_precompute_brush::resolve_placed_object_mesh_url;

use semio_s_artifact_puzzle_3d_precompute_brush::vortex_world_from_object;

use semio_s_artifact_puzzle_3d_precompute_brush::AttractionVortexContext;

use semio_s_artifact_puzzle_3d_precompute_brush::TargetVortexWorld;
use semio_s_artifact_puzzle_3d_precompute_fill::PlacedCollisionEntry;
use semio_s_artifact_puzzle_3d_precompute_geometry::pose_isometry;
use semio_s_artifact_puzzle_3d_precompute_geometry::world_bounds;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionAabb;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionBody;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexMutation;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexOwner;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexRemoval;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionMutationStep;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionPenetrationState;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionQueryStep;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionSpatialIndex;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepContext;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepResult;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::BrushCollisionFreeResult;
use semio_s_artifact_puzzle_3d_model::BrushCompatibleCandidate;
use semio_s_artifact_puzzle_3d_model::BrushPreviewState;
use semio_s_artifact_puzzle_3d_model::EngineSceneObject;
use semio_s_artifact_puzzle_3d_model::KindCatalogBundle;
use semio_s_artifact_puzzle_3d_model::Puzzle3dEngineCommand;
use semio_s_artifact_puzzle_3d_model::Puzzle3dEngineOutcome;
use semio_s_artifact_puzzle_3d_model::SceneConfig;
#[cfg(test)] use semio_s_artifact_puzzle_3d_model::{BrushPlacePayload, EngineSceneSnapshot};
use semio_s_artifact_puzzle_3d_model::Puzzle3dError;
use semio_framework_job::default_now_us;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

//#region 🔖️Clock
//#endregion 🔖️Clock

//#region 🥽️SharedBrushMeshes
#[path = "🥽️brush-meshes/🦀️.rs"]
mod brush_meshes;
pub use self::brush_meshes::*;
//#endregion 🥽️SharedBrushMeshes

//#region 🧩️PagedBrushMeshUploads
//#endregion 🧩️PagedBrushMeshUploads

//#region 🔖️Engine
#[path = "⚙️collision-engine/🦀️.rs"]
mod collision_engine;
pub use self::collision_engine::*;
//#endregion 🔖️Engine

//#region 🔖️Session
#[path = "🧵️precompute-session/🦀️.rs"]
mod precompute_session;
pub use self::precompute_session::*;
//#endregion 🔖️Session

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
