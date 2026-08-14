//! 🧊️ Puzzle 3d play app — the one `World3d` window kind. Owns the viewport's whole scene
//! projection: the instance/mesh/vortex/attraction/target-volume/reference payloads, the selection
//! and gumball descriptor, the LOD/chunking/environment blocks and the interaction channel (active
//! utility, suggestion popup, fill-build progress, reveal cutoffs) the host renderer reads. Also
//! owns the engagement HUD and collects its chrome measures from the mode's `🎚️options/*` and its own
//! `🪛️utilities/*`.
//!
//! 🪟️ One KIND, many INSTANCES: the default layout splits it into an orthographic "Top" and a
//! three-point "Perspective" pane, and every view-local option (camera, grid, LOD, vortex display,
//! sun, selection method) is per instance — see `🦀️config.rs`'s `load_window`/`save_window`.

use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use crate::apps::puzzle3d::modes::edit::options;
use crate::apps::puzzle3d::modes::edit::windows::main::utilities;
use crate::apps::puzzle3d::terminology::{puzzle3d_localized, Puzzle3dLabels};
use crate::apps::puzzle3d::{
    collect_mesh_urls, object_scale_json, puzzle3d_action, puzzle3d_vortex_full_id, quat_rotate_vector, resolve_object_mesh_url, target_volume_scale_json, Puzzle3dFixture,
    Puzzle3dFixtureMeta, Puzzle3dObject, Puzzle3dScene, Puzzle3dVortex, PUZZLE3D_FALLBACK_MESH_KIND, PUZZLE3D_VORTEX_SHOW_ALWAYS,
};
use crate::apps::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use semio_framework_plugin::{
    build_world_3d_scene, world3d_camera_projection_json, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene_extended, world3d_selection_json, SurfaceKind, UiNode,
    WindowEngagement, WindowEngagementInput, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions,
};
use serde_json::{json, Value};
use std::hash::{Hash, Hasher};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "puzzle3d-main";
pub const WINDOW_INSTANCE_TOP: &str = "puzzle3d-main-top";
pub const WINDOW_INSTANCE_PERSPECTIVE: &str = "puzzle3d-main-perspective";
pub const BODY_KEY: &str = "puzzle3d.play.composite";
pub const SURFACE_VIEWPORT: &str = "puzzle.3d.play.viewport";
/// 🪟️ Display-template id for an orthographic top pane — mirrors `encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })`.
pub const TEMPLATE_TOP: &str = r#"world-projection:{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}}"#;
/// 🪟️ Display-template id for a three-point perspective pane — mirrors `encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })`.
pub const TEMPLATE_PERSPECTIVE: &str = r#"world-projection:{"mode":{"kind":"threePoint","fov":50},"orientation":{"type":"free"}}"#;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle3d::create_puzzle3d_app`.
pub fn definition(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: puzzle3d_localized(|l| l.window_main),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "puzzle".into(),
        // 🪟️ `options.measures` stays empty: puzzle3d's chrome is config-derived per frame by
        // `ArtifactApp::window_measures`, never frozen into the static manifest.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement(envelope, labels)) },
        actions: Vec::new(),
        utilities: vec![utilities::transform::UTILITY_ID.into(), utilities::brush::UTILITY_ID.into(), utilities::volume_brush::UTILITY_ID.into(), utilities::world_relocate::UTILITY_ID.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(crate::apps::puzzle3d::PUZZLE3D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for one window instance, collected from the mode's `🎚️options/*`
/// components plus this window's own `🪛️utilities/*` option groups.
pub fn window_measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    vec![
        options::projection::measure(&envelope.runtime),
        options::vortex::show_measure(&envelope.runtime, labels),
        options::vortex::direction_measure(&envelope.runtime, labels),
        options::lod::measure(&envelope.runtime, labels),
        options::grid::measure(&envelope.runtime, labels),
        options::select::measure(&envelope.runtime, labels),
        options::sun::measure(&envelope.runtime),
        utilities::transform::options(&envelope.runtime, labels),
        utilities::brush::options(envelope, precompute, labels),
        utilities::volume_brush::options(&envelope.runtime, labels),
    ]
}
//#endregion 🔖️Definition

//#region 🔖️SceneMode
/// 🧭️ The select/brush/fill interaction mode the world engine reads, derived from the flat active
/// utility (the transform gumball and `worldRelocate` both present as `select`).
pub fn scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        "volumeBrush" => "volumeBrush",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when a transform utility is active.
pub fn transform_handle(active_utility: &str) -> Option<&'static str> {
    if active_utility == utilities::transform::UTILITY_ID {
        Some("transform")
    } else {
        None
    }
}

/// 🧭️ Whether the active utility is a transform gumball mode.
pub fn transform_utility_active(active_utility: &str) -> bool {
    transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render for the current selection and utility. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection is framework-owned and
/// `ArtifactApp::render` (this fn's only caller) never gained an `InteractionView` parameter, so this
/// can no longer see whether anything is selected — see `panels::inspection::render`'s doc comment
/// for the framework-level gap this is downstream of. Defaults to "never render an unattached
/// gumball" rather than always-on.
pub fn gumball_active(_runtime: &Puzzle3dRuntime, _active_utility: &str) -> bool {
    false
}
//#endregion 🔖️SceneMode

//#region 🔖️SceneJson
pub fn camera_json(runtime: &Puzzle3dRuntime) -> String {
    let camera = &runtime.camera;
    world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &camera.projection)
}

/// 🙈️ Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
/// `revealIndex` is omitted entirely for untagged objects rather than emitted as `null`: the host's reveal cutoff (`framework/renderer/react`'s `applyRevealCutoff`) only skips instances with no reveal index, and a JSON `null` would coerce to `0` and hide every ordinary object behind the boot cutoff.
/// Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks.
pub fn world_instances_geometry_json(fixture: &Puzzle3dFixture) -> String {
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object, &fixture.meta).map_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into(), |url| world3d_mesh_id_from_url(&url));
            let scale = if object.hidden { json!([0.0, 0.0, 0.0]) } else { json!(object_scale_json(object)) };
            let mut instance = json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": scale,
                "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                "disabled": object.locked,
            });
            if let Some(kind) = &object.object_kind {
                instance["objectKind"] = json!(kind);
            }
            if let Some(reveal_index) = object.reveal_index {
                instance["revealIndex"] = json!(reveal_index);
            }
            instance
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 🗄️ Cheap change key for everything the instance/mesh payloads (and the document tree) derive from.
pub fn fixture_geometry_fingerprint(fixture: &Puzzle3dFixture) -> u64 {
    let payload = serde_json::to_string(&(&fixture.objects, &fixture.references, &fixture.target_volumes, &fixture.meta)).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    payload.hash(&mut hasher);
    hasher.finish()
}

