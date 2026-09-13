//! 🎮️ Rust law over `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json`.
//!
//! Reads the SAME oracle the TypeScript twin (`🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts`) answers from
//! the UI-isolate side and checks the other two hops: that the wire bytes the twin produces decode into
//! this crate's own `BrowserWireEvent`, and that `stateless_dispatch` projects each one onto exactly the
//! `DispatchEvent` the fixture names. Nothing here is a restatement of the implementation — every
//! expectation is read out of the fixture file, so a change on either side of the wire that the other
//! side does not follow fails here rather than silently dropping input in a browser.

use super::{stateless_dispatch, BrowserBatch, BrowserWireEvent};
use serde_json::Value;
use ui_render::{DispatchEvent, ImeEvent, PointerButton, PointerKind};

const FIXTURE: &str = include_str!("../../🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json");

fn rows() -> Vec<Value> {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("browser input wire fixture parses");
    fixture["rows"].as_array().expect("fixture declares rows").clone()
}

fn pointer_kind_name(kind: PointerKind) -> &'static str {
    match kind {
        PointerKind::Mouse => "mouse",
        PointerKind::Touch => "touch",
        PointerKind::Pen => "pen",
        PointerKind::Eraser => "eraser",
    }
}

fn pointer_button_name(button: PointerButton) -> &'static str {
    match button {
        PointerButton::Primary => "primary",
        PointerButton::Secondary => "secondary",
        PointerButton::Middle => "middle",
    }
}

/// 🔤️ Renders one `DispatchEvent` in the fixture's own language-neutral spelling, so the comparison is
/// over the live enum rather than over a debug string the enum does not promise.
fn dispatch_json(event: &DispatchEvent) -> Value {
    let pointer_json = |pointer: &ui_render::PointerInfo, extra: Vec<(&str, Value)>, kind: &str, x: f32, y: f32| {
        let mut object = serde_json::Map::new();
        object.insert("kind".into(), Value::from(kind));
        object.insert("pointerId".into(), Value::from(pointer.id.0));
        object.insert("pointerKind".into(), Value::from(pointer_kind_name(pointer.kind)));
        object.insert("x".into(), Value::from(f64::from(x)));
        object.insert("y".into(), Value::from(f64::from(y)));
        object.insert("pressure".into(), pointer.pressure.map(|value| Value::from(f64::from(value))).unwrap_or(Value::Null));
        object.insert("tilt".into(), pointer.tilt.map(|(tilt_x, tilt_y)| Value::from(vec![f64::from(tilt_x), f64::from(tilt_y)])).unwrap_or(Value::Null));
        for (key, value) in extra {
            object.insert(key.into(), value);
        }
        Value::Object(object)
    };
    let key_json = |kind: &str, key: &str, modifiers: &ui_render::EventModifiers| {
        serde_json::json!({ "kind": kind, "key": key, "shift": modifiers.shift, "ctrl": modifiers.ctrl, "alt": modifiers.alt, "meta": modifiers.meta })
    };
    match event {
        DispatchEvent::PointerMove { pointer, x, y } => pointer_json(pointer, Vec::new(), "pointer-move", *x, *y),
        DispatchEvent::PointerDown { pointer, x, y, button } => pointer_json(pointer, vec![("button", Value::from(pointer_button_name(*button)))], "pointer-down", *x, *y),
        DispatchEvent::PointerUp { pointer, x, y, button } => pointer_json(pointer, vec![("button", Value::from(pointer_button_name(*button)))], "pointer-up", *x, *y),
        DispatchEvent::Scroll { x, y, delta_x, delta_y } => serde_json::json!({ "kind": "scroll", "x": f64::from(*x), "y": f64::from(*y), "deltaX": f64::from(*delta_x), "deltaY": f64::from(*delta_y) }),
        DispatchEvent::KeyDown { key, modifiers } => key_json("key-down", key, modifiers),
        DispatchEvent::KeyUp { key, modifiers } => key_json("key-up", key, modifiers),
        DispatchEvent::Ime(ImeEvent::Start) => serde_json::json!({ "kind": "ime-start" }),
        DispatchEvent::Ime(ImeEvent::Cancel) => serde_json::json!({ "kind": "ime-cancel" }),
        other => serde_json::json!({ "kind": format!("{other:?}") }),
    }
}

