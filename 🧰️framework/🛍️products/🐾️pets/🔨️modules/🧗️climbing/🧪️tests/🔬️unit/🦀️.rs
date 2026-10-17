//! 🧗️ Unit tests of the climbing module: holds, climbs, slides and mantles on walls, wall lines and the lunges between their pitches, the placement and the gait of ladders, shots, hook flights, zips and swings of the grappling rope, and the routes between perches — the cases of the TypeScript suite, each against its closed form, its per-tick sum or the law it has to keep, and every member the committed vectors of the wall-climbing, ladder-geometry and grapple-reach cases state, within the comparison tolerance of their oracles (the parity of the cases compares the whole projections).
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../🧪️tests/🔬️unit/🟦️.ts — the TypeScript suite these tests mirror
//! @see ../../../../🧫️fixtures/🧗️wall-climbing/🔣️.json
//! @see ../../../../🧫️fixtures/🪜️ladder-geometry/🔣️.json
//! @see ../../../../🧫️fixtures/🎣️grapple-reach/🔣️.json

use super::*;
use crate::swing::{REEL_CAP, REEL_RAMP, REEL_SPEED};
use semio_framework_pets_contract::schema::Wall;
use semio_framework_pets_contract::testkit::{entries, fixture, json, number, typed};
use semio_framework_pets_kinematics::terrain::{walls_of, WALL_LIP};
use serde_json::{json, Value};

const SOURCE: &str = include_str!("../../🦀️.rs");
const STAGES: u32 = 60;
const TOLERANCE: f64 = 1e-9;
const PET: Size = Size { width: 40.0, height: 48.0 };
const SMALL: Size = Size { width: 34.0, height: 40.0 };
const LARGE: Size = Size { width: 46.0, height: 56.0 };

fn stream(seed: u32) -> impl FnMut(u32) -> f64 {
    let mut state = seed;
    move |bound| {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((f64::from(state) / 4_294_967_296.0) * f64::from(bound)).floor()
    }
}

fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

fn perch(id: &str, x0: f64, x1: f64, y: f64) -> Perch {
    Perch { surface: id.to_string(), x0, x1, y }
}

fn pitch(id: &str, surface: &str, side: Facing, x: f64, y0: f64, y1: f64) -> Pitch {
    Pitch { wall: id.to_string(), surface: surface.to_string(), side, x, y0, y1 }
}

fn keepout(x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect { x, y, width, height }
}

fn solid(x0: f64, x1: f64, y0: f64, y1: f64) -> Rect {
    keepout(x0 - 4.0, y0, x1 - x0 + 8.0, y1 - y0)
}

fn floor() -> Perch {
    perch("floor", 0.0, 640.0, 480.0)
}

fn card() -> Perch {
    perch("card", 306.0, 494.0, 300.0)
}

fn card_box() -> Rect {
    solid(300.0, 500.0, 300.0, 480.0)
}

fn left() -> Pitch {
    pitch("card-left", "card", Facing::Left, 300.0, 300.0, 480.0)
}

fn right() -> Pitch {
    pitch("card-right", "card", Facing::Right, 500.0, 300.0, 480.0)
}

fn climbed(y: f64, goal: f64) -> Vec<f64> {
    let mut heights = Vec::new();
    let mut height = y;
    let mut tick = 0;
    while height != goal && tick < 2000 {
        height = climb_step(height, goal, tick);
        heights.push(height);
        tick += 1;
    }
    heights
}

fn holding(rect: &Rect, at: Point) -> bool {
    rect.width > 0.0 && rect.height > 0.0 && rect.x <= at.x && at.x <= rect.x + rect.width && rect.y <= at.y && at.y <= rect.y + rect.height
}

fn along(from: Point, to: Point, y: f64) -> f64 {
    from.x + ((to.x - from.x) * (y - from.y)) / (to.y - from.y)
}

fn distance(from: Point, to: Point) -> f64 {
    (to.x - from.x).hypot(to.y - from.y)
}

