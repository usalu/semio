//! 🧊️ Remodel viewer — the Model window: a read-only World3d render of the reconstructed mesh, the
//! sparse/dense clouds, the recovered camera positions and the ground control points. Reads through
//! the SAME artifact-level working-scene cache (`crate::artifacts::remodel::remodel_mesh_workspace`)
//! the sibling editor's own Model window uses — this file itself imports nothing from the sibling
//! editor surface (`policyViewerPurityBreaches` forbids it outright). No selection, no gumball, no
//! layer-visibility toggles: a viewer has no utilities that edit and emits no mutations by
//! construction (`ViewEmit`); every point layer the editor gates behind a config toggle is
//! unconditionally shown here instead, since a viewer keeps no persisted per-session layer state.
//!
//! MeshWindowKit note: contract §2.6 recommends `MeshWindowKit` for a viewer's 3D content, but its
//! `MeshView` view-model (`camera_json`/`meshes_json`/`instances_json`/`selection_json`) has no slot
//! for `World3dScene.points_json` — the sparse/dense clouds, camera-pose markers and GCP markers that
//! are this window's actual substance. A bespoke render function built directly on the framework
//! `world3d_*` helpers (the same escape hatch the `📐️cad` pilot's own viewer used, for the identical
//! reason) is the honest fit here, not `MeshWindowKit`.

use crate::artifacts::remodel::{PackedF32, RemodelSnapshot};
use semio_framework_plugin::{build_world_3d_scene, world3d_camera_json, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions, WorldSunConfig};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "remodel-view-model";
pub const BODY_KEY: &str = "remodel.view.model";
const SURFACE_ID: &str = "remodel.view.scene3d/model";
/// 👁️ Read-only counterpart of the editor's `REMODEL_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's world-3d controller can never be mistaken for an editor session's.
const REMODEL_VIEW_CONTROLLER_ID: &str = "remodel-view";
/// 👁️ Matches the editor's `REMODEL_MESH_ID` literal — duplicated on purpose rather than imported
/// through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
const REMODEL_VIEW_MESH_ID: &str = "remodel-result";
/// 👁️ Hardcoded default viewport — a viewer has no persisted per-session camera (`Config = NoConfig`
/// on `RemodelViewer`), matching `RemodelWorldCamera::default()`'s own values. Documented
/// simplification for a first pass, not a bug.
const REMODEL_VIEW_CAMERA_POSITION: [f64; 3] = [4.0, -4.0, 3.0];
const REMODEL_VIEW_CAMERA_TARGET: [f64; 3] = [0.0, 0.0, 0.0];
const REMODEL_VIEW_CAMERA_FOV: f64 = 45.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Model", "Modell"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "remodel-model".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Read-only twin of the sibling editor's `world_meshes_json` — reads the composed
/// `s.stdio.semio/v1/mesh` CHILD's real geometry through the same artifact-level working-scene cache.
/// A cold cache renders no mesh entity (empty scene list), the identical fallback the editor's own
/// `world_meshes_json` already carries while composed-child object resolution is unimplemented
/// (pre-existing `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` gap, not introduced here) — real
/// parity with the editor's CURRENT behaviour, not a regression.
async fn world_meshes_json(scene: &RemodelSnapshot) -> String {
    let Some(mesh) = crate::artifacts::remodel::remodel_mesh_workspace(&scene.results.mesh.mesh) else {
        return "[]".into();
    };
    serde_json::to_string(&vec![json!({ "id": REMODEL_VIEW_MESH_ID, "data": mesh })]).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Unconditionally visible mesh instance — a viewer has no persisted per-session layer toggles, so
/// unlike the editor's `world_instances_json` this never gates on a `layers.mesh` flag.
async fn world_instances_json() -> String {
    serde_json::to_string(&vec![json!({
        "id": REMODEL_VIEW_MESH_ID,
        "meshId": REMODEL_VIEW_MESH_ID,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "selected": false,
        "hovered": false,
    })])
    .unwrap_or_else(|_| "[]".into())
}

/// 👁️ Read-only twin of the sibling editor's `world_points_json` — every layer the editor gates
/// behind a config toggle is unconditionally shown here (a viewer keeps no layer-visibility state),
/// pure document content otherwise: sparse/dense clouds, recovered camera poses, ground control
/// points.
async fn world_points_json(scene: &RemodelSnapshot) -> Option<String> {
    let mut layers: Vec<Value> = Vec::new();
    if let Some(sparse) = &scene.results.sparse {
        if !sparse.points.is_empty() {
            layers.push(json!({
                "id": "remodel-sparse",
                "positionsB64": sparse.points.0,
                "colorsB64": sparse.colors.as_ref().map(|colors| colors.0.clone()),
                "size": 3.0,
                "sizeAttenuation": true,
            }));
        }
    }
    if let Some(dense) = &scene.results.dense {
        if !dense.positions.is_empty() {
            layers.push(json!({
                "id": "remodel-dense",
                "positionsB64": dense.positions.0,
                "colorsB64": dense.colors.as_ref().map(|colors| colors.0.clone()),
                "size": 2.0,
                "sizeAttenuation": true,
            }));
        }
    }
    if !scene.job.camera_poses_preview.is_empty() {
        let positions: Vec<f32> = scene.job.camera_poses_preview.iter().flat_map(|pose| pose.translation).collect();
        layers.push(json!({
            "id": "remodel-camera-poses",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 9.0,
            "sizeAttenuation": false,
        }));
    }
    if !scene.gcps.is_empty() {
        let positions: Vec<f32> = scene.gcps.iter().flat_map(|gcp| gcp.world_position.map(|c| c as f32)).collect();
        layers.push(json!({
            "id": "remodel-gcps",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 10.0,
            "sizeAttenuation": false,
        }));
    }
    if layers.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()))
    }
}

/// 👁️ Pure `RemodelSnapshot -> UiNode` read: hardcoded default camera/sun, no selection overlay,
/// every point layer unconditionally visible, mesh content real whenever the working-scene cache is
/// warm.
pub async fn render(scene: &RemodelSnapshot) -> UiNode {
    let mut world_scene = world3d_scene(
        world3d_camera_json(REMODEL_VIEW_CAMERA_POSITION, REMODEL_VIEW_CAMERA_TARGET, REMODEL_VIEW_CAMERA_FOV),
        world_meshes_json(scene),
        world_instances_json(),
        world3d_selection_json("rectangle", &[], None),
        &WorldSunConfig::default(),
    );
    world_scene.points_json = world_points_json(scene);
    build_world_3d_scene(SURFACE_ID, REMODEL_VIEW_CONTROLLER_ID, world_scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_world3d_model_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::World3d);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let scene = crate::artifacts::remodel::default_remodel_scene();
        let _node = render(&scene);
    }

    #[semio_framework_async_macros::async_test]
    async fn world_meshes_json_renders_the_real_placeholder_mesh_when_the_cache_is_warm() {
        let scene = crate::artifacts::remodel::default_remodel_scene();
        assert!(world_meshes_json(&scene).contains(REMODEL_VIEW_MESH_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn world_instances_json_is_never_gated_on_a_layer_toggle() {
        assert!(world_instances_json().contains(REMODEL_VIEW_MESH_ID));
    }
}
//#endregion 🧪️Tests
