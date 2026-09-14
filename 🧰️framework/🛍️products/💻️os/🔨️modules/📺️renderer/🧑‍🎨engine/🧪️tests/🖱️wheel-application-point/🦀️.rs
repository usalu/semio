//! 🖱️ Rust law over `🧫️fixtures/🖱️wheel-application-point/🔣️.json`.
//!
//! Drives the production accumulator (`AppWheel`) with each case's event stream in order and checks
//! the point AND the delta the frame would apply — the hop `🎮️wgpu-browser-input-wire/🔣️.json` stops
//! just short of. The TypeScript twin (`🧪️tests/🖱️wheel-application-point/🟦️.ts`) answers the same
//! oracle from an independent implementation of the rule, so neither side can drift alone.
//!
//! ⚖️ Every case that carries a `baselineApplied` also re-derives the PRE-FIX shape from the same
//! fixture — the delta applied at the LAST POINTER position rather than at the wheel's own — and
//! asserts it differs. That is the defect verbatim: `dispatch_normalized_event`'s
//! `Scroll { delta_y, .. }` arm dropped the position the wire had carried all the way from the DOM
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.1).

use super::AppWheel;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🖱️wheel-application-point/🔣️.json");

fn cases() -> Vec<Value> {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("wheel application point fixture parses");
    fixture["cases"].as_array().expect("fixture declares cases").clone()
}

fn number(value: &Value) -> f32 {
    value.as_f64().expect("fixture number") as f32
}

/// 🔤️ One application in the fixture's own language-neutral spelling — three numbers or nothing —
/// so the comparison is over the values and never over JSON's integer/float rendering.
fn applied_triple(applied: Option<(f32, f32, f32)>) -> Option<[f32; 3]> {
    applied.map(|(delta, x, y)| [x, y, delta])
}

fn expected_triple(value: &Value) -> Option<[f32; 3]> {
    (!value.is_null()).then(|| [number(&value["x"]), number(&value["y"]), number(&value["delta"])])
}

/// 🖱️ One frame's worth of input, in order, answered as the frame's own `WheelStart` reads it: the
/// pending wheel with its point, or nothing.
fn drive(events: &[Value]) -> (AppWheel, (f32, f32)) {
    let mut wheel = AppWheel::default();
    let mut pointer = (0.0f32, 0.0f32);
    for event in events {
        let (x, y) = (number(&event["x"]), number(&event["y"]));
        match event["kind"].as_str().expect("event kind") {
            "scroll" => {
                wheel.accumulate(x, y, number(&event["deltaY"]));
                pointer = (x, y);
            }
            "pointer-move" => pointer = (x, y),
            other => panic!("fixture event kind {other}"),
        }
    }
    (wheel, pointer)
}

#[test]
fn a_frame_applies_the_wheel_at_the_point_its_own_events_carried() {
    let mut discriminating = 0;
    for case in cases() {
        let name = case["name"].as_str().expect("case name");
        let events = case["events"].as_array().expect("events").clone();
        let (mut wheel, pointer) = drive(&events);
        let applied = wheel.take();
        assert_eq!(applied_triple(applied), expected_triple(&case["applied"]), "{name}");

        // 🔍️ The pre-fix shape, re-derived from the same fixture: the same delta at the LAST POINTER
        // position. Every case that names one must differ, which is what holds this oracle to
        // discriminating rather than merely agreeing.
        if let Some(baseline) = case.get("baselineApplied").filter(|value| !value.is_null()) {
            let pre_fix = applied.map(|(delta, _, _)| (delta, pointer.0, pointer.1));
            assert_eq!(applied_triple(pre_fix), expected_triple(baseline), "{name}: the fixture's own pre-fix shape");
            assert_ne!(applied_triple(pre_fix), expected_triple(&case["applied"]), "{name}: the pre-fix shape applies the wheel somewhere else");
            discriminating += 1;
        }

        if let Some(again) = case.get("appliedAgain") {
            assert_eq!(applied_triple(wheel.take()), expected_triple(again), "{name}: a second frame");
        } else {
            assert_eq!(applied_triple(wheel.take()), None, "{name}: taking the wheel leaves none");
        }
        println!("[DEBUG] wheel-application-point {name}: applied={:?} pointer={pointer:?}", applied_triple(applied));
    }
    assert!(discriminating >= 1, "the fixture pins at least one stream the pre-fix shape got wrong");
}
