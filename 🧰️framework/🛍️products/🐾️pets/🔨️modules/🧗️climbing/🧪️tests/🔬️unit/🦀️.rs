//! 🧗️ Unit tests of the climbing module: holds, climbs, slides and mantles on walls, wall lines and the lunges between their pitches, the placement and the gait of ladders, shots, hook flights, zips and swings of the grappling rope, and the routes between perches — the cases of the TypeScript suite, each against its closed form, its per-tick sum or the law it has to keep, and every member the committed vectors of the wall-climbing, ladder-geometry and grapple-reach cases state, within the comparison tolerance of their oracles (the parity of the cases compares the whole projections).
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../🧪️tests/🔬️unit/🟦️.ts — the TypeScript suite these tests mirror
//! @see ../../../../🧫️fixtures/🧗️wall-climbing/🔣️.json
//! @see ../../../../🧫️fixtures/🪜️ladder-geometry/🔣️.json
//! @see ../../../../🧫️fixtures/🎣️grapple-reach/🔣️.json

use super::*;
use crate::schema::tests::{entries, fixture, json, number, typed};
use crate::schema::Wall;
use crate::swing::{REEL_CAP, REEL_RAMP, REEL_SPEED};
use crate::terrain::walls_of;
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

#[test]
fn the_stage_of_this_suite_is_what_the_terrain_cuts_out_of_a_card_that_stands_on_the_floor() {
    let walls =
        [Wall { id: "card-left".to_string(), surface: "card".to_string(), side: Facing::Left, x: 300.0, y0: 300.0, y1: 480.0 }, Wall { id: "card-right".to_string(), surface: "card".to_string(), side: Facing::Right, x: 500.0, y0: 300.0, y1: 480.0 }];
    assert_eq!(walls_of(&walls, &[card_box()], 640.0, 480.0, 48.0, 72.0), [left(), right()]);
}

#[test]
fn a_line_is_sighted_past_every_keep_out_it_misses_and_every_one_that_holds_one_of_its_ends() {
    let (from, to) = (point(0.0, 0.0), point(100.0, 100.0));
    let boxed = keepout(40.0, 40.0, 20.0, 20.0);
    assert!(!sighted(from, to, &[boxed], 0.0, from, to));
    assert!(sighted(from, to, &[boxed], 0.0, point(50.0, 50.0), to) && sighted(from, to, &[boxed], 0.0, from, point(40.0, 60.0)));
    assert!(sighted(from, to, &[keepout(102.0, 95.0, 10.0, 10.0)], 0.0, from, to) && !sighted(from, to, &[keepout(102.0, 95.0, 10.0, 10.0)], 3.0, from, to));
    assert!(sighted(from, to, &[], 0.0, from, to) && sighted(from, to, &[keepout(40.0, 40.0, 0.0, 20.0)], 0.0, from, to));
}

#[test]
fn hoist_begins_on_its_start_and_ends_on_its_end_exactly_and_stays_there_beyond() {
    let (from, to) = (point(280.0, 340.8), point(328.0, 300.0));
    assert_eq!([hoist_path(from, to, 48.0, 0.0), hoist_path(from, to, 48.0, -0.5)], [from, from]);
    assert_eq!([hoist_path(from, to, 48.0, 1.0), hoist_path(from, to, 48.0, 1.5)], [to, to]);
}

#[test]
fn hoist_rises_before_it_crosses_over_tops_its_end_by_the_hump_and_settles_onto_it() {
    let (from, to) = (point(280.0, 340.8), point(328.0, 300.0));
    let path: Vec<Point> = (0..=MANTLE_TICKS).map(|tick| hoist_path(from, to, 48.0, tick as f64 / MANTLE_TICKS as f64)).collect();
    for tick in 1..path.len() {
        assert!(path[tick].x >= path[tick - 1].x, "tick {tick}");
    }
    for (tick, at) in path.iter().enumerate() {
        if tick as f64 / MANTLE_TICKS as f64 <= 0.35 {
            assert_eq!(at.x, from.x, "tick {tick}");
        }
    }
    let apex = path.iter().fold(f64::INFINITY, |least, at| least.min(at.y));
    assert!(apex > to.y - HOIST_HUMP * 48.0 - 1e-9 && apex < to.y - HOIST_HUMP * 48.0 + 0.2, "{apex}");
    let top = path.iter().position(|at| at.y == apex).unwrap_or_default();
    for tick in 1..=top {
        assert!(path[tick].y < path[tick - 1].y, "tick {tick}");
    }
    for tick in top + 1..path.len() {
        assert!(path[tick].y > path[tick - 1].y, "tick {tick}");
    }
}

#[test]
fn hoist_is_the_three_eased_parts_of_its_definition_at_every_phase_mirrors_with_the_side_and_leads_down_as_well_as_up() {
    let (from, to) = (point(280.0, 340.8), point(328.0, 300.0));
    for step in 0..=200 {
        let phase = f64::from(step) / 200.0;
        let found = hoist_path(from, to, 48.0, phase);
        let (rise, across, settle) = (smoothstep(phase / 0.65), smoothstep((phase - 0.35) / 0.65), smoothstep((phase - 0.65) / 0.35));
        assert!((found.x - (from.x + (to.x - from.x) * across)).abs() < 1e-9, "phase {phase}");
        assert!((found.y - (from.y + (to.y - 4.8 - from.y) * rise + 4.8 * settle)).abs() < 1e-9, "phase {phase}");
    }
    let mirrored = hoist_path(point(520.0, 340.8), point(472.0, 300.0), 48.0, 0.6);
    let found = hoist_path(from, to, 48.0, 0.6);
    assert!((mirrored.x - 400.0 - (400.0 - found.x)).abs() < 5e-10);
    assert_eq!(mirrored.y, found.y);
    let down = hoist_path(point(100.0, 200.0), point(130.0, 260.0), 40.0, 0.5);
    assert!(down.y > 200.0 && down.y < 260.0);
}

#[test]
fn a_wall_is_clung_to_half_a_width_out_on_the_air_side_between_the_rim_and_the_foot_of_the_pitch() {
    let (left, right) = (left(), right());
    assert_eq!([cling_of(&left, PET), cling_of(&right, PET)], [280.0, 520.0]);
    assert_eq!([ledge_of(&left, PET), ledge_of(&right, PET)], [300.0 + 20.0 + MANTLE_INSET, 500.0 - 20.0 - MANTLE_INSET]);
    assert_eq!(rim_of(&left, PET), 300.0 + HAND_HEIGHT * 48.0);
    assert_eq!(foot_of(&left, PET), 480.0 - GRIP_BITE + HAND_HEIGHT * 48.0);
    assert!(clings(&left, rim_of(&left, PET), PET) && clings(&left, foot_of(&left, PET), PET) && clings(&left, 400.0, PET));
    assert!(!clings(&left, rim_of(&left, PET) - 0.25, PET) && !clings(&left, foot_of(&left, PET) + 0.25, PET));
}

#[test]
fn a_wall_is_crowned_by_the_perch_of_its_surface_at_its_top_that_carries_the_ledge() {
    let (left, right, card) = (left(), right(), card());
    assert!(crowns(&card, &left, PET) && crowns(&card, &right, PET));
    assert!(!crowns(&floor(), &left, PET));
    assert!(!crowns(&perch("card", 306.0, 494.0, 299.5), &left, PET));
    assert!(!crowns(&perch("other", 306.0, 494.0, 300.0), &left, PET));
    assert!(crowns(&perch("card", 328.0, 494.0, 300.0), &left, PET));
    assert!(!crowns(&perch("card", 328.5, 494.0, 300.0), &left, PET));
    assert!(!crowns(&card, &pitch("card-left", "card", Facing::Left, 300.0, 320.0, 480.0), PET));
    let perches = [floor(), perch("card", 400.0, 494.0, 300.0), card];
    assert!(rim_for(&left, &perches, PET).is_some_and(|found| std::ptr::eq(found, &perches[2])));
    assert_eq!(rim_for(&left, &perches[..2], PET), None);
    assert_eq!(rim_for(&left, &[], PET), None);
}

#[test]
fn a_wall_is_taken_from_a_perch_at_its_foot_where_the_actor_can_stand_beside_it_with_its_hands_on_it() {
    let (left, right) = (left(), right());
    let hold = |x: f64, y: f64| Some(WallHold { x, y, over: false });
    assert_eq!(grip_for(&floor(), &left, PET), hold(280.0, 480.0));
    assert_eq!(grip_for(&floor(), &right, PET), hold(520.0, 480.0));
    assert_eq!(grip_for(&perch("shelf", 100.0, 280.0, 420.0), &left, PET), hold(280.0, 420.0));
    assert_eq!(grip_for(&perch("shelf", 100.0, 279.5, 420.0), &left, PET), None);
    assert_eq!(grip_for(&perch("shelf", 280.5, 296.0, 420.0), &left, PET), None);
    assert_eq!(grip_for(&perch("floor", 0.0, 640.0, 512.8), &left, PET), hold(280.0, 512.8));
    assert_eq!(grip_for(&perch("floor", 0.0, 640.0, 513.0), &left, PET), None);
    assert_eq!(grip_for(&perch("shelf", 100.0, 290.0, rim_of(&left, PET)), &left, PET), hold(280.0, rim_of(&left, PET)));
    assert_eq!(grip_for(&perch("shelf", 100.0, 290.0, 340.0), &left, PET), None);
    assert_eq!(grip_for(&floor(), &left, LARGE), hold(277.0, 480.0));
}

#[test]
fn a_wall_is_taken_over_the_rim_from_the_perch_that_crowns_it_when_the_pitch_is_long_enough_to_hold() {
    let over = Some(WallHold { x: 280.0, y: 340.8, over: true });
    assert_eq!(grip_for(&card(), &left(), PET), over);
    assert_eq!(grip_for(&card(), &right(), PET), Some(WallHold { x: 520.0, y: 340.8, over: true }));
    assert_eq!(grip_for(&card(), &pitch("card-left", "card", Facing::Left, 300.0, 300.0, 308.0), PET), over);
    assert_eq!(grip_for(&card(), &pitch("card-left", "card", Facing::Left, 300.0, 300.0, 307.5), PET), None);
    assert_eq!(grip_for(&perch("card", 340.0, 494.0, 300.0), &left(), PET), None);
    assert_eq!(grip_for(&card(), &pitch("card-left", "card", Facing::Left, 300.0, 330.0, 480.0), PET), None);
}

#[test]
fn a_wall_carries_its_climber_through_a_survey_while_it_stays_within_the_tolerance_and_free_under_its_hands_and_throws_off_whoever_lost_it() {
    let left = left();
    let moved = |dx: f64, y0: f64, y1: f64| vec![right(), pitch("card-left", "card", Facing::Left, 300.0 + dx, y0, y1)];
    assert_eq!(wall_holds(&left, &moved(0.0, 300.0, 480.0), 400.0, PET), Some(&left));
    assert_eq!(wall_holds(&left, &moved(WALL_FOLLOW, 300.0, 480.0), 400.0, PET), Some(&pitch("card-left", "card", Facing::Left, 306.0, 300.0, 480.0)));
    assert!(wall_holds(&left, &moved(-WALL_FOLLOW, 300.0, 480.0), 400.0, PET).is_some());
    assert_eq!(wall_holds(&left, &moved(WALL_FOLLOW + 0.5, 300.0, 480.0), 400.0, PET), None);
    assert_eq!(wall_holds(&left, &moved(0.0, 300.0, 360.0), 400.0, PET), None);
    assert!(wall_holds(&left, &moved(0.0, 300.0, 367.2), 400.0, PET).is_some());
    assert_eq!(wall_holds(&left, &moved(0.0, 360.0, 480.0), 400.0, PET), None);
    assert_eq!(wall_holds(&left, &[right()], 400.0, PET), None);
    assert_eq!(wall_holds(&left, &[pitch("card-left", "card", Facing::Right, 300.0, 300.0, 480.0)], 400.0, PET), None);
    let split = [pitch("card-left", "card", Facing::Left, 300.0, 300.0, 340.0), pitch("card-left", "card", Facing::Left, 300.0, 350.0, 480.0)];
    assert!(wall_holds(&left, &split, 400.0, PET).is_some_and(|found| std::ptr::eq(found, &split[1])));
    assert_eq!(wall_holds(&left, &[], 400.0, PET), None);
    assert_eq!(slip_of(&left), Toss { vx: -SLIP_PUSH, vy: -SLIP_LIFT });
    assert_eq!(slip_of(&right()), Toss { vx: SLIP_PUSH, vy: -SLIP_LIFT });
}