fn means(legs: &[Leg<'_>]) -> Vec<&'static str> {
    legs.iter()
        .map(|leg| match leg {
            Leg::Ladder { .. } => "ladder",
            Leg::Wall { .. } => "wall",
            Leg::Raise { .. } => "raise",
            Leg::Grapple { .. } => "grapple",
        })
        .collect()
}

fn group<'a>(vectors: &'a Value, name: &str) -> &'a [Value] {
    let listed = entries(&vectors[name]);
    assert!(!listed.is_empty(), "the fixture carries no {name}");
    listed
}

fn assert_close(context: &str, produced: &Value, expected: &Value) {
    match (produced, expected) {
        (Value::Number(left), Value::Number(right)) => {
            let (left, right) = (left.as_f64().unwrap_or(f64::NAN), right.as_f64().unwrap_or(f64::NAN));
            assert!((left - right).abs() <= TOLERANCE * right.abs().max(1.0), "{context}: {left} ≠ {right}");
        }
        (Value::Object(left), Value::Object(right)) => {
            for (key, value) in right {
                assert!(left.contains_key(key), "{context}: no {key}");
                assert_close(&format!("{context}/{key}"), &left[key], value);
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            assert_eq!(left.len(), right.len(), "{context}: length");
            for (index, (left, right)) in left.iter().zip(right).enumerate() {
                assert_close(&format!("{context}/{index}"), left, right);
            }
        }
        _ => assert_eq!(produced, expected, "{context}"),
    }
}

fn located<T>(list: &[T], found: Option<&T>) -> Value {
    found.map_or(Value::Null, |found| json!(list.iter().position(|candidate| std::ptr::eq(candidate, found))))
}

fn hauled(shot: &Shot, size: Size, step: fn(&Shot, Haul, Ticks, Size) -> Haul) -> Value {
    let least = REEL_LEAST * size.height;
    let mut states = Vec::new();
    let mut haul = haul_of(shot, shot.length, size);
    while haul.rope > least && states.len() < 1024 {
        haul = step(shot, haul, states.len() as Ticks, size);
        states.push(haul);
    }
    json!({ "ticks": states.len(), "ropes": states.iter().map(|state| state.rope).collect::<Vec<_>>(), "hands": states.iter().map(|state| [state.hand.x, state.hand.y]).collect::<Vec<_>>(), "feet": states.iter().map(|state| [state.x, state.y]).collect::<Vec<_>>() })
}

fn leg_value(leg: &Leg<'_>, pitches: &[Pitch], ladders: &[LadderStand]) -> Value {
    match leg {
        Leg::Ladder { at, ladder, up } => json!({ "means": "ladder", "at": at, "up": up, "ladder": located(ladders, Some(*ladder)) }),
        Leg::Wall { at, pitch, hold, exit, goal } => json!({ "means": "wall", "at": at, "hold": hold, "goal": goal, "pitch": located(pitches, Some(*pitch)), "exit": located(pitches, Some(*exit)) }),
        Leg::Raise { at, ladder } => json!({ "means": "raise", "at": at, "ladder": ladder }),
        Leg::Grapple { at, shot } => json!({ "means": "grapple", "at": at, "shot": shot }),
    }
}

#[path = "🧱️wall-and-hoist-laws/🦀️.rs"]
mod wall_and_hoist_laws;

#[path = "🫳️grip-climb-slide-and-mantle/🦀️.rs"]
mod grip_climb_slide_and_mantle;

#[path = "🪜️ladder-laws/🦀️.rs"]
mod ladder_laws;

#[path = "🪝️grapple-shot-and-haul/🦀️.rs"]
mod grapple_shot_and_haul;

#[path = "🗺️routes-and-wall-lines/🦀️.rs"]
mod routes_and_wall_lines;

#[path = "📜️committed-vectors-and-source/🦀️.rs"]
mod committed_vectors_and_source;
