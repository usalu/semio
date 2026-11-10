//! 🧲️ interaction tests (🧲️handle-drag): the Rust handle drag reproduces the language-neutral vectors the React host and an independent numpy solve share.

use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::DslValue;
use semio_os_infinite_world_interaction::{
    world3d_handle_drag_descriptor, world3d_handle_drag_parameter, world3d_handle_drag_step, world3d_handle_pick, world3d_handle_targets, WorldHandleBindings, WorldHandleDragEvent, WorldHandleDragInput, WorldHandleDragState, WorldHandleModifiers, WorldHandleTarget, WorldHandleTrigger,
};
use semio_ui_declaration::layout::ActionDescriptor;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🧲️handle-drag.json")).expect("handle drag fixture parses")
}

fn vec3(value: &Value) -> [f64; 3] {
    let list = value.as_array().expect("vec3");
    [list[0].as_f64().unwrap(), list[1].as_f64().unwrap(), list[2].as_f64().unwrap()]
}

fn modifiers(value: &Value) -> WorldHandleModifiers {
    WorldHandleModifiers { shift: value["shift"].as_bool().unwrap(), alt: value["alt"].as_bool().unwrap(), ctrl: value["ctrl"].as_bool().unwrap() }
}

fn input(value: &Value) -> WorldHandleDragInput {
    WorldHandleDragInput { handle: value["handle"].as_str().unwrap().to_string(), s: value["s"].as_f64().unwrap(), origin: value["origin"].as_f64().unwrap(), modifiers: modifiers(&value["modifiers"]), at: vec3(&value["at"]) }
}

fn event(value: &Value) -> WorldHandleDragEvent {
    match value["kind"].as_str().unwrap() {
        "press" => WorldHandleDragEvent::Press { handle: value["handle"].as_str().unwrap().to_string(), s: value["s"].as_f64().unwrap() },
        "move" => WorldHandleDragEvent::Move { s: value["s"].as_f64().unwrap(), modifiers: modifiers(&value["modifiers"]), at: vec3(&value["at"]) },
        "release" => WorldHandleDragEvent::Release { s: value["s"].as_f64().unwrap(), modifiers: modifiers(&value["modifiers"]), at: vec3(&value["at"]) },
        "cancel" => WorldHandleDragEvent::Cancel { modifiers: modifiers(&value["modifiers"]), at: vec3(&value["at"]) },
        other => panic!("fixture event kind {other}"),
    }
}

fn same(expected: &Value, actual: &DslValue) -> bool {
    match (expected, actual) {
        (Value::Number(number), DslValue::Number(actual)) => (number.as_f64().unwrap() - actual.as_f64()).abs() < 1e-12,
        (Value::String(text), DslValue::String(actual)) => text == actual,
        (Value::Bool(flag), DslValue::Bool(actual)) => flag == actual,
        (Value::Array(items), DslValue::Array(actual)) => items.len() == actual.len() && items.iter().zip(actual).all(|(item, actual)| same(item, actual)),
        (Value::Object(entries), DslValue::Object(actual)) => entries.len() == actual.len() && entries.iter().all(|(key, item)| actual.iter().any(|(name, value)| name == key && same(item, value))),
        _ => false,
    }
}

#[test]
fn parameter_vectors_match() {
    for row in fixture()["parameter"].as_array().unwrap() {
        let s = world3d_handle_drag_parameter(vec3(&row["rayOrigin"]), vec3(&row["rayDirection"]), vec3(&row["axisOrigin"]), vec3(&row["axis"]));
        match row["s"].as_f64() {
            Some(expected) => assert!((s.unwrap_or_else(|| panic!("{} has a parameter", row["name"])) - expected).abs() < 1e-9, "{}", row["name"]),
            None => assert!(s.is_none(), "{}", row["name"]),
        }
    }
}

#[test]
fn pick_vectors_match() {
    for row in fixture()["pick"].as_array().unwrap() {
        let handles: Vec<WorldHandleTarget> = row["handles"].as_array().unwrap().iter().map(|handle| WorldHandleTarget { id: handle["id"].as_str().unwrap().to_string(), screen: [handle["x"].as_f64().unwrap(), handle["y"].as_f64().unwrap()], origin: [0.0; 3], axis: [1.0, 0.0, 0.0] }).collect();
        let picked = world3d_handle_pick(&handles, row["x"].as_f64().unwrap(), row["y"].as_f64().unwrap(), row["radiusPx"].as_f64().unwrap()).map(|index| handles[index].id.as_str());
        assert_eq!(picked, row["expected"].as_str(), "{}", row["name"]);
    }
}

