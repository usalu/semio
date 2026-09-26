//! 👕️ LAW: a native board window speaks the canvas-presence contract end to end through the shell's own
//! state — the painted Board2d surface registers exactly as a frame registers it, its scene camera and the
//! shell's last pointer become the heartbeat's window view (`board_presence_views`), and the verified
//! roster becomes the overlays the chrome paints (`board_peer_overlays`) — held to the shared fixture
//! `🧑‍🎨engine/🧫️fixtures/👕️canvas-presence/🔣️.json`, whose TypeScript runner holds React's board to it.

use super::*;
use crate::canvas_presence::tests::{numeric, peer};
use store_sync::PresenceViewKind;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/👕️canvas-presence/🔣️.json")).expect("canvas presence fixture")
}

fn rect(value: &Value) -> Rect {
    let items = value.as_array().expect("bounds quad");
    Rect::new(items[0].as_f64().expect("x") as f32, items[1].as_f64().expect("y") as f32, items[2].as_f64().expect("w") as f32, items[3].as_f64().expect("h") as f32)
}

/// 🔢️ Every case painted here takes the next window ordinal: the retained-document and engine-surface
/// registries are process-wide and keyed by window, so no two painted boards of one test process may share
/// a window (and every painting law holds `engine_canvas::engine_surface_law_guard` while it paints).
static BOARD_PRESENCE_WINDOW: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// 🪪️ A fixture case with its window renamed: the contract holds for any window id, and this process
/// paints each case under a window no other painted board uses (see [`BOARD_PRESENCE_WINDOW`]).
fn under_own_window(case: &Value) -> Value {
    let from = case["windowId"].as_str().expect("window id").to_string();
    let to = format!("{from}-law-{}", BOARD_PRESENCE_WINDOW.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
    fn renamed(value: &Value, from: &str, to: &str) -> Value {
        match value {
            Value::String(text) if text == from => Value::String(to.to_string()),
            Value::String(text) => Value::String(text.replace(&format!("/{from}/"), &format!("/{to}/"))),
            Value::Array(items) => Value::Array(items.iter().map(|item| renamed(item, from, to)).collect()),
            Value::Object(fields) => Value::Object(fields.iter().map(|(key, field)| (key.clone(), renamed(field, from, to))).collect()),
            other => other.clone(),
        }
    }
    renamed(case, &from, &to)
}

/// 🖼️ A shell whose one board window, owned by `window_id`, was painted with `camera` at `bounds` and
/// mirrored into the pointer-state maps by the same body a presented frame runs.
fn painted_board(window_id: &str, bounds: Rect, camera: &Value) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock_tabs = ShellDock::default();
    let scene = ui_wgpu::wgpu::Board2dScene::base("{}".into(), camera.to_string(), true);
    let surface = ui_wgpu::wgpu::encode_surface_doc(ui_contract::SurfaceKind::Board2d, &scene).expect("bounded Board2d scene encodes");
    let records = vec![shell_input_tests::tree_pointer_record(1, window_id, ui_contract::Component::Surface(surface), &[], None)];
    let document = shell.publish_surface_records(window_id, records).expect("board document publishes");
    shell.dock_window_plan = vec![(window_id.to_string(), bounds)];
    let _input = shell_input_tests::paint_component_pointer_documents(&mut shell, &[(window_id, "s.test.board", &document, bounds)]);
    shell.sync_engine_surface_states();
    assert_eq!(shell.board2d_states.len(), 1, "the painted board registered its surface");
    shell
}

fn window_view_json(view: &PresenceWindowView) -> Value {
    let PresenceViewKind::Canvas { x, y, zoom } = view.kind else { panic!("a board publishes a canvas view") };
    let mut json = serde_json::json!({ "windowId": view.window_id, "space": view.space, "kind": { "kind": "canvas", "x": x, "y": y, "zoom": zoom }, "size": view.size });
    if let Some(pointer) = view.pointer {
        json["pointer"] = serde_json::json!(pointer);
    }
    json
}

#[test]
fn a_painted_board_publishes_its_scene_camera_and_the_world_point_under_the_shell_pointer() {
    let _serialized = crate::engine_canvas::engine_surface_law_guard();
    for case in fixture()["publish"].as_array().expect("publish cases").iter().map(under_own_window) {
        let mut shell = painted_board(case["windowId"].as_str().expect("window id"), rect(&case["bounds"]), &case["camera"]);
        shell.presence_pointer = case["pointer"].as_array().map(|pointer| (pointer[0].as_f64().expect("x") as f32, pointer[1].as_f64().expect("y") as f32));
        let (views, _) = shell.board_presence_views();
        assert_eq!(views.iter().map(window_view_json).map(|view| numeric(&view)).collect::<Vec<_>>(), vec![numeric(&case["expected"])], "{}", case["id"]);
    }
}

#[test]
fn a_painted_board_paints_the_verified_roster_and_never_its_own_actor() {
    let _serialized = crate::engine_canvas::engine_surface_law_guard();
    for case in fixture()["paint"].as_array().expect("paint cases").iter().map(under_own_window) {
        let mut shell = painted_board(case["windowId"].as_str().expect("window id"), rect(&case["bounds"]), &case["camera"]);
        shell.presence_peers = case["roster"].as_array().expect("roster").iter().map(peer).collect();
        assert!(shell.board_peer_overlays().is_empty(), "{}: nothing is painted before the hub named this connection's actor", case["id"]);
        shell.presence_self = Some((case["myActor"].as_str().expect("my actor").to_string(), case["localColor"].as_u64().expect("local color") as u8));
        let overlays = shell.board_peer_overlays();
        assert_eq!(overlays.len(), 1, "{}: one painted board", case["id"]);
        let (_, overlays) = &overlays[0];
        let cursors: Vec<Value> = overlays
            .cursors
            .iter()
            .map(|cursor| serde_json::json!({ "actor": cursor.actor, "color": cursor.color, "at": cursor.at, "viewport": cursor.viewport, "chip": cursor.chip, "cursorPath": cursor.cursor_path, "viewportPath": cursor.viewport_path }))
            .collect();
        let marks: Vec<Value> = overlays
            .marks
            .iter()
            .map(|mark| serde_json::json!({ "actor": mark.actor, "color": mark.color, "domain": mark.domain, "id": mark.id, "mark": if mark.selected { "selection" } else { "hover" }, "chip": mark.chip, "path": mark.path }))
            .collect();
        assert_eq!(numeric(&Value::Array(cursors)), numeric(&case["expected"]["cursors"]), "{} cursors", case["id"]);
        assert_eq!(numeric(&Value::Array(marks)), numeric(&case["expected"]["marks"]), "{} marks", case["id"]);
    }
}
