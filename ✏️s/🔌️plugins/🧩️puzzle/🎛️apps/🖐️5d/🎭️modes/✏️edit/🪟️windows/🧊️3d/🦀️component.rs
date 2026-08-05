//! 🧊️ Puzzle 5d play app — the `World3d` window kind: the volumetric projection of the unified 5d
//! document. Owns the world scene payload (instances/meshes/grips/fasteners, the selection and
//! gumball descriptor, the chunking/environment blocks and the interaction channel), binds the
//! transform-gumball and relocate utilities plus the two shared brush/fill ones, and scopes the
//! transform/3D-camera actions (`🎬️actions`). Its only genuinely 3D-specific chrome measure is the
//! sun group in `🎚️options/☀️sun`; the brush/fill Utility Options it shares with the 2D window come
//! from the mode's own `🎚️options/*`.

use crate::apps::puzzle5d::config::{Puzzle5dCamera3d, Puzzle5dRuntime};
use crate::apps::puzzle5d::modes::edit;
use crate::apps::puzzle5d::modes::edit::options as mode_options;
use crate::apps::puzzle5d::modes::edit::windows::board2d;
use crate::apps::puzzle5d::modes::edit::windows::world3d::{actions, options, utilities};
use crate::apps::puzzle5d::terminology::{puzzle5d_localized, Puzzle5dLabels};
use crate::apps::puzzle5d::{
    collect_mesh_urls, gumball_target_world, puzzle5d_brush_target_grip, puzzle5d_grip_full_id, puzzle5d_gumball_active, puzzle5d_scene_mode, puzzle5d_transform_handle, part_scale_json, resolve_grip_world_position, resolve_part_mesh_url,
    world_grip_direction, world_grip_position, Puzzle5dDocument, Puzzle5dScene, PUZZLE5D_FALLBACK_MESH_KIND, PUZZLE5D_PLAY_CONTROLLER_ID,
};
use crate::artifacts::puzzle5d::engine::Puzzle5dPrecomputeSession;
use semio_framework_plugin::{
    build_world_3d_scene, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, SurfaceKind, UiNode, WindowEngagement, WindowEngagementSlot,
    WindowKindDefinition, WindowMeasure, WindowOptions,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "puzzle5d-3d";
pub const BODY_KEY: &str = "puzzle.5d.play.3d";
pub const SURFACE_ID: &str = "puzzle.5d.play.3d";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle5d::create_puzzle5d_app`.
///
/// 🔁️ The `brush`/`fill` utility ids it binds resolve to the definitions declared once under the 2D
/// window (`🪟️windows/◻2d/🪛️utilities/{🖌️brush,🪣️fill}`) — both windows expose the identical utility,
/// so it is never duplicated here.
pub fn definition(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle5d_localized(|l| l.window_3d),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "puzzle5d-3d".into(),
        options: WindowOptions { measures: window_measures(envelope, precompute, labels), engagement: WindowEngagementSlot::Some(engagement(envelope, labels)) },
        actions: actions::ids(),
        utilities: vec![
            utilities::transform::MOVE_UTILITY_ID.into(),
            utilities::transform::ROTATE_UTILITY_ID.into(),
            utilities::transform::SCALE_UTILITY_ID.into(),
            board2d::utilities::brush::UTILITY_ID.into(),
            board2d::utilities::fill::UTILITY_ID.into(),
            utilities::world_relocate::UTILITY_ID.into(),
        ],
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window: its own sun group plus the mode-level brush/fill
/// Utility Options groups it shares with the 2D window.
pub fn window_measures(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    vec![options::sun::measure(&envelope.runtime), mode_options::fill::measure(envelope, labels), mode_options::brush::measure(envelope, precompute, labels)]
}

pub fn engagement(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowEngagement {
    edit::puzzle5d_engagement(envelope, WINDOW_KIND_ID, labels)
}
//#endregion 🔖️Definition

//#region 🔖️SceneJson
pub fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
    json!({ "position": camera.position, "target": camera.target, "zoom": camera.zoom, "fov": 45.0 }).to_string()
}

fn world_instances_json(document: &Puzzle5dDocument, runtime: &Puzzle5dRuntime) -> String {
    let instances: Vec<Value> = document
        .parts
        .iter()
        .map(|part| {
            let selected = runtime.selection.part_ids.contains(&part.id);
            let hovered = runtime.hovered_part_id.as_deref() == Some(part.id.as_str());
            let mesh_id = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).map_or_else(|| PUZZLE5D_FALLBACK_MESH_KIND.into(), |url| world3d_mesh_id_from_url(&url));
            json!({
                "id": part.id,
                "meshId": mesh_id,
                "position": part.part_3d.origin,
                "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": part_scale_json(part),
                "label": part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone()),
                "selected": selected,
                "hovered": hovered,
                "disabled": part.part_2d.locked.unwrap_or(false),
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(document: &Puzzle5dDocument) -> String {
    world3d_meshes_json_from_urls(&collect_mesh_urls(document))
}

fn grip_color(kind_catalogs: Option<&Value>, grip_kind: &str) -> String {
    kind_catalogs
        .and_then(|catalogs| catalogs.get("grips"))
        .and_then(|value| value.as_array())
        .and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(grip_kind)))
        .and_then(|entry| entry.get("color").and_then(|value| value.as_str()).map(str::to_string))
        .unwrap_or_else(|| "#38bdf8".into())
}

fn world_grips_json(document: &Puzzle5dDocument) -> String {
    let mut records = Vec::new();
    for part in &document.parts {
        for grip in &part.grips {
            records.push(json!({
                "fullId": puzzle5d_grip_full_id(&part.id, &grip.id),
                "objectId": part.id,
                "vortexKind": grip.grip_kind,
                "position": world_grip_position(part, grip),
                "direction": world_grip_direction(part, grip),
                "radius": grip.grip_3d.radius.max(0.36),
                "color": grip_color(document.kind_catalogs.as_ref(), &grip.grip_kind),
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_fasteners_json(document: &Puzzle5dDocument) -> String {
    let records: Vec<Value> = document
        .fasteners
        .iter()
        .filter_map(|fastener| {
            let from = resolve_grip_world_position(document, &fastener.source)?;
            let to = resolve_grip_world_position(document, &fastener.target)?;
            Some(json!({ "id": fastener.id, "from": from, "to": to, "color": "#60a5fa" }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

/// 🎯️ Base selection JSON augmented with the mesh granularity, transform tool, and gumball fields the world-3d host reads.
fn world_selection_json_ex(envelope: &Puzzle5dScene) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, runtime.selection.part_ids.as_slice(), runtime.hovered_part_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        if let Some(transform_mode) = puzzle5d_transform_handle(&envelope.active_utility) {
            object.insert("transformMode".into(), json!(transform_mode));
        }
        if let Some(active_id) = runtime.selection.part_ids.first() {
            object.insert("activeObjectId".into(), json!(active_id));
        }
        let gumball_active = puzzle5d_gumball_active(runtime, &envelope.active_utility);
        object.insert("gumballActive".into(), json!(gumball_active));
        if gumball_active {
            if let Some(target) = gumball_target_world(envelope) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn world_interaction_json(runtime: &Puzzle5dRuntime, active_utility: &str) -> String {
    json!({
        "activeUtility": puzzle5d_scene_mode(active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "fillCount": runtime.fill_count,
        "hoveredVortexFullId": runtime.selection.grip_ids.first().map(str::to_string),
    })
    .to_string()
}

/// 👻️ Ghost placement for the brush utility — only while the brush utility is actually active.
fn world_brush_preview_json(session: &Puzzle5dPrecomputeSession, envelope: &Puzzle5dScene) -> Option<String> {
    if envelope.active_utility != board2d::utilities::brush::UTILITY_ID {
        return None;
    }
    let full_id = puzzle5d_brush_target_grip(envelope)?;
    session.brush_preview_json(&full_id, envelope.runtime.brush_candidate_index)
}
//#endregion 🔖️SceneJson

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession) -> UiNode {
    let brush_preview = world_brush_preview_json(precompute, envelope);
    build_world_3d_scene(
        SURFACE_ID,
        PUZZLE5D_PLAY_CONTROLLER_ID,
        world3d_scene_extended(
            camera3d_json(&envelope.runtime.camera3d),
            world_meshes_json(&envelope.document),
            world_instances_json(&envelope.document, &envelope.runtime),
            world_selection_json_ex(envelope),
            Some(world_grips_json(&envelope.document)),
            Some(world_fasteners_json(&envelope.document)),
            None,
            None,
            brush_preview,
            Some(world_interaction_json(&envelope.runtime, &envelope.active_utility)),
            None,
            None,
            Some(world3d_chunking_json(256.0, 8000.0)),
            Some(world3d_environment_json(&envelope.runtime.sun)),
            None,
            None,
            None,
            None,
            None,
        ),
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle5d::testkit::*;

    #[test]
    fn renders_the_world_scene() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("world-3d"));
    }
}
//#endregion 🧪️Tests