#[test]
fn grip_spends_one_per_tick_of_climbing_a_quarter_per_tick_of_hanging_never_falls_below_nothing_and_comes_back_two_per_tick_of_rest_up_to_the_budget() {
    assert_eq!([grip_step(GRIP_BUDGET, Effort::Climb), grip_step(GRIP_BUDGET, Effort::Hang), grip_step(0.5, Effort::Climb), grip_step(0.0, Effort::Hang)], [383.0, 383.75, 0.0, 0.0]);
    let drained = |effort: Effort| {
        let (mut grip, mut ticks) = (GRIP_BUDGET, 0.0);
        while grip > 0.0 {
            grip = grip_step(grip, effort);
            ticks += 1.0;
        }
        ticks
    };
    assert_eq!([drained(Effort::Climb), drained(Effort::Hang)], [GRIP_BUDGET, 4.0 * GRIP_BUDGET]);
    assert_eq!([grip_step(100.0, Effort::Rest), grip_step(GRIP_BUDGET - 1.0, Effort::Rest), grip_step(GRIP_BUDGET, Effort::Rest)], [102.0, GRIP_BUDGET, GRIP_BUDGET]);
    let (mut grip, mut ticks) = (0.0, 0.0);
    while grip < GRIP_BUDGET {
        grip = grip_step(grip, Effort::Rest);
        ticks += 1.0;
    }
    assert_eq!(ticks, GRIP_BUDGET / 2.0);
}

#[test]
fn climbing_gathers_its_speed_over_the_ramp_then_climbs_at_the_full_speed_up_slower_than_down() {
    let (up, down) = (climbed(480.0, 340.8), climbed(340.8, 480.0));
    for tick in 0..12 {
        let eased = smoothstep((tick + 1) as f64 / CLIMB_RAMP as f64);
        let (up_before, down_before) = if tick == 0 { (480.0, 340.8) } else { (up[tick - 1], down[tick - 1]) };
        assert!((up_before - up[tick] - (CLIMB_RISE * eased) / 64.0).abs() < 1e-9, "up {tick}");
        assert!((down[tick] - down_before - (CLIMB_DESCENT * eased) / 64.0).abs() < 1e-9, "down {tick}");
    }
    assert!((480.0 - up[5] - (3.5 * CLIMB_RISE) / 64.0).abs() < 1e-9);
    assert!(up.len() > down.len());
}

#[test]
fn climbing_never_overshoots_ends_on_its_goal_exactly_and_stays_there() {
    for (from, goal) in [(480.0, 340.8), (340.8, 480.0), (400.0, 399.9), (400.0, 400.3), (512.8, 340.8)] {
        let heights = climbed(from, goal);
        assert_eq!(heights.last().copied(), Some(goal));
        assert!(heights.iter().all(|height| (height - goal) * (from - goal) >= 0.0), "{from} → {goal}");
        assert_eq!(climb_step(goal, goal, heights.len() as Ticks), goal);
        assert_eq!(climb_ticks(from, goal), heights.len() as Ticks);
    }
    assert_eq!(climb_ticks(400.0, 400.0), 0);
}

#[test]
fn climbing_takes_as_long_as_its_closed_form_says_and_gives_up_beyond_the_grip_of_a_rested_actor() {
    for distance in [0.1, 1.5, 1.640625, 2.0, 10.0, 92.0, 139.2, 150.0] {
        let ramp = (3.5 * CLIMB_RISE) / 64.0;
        let expected = if distance <= ramp { climbed(480.0, 480.0 - distance).len() as Ticks } else { CLIMB_RAMP + ((distance - ramp) / (CLIMB_RISE / 64.0) - 1e-9).ceil() as Ticks };
        assert_eq!(climb_ticks(480.0, 480.0 - distance), expected, "distance {distance}");
    }
    let budget = GRIP_BUDGET as Ticks;
    assert_eq!(climb_ticks(480.0, 480.0 - 178.8), budget);
    assert_eq!([climb_ticks(480.0, 480.0 - 181.0), climb_ticks(480.0, -10000.0), climb_ticks(480.0, f64::NEG_INFINITY), climb_ticks(300.0, f64::NAN)], [budget + 1; 4]);
}

#[test]
fn the_climbing_clip_runs_one_cycle_per_two_holds_counted_up_the_wall_and_advances_by_the_distance_climbed() {
    let left = left();
    let stride = 2.0 * GRIP_SPACING * 48.0;
    let turned = |y: f64, expected: f64| (((climb_phase(&left, y, PET) - expected + 1.5) % 1.0) - 0.5).abs();
    assert_eq!(climb_phase(&left, 300.0, PET), 0.0);
    assert!(turned(300.0 + stride, 0.0) < 1e-12 && turned(300.0 + 3.0 * stride, 0.0) < 1e-12);
    assert!(turned(300.0 + stride / 4.0, 0.75) < 1e-12 && turned(300.0 + stride / 2.0, 0.5) < 1e-12 && turned(300.0 - stride / 4.0, 0.25) < 1e-12);
    assert!(turned(300.0 + stride / 4.0, 0.25) > 0.4);
    let mut y = 250.0;
    while y < 520.0 {
        let phase = climb_phase(&left, y, PET);
        assert!((0.0..1.0).contains(&phase), "{y}");
        y += 0.37;
    }
    let mut heights = vec![480.0];
    heights.extend(climbed(480.0, 340.8));
    for tick in 1..heights.len() {
        let advanced = climb_phase(&left, heights[tick], PET) - climb_phase(&left, heights[tick - 1], PET);
        let expected = (heights[tick - 1] - heights[tick]) / (2.0 * GRIP_SPACING * 48.0);
        assert!((advanced - advanced.floor() - expected).abs() < 1e-9, "tick {tick}");
    }
    assert_eq!(climb_phase(&pitch("moved", "card", Facing::Left, 304.0, 310.0, 490.0), 410.0, PET), climb_phase(&left, 400.0, PET));
}

#[test]
fn sliding_starts_at_the_slip_speed_gains_speed_up_to_the_fastest_slide_and_ends_on_its_floor_exactly() {
    assert_eq!(slide_step(340.8, 0.0, 480.0), Fall { y: 340.8 + (SLIDE_START + SLIDE_GAIN / 64.0) / 64.0, vy: SLIDE_START + SLIDE_GAIN / 64.0 });
    assert_eq!(slide_step(340.8, 100.0, 480.0), Fall { y: 340.8 + 109.375 / 64.0, vy: 109.375 });
    assert_eq!(slide_step(340.8, SLIDE_SPEED, 480.0), Fall { y: 340.8 + 2.5, vy: SLIDE_SPEED });
    assert_eq!(slide_step(340.8, 5000.0, 480.0), Fall { y: 340.8 + 2.5, vy: SLIDE_SPEED });
    let (mut slide, mut ticks) = (Fall { y: 340.8, vy: 0.0 }, 0.0);
    while slide.vy < SLIDE_SPEED {
        slide = slide_step(slide.y, slide.vy, 10000.0);
        ticks += 1.0;
        if slide.vy < SLIDE_SPEED {
            assert!((slide.vy - (SLIDE_START + (ticks * SLIDE_GAIN) / 64.0)).abs() < 1e-9);
        }
    }
    assert_eq!(ticks, (((SLIDE_SPEED - SLIDE_START) * 64.0) / SLIDE_GAIN).ceil());
    let (mut slide, mut ticks) = (Fall { y: 340.8, vy: 0.0 }, 0);
    while slide.y != 480.0 && ticks < 500 {
        let next = slide_step(slide.y, slide.vy, 480.0);
        assert!(next.y > slide.y && next.y <= 480.0);
        slide = next;
        ticks += 1;
    }
    assert!(slide.y == 480.0 && ticks > 56 && ticks < 70, "{ticks}");
    assert_eq!(slide_step(480.0, slide.vy, 480.0).y, 480.0);
}

#[test]
fn the_mantle_leads_from_the_highest_hold_over_the_rim_onto_the_ledge_on_either_side() {
    let (left, right) = (left(), right());
    assert_eq!([mantle_path(&left, PET, 0.0), mantle_path(&left, PET, 1.0)], [point(280.0, 340.8), point(328.0, 300.0)]);
    assert_eq!([mantle_path(&right, PET, 0.0), mantle_path(&right, PET, 1.0)], [point(520.0, 340.8), point(472.0, 300.0)]);
    for tick in 0..=MANTLE_TICKS {
        let phase = tick as f64 / MANTLE_TICKS as f64;
        let (on_left, on_right) = (mantle_path(&left, PET, phase), mantle_path(&right, PET, phase));
        assert_eq!(on_left, hoist_path(point(280.0, 340.8), point(328.0, 300.0), 48.0, phase));
        assert!((on_right.x - 400.0 - (400.0 - on_left.x)).abs() < 1e-9 && on_right.y == on_left.y, "tick {tick}");
        assert!(on_left.x <= 300.0 || on_left.y <= 300.5, "tick {tick}");
    }
}

#[test]
fn a_ladder_stands_at_the_lean_of_real_ladders_with_its_top_a_little_under_the_rim() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let ladder = ladder_for(&floor(), &shelf, &wall, &boxed, PET).unwrap_or_else(|| panic!("no ladder"));
    assert_eq!(ladder, LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(300.0 - LADDER_LEAN * 93.0, 480.0), top: point(300.0, 380.0 + LADDER_TUCK) });
    assert_eq!(ladder_lean(&ladder), LADDER_LEAN);
    assert!((ladder_length(&ladder) - 23.25_f64.hypot(93.0)).abs() < 1e-12);
    assert_eq!(ladder_rungs(&ladder), 9);
    let mirrored = ladder_for(&floor(), &shelf, &pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0), &boxed, PET).unwrap_or_else(|| panic!("no mirrored ladder"));
    assert_eq!((mirrored.foot, mirrored.side), (point(500.0 + LADDER_LEAN * 93.0, 480.0), Facing::Right));
}

#[test]
fn a_ladder_moves_its_foot_as_near_to_that_lean_as_its_perch_lets_it_within_the_steepest_and_the_flattest_lean() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let short = |x0: f64, x1: f64| ladder_for(&perch("floor", x0, x1, 480.0), &shelf, &wall, &boxed, PET);
    let foot = |x0: f64, x1: f64| short(x0, x1).map(|ladder| ladder.foot.x);
    assert_eq!([foot(0.0, 640.0), foot(0.0, 275.0), foot(0.0, 273.0), foot(270.0, 640.0), foot(276.5, 640.0), foot(260.0, 280.0)], [Some(276.75), Some(275.0 - LADDER_FOOTING), Some(263.0), Some(270.0 + LADDER_FOOTING), Some(286.5), Some(270.0)]);
    assert!((short(0.0, 275.0).map_or(f64::NAN, |ladder| ladder_lean(&ladder)) - 35.0 / 93.0).abs() < 5e-13);
    assert!((short(270.0, 640.0).map_or(f64::NAN, |ladder| ladder_lean(&ladder)) - 20.0 / 93.0).abs() < 5e-13);
    assert_eq!([foot(0.0, 272.5), foot(277.5, 640.0), foot(260.0, 279.5), foot(310.0, 640.0)], [None; 4]);
}

