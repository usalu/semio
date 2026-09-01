//! ✏️ Puzzle 2d play app — the `edit` mode: the triptych layout (overview | detail | selection) plus
//! the pane-camera framing, board-scene and engagement-HUD builders its three windows share. Each
//! window binds these to its own pane; nothing here is pane-specific.

use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::engine::{BoardHost, BOARD_CAMERA_ZOOM_MAX, BOARD_CAMERA_ZOOM_MIN};
use crate::editor::puzzle2d::modes::edit::tools::fill;
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, puzzle2d_action, runtime_camera, Puzzle2dScene, PUZZLE2D_LOD_MODE_AUTOMATIC, PUZZLE2D_PLAY_SURFACE_ID};
use semio_framework_plugin::{create_default_layout, Board2dScene, LocalizedLabel, ModeDefinition, ToolRef, WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowLayout};
use semio_framework_ui_contract::{Buildable, BuiltNode, HasBase};
use serde_json::{json, Value};

pub const PUZZLE2D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: PUZZLE2D_PLAY_MODE_EDIT.into(),
        label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(),
        tools: vec![semio_framework::io::resolve_ready(ToolRef::new(fill::TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ Triptych play layout: a wide interactive overview beside the detail and selection panes.
pub fn layout() -> WindowLayout {
    create_default_layout(&[overview::WINDOW_KIND_ID.into(), detail::WINDOW_KIND_ID.into(), selection::WINDOW_KIND_ID.into()], "row", Some(&[50.0, 25.0, 25.0]), Some(&["Overview".into(), "Detail".into(), "Selection".into()]))
}
//#endregion 🔖️Definition

//#region 🔖️PaneCamera
const PUZZLE2D_VIEWPORT_REF_SHORT_PX: f64 = 640.0;
const PUZZLE2D_VIEWPORT_MARGIN: f64 = 0.18;
const PUZZLE2D_VIEWPORT_FRAMING_HALF_SPAN_SCALE: f64 = 2.25;
const PUZZLE2D_VIEWPORT_ZOOM_BOOST: f64 = 2.5;

fn puzzle2d_pane_zoom_scale(pane: &str) -> f64 {
    match pane {
        detail::WINDOW_KIND_ID => detail::ZOOM_SCALE,
        selection::WINDOW_KIND_ID => selection::ZOOM_SCALE,
        _ => overview::ZOOM_SCALE,
    }
}

fn puzzle2d_clamp_zoom(value: f64) -> f64 {
    value.clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX)
}

/// 📐️ World-space center and half-span of every node's extent (circle radius or rectangle half-size), used to frame pane cameras.
fn puzzle2d_fixture_world_bounds(fixture: &Value) -> (f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for node in fixture_nodes(fixture) {
        let (Some(x), Some(y)) = (node.get("x").and_then(|value| value.as_f64()), node.get("y").and_then(|value| value.as_f64())) else {
            continue;
        };
        let (half_w, half_h) = if node.get("shape").and_then(|value| value.as_str()) == Some("rectangle") {
            (node.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0) * 0.5, node.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0) * 0.5)
        } else {
            let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
            (radius, radius)
        };
        min_x = min_x.min(x - half_w);
        max_x = max_x.max(x + half_w);
        min_y = min_y.min(y - half_h);
        max_y = max_y.max(y + half_h);
    }
    if !min_x.is_finite() {
        return (0.0, 0.0, 400.0);
    }
    let half_span = (max_x - min_x).max(max_y - min_y).max(1.0) * 0.5;
    ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, half_span)
}

/// 📷️ Triptych camera for a pane: overview is zoomed out and centered on the fixture, detail zooms
/// into the last-placed node, selection frames a lower-left quadrant — mirrors the pre-migration
/// `puzzle2dPlayTriptychCameraForPane`.
pub fn puzzle2d_pane_camera(fixture: &Value, runtime: &Puzzle2dPlayRuntime, pane: &str) -> (f64, f64, f64) {
    let (camera_x, camera_y, camera_zoom) = runtime_camera(runtime);
    if pane == overview::WINDOW_KIND_ID {
        return (camera_x, camera_y, puzzle2d_clamp_zoom(camera_zoom));
    }
    let (cx, cy, half_span) = puzzle2d_fixture_world_bounds(fixture);
    let usable = PUZZLE2D_VIEWPORT_REF_SHORT_PX * (1.0 - 2.0 * PUZZLE2D_VIEWPORT_MARGIN);
    let world_span = (2.0 * half_span * PUZZLE2D_VIEWPORT_FRAMING_HALF_SPAN_SCALE).max(1.0);
    let base_zoom = puzzle2d_clamp_zoom((usable / world_span) * PUZZLE2D_VIEWPORT_ZOOM_BOOST);
    let zoom = puzzle2d_clamp_zoom(base_zoom * puzzle2d_pane_zoom_scale(pane));
    match pane {
        detail::WINDOW_KIND_ID => {
            let nodes = fixture_nodes(fixture);
            let detail_node = nodes.get(nodes.len().saturating_sub(1).min(42));
            let x = detail_node.and_then(|node| node.get("x")).and_then(|value| value.as_f64()).unwrap_or(cx) + camera_x * 0.02;
            let y = detail_node.and_then(|node| node.get("y")).and_then(|value| value.as_f64()).unwrap_or(cy) + camera_y * 0.02;
            (x, y, zoom)
        }
        selection::WINDOW_KIND_ID => (cx - half_span * 0.28 + camera_x * 0.06, cy + half_span * 0.22 + camera_y * 0.05, zoom),
        _ => (cx + camera_x * 0.04, cy + camera_y * 0.03, zoom),
    }
}
//#endregion 🔖️PaneCamera

