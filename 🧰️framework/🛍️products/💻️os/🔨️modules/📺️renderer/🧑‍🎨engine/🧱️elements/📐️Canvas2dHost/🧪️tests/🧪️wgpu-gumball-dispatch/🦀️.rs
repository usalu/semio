//! 🧫️ The shared Canvas2d gumball dispatch corpus (`🧫️fixtures/🧫️gumball-dispatch`) replayed through the wgpu twin — the
//! very cases React's `🧪️tests/🧪️gumball-dispatch` replays: the handle a press grabs and every dispatch the gesture sends,
//! numbers within the corpus tolerance.

use super::*;

const CORPUS: &str = include_str!("../../🧫️fixtures/🧫️gumball-dispatch/🔣️.json");

/// 🏷️ A handle's corpus name (`moveX`, …).
fn handle_name(kind: GumballHandle) -> &'static str {
    match kind {
        GumballHandle::MoveX => "moveX",
        GumballHandle::MoveY => "moveY",
        GumballHandle::Rotate => "rotate",
        GumballHandle::ScaleX => "scaleX",
        GumballHandle::ScaleY => "scaleY",
        GumballHandle::ScaleUniform => "scaleUniform",
    }
}

fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("{key} is a number: {value}"))
}

fn view(case: &Value) -> GumballView {
    GumballView { camera_x: number(&case["camera"], "x"), camera_y: number(&case["camera"], "y"), zoom: number(&case["camera"], "zoom"), width: number(&case["viewport"], "width"), height: number(&case["viewport"], "height") }
}

/// ▶️ One case through the twin: the handle the press grabs and every dispatch as `{action, args}`.
fn replay(case: &Value) -> (Value, Vec<Value>) {
    let meta: GumballMeta = serde_json::from_value(case["layer"]["gumball"].clone()).expect("the corpus gumball decodes");
    let view = view(case);
    let (press_x, press_y) = (number(&case["press"], "x"), number(&case["press"], "y"));
    let Some(kind) = handle_at(&meta, &view, press_x, press_y) else { return (Value::Null, Vec::new()) };
    let mut gesture = begin(&meta, kind, press_x, press_y);
    let mut dispatches = Vec::new();
    for event in case["events"].as_array().expect("events") {
        match event["kind"].as_str() {
            Some("cancel") => {
                dispatches.extend(cancel(&gesture, if event["reason"] == "blur" { "blur" } else { "captureLost" }));
                break;
            }
            Some("release") => {
                dispatches.extend(release(&gesture, &view, number(event, "x"), number(event, "y")));
                break;
            }
            _ => dispatches.extend(drag(&mut gesture, &view, number(event, "x"), number(event, "y"))),
        }
    }
    (Value::from(handle_name(kind)), dispatches.into_iter().map(|dispatch| json!({ "action": dispatch.verb, "args": dispatch.args() })).collect())
}

fn assert_close(actual: &Value, expected: &Value, tolerance: f64, path: &str) {
    match expected {
        Value::Number(expected) => {
            let (actual, expected) = (actual.as_f64().unwrap_or_else(|| panic!("{path}: {actual} is no number")), expected.as_f64().expect("a finite expectation"));
            assert!((actual - expected).abs() <= tolerance, "{path}: {actual} vs {expected}");
        }
        Value::Array(expected) => {
            let actual = actual.as_array().unwrap_or_else(|| panic!("{path}: {actual} is no array"));
            assert_eq!(actual.len(), expected.len(), "{path}: {actual:?} vs {expected:?}");
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_close(actual, expected, tolerance, &format!("{path}[{index}]"));
            }
        }
        Value::Object(expected) => {
            let actual = actual.as_object().unwrap_or_else(|| panic!("{path}: {actual} is no object"));
            assert_eq!(actual.keys().collect::<std::collections::BTreeSet<_>>(), expected.keys().collect::<std::collections::BTreeSet<_>>(), "{path}: the same keys");
            for (key, expected) in expected {
                assert_close(&actual[key], expected, tolerance, &format!("{path}.{key}"));
            }
        }
        expected => assert_eq!(actual, expected, "{path}"),
    }
}

#[test]
fn the_shared_gumball_corpus_replays_exactly_as_react_does() {
    let corpus: Value = serde_json::from_str(CORPUS).expect("the corpus parses");
    let tolerance = number(&corpus, "tolerance");
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 20, "the corpus covers every handle, phase and liveness");
    for case in cases {
        let name = case["name"].as_str().expect("a named case");
        let (handle, dispatches) = replay(case);
        assert_eq!(handle, case["expect"]["handle"], "{name}: the pressed handle");
        assert_close(&Value::Array(dispatches), &case["expect"]["dispatches"], tolerance, name);
    }
}

#[test]
fn a_meta_layer_parses_only_when_active() {
    let armed = r#"[{"id":"meta:utility","role":"meta","utility":"transform"},{"id":"meta:gumball","role":"meta","gumball":{"active":true,"liveDispatch":true,"pivotLayer":[3,4],"selectionIds":["a"],"config":{"moveAxes":true,"rotate":false,"scaleAxes":true,"scaleUniform":false}}}]"#;
    let meta = parse_meta(armed).expect("an active gumball parses");
    assert!(meta.live_dispatch && meta.model_to_layer.is_none() && meta.pivot_layer == [3.0, 4.0]);
    assert!(parse_meta(&armed.replace("\"active\":true", "\"active\":false")).is_none(), "an inactive gumball is not armed");
    assert!(parse_meta("[]").is_none() && parse_meta("not json").is_none());
}
