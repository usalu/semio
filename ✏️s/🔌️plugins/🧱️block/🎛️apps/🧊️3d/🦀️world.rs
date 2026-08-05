//! 🌐️ Block 3D play app — world-scene compute that needs both the document (`Block3dDefinition`) and
//! this app's view state (`Block3dConfig`/`Block3dWindowView`). Kept out of
//! `crate::artifacts::block3d::engine` on purpose: an artifact must never depend on an app, and every
//! function here takes at least one app-only type.

use crate::apps::block3d::config::{block3d_window_view, Block3dBrushPreview, Block3dConfig, Block3dWindowView};
use crate::artifacts::block3d::{Block3dDefinition, Block3dVortexKind};
use crate::core::BlockRepresentation;
use semio_framework_plugin::{world3d_camera_projection_json, world3d_mesh_id_from_url, world3d_selection_json, WorldProjectionConfig};
use serde_json::json;

//#region 🔖️Visibility
pub fn visible_representations<'a>(definition: &'a Block3dDefinition, view: &Block3dWindowView) -> Vec<&'a BlockRepresentation> {
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

pub fn instance_offset_for_representation(definition: &Block3dDefinition, view: &Block3dWindowView, representation_id: &str) -> [f64; 3] {
    let visible = visible_representations(definition, view);
    visible.iter().position(|representation| representation.id == representation_id).map_or([0.0, 0.0, 0.0], |index| arrangement_offset(&view.arrangement, index, view.spacing))
}
//#endregion 🔖️Visibility

//#region 🔖️Scene
pub fn effective_camera<'a>(definition: &'a Block3dDefinition, config: &'a Block3dConfig) -> &'a crate::core::BlockCamera3d {
    config.camera.as_ref().unwrap_or(&definition.camera3d)
}

pub fn representation_mesh_id(representation: &BlockRepresentation) -> String {
    representation.mesh_url.as_deref().map_or_else(|| format!("block3d-rep-{}", representation.id), world3d_mesh_id_from_url)
}

pub fn world_meshes_json(_definition: &Block3dDefinition, visible: &[&BlockRepresentation]) -> String {
    let meshes: Vec<serde_json::Value> = visible
        .iter()
        .filter_map(|representation| {
            let url = representation.mesh_url.as_deref()?;
            Some(json!({ "id": representation_mesh_id(representation), "url": url }))
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

pub fn world_instances_json(definition: &Block3dDefinition, visible: &[&BlockRepresentation], view: &Block3dWindowView) -> String {
    let label = if definition.object_kind.label.is_empty() { definition.object_kind.name.clone() } else { definition.object_kind.label.clone() };
    let instances: Vec<serde_json::Value> = visible
        .iter()
        .enumerate()
        .map(|(index, representation)| {
            let offset = arrangement_offset(&view.arrangement, index, view.spacing);
            let mesh_id = representation_mesh_id(representation);
            json!({
                "id": representation.id,
                "meshId": mesh_id,
                "position": offset,
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": format!("{} — {}", label, representation.name),
                "objectKind": definition.object_kind.id,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn vortex_kind_color(definition: &Block3dDefinition, vortex_kind_id: &str) -> String {
    definition.vortex_kinds.iter().find(|kind| kind.id == vortex_kind_id).map_or_else(|| "#888888".into(), |kind| kind.color.clone())
}

pub fn block3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    format!("{object_id}:{vortex_id}")
}

pub fn world_vortices_json(definition: &Block3dDefinition, config: &Block3dConfig, visible: &[&BlockRepresentation], view: &Block3dWindowView) -> String {
    let mut records = Vec::new();
    for (index, representation) in visible.iter().enumerate() {
        let offset = arrangement_offset(&view.arrangement, index, view.spacing);
        for vortex in &definition.vortices {
            let position = [vortex.position[0] + offset[0], vortex.position[1] + offset[1], vortex.position[2] + offset[2]];
            records.push(json!({
                "fullId": block3d_vortex_full_id(&representation.id, &vortex.id),
                "objectId": representation.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": vortex.direction,
                "radius": vortex.radius,
                "color": vortex_kind_color(definition, &vortex.vortex_kind),
            }));
        }
    }
    if let Some(preview) = &config.brush_preview {
        let direction = if config.brush_flip { [-preview.direction[0], -preview.direction[1], -preview.direction[2]] } else { preview.direction };
        records.push(json!({
            "fullId": "__brush_preview__",
            "objectId": visible.first().map_or(crate::apps::block3d::BLOCK3D_WORLD_OBJECT_ID, |r| r.id.as_str()),
            "vortexKind": config.brush_vortex_kind_id.clone().unwrap_or_else(|| "brush".into()),
            "position": preview.position,
            "direction": direction,
            "radius": config.brush_radius,
            "color": "#60a5fa88",
        }));
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_camera_json(definition: &Block3dDefinition, config: &Block3dConfig) -> String {
    let camera = effective_camera(definition, config);
    world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &WorldProjectionConfig::default())
}

pub fn world_selection_json(config: &Block3dConfig) -> String {
    let vortex_ids: Vec<String> = config.selected_ids.iter().filter(|id| id.starts_with("vortex:")).map(|id| id.strip_prefix("vortex:").unwrap_or(id).to_string()).collect();
    let mut value: serde_json::Value = serde_json::from_str(&world3d_selection_json("replace", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("vortexIds".into(), json!(vortex_ids));
        if let Some(hover) = config.hovered_vortex_full_id.as_deref() {
            object.insert("hoveredVortexFullId".into(), json!(hover));
        }
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

pub fn resolve_brush_vortex_kind_id(definition: &Block3dDefinition, config: &Block3dConfig) -> String {
    config.brush_vortex_kind_id.clone().or_else(|| definition.vortex_kinds.first().map(|kind| kind.id.clone())).unwrap_or_else(|| "vortex-kind-0".into())
}
//#endregion 🔖️Brush
