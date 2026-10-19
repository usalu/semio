//! 🌉️ World3d scene bridge: dynamic retirement, face overlays, snapshot apply and the guest scene bridge.
//!
//! Crate `semio-framework-os-infinite-world-bridge`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{quarantine_world_owner, World3dBuildContext, world_close_string, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_MESH_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_pick_keys_match, world_pick_target_bounds, world_pick_targets_from_json, world_reference_width, world_scene_raster_pool, WorldDrawRebuildCursor, WorldDrawRebuildDescriptor, WorldDynamicFault, WorldDynamicRegistry, WorldDynamicRejected, WorldInteractionId, WorldInteractionMeshRegistry, WorldInteractionMeshToken, WorldInteractionObjectKind, WorldInteractionPlan, WorldInteractionRegistryBuildPhase, WorldInteractionState, WorldInteractionStep, WorldMeshRegistry, WorldMeshSource, WorldNativeMeshAdmission, WorldNativeMeshFailure, WorldNativeMeshSource, WorldOpaqueOwner, WorldReferenceRecord, WorldSelectionTargets, WorldSessionState, WorldSubElementPaint};
use semio_framework_os_infinite_world_components::{GumballHandle, mesh_vertex, pick_targets_instance, World3dCameraFitOwner, World3dCameraFitRequest, World3dSceneGumballConfigRecord, World3dSnapshotApplyCursor, world_component_preview_target, world_mesh_component_id, world_mesh_triangle, WorldCameraState, WorldGumballState, WorldMeshBuffers, WorldNavigationSnapshot, WorldOverlayState, WorldSceneState, WorldSelectionState};
use semio_framework_os_infinite_world_cursors::{CLICK_DRAG_THRESHOLD_PX, live_world3d_mesh_version, pointer_in_pick_rect, world3d_gumball_config, WorldBrushCommitJob, WorldComponentMarqueePublishJob, WorldComponentPickCursor, WorldGumballCommitJob, WorldGumballPickCursor, WorldInteractionAuthorityStep, WorldMarqueePickCursor, WorldMarqueePublishJob, WorldObjectPickCursor, WorldRayPickCursor};
use semio_framework_os_infinite_world_scene::{environment_outline_color, GUMBALL_PLANE_MESH, mesh_id_from_url, mesh_style_paint, parse_color, publish_world3d_mesh_lease, resolve_mesh_style, retire_world_pixels, scene_lod, world3d_scene_digest, world_instance_style, WorldAssetsState};
use semio_framework_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{Camera3d, CameraProjection3d, ComponentSource3d, gumball_extent, gumball_eye, Instance3d, LineVertex3d, lod_grid_step_world, Mat4, Mesh3dField, Mesh3dLease, quat_from_basis, ray_plane_point, ray_segment_distance, rotate_vector, SceneColorSource3d, SceneDraw3d, Vec3};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_raster_owner::{SceneRasterLease, SceneRasterPool};
use semio_framework_ui_declaration::ui::{World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPageKind};
use serde::Deserialize;
use std::collections::HashSet;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::SceneMaterial3d;
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use ui_scene::math::Mesh3dResourceLoan as WorldNativeMeshLoan;
use semio_framework_ui_runtime::UiNativeMeshResource as WorldNativeMeshResource;
use semio_framework_ui_contract::UiRevision as WorldNativeUiRevision;
use serde_json::json;

/// 🧩️ World3d retirement component: the in-flight dynamic retirement cursor.
pub struct WorldRetirementState {

    pub dynamic_retirement: Option<World3dDynamicRetirement>,
}

/// 🧩️ World3d bridge component: scene bridge, snapshot apply and scene digests.
pub struct WorldBridgeState {

    pub snapshot_lease: Option<World3dSnapshotLease>,

    pub snapshot_apply: Option<World3dSnapshotApplyCursor>,

    /// 🌉️ The mesh-wire → snapshot bridge (`//#region 🌉️World3dSceneBridge`) that gives a producer
    /// publishing only plain-JSON `meshes_json`/`instances_json` the same typed snapshot a
    /// snapshot-native producer hands over.
    pub scene_bridge: Option<World3dSceneBridgeCursor>,

    pub scene_bridge_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<World3dSceneBridgeCursor>>,

    pub scene_bridge_lease: Option<World3dSnapshotLease>,

    pub scene_bridge_retired: Option<(World3dSnapshotLease, bool)>,

    pub scene_bridge_generation: u64,

    pub scene_bridge_digest: Option<u64>,

    pub scene_document_lanes_digest: Option<u64>,
}

impl WorldRetirementState {
    pub fn new() -> Self {
        Self {
            dynamic_retirement: None,
        }
    }
}

impl WorldBridgeState {
    pub fn new() -> Self {
        Self {
            snapshot_lease: None,
            scene_bridge: None,
            scene_bridge_close: None,
            scene_bridge_lease: None,
            scene_bridge_retired: None,
            scene_bridge_generation: 0,
            scene_bridge_digest: None,
            scene_document_lanes_digest: None,
            snapshot_apply: None,
        }
    }
}

#[path = "🎞️scene-bridge/🦀️.rs"]
mod scene_bridge;
pub use scene_bridge::*;

#[path = "🖌️overlays-and-gumball/🦀️.rs"]
mod overlays_and_gumball;
pub use overlays_and_gumball::*;

#[path = "🎥️camera-and-retirement/🦀️.rs"]
mod camera_and_retirement;
pub use camera_and_retirement::*;
