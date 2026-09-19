//! 🧊️ Remodeling play app — the Model window: the World3d scene carrying the reconstructed mesh, the
//! sparse/dense clouds, the recovered camera positions and the ground control points.

use crate::editor::remodeling::modes::model::windows::model::config::RemodelingModelWindowConfig;
use crate::editor::remodeling::modes::model::windows::model::options::layers;
use crate::editor::remodeling::reconstruction_session::{RECONSTRUCTION_TRACE_CAMERA_MESH, RECONSTRUCTION_TRACE_MESH_LANE, RECONSTRUCTION_TRACE_POINT_MESH};
use crate::editor::remodeling::terminology::RemodelingLabels;
use crate::{PackedF32, RemodelingSnapshot};
use semio_framework_plugin::{world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions, WorldSunConfig};
// 🧬️ Two `SurfaceKind` enums coexist: `WindowKindDefinition` carries the retained `ui_wgpu` one
// (re-exported by the SDK root), while `scene_surface` takes the semantic contract's — same spelling,
// different types, so both are imported explicitly.
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const REMODELING_PLAY_WINDOW_MAIN: &str = "remodeling-main";
pub const REMODELING_PLAY_BODY_MAIN: &str = "remodeling.play.main";
const REMODELING_PLAY_SURFACE_MAIN: &str = "remodeling.play";
const REMODELING_MESH_ID: &str = "remodeling-result";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
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
pub fn window_measures(config: &RemodelingModelWindowConfig, labels: &RemodelingLabels) -> Vec<WindowMeasure> {
    vec![layers::measure(&config.layers, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🧩️ `results.mesh.mesh` is now a composed `s.stdio.semio/v1/mesh` CHILD handle (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) — resolves only fixed constants or committed
/// reconstruction content inside the production 512/512 mesh envelope; unavailable content renders
/// no mesh entity rather than treating the handle's opaque address as geometry.
/// 🧊️ The mesh lane: the reconstruction trace markers first, at the indices `instance3d` trace records
/// name (`RECONSTRUCTION_TRACE_MESH_LANE`), then the stored result mesh.
fn world_meshes_json(scene: &RemodelingSnapshot) -> String {
    let mut meshes = vec![
        json!({ "id": RECONSTRUCTION_TRACE_MESH_LANE[RECONSTRUCTION_TRACE_POINT_MESH as usize], "data": mesh_data_json(&trace_point_marker()) }),
        json!({ "id": RECONSTRUCTION_TRACE_MESH_LANE[RECONSTRUCTION_TRACE_CAMERA_MESH as usize], "data": mesh_data_json(&trace_camera_marker()) }),
    ];
    if let Some(mesh) = crate::resolve_bounded_remodeling_mesh(&scene.durable_artifacts, &scene.results.mesh.mesh) {
        meshes.push(json!({ "id": REMODELING_MESH_ID, "data": mesh_data_json(&mesh) }));
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

/// 🔹️ Unit octahedron a traced point instances.
fn trace_point_marker() -> semio_framework::MeshData {
    semio_framework::MeshData {
        positions: vec![1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0],
        indices: vec![0, 2, 4, 2, 1, 4, 1, 3, 4, 3, 0, 4, 2, 0, 5, 1, 2, 5, 3, 1, 5, 0, 3, 5],
        ..Default::default()
    }
}

/// 📷️ Unit viewing pyramid a traced camera instances: apex at the camera center, base along +z.
fn trace_camera_marker() -> semio_framework::MeshData {
    semio_framework::MeshData { positions: vec![0.0, 0.0, 0.0, -0.6, -0.45, 1.0, 0.6, -0.45, 1.0, 0.6, 0.45, 1.0, -0.6, 0.45, 1.0], indices: vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 1, 4, 1, 2, 3, 1, 3, 4], ..Default::default() }
}

/// 🧊️ The World3d wire shape of one mesh. `semio_framework::MeshData` derives `Serialize` only under
/// `cfg(test)`, so the buffers are named here rather than through a derive that does not exist in a
/// production build.
fn mesh_data_json(mesh: &semio_framework::MeshData) -> Value {
    json!({
        "positions": mesh.positions,
        "normals": mesh.normals,
        "colors": mesh.colors,
        "indices": mesh.indices,
        "uvs": mesh.uvs,
    })
}

fn world_instances_json(config: &RemodelingModelWindowConfig) -> String {
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

/// ☁️ `World3dScene.points_json` layers: the stored sparse/dense clouds, every stored trajectory camera
/// as its own (small, unattenuated) point layer, and the GCP world positions. A running reconstruction
/// shows its cameras and points through the framework `toolRunTrace` lane instead, and its provisional
/// result through the overlay document this window already renders.
/// `PackedF32`/`PackedU8`'s inner string is already a base64 little-endian buffer, matching
/// `positionsB64`/`colorsB64`'s wire shape byte-for-byte — no decode/re-encode round trip needed.
fn world_points_json(scene: &RemodelingSnapshot, config: &RemodelingModelWindowConfig) -> Option<String> {
    let mut layers: Vec<Value> = Vec::new();
    if config.layers.sparse {
        if let Some(sparse) = &scene.results.sparse {
            if !sparse.points.is_empty() {
                layers.push(json!({
                    "id": "remodeling-sparse",
                    "positionsB64": PackedF32::from_f32_slice(&sparse.points.to_f32_vec_from(&scene.durable_artifacts)).0,
                    "colorsB64": sparse.colors.as_ref().map(|colors| colors.0.clone()),
                    // 📏️ Screen pixels, like the camera and GCP layers: the reconstruction is
                    // normalised to unit RMS radius, so an attenuated (world-unit) size of 3 drew
                    // every point as a square larger than the object and the cloud as one grey blob.
                    "size": 3.0,
                    "sizeAttenuation": false,
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
                    "sizeAttenuation": false,
                }));
            }
        }
    }
    if let Some(trajectory) = scene.results.trajectory.as_ref().filter(|trajectory| config.layers.cameras && !trajectory.poses.is_empty()) {
        let positions: Vec<f32> = trajectory.poses.iter().flat_map(|pose| pose.translation).collect();
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

pub fn render(scene: &RemodelingSnapshot, config: &RemodelingModelWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    // 🕹️ The "assets" selection now lives in the framework-owned interaction domain (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `ArtifactEditor::render` carries no
    // `InteractionView`, so this scene payload can no longer embed a live selection; every
    // not-yet-migrated `world3d_selection_json` call site in this repo already passes an empty
    // selection for the same reason.
    let camera_json = dsl::json::to_json_string(&dsl::ToValue::to_value(&config.camera));
    let mut world_scene = world3d_scene(camera_json, world_meshes_json(scene), world_instances_json(config), world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    world_scene.points_json = world_points_json(scene, config);
    semio_framework_plugin::scene_surface(REMODELING_PLAY_SURFACE_MAIN, ContractSurfaceKind::World3d, &world_scene)
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