#[test]
fn a_ladder_needs_a_rise_between_the_shortest_and_the_tallest_ladder_of_its_owner_and_a_pitch_that_reaches_down_to_its_contact() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let from = |y: f64, size: Size| ladder_for(&perch("floor", 0.0, 640.0, y), &shelf, &wall, &[], size).is_some();
    assert!(from(387.0 + LADDER_SHORT * 48.0 + 0.5, PET) && !from(387.0 + LADDER_SHORT * 48.0 - 0.5, PET));
    assert!(from(387.0 + LADDER_TALL * 48.0 - 0.5, PET) && !from(387.0 + LADDER_TALL * 48.0 + 0.5, PET));
    assert!(!from(387.0 + LADDER_SHORT * 48.0 + 0.5, LARGE) && from(387.0 + LADDER_SHORT * 56.0 + 0.5, LARGE));
    assert!(from(387.0 + LADDER_SHORT * 40.0 + 0.5, SMALL) && from(387.0 + LADDER_TALL * 48.0 + 0.5, LARGE));
    assert!(!from(380.0, PET) && !from(300.0, PET));
    assert_eq!(ladder_for(&floor(), &perch("shelf", 340.0, 494.0, 380.0), &wall, &[], PET), None);
    assert_eq!(ladder_for(&floor(), &perch("other", 306.0, 494.0, 380.0), &wall, &[], PET), None);
    assert_eq!(ladder_for(&floor(), &shelf, &pitch("shelf-left", "shelf", Facing::Left, 300.0, 390.0, 480.0), &[], PET), None);
    assert!(ladder_for(&floor(), &shelf, &pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 387.0), &[], PET).is_some());
    assert_eq!(ladder_for(&floor(), &shelf, &pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 386.5), &[], PET), None);
}

#[test]
fn a_ladder_passes_over_what_it_stands_on_and_leans_against_and_is_blocked_by_anything_else_within_its_girth() {
    let (shelf, wall) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0));
    let stage = |blocker: Rect| ladder_for(&floor(), &shelf, &wall, &[solid(300.0, 500.0, 380.0, 480.0), keepout(0.0, 480.0, 640.0, 40.0), blocker], PET);
    let ladder = stage(keepout(0.0, 0.0, 0.0, 0.0)).unwrap_or_else(|| panic!("no ladder"));
    assert!(stage(keepout(280.0, 400.0, 10.0, 30.0)).is_none() && stage(keepout(240.0, 400.0, 30.0, 30.0)).is_some());
    let reach = along(ladder.foot, ladder.top, 430.0);
    assert!(stage(keepout(reach - LADDER_GIRTH - 20.0, 420.0, 20.0, 20.0)).is_none() && stage(keepout(reach - LADDER_GIRTH - 40.0, 420.0, 20.0, 20.0)).is_some());
    assert!(stage(keepout(200.0, 440.0, 60.0, 34.0)).is_some() && stage(keepout(200.0, 440.0, 71.5, 34.5)).is_none());
}

#[test]
fn a_ladder_keeps_every_law_on_generated_stages() {
    let mut next = stream(41);
    let (mut granted, mut refused) = (0, 0);
    for stage in 0..STAGES {
        let size = [PET, SMALL, LARGE][next(3) as usize];
        let side = if next(2) == 0.0 { Facing::Left } else { Facing::Right };
        let x = 200.0 + next(800) / 4.0;
        let rim = 150.0 + next(600) / 4.0;
        let high = if side == Facing::Left { perch("high", x + 6.0, x + 180.0, rim) } else { perch("high", x - 180.0, x - 6.0, rim) };
        let face = pitch("high-side", "high", side, x, rim, rim + 20.0 + next(800) / 4.0);
        let low = perch("low", x - 150.0 + next(600) / 4.0, x + next(600) / 4.0, rim + 20.0 + next(900) / 4.0);
        let far = x - side.sign() * 186.0;
        let mut keepouts = vec![solid(x.min(far), x.max(far), rim, rim + 300.0)];
        for _ in 0..next(3) as usize {
            keepouts.push(keepout(x - 120.0 + next(960) / 4.0, rim - 40.0 + next(1200) / 4.0, next(160) / 4.0, next(160) / 4.0));
        }
        let Some(ladder) = ladder_for(&low, &high, &face, &keepouts, size) else {
            refused += 1;
            continue;
        };
        granted += 1;
        let rise = ladder.foot.y - ladder.top.y;
        assert_eq!((ladder.top, ladder.foot.y), (point(x, rim + LADDER_TUCK), low.y), "stage {stage}");
        assert!(ladder.foot.x >= low.x0 + LADDER_FOOTING && ladder.foot.x <= low.x1 - LADDER_FOOTING, "stage {stage}");
        assert!((ladder.foot.x - x) * side.sign() > 0.0, "stage {stage}");
        assert!(rise >= LADDER_SHORT * size.height && rise <= LADDER_TALL * size.height, "stage {stage}");
        assert!(ladder_lean(&ladder) >= LADDER_STEEP - 1e-12 && ladder_lean(&ladder) <= LADDER_FLAT + 1e-12, "stage {stage}");
        assert!((ladder_length(&ladder) - (ladder.foot.x - x).hypot(rise)).abs() < 1e-9, "stage {stage}");
        assert_eq!(f64::from(ladder_rungs(&ladder)), ((ladder.foot.x - x).hypot(rise) / RUNG_SPACING).floor(), "stage {stage}");
        for blocker in &keepouts {
            if !holding(blocker, ladder.foot) && !holding(blocker, ladder.top) {
                assert!(!segment_hits(ladder.foot, ladder.top, blocker, LADDER_GIRTH), "stage {stage}");
            }
        }
        assert_eq!(ladder_holds(&ladder, &[low, high], &[face], &keepouts, size).as_ref(), Some(&ladder), "stage {stage}");
    }
    assert!(granted > STAGES / 12 && refused > STAGES / 4, "{granted} granted, {refused} refused");
}

#[test]
fn a_ladder_is_climbed_with_a_gathered_speed_up_slower_than_down_ending_on_its_goal_exactly() {
    let ladder = LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(276.75, 480.0), top: point(300.0, 387.0) };
    let length = 23.25_f64.hypot(93.0);
    let exit = ladder_exit(&ladder, PET);
    assert!((exit - (length - LADDER_EXIT * 48.0)).abs() < 1e-12);
    let (mut travel, mut ticks) = (0.0, 0);
    while travel != exit && ticks < 1000 {
        let next = ladder_step(travel, exit, ticks);
        assert!((next - travel - ((LADDER_RISE * smoothstep((ticks + 1) as f64 / 6.0)) / 64.0).min(exit - travel)).abs() < 1e-9, "tick {ticks}");
        travel = next;
        ticks += 1;
    }
    assert_eq!(travel, exit);
    assert_eq!(ticks, 6 + ((exit - (3.5 * LADDER_RISE) / 64.0) / (LADDER_RISE / 64.0)).ceil() as Ticks);
    assert_eq!(ladder_step(exit, exit, ticks), exit);
    let mut down = 0;
    travel = exit;
    while travel != 0.0 && down < 1000 {
        travel = ladder_step(travel, 0.0, down);
        down += 1;
    }
    assert_eq!(travel, 0.0);
    assert_eq!(down, 6 + ((exit - (3.5 * LADDER_DESCENT) / 64.0) / (LADDER_DESCENT / 64.0)).ceil() as Ticks);
    assert!(down < ticks);
}

#[test]
fn a_ladder_carries_its_climber_along_the_rails_puts_hands_and_feet_on_the_rungs_and_lets_it_off_inside_the_rim() {
    let ladder = LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(276.75, 480.0), top: point(300.0, 387.0) };
    let length = 23.25_f64.hypot(93.0);
    assert_eq!([ladder_at(&ladder, 0.0), ladder_at(&ladder, -3.0), ladder_at(&ladder, length), ladder_at(&ladder, length + 10.0)], [ladder.foot, ladder.foot, ladder.top, ladder.top]);
    let mut travel = 0.5;
    while travel < length {
        let feet = ladder_at(&ladder, travel);
        assert!((distance(feet, ladder.foot) - travel).abs() < 1e-9 && (distance(feet, ladder.top) - (length - travel)).abs() < 1e-9, "{travel}");
        travel += 3.7;
    }
    assert_eq!(ladder_exit(&LadderStand { foot: point(298.0, 395.0), ..ladder.clone() }, PET), 0.0);
    assert_eq!([ladder_phase(0.0), ladder_phase(RUNG_SPACING), ladder_phase(2.0 * RUNG_SPACING), ladder_phase(5.25), ladder_phase(47.25)], [0.0, 0.5, 0.0, 0.25, 0.25]);
    let mut travel = 0.0;
    while travel < 120.0 {
        assert!((0.0..1.0).contains(&ladder_phase(travel)));
        travel += 0.41;
    }
    assert_eq!(ladder_landing(&ladder, PET), point(300.0 + 20.0 + MANTLE_INSET, 380.0));
    assert_eq!(ladder_landing(&ladder, LARGE), point(300.0 + 23.0 + MANTLE_INSET, 380.0));
    assert_eq!(ladder_landing(&LadderStand { side: Facing::Right, top: point(500.0, 387.0), ..ladder }, PET), point(500.0 - 20.0 - MANTLE_INSET, 380.0));
}

#[test]
fn a_toppling_ladder_drops_whoever_is_high_on_it_and_lets_whoever_is_low_step_off() {
    let ladder = LadderStand { wall: "shelf-left".to_string(), surface: "floor".to_string(), side: Facing::Left, foot: point(276.75, 480.0), top: point(300.0, 387.0) };
    assert_eq!([spill_of(&ladder, 480.0, PET), spill_of(&ladder, 480.0 - TOPPLE_STEP * 48.0 + 0.5, PET)], [None, None]);
    assert_eq!(spill_of(&ladder, 480.0 - TOPPLE_STEP * 48.0, PET), Some(Toss { vx: -TOPPLE_PUSH, vy: 0.0 }));
    assert_eq!(spill_of(&LadderStand { side: Facing::Right, ..ladder.clone() }, 400.0, PET), Some(Toss { vx: TOPPLE_PUSH, vy: 0.0 }));
    assert_eq!(spill_of(&ladder, 480.0 - TOPPLE_STEP * 48.0, LARGE), None);
}

#[test]
fn a_standing_ladder_follows_its_wall_and_its_perch_within_the_tolerances_and_topples_beyond_them() {
    let (shelf, wall, boxed) = (perch("shelf", 306.0, 494.0, 380.0), pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), solid(300.0, 500.0, 380.0, 480.0));
    let ladder = ladder_for(&floor(), &shelf, &wall, &[boxed], PET).unwrap_or_else(|| panic!("no ladder"));
    let moved = |dx: f64, dy: f64| {
        ladder_holds(&ladder, &[floor(), perch("shelf", 306.0 + dx, 494.0 + dx, 380.0 + dy)], &[pitch("shelf-left", "shelf", Facing::Left, 300.0 + dx, 380.0 + dy, 480.0 + dy)], &[solid(300.0 + dx, 500.0 + dx, 380.0 + dy, 480.0 + dy)], PET)
    };
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&ladder));
    for (dx, dy) in [(3.0, 0.0), (0.0, -5.0), (-4.5, 6.0), (0.0, LADDER_FOLLOW), (LADDER_FOLLOW, 0.0)] {
        assert_eq!(moved(dx, dy), Some(LadderStand { top: point(300.0 + dx, 387.0 + dy), ..ladder.clone() }), "{dx} {dy}");
    }
    for (dx, dy) in [(0.0, LADDER_FOLLOW + 0.5), (LADDER_FOLLOW + 0.5, 0.0), (6.0, 6.0), (-40.0, 0.0)] {
        assert_eq!(moved(dx, dy), None, "{dx} {dy}");
    }
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], &[], &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], &[pitch("shelf-left", "shelf", Facing::Right, 300.0, 380.0, 480.0)], &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], &[pitch("other", "shelf", Facing::Left, 300.0, 380.0, 480.0)], &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor()], std::slice::from_ref(&wall), &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), perch("shelf", 400.0, 494.0, 380.0)], std::slice::from_ref(&wall), &[boxed], PET), None);
    let steep = LadderStand { foot: point(300.0 - LADDER_STEEP * 93.0 - 0.5, 480.0), ..ladder.clone() };
    assert_eq!(ladder_holds(&steep, &[floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&steep));
    assert_eq!(ladder_holds(&steep, &[floor(), perch("shelf", 305.0, 493.0, 380.0)], &[pitch("shelf-left", "shelf", Facing::Left, 299.0, 380.0, 480.0)], &[solid(299.0, 499.0, 380.0, 480.0)], PET), None);
    let on = |low: Perch| ladder_holds(&ladder, &[low, shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET);
    assert_eq!(on(perch("floor", 0.0, 640.0, 480.0 + LADDER_SHIFT)), Some(LadderStand { foot: point(ladder.foot.x, 492.0), ..ladder.clone() }));
    assert_eq!(on(perch("floor", 0.0, 640.0, 474.0)), Some(LadderStand { foot: point(ladder.foot.x, 474.0), ..ladder.clone() }));
    assert_eq!(on(perch("floor", 0.0, 640.0, 480.0 + LADDER_SHIFT + 0.5)), None);
    assert_eq!(on(perch("floor", 0.0, ladder.foot.x + LADDER_FOOTING, 480.0)).as_ref(), Some(&ladder));
    assert_eq!(on(perch("floor", 0.0, ladder.foot.x + LADDER_FOOTING - 0.5, 480.0)), None);
    assert_eq!(on(perch("ground", 0.0, 640.0, 480.0)), None);
    assert_eq!(ladder_holds(&ladder, std::slice::from_ref(&shelf), std::slice::from_ref(&wall), &[boxed], PET), None);
    assert_eq!(ladder_holds(&ladder, &[perch("floor", 0.0, 100.0, 480.0), floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed], PET).as_ref(), Some(&ladder));
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf.clone()], std::slice::from_ref(&wall), &[boxed, keepout(270.0, 400.0, 30.0, 30.0)], PET), None);
    assert_eq!(ladder_holds(&ladder, &[floor(), shelf], &[wall], &[boxed, keepout(100.0, 400.0, 30.0, 30.0)], PET).as_ref(), Some(&ladder));
}

