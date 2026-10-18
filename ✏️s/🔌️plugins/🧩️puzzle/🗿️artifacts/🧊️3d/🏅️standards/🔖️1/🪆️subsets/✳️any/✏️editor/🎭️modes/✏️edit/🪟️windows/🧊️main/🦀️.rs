//! 🧊️ Puzzle 3d play app — the one `World3d` window kind. Owns the viewport's whole scene
//! projection: the instance/mesh/vortex/attraction/target-volume/reference payloads, the selection
//! and gumball descriptor, the LOD/chunking/environment blocks and the interaction channel (active
//! utility, suggestion popup) the host renderer reads. Also
//! owns the engagement HUD and collects its chrome measures from the mode's `☑️options/*` and its own
//! `🪛️utilities/*`.
//!
//! 🪟️ One KIND, many INSTANCES: the default layout splits it into an orthographic "Top" and a
//! three-point "Perspective" pane, and every view-local option (camera, grid, LOD, vortex display,
//! sun, selection method) is per instance — see `🦀️config.rs`'s `load_window`/`save_window`.

use semio_s_artifact_puzzle_3d_editor_session::hash_optional_dsl_value;
use semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID;
use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime;
use crate::options;
use semio_s_artifact_puzzle_3d_precompute::Puzzle3dPrecomputeSession;
use crate::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_terminology::{puzzle3d_localized, Puzzle3dLabels};
use {semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_action, semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id, semio_s_artifact_puzzle_3d_model::quat_rotate_vector, semio_s_artifact_puzzle_3d_editor_snapshot::target_volume_scale_json, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneMetadata, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dVortex, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_VORTEX_SHOW_ALWAYS};
use {semio_framework_plugin::world3d_camera_projection_json, semio_framework_plugin::world3d_chunking_json, semio_framework_plugin::world3d_environment_json, semio_framework_plugin::world3d_fit_json, semio_framework_plugin::World3dScene, semio_framework_plugin::world3d_selection_json, semio_framework_plugin::SurfaceKind, semio_framework_plugin::WindowEngagement, semio_framework_plugin::WindowEngagementInput, semio_framework_plugin::WindowEngagementSlot, semio_framework_plugin::WindowKindDefinition, semio_framework_plugin::WindowMeasure, semio_framework_plugin::WindowOptions, semio_framework_plugin::ToolRunView};
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsFound;
use semio_framework_ui_contract::BuiltNode;
use serde_json::{json, Value};
use std::hash::{Hash, Hasher};