pub fn world_meshes_json(fixture: &Puzzle3dFixture) -> String {
    let urls = collect_mesh_urls(fixture);
    let kinds = vec![PUZZLE3D_FALLBACK_MESH_KIND.into(), "vortex-marker".into()];
    if urls.is_empty() {
        return world3d_meshes_json_from_kinds_and_urls(&kinds, &[]);
    }
    let mut meshes_json = world3d_meshes_json_from_kinds_and_urls(&kinds, &urls);
    if !meshes_json.contains(PUZZLE3D_FALLBACK_MESH_KIND) {
        let fallback = world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_FALLBACK_MESH_KIND.into()], &[]);
        let mut merged: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_default();
        let fallback_meshes: Vec<Value> = serde_json::from_str(&fallback).unwrap_or_default();
        merged.extend(fallback_meshes);
        meshes_json = serde_json::to_string(&merged).unwrap_or(meshes_json);
    }
    meshes_json
}

fn world_vortex_direction(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn vortex_color(meta: &Puzzle3dFixtureMeta, vortex_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "vortices", vortex_kind, &["color"], "#38bdf8")
}

fn object_kind_color(meta: &Puzzle3dFixtureMeta, object_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "objects", object_kind, &["color"], "#38bdf8")
}

fn object_kind_icon(meta: &Puzzle3dFixtureMeta, object_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "objects", object_kind, &["icon", "iconId"], "box")
}