#[test]
fn a_shot_aims_from_the_muzzle_in_front_of_the_feet_at_a_point_just_above_the_edge_nearest_to_the_feet() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let boxed = [solid(300.0, 500.0, 380.0, 420.0)];
    let from = |x: f64| shot_for(point(x, 480.0), &[floor(), shelf.clone()], &boxed, PET).unwrap_or_else(|| panic!("no shot from {x}"));
    let shot = from(280.0);
    assert_eq!((shot.surface.as_str(), shot.facing), ("shelf", Facing::Right));
    assert_eq!((shot.muzzle, shot.hook), (point(280.0 + MUZZLE_FORWARD * 40.0, 480.0 - MUZZLE_HEIGHT * 48.0), point(306.0 + 20.0 + HOOK_INSET, 380.0 - HOOK_LIFT)));
    assert!((shot.length - distance(shot.hook, shot.muzzle)).abs() < 1e-12);
    let mirrored = from(520.0);
    assert_eq!((mirrored.facing, mirrored.muzzle, mirrored.hook), (Facing::Left, point(520.0 - MUZZLE_FORWARD * 40.0, 480.0 - MUZZLE_HEIGHT * 48.0), point(494.0 - 20.0 - HOOK_INSET, 379.0)));
    assert!((mirrored.length - shot.length).abs() < 5e-10);
    assert_eq!((from(400.0).hook, from(400.0).facing, from(400.0).reel), (point(400.0, 379.0), Facing::Right, Reeling::Zip));
    assert_eq!([from(440.0).hook, from(331.0).hook, from(329.0).hook], [point(440.0, 379.0), point(330.0, 379.0), point(330.0, 379.0)]);
    assert_eq!(from(331.0).facing, Facing::Left);
    let stub = [perch("stub", 380.0, 420.0, 380.0)];
    assert_eq!(shot_for(point(400.0, 480.0), &stub, &[], PET).map(|shot| shot.hook), Some(point(400.0, 379.0)));
    assert_eq!(shot_for(point(300.0, 480.0), &stub, &[], PET).map(|shot| shot.hook), Some(point(400.0, 379.0)));
    let (steep, slanted) = (from(310.0), from(280.0));
    assert!((steep.hook.x - steep.muzzle.x).abs() <= ZIP_SLANT * (steep.muzzle.y - steep.hook.y) && steep.reel == Reeling::Zip);
    assert!((slanted.hook.x - slanted.muzzle.x).abs() > ZIP_SLANT * (slanted.muzzle.y - slanted.hook.y) && slanted.reel == Reeling::Swing);
}

#[test]
fn a_shot_needs_a_perch_high_enough_a_rope_of_the_right_length_and_steep_enough_and_a_clear_line_but_for_what_lies_under_its_hook() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let boxed = solid(300.0, 500.0, 380.0, 420.0);
    let high = |y: f64, size: Size| shot_for(point(400.0, 480.0), &[perch("stub", 380.0, 420.0, y)], &[], size);
    assert!(high(480.0 - ROPE_RISE * 48.0 + 0.5, PET).is_none());
    assert!(high(413.0, PET).is_some_and(|shot| shot.length >= ROPE_SHORT * 48.0) && high(414.0, PET).is_none());
    assert!(high(297.0, PET).is_some_and(|shot| shot.length <= ROPE_LONG * 48.0) && high(296.0, PET).is_none());
    assert!(high(480.0 - MUZZLE_HEIGHT * 56.0 - 170.0, LARGE).is_some() && high(480.0 - MUZZLE_HEIGHT * 56.0 - 170.0, PET).is_none());
    assert!(high(480.0, PET).is_none() && high(520.0, PET).is_none());
    let from = |x: f64, keepouts: &[Rect]| shot_for(point(x, 480.0), &[floor(), shelf.clone()], keepouts, PET);
    assert!(from(240.0, &[boxed]).is_some() && from(140.0, &[boxed]).is_none());
    assert!(shot_for(point(100.0, 420.0), &[perch("shelf", 180.0, 494.0, 380.0)], &[], PET).is_none());
    for x in [200.0, 240.0, 280.0, 330.0, 400.0, 470.0, 520.0, 560.0] {
        if let Some(shot) = from(x, &[boxed]) {
            assert!(shot.length >= ROPE_SHORT * 48.0 && shot.length <= ROPE_LONG * 48.0 && shot.muzzle.y - shot.hook.y >= ROPE_ELEVATION * shot.length, "{x}");
        }
    }
    let clear = from(280.0, &[boxed]).unwrap_or_else(|| panic!("no shot"));
    assert!(segment_hits(clear.muzzle, clear.hook, &boxed, ROPE_MARGIN));
    assert_eq!(from(280.0, &[boxed, keepout(310.0, 380.0, 100.0, 30.0)]).as_ref(), Some(&clear));
    assert_eq!(from(280.0, &[boxed, keepout(310.0, 384.0, 100.0, 30.0)]), None);
    assert_eq!(from(280.0, &[boxed, keepout(300.0, 420.0, 16.0, 30.0)]), None);
    assert_eq!(from(280.0, &[boxed, keepout(332.0, 340.0, 100.0, 80.0)]).as_ref(), Some(&clear));
    assert_eq!(from(280.0, &[boxed, keepout(331.5, 340.0, 100.0, 80.0)]), None);
    let reach = along(clear.muzzle, clear.hook, 422.0);
    assert_eq!(from(280.0, &[boxed, keepout(reach - ROPE_MARGIN - 0.5 - 20.0, 400.0, 20.0, 20.0)]).as_ref(), Some(&clear));
    assert_eq!(from(280.0, &[boxed, keepout(reach - ROPE_MARGIN + 0.5 - 20.0, 400.0, 20.0, 20.0)]), None);
}

#[test]
fn a_shot_chooses_the_least_rope_and_detour_among_all_perches_the_first_one_among_equals() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let upper = perch("upper", 306.0, 494.0, 350.0);
    let twin = perch("twin", 306.0, 494.0, 380.0);
    let boxed = [solid(300.0, 500.0, 380.0, 420.0)];
    let surface = |x: f64, perches: &[Perch], keepouts: &[Rect]| shot_for(point(x, 480.0), perches, keepouts, PET).map(|shot| shot.surface);
    assert_eq!(surface(400.0, &[floor(), upper.clone(), shelf.clone()], &[]).as_deref(), Some("shelf"));
    assert_eq!(surface(400.0, &[floor(), shelf.clone(), upper.clone()], &[]).as_deref(), Some("shelf"));
    assert_eq!(surface(400.0, std::slice::from_ref(&upper), &[]).as_deref(), Some("upper"));
    assert_eq!(surface(400.0, &[upper.clone(), shelf.clone()], &boxed).as_deref(), Some("shelf"));
    assert_eq!(surface(400.0, &[upper], &boxed), None);
    assert_eq!(surface(400.0, &[twin.clone(), shelf.clone()], &[]).as_deref(), Some("twin"));
    assert_eq!(surface(400.0, &[shelf, twin], &[]).as_deref(), Some("shelf"));
    let (near, far) = (perch("left", 306.0, 400.0, 380.0), perch("right", 560.0, 654.0, 380.0));
    assert_eq!(surface(480.0, &[near.clone(), far.clone()], &[]).as_deref(), Some("left"));
    assert_eq!(surface(480.0, &[far.clone(), near.clone()], &[]).as_deref(), Some("right"));
    assert_eq!(surface(481.0, &[near, far], &[]).as_deref(), Some("right"));
    assert_eq!(surface(400.0, &[], &[]), None);
    assert_eq!(surface(400.0, &[floor()], &[]), None);
}

#[test]
fn a_shot_keeps_every_law_on_generated_stages() {
    let mut next = stream(43);
    let (mut granted, mut refused, mut swings) = (0, 0, 0);
    for stage in 0..STAGES {
        let size = [PET, SMALL, LARGE][next(3) as usize];
        let feet = point(100.0 + next(1600) / 4.0, 300.0 + next(800) / 4.0);
        let mut perches = Vec::new();
        for index in 0..1 + next(3) as usize {
            let x0 = next(2000) / 4.0;
            perches.push(perch(&format!("p{index}"), x0, x0 + 20.0 + next(800) / 4.0, 100.0 + next(1400) / 4.0));
        }
        let mut keepouts: Vec<Rect> = perches.iter().map(|edge| solid(edge.x0, edge.x1, edge.y, edge.y + 40.0)).collect();
        for _ in 0..next(3) as usize {
            keepouts.push(keepout(next(2400) / 4.0, 100.0 + next(1600) / 4.0, next(240) / 4.0, next(240) / 4.0));
        }
        let Some(shot) = shot_for(feet, &perches, &keepouts, size) else {
            refused += 1;
            continue;
        };
        granted += 1;
        if shot.reel == Reeling::Swing {
            swings += 1;
        }
        let target = perches.iter().find(|edge| edge.surface == shot.surface).unwrap_or_else(|| panic!("stage {stage}: no target"));
        let inset = size.width / 2.0 + HOOK_INSET;
        assert!(feet.y - target.y >= ROPE_RISE * size.height && shot.hook.y == target.y - HOOK_LIFT, "stage {stage}");
        if target.x1 - target.x0 >= 2.0 * inset {
            assert!(shot.hook.x >= target.x0 + inset && shot.hook.x <= target.x1 - inset, "stage {stage}");
        } else {
            assert_eq!(shot.hook.x, (target.x0 + target.x1) / 2.0, "stage {stage}");
        }
        assert_eq!(shot.muzzle, point(feet.x + shot.facing.sign() * MUZZLE_FORWARD * size.width, feet.y - MUZZLE_HEIGHT * size.height), "stage {stage}");
        assert_eq!(shot.facing, if shot.hook.x < feet.x { Facing::Left } else { Facing::Right }, "stage {stage}");
        assert!((shot.length - distance(shot.hook, shot.muzzle)).abs() < 1e-9, "stage {stage}");
        assert!(shot.length >= ROPE_SHORT * size.height && shot.length <= ROPE_LONG * size.height && shot.muzzle.y - shot.hook.y >= ROPE_ELEVATION * shot.length, "stage {stage}");
        assert_eq!(shot.reel, if (shot.hook.x - shot.muzzle.x).abs() <= ZIP_SLANT * (shot.muzzle.y - shot.hook.y) { Reeling::Zip } else { Reeling::Swing }, "stage {stage}");
        for blocker in &keepouts {
            if !holding(blocker, point(shot.hook.x, target.y)) {
                assert!(!segment_hits(shot.muzzle, shot.hook, blocker, ROPE_MARGIN), "stage {stage}");
            }
        }
        assert_eq!(shot_holds(&shot, &perches, &keepouts).as_ref(), Some(&shot), "stage {stage}");
    }
    assert!(granted > STAGES / 12 && swings > STAGES / 60 && refused > STAGES / 4, "{granted} granted, {swings} swings, {refused} refused");
}