//#region 🔖️Constants

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
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: puzzle3d_localized(|l| l.window_main),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "puzzle".into(),
        // 🪟️ `options.measures` stays empty: puzzle3d's chrome is config-derived per frame by
        // `ArtifactApp::window_measures`, never frozen into the static manifest.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement(envelope, labels, None)) },
        actions: Vec::new(),
        utilities: vec![semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID.into(), utilities::brush::UTILITY_ID.into(), utilities::volume_brush::UTILITY_ID.into(), utilities::world_relocate::UTILITY_ID.into()],
        interactions: vec![semio_framework_plugin::InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for one window instance, collected from the mode's `☑️options/*`
/// components plus this window's own `🪛️utilities/*` option groups.
pub fn window_measures(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    vec![
        options::projection::measure(&envelope.runtime),
        options::vortex::show_measure(&envelope.runtime, labels),
        options::vortex::direction_measure(&envelope.runtime, labels),
        options::lod::measure(&envelope.runtime, labels),
        options::grid::measure(&envelope.runtime, labels),
        options::select::measure(&envelope.runtime, labels),
        options::sun::measure(&envelope.runtime, labels.is_de()),
        semio_s_artifact_puzzle_3d_editor_transform::options(&envelope.runtime, labels),
        utilities::brush::options(envelope, labels),
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
    if active_utility == semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID {
        Some("transform")
    } else {
        None
    }
}

/// 🧭️ Whether the active utility is a transform gumball mode.
pub fn transform_utility_active(active_utility: &str) -> bool {
    transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render: the transform utility is active, at least one handle
/// flag is on (`setTransformGumballFlag` — an all-off gumball would draw nothing to grab), and the
/// live `vortex` selection holds at least one object or target volume to move. Selection comes from
/// the framework-owned domain via [`Puzzle3dInteractionSnapshot`], never from stored app state
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn gumball_active(runtime: &Puzzle3dRuntime, active_utility: &str, interaction: &Puzzle3dInteractionSnapshot) -> bool {
    transform_utility_active(active_utility) && (runtime.transform_move || runtime.transform_rotate) && !(interaction.selected_object_ids().is_empty() && interaction.selected_target_volume_ids().is_empty())
}
//#endregion 🔖️SceneMode

//#region 🔖️SceneJson
pub fn camera_json(runtime: &Puzzle3dRuntime) -> String {
    let camera = &runtime.camera;
    world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &camera.projection)
}

/// 📷️ How many bounding-box spans the opening pose sits back from the document's centre — the guest
/// twin of `World3dHost`'s own `autofitCameraFromInstances` seed (`span * 2.5`) narrowed by
/// `world3dFrameCameraFromInstances`'s `padding / 2.5` (1.35 / 2.5), i.e. `2.5 * 0.54`.
const PUZZLE3D_FRAMING_SPANS: f64 = 1.35;
/// 📷️ Floor for the opening orbit radius, so a single-object or empty document still opens on a pose
/// the user can orbit rather than one sitting inside the geometry — mirrors the host's own `1.4`.
const PUZZLE3D_FRAMING_MINIMUM_DISTANCE: f64 = 1.4;

/// 📷️ Whether this pane's camera is still the all-zero `Puzzle3dCamera::default()` no gesture and no
/// stored `WindowConfig` has ever replaced. Position == target is degenerate in any pose — a camera
/// standing exactly on what it looks at has no view direction at all — so it is the one reading that
/// can never be a real user pose.
pub fn camera_unset(camera: &semio_s_artifact_puzzle_3d_editor_config::Puzzle3dCamera) -> bool {
    camera.position == camera.target
}

/// 📷️ The axis-aligned centre and largest span of everything this document draws — objects,
/// references and target volumes, since a scene_snapshot may legitimately carry no objects at all and still
/// have something on screen to frame.
fn framing_bounds(scene_snapshot: &Puzzle3dSceneSnapshot) -> ([f64; 3], f64) {
    let origins = scene_snapshot
        .objects
        .iter()
        .map(|object| object.origin)
        .chain(scene_snapshot.references.iter().map(|reference| reference.origin))
        .chain(scene_snapshot.target_volumes.iter().map(|volume| volume.origin));
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    let mut seen = false;
    for origin in origins {
        seen = true;
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(origin[axis]);
            maximum[axis] = maximum[axis].max(origin[axis]);
        }
    }
    if !seen {
        return ([0.0, 0.0, 0.0], 1.0);
    }
    let centre = [(minimum[0] + maximum[0]) * 0.5, (minimum[1] + maximum[1]) * 0.5, (minimum[2] + maximum[2]) * 0.5];
    let span = (maximum[0] - minimum[0]).max(maximum[1] - minimum[1]).max(maximum[2] - minimum[2]).max(1.0);
    (centre, span)
}

/// 📷️ The projection a pane opens under, from the display template its layout entry declares
/// ([`TEMPLATE_TOP`] / [`TEMPLATE_PERSPECTIVE`], stitched in `🎭️modes/✏️edit/🦀️.rs`): the Top pane is
/// an orthographic plan, every other instance keeps the three-point default.
fn framing_projection(window_id: &str) -> semio_framework_plugin::WorldProjectionConfig {
    let mut projection = semio_framework_plugin::WorldProjectionConfig::default();
    if window_id == WINDOW_INSTANCE_TOP {
        projection.kind = "orthographic".into();
        projection.orthographic_view = "top".into();
    }
    projection
}

/// 📷️ The pose ONE pane opens on before any user gesture — framed on what the document actually
/// holds, oriented by that pane's own display template. Without it `Puzzle3dCamera::default()` leaves
/// every pane's published camera at all zeros until the first `setCamera` lands, so the world lane
/// carries no view direction at boot and two panes of one document publish the SAME (zero) pose
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B15; B12 §4.1 measured it live).
pub fn framed_camera(window_id: &str, scene_snapshot: &Puzzle3dSceneSnapshot) -> semio_s_artifact_puzzle_3d_editor_config::Puzzle3dCamera {
    let (target, span) = framing_bounds(scene_snapshot);
    let projection = framing_projection(window_id);
    let distance = (span * PUZZLE3D_FRAMING_SPANS).max(PUZZLE3D_FRAMING_MINIMUM_DISTANCE);
    let (position, up) = semio_framework_plugin::world3d_projection_pose(&projection, target, distance);
    semio_s_artifact_puzzle_3d_editor_config::Puzzle3dCamera { position, target, zoom: 1.0, up: Some(up), projection }
}