//#region 🔖️Canvas
/// 🗄️ Caches the last serialized fixture keyed by an fnv1a hash of the raw `document_json` it came
/// from, so the overview/detail/selection panes of the same `refreshUi` tick reuse one `String`
/// instead of each re-serializing the whole fixture graph.
fn cached_fixture_json(_document_json: &str, fixture: &Value) -> String {
    fixture.to_string()
}

fn puzzle2d_board_scene(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> Board2dScene {
    let fixture = &envelope.fixture;
    let (camera_x, camera_y, zoom) = puzzle2d_pane_camera(fixture, &envelope.runtime, pane);
    let camera_json = json!({ "x": camera_x, "y": camera_y, "zoom": zoom }).to_string();
    let glyph_catalogs_json = crate::editor::puzzle2d::board_kind_catalogs_json(fixture).unwrap_or_else(|| "{}".into());
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: selection/method used
    // to come from `runtime.selected_ids`/`selection_method`, now dissolved into the framework-owned
    // `vortex` interaction domain; `render` has no `InteractionView` to read it from (see
    // `puzzle3d`'s `world_selection_json` doc comment for the identical framework-level gap) — this
    // payload carries no live ids until that gap closes.
    let selection_json = "[]".to_string();
    let brush_weights_json = serde_json::to_string(&json!({
        "nodeWeights": envelope.runtime.node_kind_weights,
        "handleWeights": envelope.runtime.handle_kind_weights,
    }))
    .unwrap_or_else(|_| "{}".into());
    let placement_compatibility_json = fixture.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| fixture.get("kindCompatibility")).map_or_else(|| "[]".into(), |value| value.to_string());
    let lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).cloned().unwrap_or_else(|| PUZZLE2D_LOD_MODE_AUTOMATIC.to_string());
    Board2dScene {
        fixture_json: cached_fixture_json(document_json, fixture),
        camera_json,
        glyph_catalogs_json,
        selection_json,
        interactive: pane == overview::WINDOW_KIND_ID,
        hovered_id: None,
        active_utility: Some(envelope.active_utility.clone()),
        selection_method: "rectangle".into(),
        grid_snap_enabled: envelope.runtime.grid_snap_enabled,
        grid_factor: envelope.runtime.grid_factor,
        suggestion_offset: envelope.runtime.suggestion_offset,
        brush_weights_json,
        placement_compatibility_json,
        lod_mode,
    }
}

/// 🖼️ The board-2d surface node for one pane — bound by each window's own `render()`.
pub fn render_canvas(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let scene = puzzle2d_board_scene(document_json, envelope, pane);
    let props = semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::Board2d, &scene).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.scene.encode", "Board2d scene admission failed"))?;
    semio_framework_ui_contract::surface(props)
        .try_id(format!("{PUZZLE2D_PLAY_SURFACE_ID}.{pane}"))
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.scene.id", "Board2d surface id admission failed"))?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.scene.build", "Board2d surface admission failed"))
}
//#endregion 🔖️Canvas

//#region 🔖️Engagement
/// 🤝️ The engagement HUD for one pane: a text command line plus a node/edge/LOD status readout.
pub fn puzzle2d_engagement(envelope: &Puzzle2dScene, host: &BoardHost, pane: &str, labels: &Puzzle2dLabels) -> WindowEngagement {
    let overlay: Value = serde_json::from_str(&host.overlay_paint_state_json()).unwrap_or(Value::Null);
    let pane_lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).map_or(PUZZLE2D_LOD_MODE_AUTOMATIC, String::as_str);
    let lod = overlay.get("lod").and_then(|value| value.as_str()).unwrap_or(if pane_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC { "auto" } else { pane_lod_mode });
    let node_count = fixture_nodes(&envelope.fixture).len();
    let edge_count = fixture_edges(&envelope.fixture).len();
    let input_value = envelope.runtime.engagement_input_by_pane.get(pane).cloned().unwrap_or_default();
    let placeholder = match envelope.active_utility.as_str() {
        "brush" => "Brush",
        _ => "select, brush, clear",
    };
    WindowEngagement {
        session_active: Some(envelope.active_utility != "select"),
        input: Some(WindowEngagementInput {
            id: Some("puzzle2d-engagement".into()),
            value: Some(input_value),
            placeholder: Some(placeholder.into()),
            disabled: None,
            on_change: Some(puzzle2d_action("engagementInput", Some(json!({ "pane": pane })))),
            on_submit: Some(puzzle2d_action("engagementSubmit", Some(json!({ "pane": pane })))),
            on_repeat_last: None,
            on_abort: Some(puzzle2d_action("engagementAbort", Some(json!({ "pane": pane })))),
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "puzzle2d-board-status".into(), text: format!("{node_count} {} · {edge_count} {} · {} {lod}", labels.nodes.as_str(), labels.edges.as_str(), labels.lod.as_str()) }]),
        // 🧰️ The select/brush/fill switcher lives in the framework utility bar (declared via `.utility` +
        // `.window_kind_utilities`), so the engagement no longer duplicates it as toggle options.
        options: None,
        possible_engagements: None,
    }
}
//#endregion 🔖️Engagement