#[test]
fn the_hook_flies_on_a_straight_line_at_a_constant_speed_ends_on_its_target_exactly_and_comes_back_twice_as_fast() {
    let (muzzle, hook) = (point(292.0, 448.8), point(330.0, 379.0));
    let length = 38.0_f64.hypot(69.8);
    let ticks = hook_ticks(muzzle, hook, HOOK_SPEED);
    assert_eq!(ticks, (length / 10.0).ceil() as Ticks);
    assert_eq!([hook_step(muzzle, hook, HOOK_SPEED, 0), hook_step(muzzle, hook, HOOK_SPEED, -2)], [muzzle, muzzle]);
    for tick in 1..ticks {
        let tip = hook_step(muzzle, hook, HOOK_SPEED, tick);
        assert!((distance(tip, muzzle) - 10.0 * tick as f64).abs() < 1e-9, "tick {tick}");
        assert!(((tip.x - muzzle.x) * (hook.y - muzzle.y) - (tip.y - muzzle.y) * (hook.x - muzzle.x)).abs() < 1e-9, "tick {tick}");
    }
    assert_eq!([hook_step(muzzle, hook, HOOK_SPEED, ticks), hook_step(muzzle, hook, HOOK_SPEED, ticks + 5)], [hook, hook]);
    let tip = hook_step(muzzle, hook, HOOK_SPEED, 5);
    let back = hook_ticks(tip, muzzle, HOOK_RETURN);
    assert_eq!(back, 3);
    assert!((distance(hook_step(tip, muzzle, HOOK_RETURN, 1), tip) - 20.0).abs() < 5e-10);
    assert_eq!(hook_step(tip, muzzle, HOOK_RETURN, back), muzzle);
    assert_eq!(hook_ticks(muzzle, muzzle, HOOK_SPEED), 0);
    assert_eq!([hook_step(muzzle, muzzle, HOOK_SPEED, 0), hook_step(muzzle, muzzle, HOOK_SPEED, 1)], [muzzle, muzzle]);
    assert_eq!([hook_ticks(point(0.0, 0.0), point(6.0, 8.0), HOOK_SPEED), hook_ticks(point(0.0, 0.0), point(6.0, 8.5), HOOK_SPEED), hook_ticks(point(0.0, 0.0), point(60.0, 80.0), HOOK_SPEED)], [1, 2, 10]);
    assert_eq!(hook_step(point(0.0, 0.0), point(60.0, 80.0), HOOK_SPEED, 9), point(54.0, 72.0));
}

#[test]
fn the_haul_begins_at_rest_where_the_actor_stood_and_zips_at_the_gathered_speed_along_the_taut_line_to_where_the_hoist_begins() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let shot = shot_for(point(400.0, 520.0), std::slice::from_ref(&shelf), &[], PET).unwrap_or_else(|| panic!("no shot"));
    assert_eq!(shot.reel, Reeling::Zip);
    let start = haul_of(&shot, shot.length, PET);
    assert_eq!((start.rope, start.before), (shot.length, start.hand));
    assert!(distance(start.hand, shot.muzzle) < 1e-9 && (start.x - 400.0).abs() < 1e-9 && (start.y - 520.0).abs() < 1e-9);
    let hung = haul_of(&shot, 0.0, PET);
    assert_eq!(hung, Haul { rope: 0.0, hand: shot.hook, before: shot.hook, x: shot.hook.x - MUZZLE_FORWARD * 40.0, y: shot.hook.y + MUZZLE_HEIGHT * 48.0 });
    let half = haul_of(&shot, shot.length / 2.0, PET);
    assert!((half.x - (start.x + hung.x) / 2.0).abs() < 1e-9 && (half.y - (start.y + hung.y) / 2.0).abs() < 1e-9);
    let mirrored = shot_for(point(420.0, 520.0), &[perch("stub", 380.0, 420.0, 380.0)], &[], PET).unwrap_or_else(|| panic!("no mirrored shot"));
    assert_eq!(mirrored.facing, Facing::Left);
    assert_eq!(haul_of(&mirrored, 0.0, PET).x, mirrored.hook.x + MUZZLE_FORWARD * 40.0);
    let end = REEL_LEAST * 48.0;
    let (mut haul, mut ticks) = (start, 0);
    while haul.rope > end && ticks < 500 {
        let next = zip_step(&shot, haul, ticks, PET);
        assert!((haul.rope - next.rope - ((ZIP_SPEED * smoothstep((ticks + 1) as f64 / ZIP_RAMP as f64)) / 64.0).min(haul.rope - end)).abs() < 1e-9, "tick {ticks}");
        assert_eq!(next, Haul { before: haul.hand, ..haul_of(&shot, next.rope, PET) });
        assert_eq!(haul_step(&shot, haul, ticks, PET), next);
        haul = next;
        ticks += 1;
    }
    assert_eq!(haul.rope, end);
    assert_eq!(ticks, haul_ticks(&shot, PET));
    assert_eq!(ticks, ZIP_RAMP + ((shot.length - end - (5.5 * ZIP_SPEED) / 64.0) / (ZIP_SPEED / 64.0)).ceil() as Ticks);
    assert_eq!(zip_step(&shot, haul, ticks, PET).rope, end);
    let short = shot_for(point(400.0, 450.2), std::slice::from_ref(&shelf), &[], PET).unwrap_or_else(|| panic!("no short shot"));
    assert!(short.length < REEL_LEAST * 48.0);
    assert_eq!(haul_ticks(&short, PET), 0);
    assert_eq!(zip_step(&short, haul_of(&short, short.length, PET), 0, PET).rope, short.length);
    assert_eq!([zip_step(&shot, haul_of(&shot, 20.0, PET), 30, PET).rope, sway_step(&shot, haul_of(&shot, 20.0, PET), 30, PET).rope], [20.0, 20.0]);
}

#[test]
fn the_haul_swings_on_a_slanted_rope_as_the_pendulum_of_the_swing_module_the_feet_under_the_hands() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let slanted = shot_for(point(250.0, 480.0), &[floor(), shelf], &[], PET).unwrap_or_else(|| panic!("no shot"));
    let least = REEL_LEAST * 48.0;
    assert_eq!(slanted.reel, Reeling::Swing);
    let mut haul = haul_of(&slanted, slanted.length, PET);
    let (mut ticks, mut lowest, mut crossed) = (0, haul.hand.y, false);
    while haul.rope > least && ticks < 500 {
        let reel = reel_step(slanted.hook, haul.hand, haul.before, haul.rope, least, ticks);
        let next = sway_step(&slanted, haul, ticks, PET);
        assert_eq!(next, Haul { rope: reel.length, hand: reel.bob, before: haul.hand, x: reel.bob.x - MUZZLE_FORWARD * 40.0, y: reel.bob.y + MUZZLE_HEIGHT * 48.0 }, "tick {ticks}");
        assert_eq!(haul_step(&slanted, haul, ticks, PET), next);
        assert!(distance(next.hand, slanted.hook) < next.rope + 1e-9 && distance(next.hand, haul.hand) < REEL_CAP / 64.0 + 1e-9, "tick {ticks}");
        assert!((haul.rope - next.rope - ((REEL_SPEED * ((ticks + 1).min(REEL_RAMP)) as f64) / REEL_RAMP as f64 / 64.0).min(haul.rope - least)).abs() < 5e-10, "tick {ticks}");
        lowest = lowest.max(next.hand.y);
        crossed = crossed || next.hand.x > slanted.hook.x;
        haul = next;
        ticks += 1;
    }
    assert_eq!(haul.rope, least);
    assert_eq!(ticks, haul_ticks(&slanted, PET));
    assert_eq!(ticks, REEL_RAMP - 1 + ((slanted.length - least - (4.5 * REEL_SPEED) / 64.0) / (REEL_SPEED / 64.0)).ceil() as Ticks);
    assert!(crossed && lowest > slanted.muzzle.y && lowest < slanted.hook.y + slanted.length);
}

#[test]
fn the_landing_lies_inside_the_hook_towards_the_middle_of_the_perch_and_a_meant_miss_past_its_nearer_end() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let shot = shot_for(point(400.0, 520.0), std::slice::from_ref(&shelf), &[], PET).unwrap_or_else(|| panic!("no shot"));
    let hooked = |x: f64| Shot { hook: point(x, 379.0), ..shot.clone() };
    assert_eq!(landing_for(&shot, &shelf, PET), point(400.0 + 20.0 + MANTLE_INSET, 380.0));
    assert_eq!([landing_for(&hooked(330.0), &shelf, PET), landing_for(&hooked(470.0), &shelf, PET)], [point(358.0, 380.0), point(442.0, 380.0)]);
    let stub = perch("stub", 380.0, 420.0, 380.0);
    assert_eq!([landing_for(&hooked(400.0), &stub, PET), landing_for(&hooked(401.0), &stub, PET)], [point(420.0, 380.0), point(380.0, 380.0)]);
    assert_eq!(miss_of(&hooked(330.0), &shelf), point(306.0 - ROPE_MISS_OVERSHOOT, 379.0));
    assert_eq!(miss_of(&hooked(470.0), &shelf), point(494.0 + ROPE_MISS_OVERSHOOT, 379.0));
    assert_eq!(miss_of(&hooked(400.0), &shelf), point(292.0, 379.0));
}

#[test]
fn a_shot_holds_as_it_is_follows_its_edge_up_and_down_and_loses_an_edge_that_moved_too_far_left_the_hook_vanished_or_was_blocked() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let boxed = solid(300.0, 500.0, 380.0, 420.0);
    let shot = shot_for(point(280.0, 480.0), &[floor(), shelf.clone()], &[boxed], PET).unwrap_or_else(|| panic!("no shot"));
    assert_eq!(shot_holds(&shot, &[floor(), shelf.clone()], &[boxed]).as_ref(), Some(&shot));
    for dy in [-ROPE_FOLLOW, -3.0, 5.0, ROPE_FOLLOW] {
        let held = shot_holds(&shot, &[floor(), perch("shelf", 306.0, 494.0, 380.0 + dy)], &[solid(300.0, 500.0, 380.0 + dy, 420.0 + dy)]).unwrap_or_else(|| panic!("{dy}"));
        assert_eq!(held.hook, point(330.0, 379.0 + dy));
        assert!((held.length - distance(point(330.0, 379.0 + dy), shot.muzzle)).abs() < 1e-12);
        assert_eq!(Shot { hook: shot.hook, length: shot.length, ..held }, shot);
    }
    let lost = |perches: &[Perch]| shot_holds(&shot, perches, &[]).is_none();
    assert!(lost(&[floor(), perch("shelf", 306.0, 494.0, 380.0 + ROPE_FOLLOW + 0.5)]) && lost(&[floor(), perch("shelf", 306.0, 494.0, 380.0 - ROPE_FOLLOW - 0.5)]));
    assert!(lost(&[floor(), perch("shelf", 330.5, 494.0, 380.0)]) && !lost(&[floor(), perch("shelf", 330.0, 494.0, 380.0)]));
    assert!(lost(&[floor(), perch("other", 306.0, 494.0, 380.0)]) && lost(&[floor()]));
    assert_eq!(shot_holds(&shot, &[floor(), shelf.clone()], &[boxed, keepout(300.0, 420.0, 16.0, 30.0)]), None);
    assert_eq!(shot_holds(&shot, &[floor(), shelf], &[boxed, keepout(100.0, 420.0, 16.0, 30.0)]).as_ref(), Some(&shot));
}