/// 📷️ Gives a scene whose pane has never been framed its opening pose, and leaves a pane the user (or
/// a stored `WindowConfig`) has already posed exactly as it is. Idempotent, so every call site on the
/// render/measure/dispatch paths may run it unconditionally.
pub fn frame_unset_camera(scene: &mut Puzzle3dScene, window_id: &str) {
    if camera_unset(&scene.runtime.camera) {
        scene.runtime.camera = framed_camera(window_id, &scene.scene_snapshot);
    }
}

/// 🎯️ Padding `WorldAutoFit` frames a swapped document with — 1.25 leaves a quarter of the radius of
/// air around the bounding sphere, matching the host's own default.
pub const PUZZLE3D_FIT_PADDING: f64 = 1.25;

/// 🎯️ Document IDENTITY for the world's `fit` lane: what this document IS (its schema, its domain and
/// its kind catalogs), never what its geometry currently holds.
///
/// `WorldAutoFit` refits once per `${revision}:${meshes}` key, so this is the difference between
/// "frame the new scene_snapshot when the user switches example" and "yank the camera every time an object
/// moves": a catalog swap is a document swap, an object edit is not. Without the lane at all the
/// camera after a swap is whatever the previous document left in `cameraJson` — Nakagin happens to
/// sit inside Concrete Forest's framing, a scene_snapshot centred elsewhere would simply be off-screen
/// (ticket 26/09/02 W-P5 §7).
pub fn world_fit_revision(scene_snapshot: &Puzzle3dSceneSnapshot) -> u32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    scene_snapshot.schema.hash(&mut hasher);
    scene_snapshot.domain.hash(&mut hasher);
    hash_optional_dsl_value(scene_snapshot.meta.kind_catalogs.as_ref(), &mut hasher);
    hash_optional_dsl_value(scene_snapshot.meta.kind_compatibility.as_ref(), &mut hasher);
    (hasher.finish() >> 32) as u32
}

fn world_vortex_direction(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn vortex_color(meta: &Puzzle3dSceneMetadata, vortex_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "vortices", vortex_kind, &["color"], "#38bdf8")
}

fn object_kind_color(meta: &Puzzle3dSceneMetadata, object_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "objects", object_kind, &["color"], "#38bdf8")
}

fn object_kind_icon(meta: &Puzzle3dSceneMetadata, object_kind: Option<&str>) -> String {
    catalog_entry_field(meta, "objects", object_kind, &["icon", "iconId"], "box")
}

/// 🏷️ The authored catalog label of an object kind, else its id — what a suggestion row names the kind by.
fn object_kind_label(meta: &Puzzle3dSceneMetadata, object_kind: &str) -> String {
    catalog_entry_field(meta, "objects", Some(object_kind), &["label", "name"], object_kind)
}

