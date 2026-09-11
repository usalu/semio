//! 🧊️ Puzzle 3d play app — the one `World3d` window kind. Owns the viewport's whole scene
//! projection: the instance/mesh/vortex/attraction/target-volume/reference payloads, the selection
//! and gumball descriptor, the LOD/chunking/environment blocks and the interaction channel (active
//! utility, suggestion popup, fill-build progress, reveal cutoffs) the host renderer reads. Also
//! owns the engagement HUD and collects its chrome measures from the mode's `☑️options/*` and its own
//! `🪛️utilities/*`.
//!
//! 🪟️ One KIND, many INSTANCES: the default layout splits it into an orthographic "Top" and a
//! three-point "Perspective" pane, and every view-local option (camera, grid, LOD, vortex display,
//! sun, selection method) is per instance — see `🦀️config.rs`'s `load_window`/`save_window`.

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::modes::edit::options;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities;
use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::terminology::{puzzle3d_localized, Puzzle3dLabels};
use crate::editor::puzzle3d::{
    collect_mesh_urls, object_scale_json, puzzle3d_action, puzzle3d_vortex_full_id, quat_rotate_vector, target_volume_scale_json, Puzzle3dFixture, Puzzle3dFixtureMeta, Puzzle3dInteractionSnapshot, Puzzle3dKindMeshIndex, Puzzle3dObject,
    Puzzle3dScene, Puzzle3dVortex, PUZZLE3D_FALLBACK_MESH_KIND, PUZZLE3D_INTERACTION_DOMAIN, PUZZLE3D_VORTEX_SHOW_ALWAYS,
};
use semio_framework_plugin::{
    world3d_camera_projection_json, world3d_chunking_json, world3d_environment_json, world3d_fit_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, World3dScene, world3d_selection_json, SurfaceKind, WindowEngagement,
    WindowEngagementInput, WindowEngagementOption, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions,
};
use semio_framework_ui_contract::BuiltNode;
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
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
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
        interactions: vec![semio_framework_plugin::InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for one window instance, collected from the mode's `☑️options/*`
/// components plus this window's own `🪛️utilities/*` option groups. `interaction` is the live
/// `vortex`-domain read the Brush placement picker gates itself on.
pub fn window_measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels, interaction: &Puzzle3dInteractionSnapshot) -> Vec<WindowMeasure> {
    vec![
        options::projection::measure(&envelope.runtime),
        options::vortex::show_measure(&envelope.runtime, labels),
        options::vortex::direction_measure(&envelope.runtime, labels),
        options::lod::measure(&envelope.runtime, labels),
        options::grid::measure(&envelope.runtime, labels),
        options::select::measure(&envelope.runtime, labels),
        options::sun::measure(&envelope.runtime),
        utilities::transform::options(&envelope.runtime, labels),
        utilities::brush::options(envelope, precompute, labels, interaction),
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
pub fn camera_unset(camera: &crate::editor::puzzle3d::config::Puzzle3dCamera) -> bool {
    camera.position == camera.target
}

/// 📷️ The axis-aligned centre and largest span of everything this document draws — objects,
/// references and target volumes, since a fixture may legitimately carry no objects at all and still
/// have something on screen to frame.
fn framing_bounds(fixture: &Puzzle3dFixture) -> ([f64; 3], f64) {
    let origins = fixture
        .objects
        .iter()
        .map(|object| object.origin)
        .chain(fixture.references.iter().map(|reference| reference.origin))
        .chain(fixture.target_volumes.iter().map(|volume| volume.origin));
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
pub fn framed_camera(window_id: &str, fixture: &Puzzle3dFixture) -> crate::editor::puzzle3d::config::Puzzle3dCamera {
    let (target, span) = framing_bounds(fixture);
    let projection = framing_projection(window_id);
    let distance = (span * PUZZLE3D_FRAMING_SPANS).max(PUZZLE3D_FRAMING_MINIMUM_DISTANCE);
    let (position, up) = semio_framework_plugin::world3d_projection_pose(&projection, target, distance);
    crate::editor::puzzle3d::config::Puzzle3dCamera { position, target, zoom: 1.0, up: Some(up), projection }
}

/// 📷️ Gives a scene whose pane has never been framed its opening pose, and leaves a pane the user (or
/// a stored `WindowConfig`) has already posed exactly as it is. Idempotent, so every call site on the
/// render/measure/dispatch paths may run it unconditionally.
pub fn frame_unset_camera(scene: &mut Puzzle3dScene, window_id: &str) {
    if camera_unset(&scene.runtime.camera) {
        scene.runtime.camera = framed_camera(window_id, &scene.fixture);
    }
}

/// 🙈️ Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
/// `revealIndex` is omitted entirely for untagged objects rather than emitted as `null`: the host's reveal cutoff (`framework/renderer/react`'s `applyRevealCutoff`) only skips instances with no reveal index, and a JSON `null` would coerce to `0` and hide every ordinary object behind the boot cutoff.
/// Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks.
pub fn world_instances_geometry_json(fixture: &Puzzle3dFixture) -> String {
    let kind_meshes = Puzzle3dKindMeshIndex::of(&fixture.meta);
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .map(|object| {
            let mesh_id = kind_meshes.resolve(object).map_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into(), world3d_mesh_id_from_url);
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

/// 🎯️ Padding `WorldAutoFit` frames a swapped document with — 1.25 leaves a quarter of the radius of
/// air around the bounding sphere, matching the host's own default.
pub const PUZZLE3D_FIT_PADDING: f64 = 1.25;

/// 🎯️ Document IDENTITY for the world's `fit` lane: what this document IS (its schema, its domain and
/// its kind catalogs), never what its geometry currently holds.
///
/// `WorldAutoFit` refits once per `${revision}:${meshes}` key, so this is the difference between
/// "frame the new fixture when the user switches example" and "yank the camera every time an object
/// moves": a catalog swap is a document swap, an object edit is not. Without the lane at all the
/// camera after a swap is whatever the previous document left in `cameraJson` — Nakagin happens to
/// sit inside Concrete Forest's framing, a fixture centred elsewhere would simply be off-screen
/// (ticket 26/09/02 W-P5 §7).
pub fn world_fit_revision(fixture: &Puzzle3dFixture) -> u32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    fixture.schema.hash(&mut hasher);
    fixture.domain.hash(&mut hasher);
    dsl::os_pack::json::to_json_string(&fixture.meta).hash(&mut hasher);
    (hasher.finish() >> 32) as u32
}

/// 🗄️ Cheap change key for everything the instance/mesh payloads (and the document tree) derive from.
pub fn fixture_geometry_fingerprint(fixture: &Puzzle3dFixture) -> u64 {
    let payload = format!(
        "{}{}{}{}",
        dsl::os_pack::json::to_json_string(&fixture.objects),
        dsl::os_pack::json::to_json_string(&fixture.references),
        dsl::os_pack::json::to_json_string(&fixture.target_volumes),
        dsl::os_pack::json::to_json_string(&fixture.meta),
    );
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
pub fn world_vortices_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime, interaction: &Puzzle3dInteractionSnapshot, active_utility: &str) -> String {
    let selected_vortices = interaction.selected_vortex_ids();
    let mut records = Vec::new();
    for object in &fixture.objects {
        if !object_vortices_visible(object, runtime, interaction, active_utility) {
            continue;
        }
        for vortex in &object.vortices {
            let position = crate::editor::puzzle3d::world_vortex_position(object, vortex);
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
                "selected": selected_vortices.iter().any(|id| id == &full_id),
                "hovered": interaction.hovered.iter().any(|id| id == &full_id),
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
            let from = crate::editor::puzzle3d::resolve_vortex_world_position(fixture, &attraction.attracting)?;
            let to = crate::editor::puzzle3d::resolve_vortex_world_position(fixture, &attraction.attracted)?;
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

/// 🖌️ How many placement candidates ONE suggestion popup publishes. The scene surface it travels on is
/// a fixed-capacity payload (`semio_framework_ui_scene::encode`), and a vortex on a richly catalogued
/// document resolves arbitrarily many collision-free candidates — an unbounded list would make the whole
/// 3D render of that window fail closed the moment the popup opened. It is also the interaction answer:
/// a picker the user reads at a glance shows a bounded page, and `cycleBrushCandidate` walks the rest.
pub const PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE: usize = 8;

pub fn world_interaction_json(envelope: &Puzzle3dScene, session: &Puzzle3dPrecomputeSession, interaction: &Puzzle3dInteractionSnapshot, brush_preview: Option<&str>) -> String {
    let runtime = &envelope.runtime;
    let suggestion_menu = runtime.suggestion_menu.as_ref().map(|menu| {
        let (pending, candidates) = if !menu.vortex_full_id.is_empty() {
                let result = session.brush_candidates(&menu.vortex_full_id);
                let candidates: Vec<Value> = result
                    .free
                    .iter()
                    .take(PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE)
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
            } else { (false, Vec::new()) };
        eprintln!("[DEBUG] puzzle3d.openVortex.cache menu vortex={} pending={pending} candidates={}", menu.vortex_full_id, candidates.len());
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
        "fillBuild": fill_build,
        "revealCutoffs": { "puzzle3d-fill": runtime.fill_count },
        "meshResidency": crate::editor::puzzle3d::precompute::shared_brush_mesh_installs(),
        "meshReuploadUrls": session.mesh_reupload_requests(),
    });
    // 🐁️ `hoveredVortexFullId` is the host's Alt+right-click suggestion target and its context-menu
    // priority key (`resolveWorldContextMenuTarget`) — it lives on the interaction record, not on
    // `selectionJson`, so it is projected from the live `vortex`-domain hover here.
    if let (Some(object), Some(hovered)) = (value.as_object_mut(), interaction.hovered_vortex_full_id(&envelope.fixture)) {
        object.insert("hoveredVortexFullId".into(), json!(hovered));
    }
    if let (Some(object), Some(preview)) = (value.as_object_mut(), brush_preview.filter(|payload| !payload.is_empty())) {
        object.insert("brushPreviewJson".into(), json!(preview));
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

/// 👻️ Ghost placement for the brush utility, or for a one-shot context-menu / Alt+right-click
/// suggestion popup (`suggestion_menu`) that must not switch the host-owned active utility into brush.
pub fn world_brush_preview_target(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    envelope
        .runtime
        .suggestion_menu
        .as_ref()
        .map(|menu| menu.vortex_full_id.clone())
        .filter(|id| !id.is_empty())
        .or_else(|| crate::editor::puzzle3d::puzzle3d_brush_target_vortex(envelope, interaction))
        .or_else(|| session.brush_live_target().map(str::to_string))
}

pub fn world_brush_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, interaction: &Puzzle3dInteractionSnapshot) -> Option<String> {
    let brush = envelope.active_utility == utilities::brush::UTILITY_ID;
    let menu = envelope.runtime.suggestion_menu.is_some();
    if !brush && !menu {
        return None;
    }
    let vortex_id = world_brush_preview_target(session, envelope, interaction);
    eprintln!("[DEBUG] puzzle3d.brushPreview.hover utility={} brush={brush} menu={menu} vortex={:?}", envelope.active_utility, vortex_id);
    let Some(vortex_id) = vortex_id else {
        eprintln!("[DEBUG] puzzle3d.brushPreview.gate reason=no-target utility={} brush={brush} menu={menu}", envelope.active_utility);
        return None;
    };
    let cache = session.brush_candidates(&vortex_id);
    eprintln!("[DEBUG] puzzle3d.brushPreview.cache vortex={vortex_id} free={} pending={} resume={}", cache.free.len(), cache.unknown_pending, cache.resume_candidate_index);
    let Some(preview) = session.brush_preview(&vortex_id, envelope.runtime.brush_candidate_index) else {
        eprintln!(
            "[DEBUG] puzzle3d.brushPreview.gate reason=no-free-candidate vortex={vortex_id} free={} pending={} index={}",
            cache.free.len(),
            cache.unknown_pending,
            envelope.runtime.brush_candidate_index
        );
        return None;
    };
    let color = object_kind_color(&envelope.fixture.meta, Some(preview.object_kind_id.as_str()));
    let mut value = dsl::ToValue::to_value(&preview);
    if let dsl::DslValue::Object(entries) = &mut value {
        entries.push(("color".to_string(), dsl::DslValue::String(color)));
    }
    let json = dsl::json::to_json_string(&value);
    eprintln!("[DEBUG] puzzle3d.brushPreview.compute vortex={vortex_id} bytes={}", json.len());
    Some(json)
}

/// 🪣️ Latest-wins bounded fill diagnostic, with an optional ghost projection.
pub fn world_fill_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> Option<String> {
    if envelope.active_utility != "fill" {
        return None;
    }
    let object_kind = session.fill_preview_object_kind();
    let color = object_kind_color(&envelope.fixture.meta, object_kind.as_deref());
    session.fill_preview_json_page(&color, labels.fill_progress.as_str())
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
    let hovered_id = interaction.hovered_object_id(&envelope.fixture).map(str::to_string).or_else(|| interaction.hovered_reference_id(&envelope.fixture).map(|id| format!("reference:{id}")));
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
        if let Some(kind) = hovered_kind_id(&envelope.fixture, interaction) {
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
fn hovered_kind_id<'a>(fixture: &Puzzle3dFixture, interaction: &'a Puzzle3dInteractionSnapshot) -> Option<&'a str> {
    let catalogs = fixture.meta.kind_catalogs.as_ref()?;
    let entries = catalogs.get("objects").and_then(|value| value.as_array())?;
    interaction.hovered.iter().find(|id| entries.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).map(String::as_str)
}

//#endregion 🔖️SceneJson

//#region 🔖️Render
/// 🖼️ The world-3d surface node for this window — `instances_json`/`meshes_json` come pre-computed
/// from `Puzzle3dPlayApp`'s geometry cache (they only change with the fixture's geometry fingerprint).
pub fn render(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels, instances_json: String, meshes_json: String, interaction: &Puzzle3dInteractionSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let brush_preview = world_fill_preview_json(precompute, envelope, labels).or_else(|| world_brush_preview_json(precompute, envelope, interaction));
    let vortices = world_vortices_json(&envelope.fixture, &envelope.runtime, interaction, envelope.active_utility.as_str());
    eprintln!(
        "[DEBUG] puzzle3d.brushPreview.lane utility={} preview={} vortices={}",
        envelope.active_utility,
        brush_preview.as_ref().map(String::len).unwrap_or(0),
        vortices.len()
    );
    eprintln!("[DEBUG] puzzle3d.vortices.publish utility={} bytes={} brush_or_volume={}", envelope.active_utility, vortices.len(), matches!(envelope.active_utility.as_str(), "brush" | "volumeBrush"));
    let mut scene = World3dScene::base(camera_json(&envelope.runtime), meshes_json, instances_json, world_selection_json(envelope, interaction));
    scene.vortices_json = Some(vortices);
    scene.attractions_json = Some(world_attractions_json(&envelope.fixture));
    scene.target_volumes_json = Some(world_target_volumes_json(&envelope.fixture));
    scene.references_json = Some(world_references_json(&envelope.fixture));
    scene.interaction_json = Some(world_interaction_json(envelope, precompute, interaction, brush_preview.as_deref()));
    scene.brush_preview_json = Some(brush_preview.unwrap_or_default());
    scene.lod_json = Some(world3d_lod_json(&envelope.runtime));
    scene.chunking_json = Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0));
    scene.environment_json = Some(world3d_environment_json(&envelope.runtime.sun));
    scene.fit_json = Some(world3d_fit_json(world_fit_revision(&envelope.fixture), PUZZLE3D_FIT_PADDING));
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): bound, so `World3dHost`'s generic
    // dispatch path emits `interactionSelect`/`interactionHover` for this domain
    // (`world3dSelectionActionArgs`/`world3dHoverActionArgs`) instead of the legacy
    // `worldPick`/`worldSelect`/`setHover` verbs this crate has no handler for. Granularity is
    // declared explicitly because the host's own default (`"handle"`) is not one of this domain's
    // granularities and `validate_state` would prune every id picked under it.
    scene.domain_id = Some(PUZZLE3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT.into());
    semio_framework_plugin::scene_surface(SURFACE_VIEWPORT, semio_framework_ui_contract::SurfaceKind::World3d, &scene)
}

/// 🤝️ The engagement HUD for this window: the select/brush/fill switcher lives in the framework
/// utility bar (declared via `.utility` + `.window_kind_utilities`); the fill-count slider, voxel
/// steppers and brush placement picker are tagged [`WindowMeasure::Group`]s surfaced in the dedicated
/// "Utility Options" rail. The remaining chrome is the Add Object dialog opener, a command input, and a status line.
pub fn engagement(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowEngagement {
    let object_count = envelope.fixture.objects.len();
    let attraction_count = envelope.fixture.attractions.len();
    let active_utility = envelope.active_utility.as_str();
    let objects_label = labels.objects.as_str();
    let attractions_label = labels.attractions.as_str();
    let object_word = labels.object.as_str();
    let add_object_label = if object_word.eq_ignore_ascii_case("objekt") { format!("{object_word} hinzufügen…") } else { format!("Add {object_word}…") };
    WindowEngagement {
        session_active: Some(engagement_session_active(active_utility)),
        options: Some(vec![WindowEngagementOption {
            id: "shell-menu.action.openAddObjectDialog".into(),
            label: Some(add_object_label),
            icon_id: Some("plus".into()),
            pressed: None,
            disabled: None,
            action: Some(puzzle3d_action("openAddObjectDialog", None)),
        }]),
        input: Some(WindowEngagementInput {
            id: Some("puzzle3d-engagement".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some(crate::editor::puzzle3d::commands::engagement_submit::PUZZLE3D_ENGAGEMENT_VERBS.join(", ")),
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

#[cfg(test)]
mod vortex_payload_laws {
    use super::*;
    use crate::editor::puzzle3d::{empty_fixture, PUZZLE3D_VORTEX_SHOW_SELECTED};

    fn forest_table_object() -> Puzzle3dObject {
        let positions = [
            [4.05001, 4.676537, 3.0],
            [6.75001, 4.676537, 3.0],
            [9.45001, 4.676537, 3.0],
            [6.75001, 0.0, 3.0],
            [4.05001, 0.0, 3.0],
            [1.35001, 0.0, 3.0],
            [9.45001, 2.338269, 3.0],
            [2.70001, 2.338269, 0.0],
            [2.70001, 2.338269, 3.0],
            [8.10001, 2.338269, 0.0],
            [8.10001, 2.338269, 3.0],
        ];
        Puzzle3dObject {
            id: "seed-left-001".into(),
            label: None,
            object_kind: None,
            origin: [0.0; 3],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: positions
                .iter()
                .copied()
                .enumerate()
                .map(|(index, position)| Puzzle3dVortex { id: format!("seed-left-001:v{index}"), position, radius: Some(0.36), ..Default::default() })
                .collect(),
            hidden: false,
            locked: false,
            reveal_index: None,
        }
    }

    fn forest_store() -> Puzzle3dFixture {
        let mut fixture = empty_fixture();
        fixture.objects.push(forest_table_object());
        fixture
    }

    fn parse_records(json: &str) -> Vec<Value> {
        serde_json::from_str(json).expect("vorticesJson")
    }

    #[test]
    fn world_vortices_json_carries_store_vortices_when_show_is_always() {
        let fixture = forest_store();
        assert_eq!(fixture.objects[0].vortices.len(), 11, "Concrete Forest seed-left-001 ships 11 vortex records");
        let mut runtime = Puzzle3dRuntime::default();
        runtime.vortex_show = PUZZLE3D_VORTEX_SHOW_ALWAYS.into();
        let records = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), ""));
        assert_eq!(records.len(), 11);
    }

    #[test]
    fn world_vortices_json_carries_store_vortices_when_brush_is_armed() {
        let fixture = forest_store();
        let runtime = Puzzle3dRuntime::default();
        assert_eq!(runtime.vortex_show, PUZZLE3D_VORTEX_SHOW_SELECTED);
        let brush = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "brush"));
        assert_eq!(brush.len(), 11);
        let volume = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "volumeBrush"));
        assert_eq!(volume.len(), 11);
    }

    #[test]
    fn world_vortices_json_stays_empty_in_selected_mode_without_a_touch_or_brush() {
        let fixture = forest_store();
        let runtime = Puzzle3dRuntime::default();
        let idle = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), ""));
        assert_eq!(idle.len(), 0);
        let transform = parse_records(&world_vortices_json(&fixture, &runtime, &Puzzle3dInteractionSnapshot::default(), "transform"));
        assert_eq!(transform.len(), 0);
    }

    #[test]
    fn brush_preview_target_falls_back_to_session_live_target() {
        let mut session = Puzzle3dPrecomputeSession::new();
        session.set_brush_live_target(Some("seed-left-001:v0".into()));
        let envelope = Puzzle3dScene { fixture: forest_store(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
        assert_eq!(world_brush_preview_target(&session, &envelope, &Puzzle3dInteractionSnapshot::default()).as_deref(), Some("seed-left-001:v0"));
    }
}
