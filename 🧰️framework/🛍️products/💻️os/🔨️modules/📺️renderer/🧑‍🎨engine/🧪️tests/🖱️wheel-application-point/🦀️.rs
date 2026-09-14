//! 🖱️ Rust law over `🧫️fixtures/🖱️wheel-application-point/🔣️.json`.
//!
//! Drives the production accumulator (`AppWheel`) with each case's event stream in order and checks
//! every application the frame would make — the point AND the delta of each, in order — the hop
//! `🎮️wgpu-browser-input-wire/🔣️.json` stops just short of. The TypeScript twin
//! (`🧪️tests/🖱️wheel-application-point/🟦️.ts`) answers the same oracle from an independent
//! implementation of the rule, so neither side can drift alone.
//!
//! ⚖️ Every case that carries `baselineApplications` also re-derives a PRE-FIX shape from the same
//! fixture — the whole coalesced delta as ONE application, at the newest point the stream carried —
//! and asserts it differs. That is the defect family verbatim: `dispatch_normalized_event`'s
//! `Scroll { delta_y, .. }` arm first dropped the position the wire had carried from the DOM, and
//! the accumulator that fixed it still merged notches ACROSS points, so a stream that travels — the
//! battery's own `h7_wheel_zoom`, which nudges the pointer 1 px between notches — collapsed into one
//! application in the corner (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.1 and
//! `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §2).

use super::{AppWheel, WHEEL_PENDING_APPLICATIONS};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🖱️wheel-application-point/🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("wheel application point fixture parses")
}

fn cases() -> Vec<Value> {
    fixture()["cases"].as_array().expect("fixture declares cases").clone()
}

fn number(value: &Value) -> f32 {
    value.as_f64().expect("fixture number") as f32
}

/// 🔤️ The fixture's own language-neutral spelling of one application — three numbers — so the
/// comparison is over values and never over JSON's integer/float rendering.
fn expected(value: &Value) -> Vec<[f32; 3]> {
    value.as_array().expect("fixture applications").iter().map(|entry| [number(&entry["x"]), number(&entry["y"]), number(&entry["delta"])]).collect()
}

/// 🖱️ One frame's worth of input, in order, answered as the frame's own `WheelStart` ladder reads
/// it: every pending application, oldest first, plus where the pointer ended up.
fn drive(events: &[Value]) -> (Vec<[f32; 3]>, (f32, f32)) {
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
    let mut applications = Vec::new();
    while let Some((delta, x, y)) = wheel.take() {
        applications.push([x, y, delta]);
        assert!(applications.len() <= WHEEL_PENDING_APPLICATIONS, "a frame drains at most its own credits");
    }
    assert!(!wheel.pending(), "draining the wheel leaves none");
    (applications, pointer)
}

#[test]
fn a_frame_applies_every_wheel_notch_at_the_point_it_was_scrolled_at() {
    let mut discriminating = 0;
    for case in cases() {
        let name = case["name"].as_str().expect("case name");
        let events = case["events"].as_array().expect("events").clone();
        let (applications, pointer) = drive(&events);
        assert_eq!(applications, expected(&case["applications"]), "{name}");

        // 🔍️ Both pre-fix shapes, re-derived from the same fixture with one rule: ONE application
        // carrying the whole delta, at wherever the pointer ended up. That is what the original
        // accumulator did (`last_pointer_x/y`) and, whenever the stream's last event is itself a
        // scroll, also what the newest-point-wins accumulator did. Every case that names a baseline
        // must differ from it, which holds this oracle to discriminating rather than agreeing.
        if let Some(baseline) = case.get("baselineApplications") {
            let total: f32 = applications.iter().map(|application| application[2]).sum();
            let pre_fix = vec![[pointer.0, pointer.1, total]];
            assert_eq!(pre_fix, expected(baseline), "{name}: the fixture's own pre-fix shape");
            assert_ne!(pre_fix, expected(&case["applications"]), "{name}: the pre-fix shape loses an application");
            discriminating += 1;
        }
        println!("[DEBUG] wheel-application-point {name}: applications={applications:?} pointer={pointer:?}");
    }
    assert!(discriminating >= 2, "the fixture pins the streams both pre-fix shapes got wrong");
}

#[test]
fn the_pending_wheel_carries_exactly_the_credits_the_fixture_declares() {
    let declared = fixture()["rule"]["capacity"].as_u64().expect("the rule declares its credits") as usize;
    assert_eq!(declared, WHEEL_PENDING_APPLICATIONS);

    let mut wheel = AppWheel::default();
    assert!(!wheel.pending(), "an untouched wheel owes nothing");
    for index in 0..(WHEEL_PENDING_APPLICATIONS * 4) {
        let point = (index as f32 + 1.0) * 10.0;
        wheel.accumulate(point, point, 10.0);
        assert!(wheel.pending(), "a scrolled wheel owes an application");
    }
    let mut applications = 0;
    while wheel.take().is_some() {
        applications += 1;
    }
    assert_eq!(applications, WHEEL_PENDING_APPLICATIONS, "a stream longer than the credits coalesces rather than growing");
}