#[test]
fn routes_offer_every_way_the_gear_allows_a_standing_ladder_first_then_the_wall_then_a_ladder_of_its_own_then_the_rope() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let (left, right) = (pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let pitches = [left.clone(), right.clone()];
    let standing = [ladder_for(&floor(), &shelf, &right, &boxed, LARGE).unwrap_or_else(|| panic!("no standing ladder"))];
    let up = |gear: &[Gear], ladders: &[LadderStand], grip: f64, x: f64| route_of(x, &floor(), &shelf, gear, PET, grip, &pitches, ladders, &boxed).map(|legs| means(&legs));
    let every = [Gear::Climb, Gear::Ladder, Gear::Grapple];
    let legs = route_of(200.0, &floor(), &shelf, &every, PET, GRIP_BUDGET, &pitches, &[], &boxed).unwrap_or_else(|| panic!("no legs"));
    assert_eq!(means(&legs), ["wall", "wall", "raise", "raise", "grapple"]);
    assert_eq!(legs[0], Leg::Wall { at: 280.0, pitch: &pitches[0], hold: WallHold { x: 280.0, y: 480.0, over: false }, exit: &pitches[0], goal: rim_of(&left, PET) });
    assert_eq!(legs[1], Leg::Wall { at: 520.0, pitch: &pitches[1], hold: WallHold { x: 520.0, y: 480.0, over: false }, exit: &pitches[1], goal: rim_of(&right, PET) });
    assert_eq!(legs[2], Leg::Raise { at: 300.0 - LADDER_LEAN * 93.0, ladder: ladder_for(&floor(), &shelf, &left, &boxed, PET).unwrap_or_else(|| panic!("no ladder")) });
    assert_eq!(legs[4], Leg::Grapple { at: 200.0, shot: shot_for(point(200.0, 480.0), std::slice::from_ref(&shelf), &boxed, PET).unwrap_or_else(|| panic!("no shot")) });
    let joined = route_of(200.0, &floor(), &shelf, &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed).unwrap_or_else(|| panic!("no legs"));
    assert_eq!(means(&joined), ["ladder", "wall", "wall", "grapple"]);
    assert!(matches!(joined[0], Leg::Ladder { at, ladder, up: true } if at == standing[0].foot.x && std::ptr::eq(ladder, &standing[0])));
    assert_eq!(up(&[Gear::Climb], &[], GRIP_BUDGET, 200.0), Some(vec!["wall", "wall"]));
    assert_eq!(up(&[Gear::Ladder], &[], GRIP_BUDGET, 200.0), Some(vec!["raise", "raise"]));
    assert_eq!(up(&[Gear::Grapple], &[], GRIP_BUDGET, 200.0), Some(vec!["grapple"]));
    assert_eq!(up(&[Gear::Grapple], &standing, GRIP_BUDGET, 200.0), Some(vec!["ladder", "grapple"]));
    assert_eq!([up(&[Gear::Parachute], &standing, GRIP_BUDGET, 200.0), up(&[], &standing, GRIP_BUDGET, 200.0), up(&[Gear::Parachute], &[], GRIP_BUDGET, 200.0)], [None, None, None]);
}

#[test]
fn routes_shoot_from_where_the_actor_stands_when_that_works_else_from_the_point_of_its_perch_nearest_to_the_target_and_refuse_a_wall_the_grip_does_not_last_for() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let (left, right) = (pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let pitches = [left.clone(), right];
    let shot = |x: f64, from: &Perch| {
        route_of(x, from, &shelf, &[Gear::Grapple], PET, GRIP_BUDGET, &[], &[], &boxed).map(|legs| match &legs[0] {
            Leg::Grapple { at, .. } => *at,
            _ => f64::NAN,
        })
    };
    assert_eq!(
        route_of(400.0, &floor(), &shelf, &[Gear::Grapple], PET, GRIP_BUDGET, &pitches, &[], &boxed).map(|legs| legs[0].clone()),
        Some(Leg::Grapple { at: 400.0, shot: shot_for(point(400.0, 480.0), std::slice::from_ref(&shelf), &boxed, PET).unwrap_or_else(|| panic!("no shot")) })
    );
    assert_eq!(shot(100.0, &floor()), Some(306.0));
    assert_eq!(shot(100.0, &perch("floor", 0.0, 150.0, 480.0)), None);
    assert_eq!([shot(100.0, &perch("floor", 0.0, 250.0, 480.0)), shot(600.0, &perch("floor", 540.0, 640.0, 480.0)), shot(630.0, &perch("floor", 540.0, 640.0, 480.0))], [Some(250.0), Some(600.0), Some(540.0)]);
    let need = (WALL_GRAB_TICKS + climb_ticks(480.0, rim_of(&left, PET)) + MANTLE_TICKS) as f64;
    let climbs = |gear: &[Gear], grip: f64| route_of(200.0, &floor(), &shelf, gear, PET, grip, &pitches, &[], &boxed).map(|legs| means(&legs));
    assert_eq!(climbs(&[Gear::Climb], need), Some(vec!["wall", "wall"]));
    assert_eq!(climbs(&[Gear::Climb], need - 1.0), None);
    assert_eq!(climbs(&[Gear::Climb, Gear::Grapple], 10.0), Some(vec!["grapple"]));
}

#[test]
fn routes_lead_down_over_the_rim_by_the_wall_or_a_standing_ladder_never_by_a_rope_or_a_new_ladder_and_find_nothing_where_nothing_joins() {
    let shelf = perch("shelf", 306.0, 494.0, 380.0);
    let (left, right) = (pitch("shelf-left", "shelf", Facing::Left, 300.0, 380.0, 480.0), pitch("shelf-right", "shelf", Facing::Right, 500.0, 380.0, 480.0));
    let boxed = [solid(300.0, 500.0, 380.0, 480.0)];
    let pitches = [left.clone(), right.clone()];
    let standing = [ladder_for(&floor(), &shelf, &right, &boxed, LARGE).unwrap_or_else(|| panic!("no standing ladder"))];
    let every = [Gear::Climb, Gear::Ladder, Gear::Grapple];
    let legs = route_of(400.0, &shelf, &floor(), &every, PET, GRIP_BUDGET, &pitches, &[], &boxed).unwrap_or_else(|| panic!("no legs"));
    assert_eq!(
        legs,
        [
            Leg::Wall { at: 328.0, pitch: &pitches[0], hold: WallHold { x: 280.0, y: rim_of(&left, PET), over: true }, exit: &pitches[0], goal: 480.0 },
            Leg::Wall { at: 472.0, pitch: &pitches[1], hold: WallHold { x: 520.0, y: rim_of(&right, PET), over: true }, exit: &pitches[1], goal: 480.0 }
        ]
    );
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Ladder, Gear::Grapple], PET, GRIP_BUDGET, &pitches, &[], &boxed), None);
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Ladder], PET, GRIP_BUDGET, &pitches, &standing, &boxed), Some(vec![Leg::Ladder { at: ledge_of(&right, PET), ladder: &standing[0], up: false }]));
    let need = (WALL_HANG_TICKS + climb_ticks(rim_of(&left, PET), 480.0)) as f64;
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Climb], PET, need, &pitches, &[], &boxed).map(|legs| legs.len()), Some(2));
    assert_eq!(route_of(400.0, &shelf, &floor(), &[Gear::Climb], PET, need - 1.0, &pitches, &[], &boxed), None);
    let far = perch("far", 20.0, 120.0, 200.0);
    assert_eq!(route_of(200.0, &floor(), &far, &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed), None);
    assert_eq!(route_of(400.0, &shelf, &shelf, &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed), None);
    assert_eq!(route_of(200.0, &floor(), &floor(), &every, PET, GRIP_BUDGET, &pitches, &standing, &boxed), None);
    assert_eq!(route_of(200.0, &floor(), &shelf, &[Gear::Climb, Gear::Ladder], PET, GRIP_BUDGET, &[], &[], &boxed), None);
}

fn column() -> [Pitch; 2] {
    [pitch("top-left", "top", Facing::Left, 300.0, 300.0, 380.0), pitch("low-left", "low", Facing::Left, 300.0, 410.0, 480.0)]
}

#[test]
fn a_lunge_crosses_between_two_pitches_of_one_wall_line_one_above_the_other_within_its_reach() {
    let [top, low] = column();
    assert!(crossable(&low, &top, PET) && crossable(&top, &low, PET));
    assert!(!crossable(&top, &top, PET));
    assert!(!crossable(&low, &pitch("top-right", "top", Facing::Right, 300.0, 300.0, 380.0), PET));
    assert!(crossable(&low, &pitch("top-left", "top", Facing::Left, 300.0 + WALL_FOLLOW, 300.0, 380.0), PET));
    assert!(!crossable(&low, &pitch("top-left", "top", Facing::Left, 300.0 + WALL_FOLLOW + 0.5, 300.0, 380.0), PET));
    assert!(!crossable(&low, &pitch("top-left", "top", Facing::Left, 300.0, 300.0, 411.0), PET));
    let reach = CROSS_REACH * 48.0 - GRIP_BITE;
    assert!(crossable(&pitch("low-left", "low", Facing::Left, 300.0, 380.0 + reach, 480.0), &top, PET));
    assert!(!crossable(&pitch("low-left", "low", Facing::Left, 300.0, 380.0 + reach + 0.5, 480.0), &top, PET));
    assert!(!crossable(&low, &pitch("stub", "top", Facing::Left, 300.0, 372.5, 380.0), PET));
    assert!(crossable(&low, &pitch("stub", "top", Facing::Left, 300.0, 372.0, 380.0), PET));
}

#[test]
fn a_lunge_leads_from_the_nearest_hold_of_one_pitch_to_the_nearest_hold_of_the_other() {
    let [top, low] = column();
    assert_eq!([lunge_path(&low, &top, PET, 0.0), lunge_path(&low, &top, PET, 1.0)], [point(280.0, rim_of(&low, PET)), point(280.0, foot_of(&top, PET))]);
    assert_eq!([lunge_path(&top, &low, PET, 0.0), lunge_path(&top, &low, PET, 1.0)], [point(280.0, foot_of(&top, PET)), point(280.0, rim_of(&low, PET))]);
    for step in 0..=LUNGE_TICKS {
        let phase = step as f64 / LUNGE_TICKS as f64;
        assert_eq!(lunge_path(&low, &top, PET, phase), hoist_path(point(280.0, rim_of(&low, PET)), point(280.0, foot_of(&top, PET)), 48.0, phase));
    }
}

#[test]
fn a_wall_line_holds_every_pitch_reached_by_lunges_from_the_top_down_the_nearest_first_among_equals() {
    let [top, low] = column();
    let far = pitch("far-left", "far", Facing::Left, 300.0, 100.0, 200.0);
    let twin = pitch("twin-left", "twin", Facing::Left, 302.0, 300.0, 380.0);
    let pitches = [far, low.clone(), top.clone(), twin];
    let names = |chain: Vec<&Pitch>| chain.iter().map(|member| member.wall.clone()).collect::<Vec<_>>();
    assert_eq!(names(chain_of(&pitches[1], &pitches, PET)), ["top-left", "low-left"]);
    assert!(std::ptr::eq(chain_of(&pitches[1], &pitches, PET)[0], &pitches[2]));
    assert_eq!(names(chain_of(&pitches[2], &pitches, PET)), ["top-left", "low-left"]);
    assert_eq!(names(chain_of(&pitches[3], &pitches, PET)), ["twin-left", "low-left"]);
    assert_eq!(names(chain_of(&pitches[0], &pitches, PET)), ["far-left"]);
    let higher = pitch("higher-left", "top", Facing::Left, 300.0, 300.0, 370.0);
    let ordered = [higher, top, low.clone()];
    assert_eq!(names(chain_of(&ordered[2], &ordered, PET)), ["top-left", "low-left"]);
    assert_eq!(names(chain_of(&low, &[], PET)), ["low-left"]);
}

