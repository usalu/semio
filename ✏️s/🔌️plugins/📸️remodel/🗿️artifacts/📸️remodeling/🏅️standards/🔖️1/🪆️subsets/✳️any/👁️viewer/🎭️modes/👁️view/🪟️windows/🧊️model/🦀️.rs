//! 🧊️ Remodeling viewer — the Model window: a read-only World3d render of the reconstructed mesh, the
//! sparse/dense clouds, the recovered camera positions and the ground control points. Reads through
//! the same fixed-constant/committed-bounded resolver the sibling editor's Model window uses — this
//! file itself imports nothing from the sibling
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

use crate::{PackedF32, RemodelingSnapshot};
use semio_framework_plugin::{world3d_scene, world3d_selection_json, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions, WorldSunConfig};
// 🧬️ Two `SurfaceKind` enums coexist: `WindowKindDefinition` carries the retained `ui_wgpu` one
// (re-exported by the SDK root), while `scene_surface` takes the semantic contract's — same spelling,
// different types, so both are imported explicitly.
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "remodeling-view-model";
pub const BODY_KEY: &str = "remodeling.view.model";
const SURFACE_ID: &str = "remodeling.view.scene3d/model";
/// 👁️ Matches the editor's `REMODELING_MESH_ID` literal — duplicated on purpose rather than imported
/// through the sibling editor module, which `policyViewerPurityBreaches` forbids outright.
const REMODELING_VIEW_MESH_ID: &str = "remodeling-result";
/// 👁️ A viewer has no persisted navigation owner, so every render uses this explicit shared pose.
const REMODELING_VIEW_VIEWPORT: store::Viewport3dOrbit = store::Viewport3dOrbit { position: [4.0, -4.0, 3.0], target: [0.0; 3], zoom: 1.0, up: None };
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Model", "Modell"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "remodeling-model".into(),
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
/// `s.stdio.semio/v1/mesh` CHILD's real geometry through the same production bounded resolver. An
/// unavailable durable handle renders no mesh entity, matching the editor's behavior.
fn world_meshes_json(scene: &RemodelingSnapshot) -> String {
    let Some(mesh) = crate::resolve_bounded_remodeling_mesh(&scene.durable_artifacts, &scene.results.mesh.mesh) else {
        return "[]".into();
    };
    serde_json::to_string(&vec![json!({ "id": REMODELING_VIEW_MESH_ID, "data": mesh_data_json(&mesh) })]).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Read-only twin of the sibling editor's `mesh_data_json` — duplicated on purpose rather than
/// imported through the editor module, which `policyViewerPurityBreaches` forbids outright.
/// `semio_framework::MeshData` derives `Serialize` only under `cfg(test)`, so the buffers are named.
fn mesh_data_json(mesh: &semio_framework::MeshData) -> Value {
    json!({
        "positions": mesh.positions,
        "normals": mesh.normals,
        "colors": mesh.colors,
        "indices": mesh.indices,
        "uvs": mesh.uvs,
    })
}

/// 👁️ Unconditionally visible mesh instance — a viewer has no persisted per-session layer toggles, so
/// unlike the editor's `world_instances_json` this never gates on a `layers.mesh` flag.
fn world_instances_json() -> String {
    serde_json::to_string(&vec![json!({
        "id": REMODELING_VIEW_MESH_ID,
        "meshId": REMODELING_VIEW_MESH_ID,
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
fn world_points_json(scene: &RemodelingSnapshot) -> Option<String> {
    let mut layers: Vec<Value> = Vec::new();
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
    if !scene.job.camera_poses_preview.is_empty() {
        let positions: Vec<f32> = scene.job.camera_poses_preview.iter().flat_map(|pose| pose.translation).collect();
        layers.push(json!({
            "id": "remodeling-camera-poses",
            "positionsB64": PackedF32::from_f32_slice(&positions).0,
            "colorsB64": Value::Null,
            "size": 9.0,
            "sizeAttenuation": false,
        }));
    }
    if !scene.gcps.is_empty() {
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

/// 👁️ Pure `RemodelingSnapshot -> BuiltNode` read: hardcoded default camera/sun, no selection overlay,
/// every point layer unconditionally visible, mesh content real whenever the working-scene cache is
/// warm.
pub fn render(scene: &RemodelingSnapshot) -> UiAssemblyResult<BuiltNode> {
    let mut world_scene = world3d_scene(
        dsl::json::to_json_string(&dsl::ToValue::to_value(&REMODELING_VIEW_VIEWPORT)),
        world_meshes_json(scene),
        world_instances_json(),
        world3d_selection_json("rectangle", &[], None),
        &WorldSunConfig::default(),
    );
    world_scene.points_json = world_points_json(scene);
    semio_framework_plugin::scene_surface(SURFACE_ID, ContractSurfaceKind::World3d, &world_scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