/// 🎨️ First present `fields` entry on the `section` catalog row whose `id` is `kind_id`, else `fallback`.
fn catalog_entry_field(meta: &Puzzle3dFixtureMeta, section: &str, kind_id: Option<&str>, fields: &[&str], fallback: &str) -> String {
    let Some(kind_id) = kind_id else {
        return fallback.into();
    };
    let Some(catalogs) = meta.kind_catalogs.as_ref() else {
        return fallback.into();
    };
    let Some(entries) = catalogs.get(section).and_then(|value| value.as_array()) else {
        return fallback.into();
    };
    for entry in entries {
        if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
            return fields
                .iter()
                .find_map(|field| entry.get(*field).and_then(|value| value.as_str()).filter(|text| !text.is_empty()))
                .unwrap_or(fallback)
                .to_string();
        }
    }
    fallback.into()
}

/// 👁️ True when this object's vortices should render — always when `vortex_show` is Always. 🕹️
/// ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: used to also show when the
/// parent object (or any of its vortices) was hovered/selected, but `render` has no live
/// selection/hover to check (see `gumball_active`'s doc comment) — `PUZZLE3D_VORTEX_SHOW_SELECTED`
/// mode's markers are unreachable until that framework gap closes.
fn object_vortices_visible(_object: &Puzzle3dObject, runtime: &Puzzle3dRuntime) -> bool {
    runtime.vortex_show == PUZZLE3D_VORTEX_SHOW_ALWAYS
}

