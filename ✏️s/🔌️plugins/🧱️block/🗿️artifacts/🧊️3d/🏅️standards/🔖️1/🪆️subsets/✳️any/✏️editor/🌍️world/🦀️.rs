//! 🌐️ Block 3D play app — world-scene compute that needs both the document (`Block3dSnapshot`) and
//! this app's view state (`Block3dConfig`/`Block3dWindowView`). Kept out of
//! `crate::artifacts::block3d::schema`/`crate::artifacts::block3d::schema::inferences` on purpose: an
//! artifact must never depend on an app, and every function here takes at least one app-only type.

use crate::artifacts::block3d::{Block3dBrushPreview, Block3dWindowView};
use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::editor::block3d::config::{block3d_window_view, Block3dConfig};
use crate::BlockRepresentation;
use semio_framework_plugin::{world3d_camera_projection_json, world3d_mesh_id_from_url, world3d_selection_json, WorldProjectionConfig};
use dsl::json;
use dsl::os_pack::json::{parse, Value};

fn vec3(v: [f64; 3]) -> Value {
    Value::from(v.iter().map(|c| Value::from(*c)).collect::<Vec<Value>>())
}

//#region 🔖️Visibility
pub fn visible_representations<'a>(definition: &'a Block3dSnapshot, view: &Block3dWindowView) -> Vec<&'a BlockRepresentation> {
    if view.representation_ids.is_empty() {
        return definition.representations.iter().collect();
    }
    view.representation_ids.iter().filter_map(|id| definition.representations.iter().find(|representation| representation.id == *id)).collect()
}

pub fn arrangement_offset(arrangement: &str, index: usize, spacing: f64) -> [f64; 3] {
    let step = index as f64 * spacing;
    match arrangement {
        "x" => [step, 0.0, 0.0],
        "y" => [0.0, step, 0.0],
        "z" => [0.0, 0.0, step],
        _ => [0.0, 0.0, 0.0],
    }
}

pub fn instance_offset_for_representation(definition: &Block3dSnapshot, view: &Block3dWindowView, representation_id: &str) -> [f64; 3] {
    let visible = visible_representations(definition, view);
    visible.iter().position(|representation| representation.id == representation_id).map_or([0.0, 0.0, 0.0], |index| arrangement_offset(&view.arrangement, index, view.spacing))
}
//#endregion 🔖️Visibility

//#region 🔖️Scene
pub fn effective_camera<'a>(definition: &'a Block3dSnapshot, config: &'a Block3dConfig) -> &'a crate::BlockCamera3d {
    config.camera.as_ref().unwrap_or(&definition.camera3d)
}

pub fn representation_mesh_id(representation: &BlockRepresentation) -> String {
    representation.mesh_url.as_deref().map_or_else(|| format!("block3d-rep-{}", representation.id), world3d_mesh_id_from_url)
}

pub fn world_meshes_json(_definition: &Block3dSnapshot, visible: &[&BlockRepresentation]) -> String {
    let meshes: Vec<Value> = visible
        .iter()
        .filter_map(|representation| {
            let url = representation.mesh_url.as_deref()?;
            Some(json!({ "id": representation_mesh_id(representation), "url": url }))
        })
        .collect();
    Value::from(meshes).to_string()
}

pub fn world_instances_json(definition: &Block3dSnapshot, visible: &[&BlockRepresentation], view: &Block3dWindowView) -> String {
    let label = if definition.object_kind.label.is_empty() { definition.object_kind.name.clone() } else { definition.object_kind.label.clone() };
    let instances: Vec<Value> = visible
        .iter()
        .enumerate()
        .map(|(index, representation)| {
            let offset = arrangement_offset(&view.arrangement, index, view.spacing);
            let mesh_id = representation_mesh_id(representation);
            json!({
                "id": representation.id.as_str(),
                "meshId": mesh_id,
                "position": vec3(offset),
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("{} — {}", label, representation.name),
                "objectKind": definition.object_kind.id.as_str(),
            })
        })
        .collect();
    Value::from(instances).to_string()
}

fn vortex_kind_color(definition: &Block3dSnapshot, vortex_kind_id: &str) -> String {
    crate::artifacts::block3d::vortex_kinds_of(definition).iter().find(|kind| kind.id == vortex_kind_id).map_or_else(|| "#888888".into(), |kind| kind.color.clone())
}