/// 🎨️ First present `fields` entry on the `section` catalog row whose `id` is `kind_id`, else `fallback`.
fn catalog_entry_field(meta: &Puzzle3dSceneMetadata, section: &str, kind_id: Option<&str>, fields: &[&str], fallback: &str) -> String {
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
            return fields.iter().find_map(|field| entry.get(field).and_then(|value| value.as_str()).filter(|text| !text.is_empty())).unwrap_or(fallback).to_string();
        }
    }
    fallback.into()
}

/// 🖌️ Placement utilities that publish every object's vortex markers so the host can hit-test them
/// without a prior object selection (those utilities block instance pick).
fn vortex_markers_publish_for_utility(active_utility: &str) -> bool {
    matches!(active_utility, "brush" | "volumeBrush")
}

/// 👁️ True when this object's vortices should render — Always mode, a live hover/selection touch, or
/// an armed placement utility. `PUZZLE3D_VORTEX_SHOW_SELECTED` otherwise hides markers until the
/// object (or one of its own vortices) is marked.
fn object_vortices_visible(object: &Puzzle3dObject, runtime: &Puzzle3dRuntime, interaction: &Puzzle3dInteractionSnapshot, active_utility: &str) -> bool {
    runtime.vortex_show == PUZZLE3D_VORTEX_SHOW_ALWAYS || interaction.touches_object(object) || vortex_markers_publish_for_utility(active_utility)
}