/// 📏️ Compares a projected dispatch with the fixture's, reading numbers at the wire's own precision.
///
/// The wire carries coordinates as `f32` — `160.696` physical pixels round-trips as
/// `160.6959991455078` — while the fixture states them in decimal, as a language-neutral oracle must:
/// its TypeScript twin reads the same rows as `f64`. Structure and spelling are compared exactly;
/// only the numeric leaves get the one ulp-scale tolerance a 32-bit carrier makes unavoidable.
fn values_match(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => match (left.as_f64(), right.as_f64()) {
            (Some(left), Some(right)) => (left - right).abs() <= 1e-3 * right.abs().max(1.0),
            _ => left == right,
        },
        (Value::Array(left), Value::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(left, right)| values_match(left, right)),
        (Value::Object(left), Value::Object(right)) => left.len() == right.len() && left.iter().all(|(key, value)| right.get(key).is_some_and(|other| values_match(value, other))),
        _ => left == right,
    }
}

/// 🎯️ Every fixture row's wire bytes decode, and project onto the dispatch the fixture names.
#[test]
fn every_wire_event_projects_onto_the_fixture_dispatch() {
    let rows = rows();
    assert!(rows.len() >= 12, "the oracle must cover every admitted input kind, found {}", rows.len());
    for row in rows {
        let id = row["id"].as_str().expect("row declares an id").to_string();
        let wire_text = serde_json::to_string(&row["wire"]).expect("row wire re-serializes");
        let batch: BrowserBatch = serde_json::from_str(&format!("{{\"replaceable\":[{wire_text}],\"lossless\":[]}}")).unwrap_or_else(|error| panic!("{id}: wire bytes must decode into BrowserWireEvent: {error}"));
        let event = batch.replaceable.into_iter().next().expect("one decoded wire event");
        match (stateless_dispatch(&event), &row["dispatch"]) {
            (Some(dispatch), expected) => {
                let projected = dispatch_json(&dispatch);
                assert!(values_match(&projected, expected), "{id}: dispatch projection\n  projected {projected}\n  fixture   {expected}");
            }
            (None, Value::Null) => {}
            (None, expected) => panic!("{id}: fixture expects {expected} but the wire event projects onto no dispatch"),
        }
    }
}

/// 📏️ The two wire events that are deliberately NOT dispatches stay that way, named rather than
/// inferred: a resize is the surface's metrics and a text chunk belongs to the Worker's stream table.
#[test]
fn resize_and_text_chunks_are_not_dispatch_events() {
    let resize: BrowserWireEvent = serde_json::from_str(r#"{"kind":"resize","width":1434,"height":836,"dpr":2}"#).expect("resize decodes");
    assert!(stateless_dispatch(&resize).is_none());
    let chunk: BrowserWireEvent = serde_json::from_str(r#"{"kind":"text-chunk","streamId":1,"target":"text","text":"ab","totalBytes":2,"final":true}"#).expect("text chunk decodes");
    assert!(stateless_dispatch(&chunk).is_none());
}

/// 🔤️ A camelCase field name is the contract, not an accident: the UI isolate spells the wire in
/// camelCase and a snake_case payload must be refused rather than silently defaulted.
#[test]
fn the_wire_spelling_is_the_contract() {
    assert!(serde_json::from_str::<BrowserWireEvent>(r#"{"kind":"pointer-down","pointer_id":1,"pointer_kind":"mouse","x":1,"y":2,"button":"primary"}"#).is_err());
    assert!(serde_json::from_str::<BrowserWireEvent>(r#"{"kind":"pointerdown","pointerId":1,"pointerKind":"mouse","x":1,"y":2,"button":"primary"}"#).is_err());
    let accepted: BrowserWireEvent = serde_json::from_str(r#"{"kind":"pointer-down","pointerId":1,"pointerKind":"mouse","x":1,"y":2,"button":"primary"}"#).expect("the camelCase spelling is accepted");
    assert!(matches!(stateless_dispatch(&accepted), Some(DispatchEvent::PointerDown { .. })));
}