pub fn block3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    format!("{object_id}:{vortex_id}")
}

pub fn world_vortices_json(definition: &Block3dSnapshot, config: &Block3dConfig, visible: &[&BlockRepresentation], view: &Block3dWindowView) -> String {
    let mut records = Vec::new();
    for (index, representation) in visible.iter().enumerate() {
        let offset = arrangement_offset(&view.arrangement, index, view.spacing);
        for vortex in &definition.vortices {
            let position = [vortex.position[0] + offset[0], vortex.position[1] + offset[1], vortex.position[2] + offset[2]];
            records.push(json!({
                "fullId": block3d_vortex_full_id(&representation.id, &vortex.id),
                "objectId": representation.id.as_str(),
                "vortexKind": vortex.vortex_kind.as_str(),
                "position": vec3(position),
                "direction": vec3(vortex.direction),
                "radius": vortex.radius,
                "color": vortex_kind_color(definition, &vortex.vortex_kind),
            }));
        }
    }
    if let Some(preview) = &config.brush_preview {
        let direction = if config.brush_flip { [-preview.direction[0], -preview.direction[1], -preview.direction[2]] } else { preview.direction };
        records.push(json!({
            "fullId": "__brush_preview__",
            "objectId": visible.first().map_or(crate::editor::block3d::BLOCK3D_WORLD_OBJECT_ID, |r| r.id.as_str()),
            "vortexKind": config.brush_vortex_kind_id.clone().unwrap_or_else(|| "brush".into()),
            "position": vec3(preview.position),
            "direction": vec3(direction),
            "radius": config.brush_radius,
            "color": "#60a5fa88",
        }));
    }
    Value::from(records).to_string()
}

pub fn world_camera_json(definition: &Block3dSnapshot, config: &Block3dConfig) -> String {
    let camera = effective_camera(definition, config);
    world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &WorldProjectionConfig::default())
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_ids`/`hovered_vortex_full_id`
/// used to live on `Block3dConfig`; both are now framework-owned (`vortex` domain,
/// `Block3dPlayApp::interaction_topology`). `ArtifactEditor::render` is not handed an `InteractionView`
/// (only `handle`/`copy_fragment`/`cut_operations`/`interaction_topology` are — see the SDK's
/// `ArtifactEditor` trait), so this can no longer embed the CURRENT selection/hover into the scene at
/// render time; it still declares the domain/granularity/mode the client uses to interpret picks.
/// Flagged as a known gap for a follow-up wave, mirroring the SDK's own `dispatch_emit_group` gap note.
pub fn world_selection_json(_config: &Block3dConfig) -> String {
    let mut value: Value = parse(&world3d_selection_json("replace", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity", json!("mesh"));
        object.insert("selectionMode", json!("mesh"));
        object.insert("vortexIds", json!([]));
    }
    value.to_string()
}

pub fn world_interaction_json(config: &Block3dConfig, window_id: &str) -> String {
    json!({ "activeUtility": block3d_window_view(config, window_id).active_utility }).to_string()
}
//#endregion 🔖️Scene

//#region 🔖️Brush
pub fn world_hit_to_local_vortex(position: [f64; 3], normal: [f64; 3], instance_offset: [f64; 3], brush_flip: bool) -> (Block3dBrushPreview, [f64; 3]) {
    let local_position = [position[0] - instance_offset[0], position[1] - instance_offset[1], position[2] - instance_offset[2]];
    let direction = if brush_flip { [-normal[0], -normal[1], -normal[2]] } else { normal };
    (Block3dBrushPreview { position: local_position, direction }, local_position)
}

pub fn default_vortex_kind() -> Block3dVortexKind {
    Block3dVortexKind { id: "vortex-kind-0".into(), name: "connector".into(), label: "Connector".into(), color: "#60a5fa".into(), default_cable_kind: "cable.link".into() }
}

pub fn resolve_brush_vortex_kind_id(definition: &Block3dSnapshot, config: &Block3dConfig) -> String {
    config.brush_vortex_kind_id.clone().or_else(|| crate::artifacts::block3d::vortex_kinds_of(definition).first().map(|kind| kind.id.clone())).unwrap_or_else(|| "vortex-kind-0".into())
}
//#endregion 🔖️Brush