/// 🌀️ Per-vortex marker records. `selected`/`hovered` are painted from the live `vortex` domain —
/// `WorldVortexMarkers` reads them off each record (its own palette lookup), not off `selectionJson`.
pub fn world_vortices_json(scene_snapshot: &Puzzle3dSceneSnapshot, runtime: &Puzzle3dRuntime, interaction: &Puzzle3dInteractionSnapshot, active_utility: &str) -> String {
    let selected_vortices = interaction.selected_vortex_ids();
    let mut records = Vec::new();
    for object in &scene_snapshot.objects {
        if !object_vortices_visible(object, runtime, interaction, active_utility) {
            continue;
        }
        for vortex in &object.vortices {
            let position = semio_s_artifact_puzzle_3d_editor_snapshot::world_vortex_position(object, vortex);
            let direction = world_vortex_direction(object, vortex);
            let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            records.push(json!({
                "fullId": full_id,
                "objectId": object.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": direction,
                "radius": vortex.radius.unwrap_or(0.36),
                "color": vortex_color(&scene_snapshot.meta, vortex.vortex_kind.as_deref()),
                "displayDirection": runtime.vortex_direction,
                "selected": selected_vortices.iter().any(|id| id == &full_id),
                "hovered": interaction.hovered.iter().any(|id| id == &full_id),
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

/// 🧲️ Attraction segments, resolving both ends through ONE vortex index built per call.
///
/// ⏱️ Resolving each end with `resolve_vortex_world_position` walked every vortex of every object and allocated
/// its full id per comparison — attractions × vortices, and a fill run adds both with every placement: 8 ms of a
/// 9 ms world render at 96 placements (unoptimized native), re-paid by every refresh of the live run. The index
/// keeps the resolver's first-match order (`or_insert`), so a full id two vortices spell alike still names the
/// first one.
pub fn world_attractions_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let mut vortices = std::collections::HashMap::with_capacity(scene_snapshot.objects.iter().map(|object| object.vortices.len()).sum());
    for object in &scene_snapshot.objects {
        for vortex in &object.vortices {
            vortices.entry(puzzle3d_vortex_full_id(&object.id, &vortex.id)).or_insert((object, vortex));
        }
    }
    let position = |full_id: &str| vortices.get(full_id).map(|(object, vortex)| semio_s_artifact_puzzle_3d_editor_snapshot::world_vortex_position(object, vortex));
    let records: Vec<Value> = scene_snapshot
        .attractions
        .iter()
        .filter_map(|attraction| {
            let from = position(&attraction.attracting)?;
            let to = position(&attraction.attracted)?;
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

pub fn world_target_volumes_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let records: Vec<Value> = scene_snapshot
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

pub fn world_references_json(scene_snapshot: &Puzzle3dSceneSnapshot) -> String {
    let records: Vec<Value> = scene_snapshot
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

/// 🖌️ How many placement candidates ONE suggestion popup publishes. The scene surface it travels on is
/// a fixed-capacity payload (`semio_framework_ui_scene::encode`), and a vortex on a richly catalogued
/// document resolves arbitrarily many collision-free candidates — an unbounded list would make the whole
/// 3D render of that window fail closed the moment the popup opened. It is also the interaction answer:
/// a picker the user reads at a glance shows a bounded page, and `cycleBrushCandidate` walks the rest.
pub const PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE: usize = 8;

pub fn world_interaction_json(envelope: &Puzzle3dScene, session: &Puzzle3dPrecomputeSession, interaction: &Puzzle3dInteractionSnapshot, suggestions: Option<&BrushSuggestionsFound>) -> String {
    let runtime = &envelope.runtime;
    let suggestion_menu = runtime.suggestion_menu.as_ref().map(|menu| {
        let found = suggestions.filter(|found| !menu.vortex_full_id.is_empty() && found.target == menu.vortex_full_id);
        let candidates: Vec<Value> = found
            .into_iter()
            .flat_map(BrushSuggestionsFound::free_keyed)
            .take(PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE)
            .enumerate()
            .map(|(index, (key, candidate))| {
                let object_kind = Some(candidate.object_kind_id.as_str());
                json!({
                    "index": index,
                    "key": key,
                    "objectLabel": object_kind_label(&envelope.scene_snapshot.meta, &candidate.object_kind_id),
                    "vortexLabel": format!("vortex {}", candidate.source_vortex_index),
                    "icon": object_kind_icon(&envelope.scene_snapshot.meta, object_kind),
                    "color": object_kind_color(&envelope.scene_snapshot.meta, object_kind),
                })
            })
            .collect();
        // 🔎️ The popup lists the free candidates the brush suggestions run has resolved so far and stays
        // pending only while it has none and the search is still going; every tested candidate and its verdict
        // is the run's trace, painted in the viewport.
        let pending = candidates.is_empty() && !found.is_some_and(|found| found.done);
        json!({
            "open": true,
            "x": menu.x,
            "y": menu.y,
            "windowId": menu.window_id,
            "vortexFullId": menu.vortex_full_id,
            "submenu": menu.submenu,
            "pending": pending,
            "candidates": candidates,
        })
    });
    // 🥽️ Brush-mesh residency, the client's only handle on the fact that what a guest instantiation
    // holds does not outlive that instantiation: `meshResidency` is the guest's monotone install
    // counter (a lower value than the client last saw proves a restart, so its "already uploaded"
    // bookkeeping is void), and `meshReuploadUrls` names the identities a refused id-only announcement
    // is waiting on bytes for. Both are read by `Puzzle3dBrushMeshRegistry`
    // (`🧰️framework/…/🛠️ShellHelpers/🟦️.tsx`).
    let mut value = json!({
        "activeUtility": scene_mode(&envelope.active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "voxelDims": runtime.voxel_dims,
        "gridFactor": runtime.grid_spacing,
        "suggestionMenu": suggestion_menu,
        "meshResidency": semio_s_artifact_puzzle_3d_precompute::shared_brush_mesh_installs(),
        "meshReuploadUrls": session.mesh_reupload_requests(),
    });
    // 🐁️ `hoveredVortexFullId` is the host's Alt+right-click suggestion target and its context-menu
    // priority key (`resolveWorldContextMenuTarget`) — it lives on the interaction record, not on
    // `selectionJson`, so it is projected from the live `vortex`-domain hover here.
    if let (Some(object), Some(hovered)) = (value.as_object_mut(), interaction.hovered_vortex_full_id(&envelope.scene_snapshot)) {
        object.insert("hoveredVortexFullId".into(), json!(hovered));
    }
    value.to_string()
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

/// 🕹️ The host's `WorldSelectionRecord` (`World3dHost/🟦️.tsx` `parseSelection`) for this window: the
/// framework-owned `vortex` domain projected onto exactly the field names that parser reads —
/// `ids`/`activeObjectId` (object instances), `targetVolumeIds`, `referenceSelectedId`,
/// `hoveredId`/`hoveredKindId`, plus the gumball descriptor. Vortex selection/hover is deliberately
/// NOT here: `WorldSelectionRecord` has no vortex field, so a marker's own `selected`/`hovered` flags
/// travel on `vorticesJson` (see `world_vortices_json`), which is where `WorldVortexMarkers` reads
/// them. Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
pub fn world_selection_json(envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> String {
    let runtime = &envelope.runtime;
    let object_ids = interaction.selected_object_ids();
    let hovered_id = interaction.hovered_object_id(&envelope.scene_snapshot).map(str::to_string).or_else(|| interaction.hovered_reference_id(&envelope.scene_snapshot).map(|id| format!("reference:{id}")));
    let mut value: Value = serde_json::from_str(&world3d_selection_json(runtime.selection_method.as_str(), object_ids, hovered_id.as_deref())).unwrap_or_else(|_| json!({}));
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
        object.insert("targetVolumeIds".into(), json!(interaction.selected_target_volume_ids()));
        if let Some(id) = object_ids.first() {
            object.insert("activeObjectId".into(), json!(id));
        }
        if let Some(id) = interaction.selected_reference_ids().first() {
            object.insert("referenceSelectedId".into(), json!(id));
        }
        if let Some(kind) = hovered_kind_id(&envelope.scene_snapshot, interaction) {
            object.insert("hoveredKindId".into(), json!(kind));
        }
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
        object.insert("gumballActive".into(), json!(gumball_active(runtime, &envelope.active_utility, interaction)));
    }
    value.to_string()
}

/// 🎨️ The hovered CATALOGUE kind id, when the `kind` granularity is what the pointer is over — the
/// host highlights every instance sharing that `objectKind` from this one field.
fn hovered_kind_id<'a>(scene_snapshot: &Puzzle3dSceneSnapshot, interaction: &'a Puzzle3dInteractionSnapshot) -> Option<&'a str> {
    let catalogs = scene_snapshot.meta.kind_catalogs.as_ref()?;
    let entries = catalogs.get("objects").and_then(|value| value.as_array())?;
    interaction.hovered.iter().find(|id| entries.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).map(String::as_str)
}

//#endregion 🔖️SceneJson

//#region 🔖️Render
/// 🖼️ The world-3d surface node for this window — `instances_json`/`meshes_json` come pre-computed
/// from `Puzzle3dPlayApp`'s geometry cache (they only change with the scene_snapshot's geometry fingerprint).
pub fn render(
    envelope: &Puzzle3dScene,
    precompute: &Puzzle3dPrecomputeSession,
    instances_json: String,
    meshes_json: String,
    instances_delta_json: Option<String>,
    interaction: &Puzzle3dInteractionSnapshot,
    suggestions: Option<&BrushSuggestionsFound>,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let vortices = world_vortices_json(&envelope.scene_snapshot, &envelope.runtime, interaction, envelope.active_utility.as_str());
    let mut scene = World3dScene::base(camera_json(&envelope.runtime), meshes_json, instances_json, world_selection_json(envelope, interaction));
    scene.instances_delta_json = instances_delta_json;
    scene.vortices_json = Some(vortices);
    scene.attractions_json = Some(world_attractions_json(&envelope.scene_snapshot));
    scene.target_volumes_json = Some(world_target_volumes_json(&envelope.scene_snapshot));
    scene.references_json = Some(world_references_json(&envelope.scene_snapshot));
    scene.interaction_json = Some(world_interaction_json(envelope, precompute, interaction, suggestions));
    scene.lod_json = Some(world3d_lod_json(&envelope.runtime));
    scene.chunking_json = Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0));
    scene.environment_json = Some(world3d_environment_json(&envelope.runtime.sun));
    scene.fit_json = Some(world3d_fit_json(world_fit_revision(&envelope.scene_snapshot), PUZZLE3D_FIT_PADDING, None));
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): bound, so `World3dHost`'s generic
    // dispatch path emits `interactionSelect`/`interactionHover` for this domain
    // (`world3dSelectionActionArgs`/`world3dHoverActionArgs`) instead of the legacy
    // `worldPick`/`worldSelect`/`setHover` verbs this crate has no handler for. Granularity is
    // declared explicitly because the host's own default (`"handle"`) is not one of this domain's
    // granularities and `validate_state` would prune every id picked under it.
    scene.domain_id = Some(PUZZLE3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT.into());
    semio_framework_plugin::scene_surface(SURFACE_VIEWPORT, semio_framework_ui_contract::SurfaceKind::World3d, &scene)
}

/// 🤝️ The engagement HUD for this window: the select/brush/fill switcher lives in the framework
/// utility bar (declared via `.utility` + `.window_kind_utilities`); the fill-count slider, voxel
/// steppers and brush contact tolerance are tagged [`WindowMeasure::Group`]s surfaced in the dedicated
/// "Utility Options" rail. The remaining chrome is the command input and a status line — object
/// placement lives on the catalogue panel (drag-and-drop) and the shell menu (`openAddObjectDialog`).
/// While a fill run is non-terminal, Escape in the input aborts that run (`toolRunAbort` with its current
/// identity) instead of disarming the engagement.
pub fn engagement(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, tool_run: Option<&ToolRunView>) -> WindowEngagement {
    let object_count = envelope.scene_snapshot.objects.len();
    let attraction_count = envelope.scene_snapshot.attractions.len();
    let active_utility = envelope.active_utility.as_str();
    let objects_label = labels.objects.as_str();
    let attractions_label = labels.attractions.as_str();
    WindowEngagement {
        session_active: Some(engagement_session_active(active_utility)),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("puzzle3d-engagement".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_ENGAGEMENT_VERBS.join(", ")),
            disabled: None,
            on_change: Some(puzzle3d_action("engagementInput", None)),
            on_submit: Some(puzzle3d_action("engagementSubmit", None)),
            on_repeat_last: Some(puzzle3d_action("engagementRepeatLast", None)),
            on_abort: Some(fill_tool::abort_action(tool_run).unwrap_or_else(|| puzzle3d_action("engagementAbort", None))),
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod vortex_payload_laws;

/// 🧾️ Rebuilds the transient render bundle for one `(document, config, window)` triple, with the
/// window instance's own view-local options materialized onto the runtime.
///
/// ⏱️ Straight off the snapshot's typed authority ([`scene_from_snapshot`]). Every refresh calls this
/// once per window body, engagement and measure; through the `Value` projection bridge it cost 12 ms of a
/// 33 ms main window render on a 100-placement fill run (unoptimized native), growing with the document, so
/// a live tool run's refreshes slowed as the run placed objects.
pub fn scene_for(snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dRuntime, view_state: Option<&semio_framework_plugin::ViewModel>, window_id: &str) -> Puzzle3dScene {
    let active_utility = puzzle3d_scene_active_utility(config, view_state, Some(window_id));
    let mut scene = scene_from_snapshot(snapshot.typed(), config.clone(), &active_utility);
    frame_unset_camera(&mut scene, window_id);
    scene
}

use crate::windows::main;
use semio_framework::kernel::UiDirtyScope;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility;
use semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot;

pub fn puzzle3d_viewport_scope() -> UiDirtyScope {
    UiDirtyScope::Partial { window_bodies: vec![main::BODY_KEY.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false }
}