#[test]
fn a_way_along_a_wall_line_takes_hold_climbs_lunges_and_mantles_tick_by_tick_and_costs_every_tick() {
    let pitches = column();
    let chain: Vec<&Pitch> = pitches.iter().collect();
    let goal = rim_of(&pitches[0], PET);
    let path = wall_path(&chain, 1, Entry::Grab, 270.0, 480.0, 0, goal, true, PET);
    let works: Vec<Handwork> = path.iter().map(|clamber| clamber.work).collect();
    let count = |work: Handwork| works.iter().filter(|&&each| each == work).count() as Ticks;
    assert_eq!([count(Handwork::Grab), count(Handwork::Lunge), count(Handwork::Mantle), count(Handwork::Hang)], [WALL_GRAB_TICKS, LUNGE_TICKS, MANTLE_TICKS, 0]);
    assert_eq!(path[WALL_GRAB_TICKS as usize - 1], Clamber { x: 280.0, y: 480.0, hold: 1, work: Handwork::Grab });
    assert!(path[..WALL_GRAB_TICKS as usize].windows(2).all(|pair| pair[0].x <= pair[1].x));
    let climbed: Vec<f64> = path.iter().filter(|clamber| clamber.work == Handwork::Climb && clamber.hold == 1).map(|clamber| clamber.y).collect();
    assert_eq!(climbed, climbed_to(480.0, rim_of(&pitches[1], PET)));
    let lunged = path.iter().position(|clamber| clamber.work == Handwork::Lunge).unwrap_or_default();
    assert_eq!(path[lunged + LUNGE_TICKS as usize - 1], Clamber { x: 280.0, y: foot_of(&pitches[0], PET), hold: 0, work: Handwork::Lunge });
    let upper: Vec<f64> = path.iter().filter(|clamber| clamber.work == Handwork::Climb && clamber.hold == 0).map(|clamber| clamber.y).collect();
    assert_eq!(upper, climbed_to(foot_of(&pitches[0], PET), goal));
    assert_eq!(path.last().copied(), Some(Clamber { x: ledge_of(&pitches[0], PET), y: 300.0, hold: 0, work: Handwork::Mantle }));
    assert_eq!(wall_cost(&path), path.len() as f64 * GRIP_CLIMB);
    let single = [left()];
    let alone: Vec<&Pitch> = single.iter().collect();
    let up = wall_path(&alone, 0, Entry::Grab, 280.0, 480.0, 0, rim_of(&single[0], PET), true, PET);
    assert_eq!(wall_cost(&up), (WALL_GRAB_TICKS + climb_ticks(480.0, rim_of(&single[0], PET)) + MANTLE_TICKS) as f64 * GRIP_CLIMB);
    let down = wall_path(&alone, 0, Entry::Hang, 0.0, 0.0, 0, 480.0, false, PET);
    assert_eq!(wall_cost(&down), (WALL_HANG_TICKS + climb_ticks(rim_of(&single[0], PET), 480.0)) as f64 * GRIP_CLIMB);
    assert_eq!(down[0], Clamber { x: mantle_path(&single[0], PET, 0.9).x, y: mantle_path(&single[0], PET, 0.9).y, hold: 0, work: Handwork::Hang });
    assert_eq!(down[WALL_HANG_TICKS as usize - 1], Clamber { x: 280.0, y: rim_of(&single[0], PET), hold: 0, work: Handwork::Hang });
    let still = wall_path(&alone, 0, Entry::Cling, 0.0, 400.0, 0, 400.0, false, PET);
    assert!(still.is_empty());
}

fn climbed_to(from: f64, goal: f64) -> Vec<f64> {
    let mut heights = Vec::new();
    let mut height = from;
    let mut tick = 0;
    while height != goal {
        height = climb_step(height, goal, tick);
        heights.push(height);
        tick += 1;
    }
    heights
}

#[test]
fn routes_climb_a_wall_line_up_and_down_and_name_the_pitch_their_climb_ends_on() {
    let pitches = column();
    let (floor, top, low) = (floor(), perch("top", 306.0, 494.0, 300.0), perch("low", 306.0, 494.0, 410.0));
    let up = route_of(200.0, &floor, &top, &[Gear::Climb], PET, GRIP_BUDGET, &pitches, &[], &[]).unwrap_or_else(|| panic!("no way up"));
    assert_eq!(up, [Leg::Wall { at: 280.0, pitch: &pitches[1], hold: WallHold { x: 280.0, y: 480.0, over: false }, exit: &pitches[0], goal: rim_of(&pitches[0], PET) }]);
    let down = route_of(400.0, &top, &floor, &[Gear::Climb], PET, GRIP_BUDGET, &pitches, &[], &[]).unwrap_or_else(|| panic!("no way down"));
    assert_eq!(down, [Leg::Wall { at: ledge_of(&pitches[0], PET), pitch: &pitches[0], hold: WallHold { x: 280.0, y: rim_of(&pitches[0], PET), over: true }, exit: &pitches[1], goal: 480.0 }]);
    let chain: Vec<&Pitch> = pitches.iter().collect();
    let need = wall_cost(&wall_path(&chain, 1, Entry::Grab, 280.0, 480.0, 0, rim_of(&pitches[0], PET), true, PET));
    assert!(route_of(200.0, &floor, &top, &[Gear::Climb], PET, need, &pitches, &[], &[]).is_some());
    assert!(route_of(200.0, &floor, &top, &[Gear::Climb], PET, need - 1.0, &pitches, &[], &[]).is_none());
    assert_eq!(route_of(200.0, &floor, &low, &[Gear::Climb], PET, GRIP_BUDGET, &pitches, &[], &[]).map(|legs| legs.len()), Some(1));
}

#[test]
fn the_constants_are_the_starting_values_of_the_research_brief() {
    assert_eq!([CROSS_REACH, LUNGE_TICKS as f64], [1.5, 16.0]);
    assert_eq!([CLIMB_RISE, CLIMB_DESCENT, GRIP_BUDGET, SLIDE_START, SLIDE_GAIN, SLIDE_SPEED, SLIP_PUSH, SLIP_LIFT], [30.0, 40.0, 384.0, 30.0, 600.0, 160.0, 140.0, 160.0]);
    assert_eq!([LADDER_LEAN, LADDER_STEEP, LADDER_FLAT, LADDER_SHORT, LADDER_TALL, LADDER_RISE, LADDER_DESCENT], [0.25, 0.14, 0.4, 0.8, 3.6, 26.0, 34.0]);
    assert_eq!([HOOK_SPEED, HOOK_RETURN, ROPE_SHORT, ROPE_LONG, ROPE_ELEVATION, ZIP_SLANT, ZIP_SPEED], [640.0, 1280.0, 0.8, 3.2, 0.42, 0.35, 150.0]);
    assert_eq!([REEL_LEAST, LADDER_FOLLOW, LADDER_SHIFT, ROPE_DETOUR], [0.9, 8.0, 12.0, 0.3]);
    assert_eq!([MANTLE_TICKS, CLIMB_RAMP], [28, 6]);
}

#[test]
fn committed_wall_climbing_holds_surveys_climbs_grips_slides_mantles_and_routes_are_reproduced() {
    let vectors = fixture("wall-climbing");
    let constants = json!({ "wallLip": crate::terrain::WALL_LIP, "handHeight": HAND_HEIGHT, "climbRise": CLIMB_RISE, "climbDescent": CLIMB_DESCENT, "climbRamp": CLIMB_RAMP, "gripSpacing": GRIP_SPACING, "gripBudget": GRIP_BUDGET, "gripClimb": GRIP_CLIMB, "gripHang": GRIP_HANG, "gripRest": GRIP_REST, "gripBite": GRIP_BITE, "wallFollow": WALL_FOLLOW, "slideStart": SLIDE_START, "slideGain": SLIDE_GAIN, "slideSpeed": SLIDE_SPEED, "slipPush": SLIP_PUSH, "slipLift": SLIP_LIFT, "wallGrabTicks": WALL_GRAB_TICKS, "wallHangTicks": WALL_HANG_TICKS, "mantleTicks": MANTLE_TICKS, "mantleInset": MANTLE_INSET, "hoistHump": HOIST_HUMP, "hoistRise": HOIST_RISE, "crossReach": CROSS_REACH, "lungeTicks": LUNGE_TICKS });
    assert_close("constants", &constants, &vectors["constants"]);
    for vector in entries(&vectors["crossings"]) {
        let (size, pitches): (Size, Vec<Pitch>) = (typed(&vector["size"]), typed(&vector["pitches"]));
        let crossings: Vec<Vec<bool>> = pitches.iter().map(|from| pitches.iter().map(|to| crossable(from, to, size)).collect()).collect();
        let chains: Vec<Vec<Value>> = pitches.iter().map(|held| chain_of(held, &pitches, size).into_iter().map(|member| located(&pitches, Some(member))).collect()).collect();
        let mut ways = Vec::new();
        for wish in entries(&vector["ways"]) {
            let held = &pitches[typed::<usize>(&wish["pitch"])];
            let chain = chain_of(held, &pitches, size);
            let start = chain.iter().position(|member| std::ptr::eq(*member, held)).unwrap_or_default();
            let path = wall_path(&chain, start, typed(&wish["entry"]), number(&wish["x"]), number(&wish["y"]), typed(&wish["end"]), number(&wish["goal"]), wish["mantle"] == true, size);
            ways.push(json!({ "path": path.iter().map(|clamber| json!([clamber.x, clamber.y, clamber.hold, clamber.work])).collect::<Vec<_>>(), "cost": wall_cost(&path) }));
        }
        assert_close(&format!("crossing {}", vector["id"]), &json!({ "crossable": crossings, "chains": chains, "ways": ways }), &vector["expected"]);
    }
    for stage in group(&vectors, "holds") {
        let (size, perches, pitches): (Size, Vec<Perch>, Vec<Pitch>) = (typed(&stage["size"]), typed(&stage["perches"]), typed(&stage["pitches"]));
        let grips: Vec<Vec<Option<WallHold>>> = perches.iter().map(|perch| pitches.iter().map(|pitch| grip_for(perch, pitch, size)).collect()).collect();
        let rims: Vec<Value> = pitches.iter().map(|pitch| located(&perches, rim_for(pitch, &perches, size))).collect();
        let measures: Vec<Value> = pitches.iter().map(|pitch| json!({ "cling": cling_of(pitch, size), "ledge": ledge_of(pitch, size), "rim": rim_of(pitch, size), "foot": foot_of(pitch, size) })).collect();
        assert_close(&format!("hold {}", stage["id"]), &json!({ "grips": grips, "rims": rims, "measures": measures }), &stage["expected"]);
    }
    for vector in group(&vectors, "surveys") {
        let (size, held, pitches): (Size, Pitch, Vec<Pitch>) = (typed(&vector["size"]), typed(&vector["pitch"]), typed(&vector["pitches"]));
        let found = wall_holds(&held, &pitches, number(&vector["y"]), size);
        assert_close(&format!("survey {}", vector["id"]), &json!({ "pitch": located(&pitches, found), "slip": found.map_or_else(|| json(&slip_of(&held)), |_| Value::Null) }), &vector["expected"]);
    }
    for vector in group(&vectors, "climbs") {
        let (size, held, from, goal): (Size, Pitch, f64, f64) = (typed(&vector["size"]), typed(&vector["pitch"]), number(&vector["from"]), number(&vector["goal"]));
        let ticks = climb_ticks(from, goal);
        let mut heights = Vec::new();
        let mut height = from;
        for tick in 0..ticks {
            if (ticks as f64) > GRIP_BUDGET {
                break;
            }
            height = climb_step(height, goal, tick);
            heights.push(height);
        }
        let phases: Vec<f64> = heights.iter().map(|&reached| climb_phase(&held, reached, size)).collect();
        assert_close(&format!("climb {}", vector["id"]), &json!({ "ticks": ticks, "heights": heights, "phases": phases }), &vector["expected"]);
    }
    for vector in group(&vectors, "grips") {
        let mut left = number(&vector["grip"]);
        let mut grips = Vec::new();
        for run in entries(&vector["runs"]) {
            let (effort, ticks): (Effort, usize) = (typed(&run[0]), typed(&run[1]));
            for _ in 0..ticks {
                left = grip_step(left, effort);
            }
            grips.push(left);
        }
        assert_close(&format!("grip {}", vector["id"]), &json!(grips), &vector["expected"]);
    }
    for vector in group(&vectors, "slides") {
        let (mut slide, floor, ticks): (Fall, f64, usize) = (Fall { y: number(&vector["y"]), vy: number(&vector["vy"]) }, number(&vector["floor"]), typed(&vector["ticks"]));
        let (mut heights, mut speeds) = (Vec::new(), Vec::new());
        for _ in 0..ticks {
            slide = slide_step(slide.y, slide.vy, floor);
            heights.push(slide.y);
            speeds.push(slide.vy);
        }
        assert_close(&format!("slide {}", vector["id"]), &json!({ "heights": heights, "speeds": speeds }), &vector["expected"]);
    }
    for vector in group(&vectors, "mantles") {
        let (size, held): (Size, Pitch) = (typed(&vector["size"]), typed(&vector["pitch"]));
        let path: Vec<[f64; 2]> = (0..=MANTLE_TICKS).map(|tick| mantle_path(&held, size, tick as f64 / MANTLE_TICKS as f64)).map(|at| [at.x, at.y]).collect();
        assert_close(&format!("mantle {}", vector["id"]), &json!(path), &vector["expected"]);
    }
    for vector in group(&vectors, "hoists") {
        let (from, to, height, ticks): (Point, Point, f64, Ticks) = (typed(&vector["from"]), typed(&vector["to"]), number(&vector["height"]), typed(&vector["ticks"]));
        let path: Vec<[f64; 2]> = (0..=ticks).map(|tick| hoist_path(from, to, height, tick as f64 / ticks as f64)).map(|at| [at.x, at.y]).collect();
        assert_close(&format!("hoist {}", vector["id"]), &json!(path), &vector["expected"]);
    }
    for vector in group(&vectors, "routes") {
        let (size, perches, pitches): (Size, Vec<Perch>, Vec<Pitch>) = (typed(&vector["size"]), typed(&vector["perches"]), typed(&vector["pitches"]));
        let (from, to): (usize, usize) = (typed(&vector["from"]), typed(&vector["to"]));
        let legs = route_of(number(&vector["x"]), &perches[from], &perches[to], &[Gear::Climb], size, number(&vector["grip"]), &pitches, &[], &[]);
        let projected = legs.map_or(Value::Null, |legs| {
            Value::Array(
                legs.iter()
                    .map(|leg| match leg {
                        Leg::Wall { at, pitch, hold, exit, goal } => json!({ "at": at, "hold": hold, "goal": goal, "pitch": located(&pitches, Some(*pitch)), "exit": located(&pitches, Some(*exit)) }),
                        other => leg_value(other, &pitches, &[]),
                    })
                    .collect(),
            )
        });
        assert_close(&format!("route {}", vector["id"]), &projected, &vector["expected"]);
    }
}

