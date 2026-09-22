//! 🧊️ Puzzle 5d play app — the `World3d` window kind: the volumetric projection of the unified 5d
//! document. Owns the world scene payload (instances/meshes/grips/fasteners, the selection and
//! gumball descriptor, the chunking/environment blocks and the interaction channel), binds the
//! transform-gumball and relocate utilities plus the two shared brush/fill ones, and scopes the
//! transform/3D-camera actions (`🎬️actions`). Its own chrome lives in `☑️options/*` — projection,
//! grip show/direction, LOD, grid and sun are 3D-specific; the selection group is the shared body the
//! board pane also renders. The brush Utility Options it shares with the 2D window come from the
//! mode's own `☑️options/*`, and fill's from the mode-level tool `🛠️tools/🪣️fill`.

use crate::editor::puzzle5d::config::{Puzzle5dCamera3d, Puzzle5dRuntime};
use crate::editor::puzzle5d::modes::edit;
use crate::editor::puzzle5d::modes::edit::options as mode_options;
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::modes::edit::windows::world3d::{options, utilities};
use crate::editor::puzzle5d::precompute::{puzzle5d_placement_entity, PUZZLE5D_WORLD_MESH_KINDS};
use crate::editor::puzzle5d::terminology::{puzzle5d_localized, Puzzle5dLabels};
use crate::editor::puzzle5d::{
    part_scale_json, puzzle5d_grip_full_id, puzzle5d_gumball_active, puzzle5d_scene_mode, puzzle5d_transform_handle, resolve_grip_world_position, resolve_part_mesh_url, world_grip_direction,
    target_volume_scale_json, world_grip_position, Puzzle5dDocument, Puzzle5dInteractionSnapshot, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_FALLBACK_MESH_KIND, PUZZLE5D_GRIP_SHOW_ALWAYS,
    PUZZLE5D_TARGET_VOLUME_COLOR,
};
use semio_framework_plugin::{
    world3d_camera_projection_json, world3d_chunking_json, world3d_environment_json, world3d_fit_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, World3dScene, world3d_selection_json, SurfaceKind, ToolRunView, WindowEngagement,
    WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions,
};
use semio_framework_ui_contract::BuiltNode;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "puzzle5d-3d";
pub const BODY_KEY: &str = "puzzle.5d.play.3d";
pub const SURFACE_ID: &str = "puzzle.5d.play.3d";

