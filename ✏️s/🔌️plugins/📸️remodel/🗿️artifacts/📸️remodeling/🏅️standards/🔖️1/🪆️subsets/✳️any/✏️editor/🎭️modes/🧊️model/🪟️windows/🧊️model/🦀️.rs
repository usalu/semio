//! 🧊️ Remodeling play app — the Model window: the World3d scene carrying the reconstructed mesh, the
//! sparse/dense clouds, the recovered camera positions and the ground control points.

use crate::artifacts::remodeling::{PackedF32, RemodelingSnapshot};
use crate::editor::remodeling::config::RemodelingConfig;
use crate::editor::remodeling::modes::model::windows::model::options::layers;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{
    build_world_3d_scene, world3d_camera_json, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions, WorldSunConfig,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const REMODELING_PLAY_WINDOW_MAIN: &str = "remodeling-main";
pub const REMODELING_PLAY_BODY_MAIN: &str = "remodeling.play.main";
const REMODELING_PLAY_SURFACE_MAIN: &str = "remodeling.play";
const REMODELING_MESH_ID: &str = "remodeling-result";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: REMODELING_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Model", "Modell"),
        body_key: REMODELING_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "remodeling-model".into(),
        // 🎚️ `measures` stays empty: they are config-derived per frame by `ArtifactEditor::window_measures`.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::None },
        actions: Vec::new(),
        utilities: ["select", "measure", "sculpt"].iter().map(|id| UtilityRef::from(*id)).collect(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// ☑️ The live chrome measures for this window, collected from its own `☑️options/*`.
pub async fn window_measures(config: &RemodelingConfig, labels: &RemodelingLabels) -> Vec<WindowMeasure> {
    vec![layers::measure(&config.layers, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🧩️ `results.mesh.mesh` is now a composed `s.stdio.semio/v1/mesh` CHILD handle (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) — resolves only fixed constants or committed
/// reconstruction content inside the production 512/512 mesh envelope; unavailable content renders
/// no mesh entity rather than treating the handle's opaque address as geometry.
async fn world_meshes_json(scene: &RemodelingSnapshot) -> String {
    let Some(mesh) = crate::artifacts::remodeling::resolve_bounded_remodeling_mesh(&scene.durable_artifacts, &scene.results.mesh.mesh) else {
        return "[]".into();
    };
    serde_json::to_string(&vec![json!({ "id": REMODELING_MESH_ID, "data": mesh })]).unwrap_or_else(|_| "[]".into())
}

async fn world_instances_json(config: &RemodelingConfig) -> String {
    if !config.layers.mesh {
        return "[]".into();
    }
    serde_json::to_string(&vec![json!({
        "id": REMODELING_MESH_ID,
        "meshId": REMODELING_MESH_ID,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "selected": false,
        "hovered": false,
    })])
    .unwrap_or_else(|_| "[]".into())
}

/// ☁️ `World3dScene.points_json` layers: the finished sparse/dense clouds once a run has produced them,
/// and every recovered camera pose as its own (small, unattenuated) point layer — a documented
/// simplification standing in for a real camera-frustum gizmo, which `points_json` alone cannot express.
/// GCP world positions are a fourth, always-static layer. There is no in-progress live sparse preview
/// layer: a synchronous run only ever publishes the FINAL sparse cloud, never an interior one.
/// `PackedF32`/`PackedU8`'s inner string is already a base64 little-endian buffer, matching
/// `positionsB64`/`colorsB64`'s wire shape byte-for-byte — no decode/re-encode round trip needed.
async fn world_points_json(scene: &RemodelingSnapshot, config: &RemodelingConfig) -> Option<String> {
    let mut layers: Vec<Value> = Vec::new();
    if config.layers.sparse {
        if let Some(sparse) = &scene.results.sparse {
            if !sparse.points.is_empty() {
                layers.push(json!({
                    "id": "remodeling-sparse",
                    "positionsB64": sparse.points.0,
                    "colorsB64": sparse.colors.as_ref().map(|colors| colors.0.clone()),
                    "size": 3.0,
                    "sizeAttenuation": true,
                }));
            }
        }
    }
    if config.layers.dense {
        if let Some(dense) = &scene.results.dense {
            if !dense.positions.is_empty() {
                layers.push(json!({
                    "id": "remodeling-dense",
                    "positionsB64": dense.positions.0,
                    "colorsB64": dense.colors.as_ref().map(|colors| colors.0.clone()),
                    "size": 2.0,
                    "sizeAttenuation": true,
                }));
            }
        }
    }
    if config.layers.cameras && !scene.job.camera_poses_preview.is_empty() {
        let positions: Vec<f32> = scene.job.camera_poses_preview.iter().flat_map(|pose| pose.translation).collect();
        layers.push(json!({
            "id": "remodeling-camera-poses",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 9.0,
            "sizeAttenuation": false,
        }));
    }
    if config.layers.gcps && !scene.gcps.is_empty() {
        let positions: Vec<f32> = scene.gcps.iter().flat_map(|gcp| gcp.world_position.map(|c| c as f32)).collect();
        layers.push(json!({
            "id": "remodeling-gcps",
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

pub async fn render(scene: &RemodelingSnapshot, config: &RemodelingConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    // 🕹️ The "assets" selection now lives in the framework-owned interaction domain (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `ArtifactEditor::render` carries no
    // `InteractionView`, so this scene payload can no longer embed a live selection; every
    // not-yet-migrated `world3d_selection_json` call site in this repo already passes an empty
    // selection for the same reason.
    let mut world_scene =
        world3d_scene(world3d_camera_json(config.camera.position, config.camera.target, config.camera.fov), world_meshes_json(scene), world_instances_json(config), world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    world_scene.points_json = world_points_json(scene, config);
    build_world_3d_scene(REMODELING_PLAY_SURFACE_MAIN, crate::editor::remodeling::REMODELING_PLAY_APP_ID, world_scene)
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodeling::default_remodeling_scene;
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn default_scene_seeds_the_world3d_mesh_json() {
        let scene = default_remodeling_scene();
        assert!(world_meshes_json(&scene).contains(REMODELING_MESH_ID));
        let config = RemodelingConfig::default();
        assert!(world_instances_json(&config).contains(REMODELING_MESH_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn hiding_the_mesh_layer_drops_the_instance() {
        let mut config = RemodelingConfig::default();
        config.layers.mesh = false;
        assert_eq!(world_instances_json(&config), "[]");
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_a_world_3d_surface() {
        let mut app = app();
        assert!(render_body(&mut app, REMODELING_PLAY_BODY_MAIN).contains("world-3d"));
    }
}
//#endregion 🧪️Tests