#[test]
fn committed_ladder_geometry_vectors_are_reproduced() {
    let vectors = fixture("ladder-geometry");
    let constants = json!({ "ladderLean": LADDER_LEAN, "ladderSteep": LADDER_STEEP, "ladderFlat": LADDER_FLAT, "ladderShort": LADDER_SHORT, "ladderTall": LADDER_TALL, "ladderTuck": LADDER_TUCK, "ladderHorns": LADDER_HORNS, "rungSpacing": RUNG_SPACING, "ladderFooting": LADDER_FOOTING, "ladderGirth": LADDER_GIRTH, "ladderRise": LADDER_RISE, "ladderDescent": LADDER_DESCENT, "ladderRamp": LADDER_RAMP, "ladderExit": LADDER_EXIT, "ladderFollow": LADDER_FOLLOW, "ladderShift": LADDER_SHIFT, "ladderMountTicks": LADDER_MOUNT_TICKS, "ladderDismountTicks": LADDER_DISMOUNT_TICKS, "ladderRaiseTicks": LADDER_RAISE_TICKS, "ladderIdle": LADDER_IDLE, "ladderLife": LADDER_LIFE, "toppleStiffness": TOPPLE_STIFFNESS, "toppleDamping": TOPPLE_DAMPING, "topplePush": TOPPLE_PUSH, "toppleStep": TOPPLE_STEP });
    assert_close("constants", &constants, &vectors["constants"]);
    for vector in group(&vectors, "placements") {
        let (size, low, high, held, keepouts): (Size, Perch, Perch, Pitch, Vec<Rect>) = (typed(&vector["size"]), typed(&vector["low"]), typed(&vector["high"]), typed(&vector["pitch"]), typed(&vector["keepouts"]));
        let placed = ladder_for(&low, &high, &held, &keepouts, size)
            .map_or(Value::Null, |ladder| json!({ "length": ladder_length(&ladder), "rungs": ladder_rungs(&ladder), "lean": ladder_lean(&ladder), "exit": ladder_exit(&ladder, size), "landing": ladder_landing(&ladder, size), "ladder": ladder }));
        assert_close(&format!("placement {}", vector["id"]), &placed, &vector["expected"]);
    }
    for vector in group(&vectors, "climbs") {
        let (size, ladder): (Size, LadderStand) = (typed(&vector["size"]), typed(&vector["ladder"]));
        let exit = ladder_exit(&ladder, size);
        let goal = if vector["down"] == true { 0.0 } else { exit };
        let mut travel = if vector["down"] == true { exit } else { 0.0 };
        let mut travels = Vec::new();
        while travel != goal && travels.len() < 1024 {
            travel = ladder_step(travel, goal, travels.len() as Ticks);
            travels.push(travel);
        }
        let feet: Vec<[f64; 2]> = travels.iter().map(|&reached| ladder_at(&ladder, reached)).map(|at| [at.x, at.y]).collect();
        let phases: Vec<f64> = travels.iter().map(|&reached| ladder_phase(reached)).collect();
        assert_close(&format!("climb {}", vector["id"]), &json!({ "ticks": travels.len(), "travels": travels, "feet": feet, "phases": phases }), &vector["expected"]);
    }
    for vector in group(&vectors, "surveys") {
        let (size, ladder, perches, pitches, keepouts): (Size, LadderStand, Vec<Perch>, Vec<Pitch>, Vec<Rect>) = (typed(&vector["size"]), typed(&vector["ladder"]), typed(&vector["perches"]), typed(&vector["pitches"]), typed(&vector["keepouts"]));
        assert_close(&format!("survey {}", vector["id"]), &json(&ladder_holds(&ladder, &perches, &pitches, &keepouts, size)), &vector["expected"]);
    }
    for vector in group(&vectors, "spills") {
        let (size, ladder, heights): (Size, LadderStand, Vec<f64>) = (typed(&vector["size"]), typed(&vector["ladder"]), typed(&vector["heights"]));
        let spills: Vec<Option<Toss>> = heights.iter().map(|&height| spill_of(&ladder, height, size)).collect();
        assert_close(&format!("spill {}", vector["id"]), &json(&spills), &vector["expected"]);
    }
}

#[test]
fn committed_grapple_reach_vectors_are_reproduced() {
    let vectors = fixture("grapple-reach");
    let constants = json!({ "hookSpeed": HOOK_SPEED, "hookReturn": HOOK_RETURN, "ropeShort": ROPE_SHORT, "ropeLong": ROPE_LONG, "ropeElevation": ROPE_ELEVATION, "ropeMargin": ROPE_MARGIN, "ropeDetour": ROPE_DETOUR, "ropeFollow": ROPE_FOLLOW, "ropeRise": ROPE_RISE, "muzzleForward": MUZZLE_FORWARD, "muzzleHeight": MUZZLE_HEIGHT, "hookInset": HOOK_INSET, "hookLift": HOOK_LIFT, "zipSlant": ZIP_SLANT, "zipSpeed": ZIP_SPEED, "zipRamp": ZIP_RAMP, "ropeAimTicks": ROPE_AIM_TICKS, "ropeRecoilTicks": ROPE_RECOIL_TICKS, "ropeTugTicks": ROPE_TUG_TICKS, "ropeHoistTicks": ROPE_HOIST_TICKS, "ropeShrugTicks": ROPE_SHRUG_TICKS, "ropeMissChance": ROPE_MISS_CHANCE, "ropeMissOvershoot": ROPE_MISS_OVERSHOOT, "ropeRest": ROPE_REST, "ropeSulk": ROPE_SULK });
    assert_close("constants", &constants, &vectors["constants"]);
    for vector in group(&vectors, "shots") {
        let (size, feet, perches, keepouts): (Size, Point, Vec<Perch>, Vec<Rect>) = (typed(&vector["size"]), typed(&vector["feet"]), typed(&vector["perches"]), typed(&vector["keepouts"]));
        assert_close(&format!("shot {}", vector["id"]), &json(&shot_for(feet, &perches, &keepouts, size)), &vector["expected"]);
    }
    for vector in group(&vectors, "flights") {
        let (from, to, speed): (Point, Point, f64) = (typed(&vector["from"]), typed(&vector["to"]), number(&vector["speed"]));
        let ticks = hook_ticks(from, to, speed);
        let path: Vec<[f64; 2]> = (0..ticks + 2).map(|tick| hook_step(from, to, speed, tick)).map(|at| [at.x, at.y]).collect();
        assert_close(&format!("flight {}", vector["id"]), &json!({ "ticks": ticks, "path": path }), &vector["expected"]);
    }
    for vector in group(&vectors, "zips") {
        let (size, shot): (Size, Shot) = (typed(&vector["size"]), typed(&vector["shot"]));
        assert_close(&format!("zip {}", vector["id"]), &hauled(&shot, size, zip_step), &vector["expected"]);
    }
    for vector in group(&vectors, "swings") {
        let (size, shot): (Size, Shot) = (typed(&vector["size"]), typed(&vector["shot"]));
        assert_close(&format!("swing {}", vector["id"]), &hauled(&shot, size, sway_step), &vector["expected"]);
    }
    for vector in group(&vectors, "surveys") {
        let (shot, perches, keepouts): (Shot, Vec<Perch>, Vec<Rect>) = (typed(&vector["shot"]), typed(&vector["perches"]), typed(&vector["keepouts"]));
        assert_close(&format!("survey {}", vector["id"]), &json(&shot_holds(&shot, &perches, &keepouts)), &vector["expected"]);
    }
    for vector in group(&vectors, "landings") {
        let (size, shot, edge): (Size, Shot, Perch) = (typed(&vector["size"]), typed(&vector["shot"]), typed(&vector["perch"]));
        assert_close(&format!("landing {}", vector["id"]), &json!({ "landing": landing_for(&shot, &edge, size), "miss": miss_of(&shot, &edge) }), &vector["expected"]);
    }
    for vector in group(&vectors, "routes") {
        let (size, gear, perches): (Size, Vec<Gear>, Vec<Perch>) = (typed(&vector["size"]), typed(&vector["gear"]), typed(&vector["perches"]));
        let (pitches, ladders, keepouts): (Vec<Pitch>, Vec<LadderStand>, Vec<Rect>) = (typed(&vector["pitches"]), typed(&vector["ladders"]), typed(&vector["keepouts"]));
        let (from, to): (usize, usize) = (typed(&vector["from"]), typed(&vector["to"]));
        let legs = route_of(number(&vector["x"]), &perches[from], &perches[to], &gear, size, number(&vector["grip"]), &pitches, &ladders, &keepouts);
        let projected = legs.map_or(Value::Null, |legs| Value::Array(legs.iter().map(|leg| leg_value(leg, &pitches, &ladders)).collect()));
        assert_close(&format!("route {}", vector["id"]), &projected, &vector["expected"]);
    }
}

#[test]
fn source_uses_nothing_but_the_exact_operations_of_the_design() {
    let code: Vec<&str> = SOURCE.lines().filter(|line| !line.trim_start().starts_with("//")).collect();
    let transcendental = [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt("];
    let native = [".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprintln!", "dbg!", "static ", "thread_local!"];
    for banned in transcendental.into_iter().chain(native) {
        assert!(code.iter().all(|line| !line.contains(banned)), "{banned}");
    }
    for exact in [".sqrt()", ".abs()", ".floor()", "larger(", "smaller("] {
        assert!(code.iter().any(|line| line.contains(exact)), "{exact}");
    }
}