pub fn world_vortices_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
    let mut records = Vec::new();
    for object in &fixture.objects {
        if !object_vortices_visible(object, runtime) {
            continue;
        }
        for vortex in &object.vortices {
            let position = crate::apps::puzzle3d::world_vortex_position(object, vortex);
            let direction = world_vortex_direction(object, vortex);
            let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            records.push(json!({
                "fullId": full_id,
                "objectId": object.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": direction,
                "radius": vortex.radius.unwrap_or(0.36),
                "color": vortex_color(&fixture.meta, vortex.vortex_kind.as_deref()),
                "displayDirection": runtime.vortex_direction,
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_attractions_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .attractions
        .iter()
        .filter_map(|attraction| {
            let from = crate::apps::puzzle3d::resolve_vortex_world_position(fixture, &attraction.attracting)?;
            let to = crate::apps::puzzle3d::resolve_vortex_world_position(fixture, &attraction.attracted)?;
            Some(json!({
                "id": attraction.id,
                "from": from,
                "to": to,
                "color": "#60a5fa",
            }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_target_volumes_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            json!({
                "id": volume.id,
                "origin": volume.origin,
                "orientation": volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": target_volume_scale_json(volume),
                "color": "#f472b6",
                "hidden": volume.hidden,
                "locked": volume.locked,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_references_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .references
        .iter()
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source.url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

pub fn world_interaction_json(envelope: &Puzzle3dScene, session: &Puzzle3dPrecomputeSession) -> String {
    let runtime = &envelope.runtime;
    let suggestion_menu = runtime.suggestion_menu.as_ref().map(|menu| {
        let (pending, candidates) = (!menu.vortex_full_id.is_empty())
            .then(|| {
                let result = session.brush_candidates(&menu.vortex_full_id);
                let candidates: Vec<Value> = result
                    .free
                    .iter()
                    .enumerate()
                    .map(|(index, candidate)| {
                        let object_kind = Some(candidate.object_kind_id.as_str());
                        let object_label = candidate.object_kind_id.as_str();
                        let source_vortex_index = candidate.source_vortex_index;
                        let color = object_kind_color(&envelope.fixture.meta, object_kind);
                        let icon = object_kind_icon(&envelope.fixture.meta, object_kind);
                        json!({
                            "index": index,
                            "objectLabel": object_label,
                            "vortexLabel": format!("vortex {source_vortex_index}"),
                            "icon": icon,
                            "color": color,
                        })
                    })
                    .collect();
                (result.unknown_pending, candidates)
            })
            .unwrap_or((false, Vec::new()));
        json!({
            "open": true,
            "x": menu.x,
            "y": menu.y,
            "windowId": menu.window_id,
            "vortexFullId": menu.vortex_full_id,
            "pending": pending,
            "candidates": candidates,
        })
    });
    let fill_build = session.fill_progress_summary();
    let fill_build = json!({
        "count": fill_build.count,
        "appliedCount": fill_build.applied_count,
        "maxCount": fill_build.max_count,
        "done": fill_build.done,
    });
    // 🪣️ Committed fill count as a viewport reveal cutoff — instances tagged `revealIndex` (see
    // `world_instances_geometry_json`) below this value are shown, the rest (already planned, not yet
    // committed) stay hidden until the host commits a higher value or the live drag store overrides
    // it locally. Keyed so future reveal-driven measures/tools can share the same channel.
    json!({
        "activeUtility": scene_mode(&envelope.active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "voxelDims": runtime.voxel_dims,
        "gridFactor": runtime.grid_spacing,
        "suggestionMenu": suggestion_menu,
        "fillBuild": fill_build,
        "revealCutoffs": { "puzzle3d-fill": runtime.fill_count },
    })
    .to_string()
}

pub fn world3d_lod_json(runtime: &Puzzle3dRuntime) -> String {
    json!({
        "gridFactor": runtime.grid_spacing,
        "gridSnapEnabled": runtime.grid_snap_enabled,
        "showLodGrid": runtime.grid_visible,
        "automaticLod": runtime.lod_automatic,
        "depthVariableLod": runtime.lod_depth_variable,
        "manualLod": runtime.lod_manual,
    })
    .to_string()
}

/// 👻️ Ghost placement for the brush utility, or for a one-shot context-menu / Alt+right-click
/// suggestion popup (`suggestion_menu`) that must not switch the host-owned active utility into brush.
pub fn world_brush_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) -> Option<String> {
    if envelope.active_utility != utilities::brush::UTILITY_ID && envelope.runtime.suggestion_menu.is_none() {
        return None;
    }
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: the suggestion-menu
    // path has a real target (`menu.vortex_full_id`, stored explicitly now); the plain brush-utility
    // hover path has no live hover to read here (see `puzzle3d_brush_target_vortex`'s doc comment)
    // until `render` gains an `InteractionView`.
    let vortex_id = envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty())?;
    let preview = session.brush_preview(&vortex_id, envelope.runtime.brush_candidate_index)?;
    let color = object_kind_color(&envelope.fixture.meta, Some(preview.object_kind_id.as_str()));
    let mut value = serde_json::to_value(&preview).ok()?;
    if let Some(obj) = value.as_object_mut() {
        obj.insert("color".into(), json!(color));
    }
    serde_json::to_string(&value).ok()
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection/hover ids,
/// merge mode, active-object id, and the gumball target/active flag all used to come from
/// `runtime.selection`/`hovered_*`, now dissolved into the framework-owned `vortex` interaction
/// domain. `render` never gained an `InteractionView` parameter (see `gumball_active`'s doc comment),
/// so this payload carries no live ids at all until that framework gap closes — the world-3d host
/// renders an always-empty selection/hover overlay in the meantime.
pub fn world_selection_json(envelope: &Puzzle3dScene) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json("pick", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert(
            "targets".into(),
            json!({
                "mesh": true,
                "vertex": false,
                "edge": false,
                "face": false,
            }),
        );
        object.insert("targetVolumeIds".into(), json!([]));
        object.insert("vortexIds".into(), json!([]));
        if let Some(transform_mode) = transform_handle(&envelope.active_utility) {
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
        object.insert("gumballActive".into(), json!(gumball_active(runtime, &envelope.active_utility)));
    }
    value.to_string()
}

//#endregion 🔖️SceneJson

//#region 🔖️Render
/// 🖼️ The world-3d surface node for this window — `instances_json`/`meshes_json` come pre-computed
/// from `Puzzle3dPlayApp`'s geometry cache (they only change with the fixture's geometry fingerprint).
pub fn render(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, instances_json: String, meshes_json: String) -> UiNode {
    let brush_preview = world_brush_preview_json(precompute, envelope);
    build_world_3d_scene(
        SURFACE_VIEWPORT,
        crate::apps::puzzle3d::PUZZLE3D_PLAY_APP_ID,
        world3d_scene_extended(
            camera_json(&envelope.runtime),
            meshes_json,
            instances_json,
            world_selection_json(envelope),
            Some(world_vortices_json(&envelope.fixture, &envelope.runtime)),
            Some(world_attractions_json(&envelope.fixture)),
            Some(world_target_volumes_json(&envelope.fixture)),
            Some(world_references_json(&envelope.fixture)),
            brush_preview,
            Some(world_interaction_json(envelope, precompute)),
            None,
            Some(world3d_lod_json(&envelope.runtime)),
            Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0)),
            Some(world3d_environment_json(&envelope.runtime.sun)),
            None,
            None,
            None,
            None,
            None,
            // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): not wired here — this app
            // already emits its own `interactionSelect`/`interactionHover` for `PUZZLE3D_INTERACTION_DOMAIN`
            // ("vortex") from bespoke vortex-fit pick logic elsewhere in this crate, independent of the
            // OS `♾️infinite` surface's generic `pick_select_action`/`pick_hover_action`; binding this
            // scene's plain-pick fallback to the same domain without first confirming the two paths
            // can't double-emit is left as a follow-up, not attempted here.
            None,
            None,
        ),
    )
}

/// 🤝️ The engagement HUD for this window: the select/brush/fill switcher lives in the framework
/// utility bar (declared via `.utility` + `.window_kind_utilities`); the fill-count slider, voxel
/// steppers and brush placement picker are tagged [`WindowMeasure::Group`]s surfaced in the dedicated
/// "Utility Options" rail, so what is left here is a bare command input plus a status line.
pub fn engagement(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowEngagement {
    let object_count = envelope.fixture.objects.len();
    let attraction_count = envelope.fixture.attractions.len();
    let active_utility = envelope.active_utility.as_str();
    let objects_label = labels.objects.as_str();
    let attractions_label = labels.attractions.as_str();
    WindowEngagement {
        session_active: Some(engagement_session_active(active_utility)),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("puzzle3d-engagement".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some("brush, fill <n>, zoom, clear, rectangle, lasso".into()),
            disabled: None,
            on_change: Some(puzzle3d_action("engagementInput", None)),
            on_submit: Some(puzzle3d_action("engagementSubmit", None)),
            on_repeat_last: Some(puzzle3d_action("engagementRepeatLast", None)),
            on_abort: Some(puzzle3d_action("engagementAbort", None)),
        }),
        control: None,
        controls: None,
        status: Some(vec![semio_framework_plugin::WindowEngagementStatus { id: "puzzle3d-world-status".into(), text: format!("{object_count} {objects_label} · {attraction_count} {attractions_label}") }]),
        possible_engagements: None,
    }
}

/// 🧭️ Whether the engagement HUD should mark an active session for the given utility.
fn engagement_session_active(active_utility: &str) -> bool {
    matches!(active_utility, "brush" | "fill" | "worldRelocate")
}
//#endregion 🔖️Render
