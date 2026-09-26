//! 🧪️ The wgpu canvas-presence twin against the shared fixture `🧫️fixtures/👕️canvas-presence/🔣️.json`
//! (schema `🧬️schema/👕️canvas-presence`). Its TypeScript runner (`🧪️tests/👕️canvas-presence/🟦️.ts`) holds
//! React's `puzzle2dScreenToWorld`, `peersForWindow` and `PEER_OVERLAY_LABELS` to the same rows and checks
//! the publish transform against gl-matrix's `mat2d` inverse as the third-party oracle.

use super::*;
use replication::{PresenceDomain, PresenceInteraction};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/👕️canvas-presence/🔣️.json")).expect("canvas presence fixture")
}

/// 🔢️ A fixture value with every number as `f64`, so an `f32` screen coordinate compares to its JSON integer.
pub(crate) fn numeric(value: &Value) -> Value {
    match value {
        Value::Number(number) => serde_json::Number::from_f64(number.as_f64().expect("finite number")).map_or(Value::Null, Value::Number),
        Value::Array(items) => Value::Array(items.iter().map(numeric).collect()),
        Value::Object(fields) => Value::Object(fields.iter().map(|(key, field)| (key.clone(), numeric(field))).collect()),
        other => other.clone(),
    }
}

fn numbers(value: &Value) -> Vec<f64> {
    value.as_array().expect("number array").iter().map(|number| number.as_f64().expect("number")).collect()
}

fn rect(value: &Value) -> Rect {
    let [x, y, w, h] = numbers(value)[..] else { panic!("rect has four numbers") };
    Rect::new(x as f32, y as f32, w as f32, h as f32)
}

fn camera(value: &Value) -> (f64, f64, f64) {
    (value["x"].as_f64().expect("camera x"), value["y"].as_f64().expect("camera y"), value["zoom"].as_f64().expect("camera zoom"))
}

fn window_view(value: &Value) -> PresenceWindowView {
    let kind = &value["kind"];
    let size = numbers(&value["size"]);
    PresenceWindowView {
        window_id: value["windowId"].as_str().expect("window id").to_string(),
        space: value["space"].as_str().expect("space").to_string(),
        kind: PresenceViewKind::Canvas { x: kind["x"].as_f64().expect("x"), y: kind["y"].as_f64().expect("y"), zoom: kind["zoom"].as_f64().expect("zoom") },
        size: [size[0], size[1]],
        pointer: value.get("pointer").map(|pointer| {
            let pointer = numbers(pointer);
            [pointer[0], pointer[1], pointer[2]]
        }),
        ray_origin: None,
    }
}

/// 👥️ One fixture roster row as the wire's `PresencePeer` — shared with the shell's board-presence law.
pub(crate) fn peer(value: &Value) -> PresencePeer {
    let strings = |value: &Value| value.as_array().expect("id array").iter().map(|id| id.as_str().expect("id").to_string()).collect::<Vec<_>>();
    PresencePeer {
        actor: value["actor"].as_str().expect("actor").to_string(),
        connected_at_ms: value["connectedAtMs"].as_i64().expect("connected at"),
        label: value.get("label").and_then(Value::as_str).map(str::to_string),
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: value.get("interaction").map(|interaction| PresenceInteraction {
            app_id: interaction["app_id"].as_str().expect("app id").to_string(),
            domains: interaction["domains"]
                .as_array()
                .expect("domains")
                .iter()
                .map(|domain| PresenceDomain { domain: domain["domain"].as_str().expect("domain").to_string(), granularity: domain["granularity"].as_str().expect("granularity").to_string(), selected: strings(&domain["selected"]), hovered: strings(&domain["hovered"]) })
                .collect(),
        }),
        color: value.get("color").and_then(Value::as_u64).map(|color| color as u8),
        surface: None,
        views: value["views"].as_array().expect("views").iter().map(window_view).collect(),
        ui: None,
        tool_run: None,
        principal_kind: None,
        active_tool: value.get("activeTool").and_then(Value::as_str).map(str::to_string),
    }
}

#[test]
fn every_fixture_board_publishes_its_camera_and_the_world_point_under_the_pointer() {
    for case in fixture()["publish"].as_array().expect("publish cases") {
        let pointer = case["pointer"].as_array().map(|pointer| (pointer[0].as_f64().expect("x") as f32, pointer[1].as_f64().expect("y") as f32));
        let view = board_presence_view(case["windowId"].as_str().expect("window id"), rect(&case["bounds"]), camera(&case["camera"]), pointer);
        assert_eq!(view, window_view(&case["expected"]), "{}", case["id"]);
    }
}

#[test]
fn every_fixture_roster_paints_the_cursors_viewports_and_marks_react_paints() {
    for case in fixture()["paint"].as_array().expect("paint cases") {
        let roster: Vec<PresencePeer> = case["roster"].as_array().expect("roster").iter().map(peer).collect();
        let overlays = board_peer_overlays(
            &roster,
            case["windowId"].as_str().expect("window id"),
            rect(&case["bounds"]),
            camera(&case["camera"]),
            case["myActor"].as_str().expect("my actor"),
            case["localColor"].as_u64().expect("local color") as u8,
            case["scenePath"].as_str().expect("scene path"),
        );
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

#[test]
fn every_peer_overlay_names_itself_in_both_tongues() {
    for case in fixture()["labels"].as_array().expect("label cases") {
        let locale = if case["locale"] == "de" { Locale::De } else { Locale::En };
        let name = case["name"].as_str().expect("name");
        for (label, key) in [(PeerOverlayLabel::Cursor, "cursor"), (PeerOverlayLabel::Viewport, "viewport"), (PeerOverlayLabel::Selection, "selection"), (PeerOverlayLabel::Hover, "hover")] {
            assert_eq!(peer_overlay_label(label, name, locale), case[key].as_str().expect("label"), "{} {key}", case["locale"]);
        }
        assert_eq!(peer_tool_chip(name, case["tool"].as_str().expect("tool")), case["chip"].as_str().expect("chip"));
    }
}