/// 🎯️ Padding `WorldAutoFit` frames a swapped document with — a quarter of the bounding sphere's
/// radius of air, the same air `puzzle3d` (`PUZZLE3D_FIT_PADDING`) and `block3d` leave.
pub const PUZZLE5D_FIT_PADDING: f64 = 1.25;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
///
/// 🔁️ The `brush` utility id it binds resolves to the definition declared once under the 2D window
/// (`🪟️windows/◻️2d/🪛️utilities/🖌️brush`) — both windows expose the identical utility, so it is never
/// duplicated here.
pub fn definition(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle5d_localized(|l| l.window_3d),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "puzzle5d-3d".into(),
        options: WindowOptions { measures: window_measures(envelope, labels), engagement: WindowEngagementSlot::Some(engagement(envelope, labels)) },
        actions: Vec::new(),
        utilities: vec![
            utilities::transform::MOVE_UTILITY_ID.into(),
            utilities::transform::ROTATE_UTILITY_ID.into(),
            utilities::transform::SCALE_UTILITY_ID.into(),
            board2d::utilities::brush::UTILITY_ID.into(),
            utilities::volume_brush::UTILITY_ID.into(),
            utilities::world_relocate::UTILITY_ID.into(),
        ],
        interactions: vec![semio_framework_plugin::InteractionRef::new(crate::editor::puzzle5d::PUZZLE5D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window: its own projection / grip / LOD / grid / selection /
/// sun groups plus the transform gumball's Utility Options and the mode-level brush Utility Options
/// group it shares with the 2D window. Fill is a mode-level TOOL, so its count and distribution measures are
/// the tool options rail's (`🛠️tools/🪣️fill`), never this window's.
pub fn window_measures(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let mut measures = vec![
        options::projection::measure(&envelope.runtime),
        options::grip::show_measure(&envelope.runtime, labels),
        options::grip::direction_measure(&envelope.runtime, labels),
        options::lod::measure(&envelope.runtime, labels),
        options::grid::measure(&envelope.runtime, labels),
        options::select::measure(&envelope.runtime, labels),
        options::sun::measure(&envelope.runtime, labels.is_de()),
    ];
    measures.extend(utilities::transform::options(&envelope.runtime, labels));
    measures.push(utilities::volume_brush::options(&envelope.runtime, labels));
    measures.push(mode_options::brush::measure(envelope, labels));
    measures
}

pub fn engagement(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowEngagement {
    edit::puzzle5d_engagement(envelope, WINDOW_KIND_ID, labels, None)
}
//#endregion 🔖️Definition

//#region 🔖️SceneJson
/// 🎥️ The camera payload the world host reads, including this pane's own projection taxonomy
/// (`setProjection`/`setProjectionParam`) — `world3d_camera_projection_json` derives the host's `fov`
/// and orientation from it, so the hand-rolled fixed-45° object it replaces is gone.
pub fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
    world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &camera.projection)
}

/// 🕹️ `selected`/`hovered` per instance are painted from the live `vortex` domain the ONE 5d
/// interaction snapshot reads — the same domain the board pane writes, so a part picked on the board
/// lights up here. A part a tool run holds provisionally carries `provisional: true`.
pub fn world_instances_json(document: &Puzzle5dDocument, interaction: &Puzzle5dInteractionSnapshot, tool_run: Option<&ToolRunView>) -> String {
    let selected = interaction.selected_part_ids();
    let instances: Vec<Value> = document
        .parts
        .iter()
        .map(|part| {
            let mesh_id = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).map_or_else(|| PUZZLE5D_FALLBACK_MESH_KIND.into(), |url| world3d_mesh_id_from_url(&url));
            json!({
                "id": part.id,
                "meshId": mesh_id,
                "position": part.part_3d.origin,
                "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": part_scale_json(part),
                "label": part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone()),
                "selected": selected.iter().any(|id| id == &part.id),
                "hovered": interaction.hovered.iter().any(|id| id == &part.id),
                "disabled": part.part_2d.locked.unwrap_or(false),
                "provisional": tool_run.is_some_and(|run| run.provisional_entities.contains(&puzzle5d_placement_entity(&part.id))),
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 🎯️ Document IDENTITY for the world's `fit` lane: what this document IS (its schema, its domain,
/// its label and the kind catalogs its parts resolve their meshes through), never where its parts
/// currently sit. `WorldAutoFit` refits once per revision, so this is the difference between "frame
/// the example that was just loaded" and "yank the camera on every part drag".
///
/// 🐛️ ticket 26/09/19 (play grid, visual audit): the `puzzle5d` pane booted ready with its curated
/// example, the board pane drew the assembled ring of capsules and the 3D viewport drew nothing but
/// the camera gizmo and the grid. A world window that publishes NO fit lane can be framed by nothing
/// but the camera the document authored, so a document whose parts sit anywhere else is outside the
/// frustum and the viewport looks empty — the identical defect `block3d` carried, closed the
/// identical way. The bounds stay unpublished (`world3d_fit_json(..., None)`, as puzzle3d and block3d
/// do): a part is a mesh URL here, so this crate knows what it asked for but never how big the
/// delivered geometry is — only the render host, which loaded it, can measure that.
pub fn world_fit_revision(document: &Puzzle5dDocument) -> u32 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    document.schema.hash(&mut hasher);
    document.domain.hash(&mut hasher);
    document.label.hash(&mut hasher);
    document.kind_catalogs.as_ref().map(ToString::to_string).hash(&mut hasher);
    document.kind_compatibility.as_ref().map(ToString::to_string).hash(&mut hasher);
    (hasher.finish() >> 32) as u32
}

/// 🧵️ `meshesJson` from the world mesh lane, in exactly the order the fill run's trace subjects index.
fn world_meshes_json(mesh_lane: &[String]) -> String {
    let kinds = PUZZLE5D_WORLD_MESH_KINDS.min(mesh_lane.len());
    world3d_meshes_json_from_kinds_and_urls(&mesh_lane[..kinds], &mesh_lane[kinds..])
}

fn grip_color(kind_catalogs: Option<&Value>, grip_kind: &str) -> String {
    kind_catalogs
        .and_then(|catalogs| catalogs.get("grips"))
        .and_then(|value| value.as_array())
        .and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(grip_kind)))
        .and_then(|entry| entry.get("color").and_then(|value| value.as_str()).map(str::to_string))
        .unwrap_or_else(|| "#38bdf8".into())
}

/// 🖌️ Placement utilities publish every part's grip markers so the host can hit-test them without a
/// prior part selection (those utilities block instance pick).
fn grip_markers_publish_for_utility(active_utility: &str) -> bool {
    matches!(active_utility, "brush" | "fill")
}

/// 👁️ True when this part's grips should render — `PUZZLE5D_GRIP_SHOW_ALWAYS`, a live hover/selection
/// touch, or an armed placement utility. `…_SELECTED` otherwise hides markers until the part (or one
/// of its own grips) is marked.
fn part_grips_visible(part: &Puzzle5dPart, runtime: &Puzzle5dRuntime, interaction: &Puzzle5dInteractionSnapshot, active_utility: &str) -> bool {
    runtime.grip_show == PUZZLE5D_GRIP_SHOW_ALWAYS || interaction.touches_part(part) || grip_markers_publish_for_utility(active_utility)
}

/// 🤏️ Per-grip marker records. `selected`/`hovered` are painted from the live `vortex` domain —
/// `WorldVortexMarkers` reads them off each record, not off `selectionJson`.
pub fn world_grips_json(document: &Puzzle5dDocument, runtime: &Puzzle5dRuntime, interaction: &Puzzle5dInteractionSnapshot, active_utility: &str) -> String {
    let selected_grips = interaction.selected_grip_ids();
    let mut records = Vec::new();
    for part in &document.parts {
        if !part_grips_visible(part, runtime, interaction, active_utility) {
            continue;
        }
        for grip in &part.grips {
            let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
            records.push(json!({
                "fullId": full_id,
                "objectId": part.id,
                "vortexKind": grip.grip_kind,
                "position": world_grip_position(part, grip),
                "direction": world_grip_direction(part, grip),
                "radius": grip.grip_3d.radius.max(0.36),
                "color": grip_color(document.kind_catalogs.as_ref(), &grip.grip_kind),
                "displayDirection": runtime.grip_direction,
                "selected": selected_grips.iter().any(|id| id == &full_id),
                "hovered": interaction.hovered.iter().any(|id| id == &full_id),
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

/// 🎯️ The host's `WorldSelectionRecord` for this pane: the framework-owned `vortex` domain projected
/// onto exactly the field names `World3dHost`'s parser reads (`ids`/`activeObjectId`/`hoveredId`),
/// plus the mesh granularity, transform mode and gumball descriptor. Grip selection/hover is
/// deliberately NOT here — `WorldSelectionRecord` has no grip field, so a marker's own flags travel
/// on `vorticesJson` (see `world_grips_json`).
pub fn world_selection_json_ex(envelope: &Puzzle5dScene) -> String {
    let runtime = &envelope.runtime;
    let interaction = &envelope.interaction;
    let part_ids = interaction.selected_part_ids();
    let hovered_id = interaction.hovered_part_id(&envelope.document).map(str::to_string);
    let mut value: Value = serde_json::from_str(&world3d_selection_json("pick", part_ids, hovered_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        if let Some(id) = part_ids.first() {
            object.insert("activeObjectId".into(), json!(id));
        }
        if let Some(transform_mode) = puzzle5d_transform_handle(&envelope.active_utility) {
            object.insert("transformMode".into(), json!(transform_mode));
            object.insert(
                "gumballConfig".into(),
                json!({
                    "moveAxes": runtime.transform_move,
                    "movePlanes": runtime.transform_move,
                    "rotate": runtime.transform_rotate,
                    "scaleAxes": false,
                    "scalePlanes": false,
                    "scaleUniform": false,
                }),
            );
        }
        object.insert("gumballActive".into(), json!(puzzle5d_gumball_active(runtime, &envelope.active_utility, interaction)));
    }
    value.to_string()
}

fn world_interaction_json(runtime: &Puzzle5dRuntime, active_utility: &str) -> String {
    json!({
        "activeUtility": puzzle5d_scene_mode(active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "fillCount": runtime.fill_count,
    })
    .to_string()
}

/// 🧊️ The target volumes the world host paints as wireframe boxes — the same record shape puzzle 3d
/// publishes, so `World3dHost`'s generic `targetVolumesJson` lane reads both artifacts unchanged.
pub fn world_target_volumes_json(document: &Puzzle5dDocument) -> String {
    let records: Vec<Value> = document
        .target_volumes
        .iter()
        .map(|volume| {
            json!({
                "id": volume.id,
                "origin": volume.origin,
                "orientation": volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": target_volume_scale_json(volume),
                "color": PUZZLE5D_TARGET_VOLUME_COLOR,
                "hidden": volume.hidden,
                "locked": volume.locked,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

//#endregion 🔖️SceneJson

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, tool_run: Option<&ToolRunView>, mesh_lane: &[String]) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let mut scene = World3dScene::base(camera3d_json(&envelope.runtime.camera3d), world_meshes_json(mesh_lane), world_instances_json(&envelope.document, &envelope.interaction, tool_run), world_selection_json_ex(envelope));
    scene.vortices_json = Some(world_grips_json(&envelope.document, &envelope.runtime, &envelope.interaction, &envelope.active_utility));
    scene.attractions_json = Some(world_fasteners_json(&envelope.document));
    scene.interaction_json = Some(world_interaction_json(&envelope.runtime, &envelope.active_utility));
    scene.fit_json = Some(world3d_fit_json(world_fit_revision(&envelope.document), PUZZLE5D_FIT_PADDING, None));
    scene.chunking_json = Some(world3d_chunking_json(256.0, 8000.0));
    scene.environment_json = Some(world3d_environment_json(&envelope.runtime.sun));
    scene.target_volumes_json = Some(world_target_volumes_json(&envelope.document));
    semio_framework_plugin::scene_surface(SURFACE_ID, semio_framework_ui_contract::SurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