#[test]
fn sequences_dispatch_exactly_the_fixture() {
    for row in fixture()["sequences"].as_array().unwrap() {
        let mut state: Option<WorldHandleDragState> = None;
        let sent: Vec<Option<(WorldHandleTrigger, WorldHandleDragInput)>> = row["events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|raw| {
                let step = world3d_handle_drag_step(state.as_ref(), event(raw));
                state = step.state;
                step.dispatch
            })
            .collect();
        let expected: Vec<Option<(WorldHandleTrigger, WorldHandleDragInput)>> = row["dispatches"]
            .as_array()
            .unwrap()
            .iter()
            .map(|raw| {
                (!raw.is_null()).then(|| {
                    let trigger = match raw["trigger"].as_str().unwrap() {
                        "delta" => WorldHandleTrigger::Delta,
                        "commit" => WorldHandleTrigger::Commit,
                        "abort" => WorldHandleTrigger::Abort,
                        other => panic!("fixture trigger {other}"),
                    };
                    (trigger, input(&raw["input"]))
                })
            })
            .collect();
        assert_eq!(sent, expected, "{}", row["name"]);
        assert!(state.is_none(), "{} ends released", row["name"]);
    }
}

#[test]
fn payloads_merge_over_the_authored_arguments() {
    for row in fixture()["payload"].as_array().unwrap() {
        let authored = (!row["authored"].is_null()).then(|| DslValue::from(row["authored"].clone()));
        let bindings = WorldHandleBindings { delta: Some(ActionDescriptor { controller_id: "norm#editor".into(), action: "setFields".into(), args: authored }), ..WorldHandleBindings::default() };
        let descriptor = world3d_handle_drag_descriptor(&bindings, WorldHandleTrigger::Delta, &input(&row["input"])).expect("a bound moment dispatches");
        assert_eq!((descriptor.controller_id.as_str(), descriptor.action.as_str()), ("norm#editor", "setFields"));
        assert!(same(&row["expected"], descriptor.args.as_ref().expect("arguments")), "{}", row["name"]);
    }
}

#[test]
fn an_unbound_moment_dispatches_nothing() {
    let bindings = WorldHandleBindings::default();
    let sent = input(&fixture()["sequences"][0]["dispatches"][1]["input"]);
    assert!(!bindings.binds_any());
    assert!(world3d_handle_drag_descriptor(&bindings, WorldHandleTrigger::Commit, &sent).is_none());
}

#[test]
fn targets_are_the_visible_axis_markers() {
    for row in fixture()["targets"].as_array().unwrap() {
        let layer: semio_ui_declaration::ui::World3dAnnotationLayer = serde_json::from_value(row["layer"].clone()).expect("annotation layer");
        let matrix: Vec<f32> = row["viewProj"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
        let targets = world3d_handle_targets(&layer, &matrix.try_into().expect("sixteen entries"), row["width"].as_f64().unwrap() as f32, row["height"].as_f64().unwrap() as f32);
        let expected = row["expected"].as_array().unwrap();
        assert_eq!(targets.len(), expected.len(), "{}", row["name"]);
        for (target, expected) in targets.iter().zip(expected) {
            assert_eq!(target.id, expected["id"].as_str().unwrap());
            assert!((target.screen[0] - expected["x"].as_f64().unwrap()).abs() < 1e-6 && (target.screen[1] - expected["y"].as_f64().unwrap()).abs() < 1e-6, "{}", target.id);
            assert_eq!((target.origin, target.axis), (vec3(&expected["origin"]), vec3(&expected["axis"])));
        }
    }
}

#[test]
fn escape_aborts_without_a_commit() {
    let fixture = fixture();
    let row = fixture["sequences"].as_array().unwrap().iter().find(|row| row["name"].as_str().unwrap().starts_with("escape after travel")).expect("escape sequence");
    let mut state: Option<WorldHandleDragState> = None;
    let mut triggers = Vec::new();
    for raw in row["events"].as_array().unwrap() {
        let step = world3d_handle_drag_step(state.as_ref(), event(raw));
        state = step.state;
        triggers.extend(step.dispatch.map(|(trigger, _)| trigger));
    }
    assert_eq!(triggers, [WorldHandleTrigger::Delta, WorldHandleTrigger::Delta, WorldHandleTrigger::Abort]);
    assert!(state.is_none());
}
