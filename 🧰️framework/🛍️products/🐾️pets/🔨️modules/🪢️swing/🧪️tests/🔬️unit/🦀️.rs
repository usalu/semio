//! 🪢️ Unit tests of the swing module: the constrained step against its own invariants, the rotation of the lean against the platform's sine and cosine, the drags of MECH §1.3, the follow spring, the cone, the release, the reel and the parachute in the cases of the TypeScript suite, and the committed vectors of the swing-dynamics and parachute-descent cases, number for number.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../🧪️tests/🔬️unit/🟦️.ts — the TypeScript suite these tests mirror
//! @see ../../../../🧫️fixtures/🪢️swing-dynamics/🔣️.json — the restated swings scipy and numpy confirmed
//! @see ../../../../🧫️fixtures/🪂️parachute-descent/🔣️.json — the restated descents scipy and numpy confirmed

use super::*;
use crate::randomness::random_unit;
use crate::schema::tests::{assert_same, bits, entries, fixture, json, number, typed};
use serde_json::{json, Value};

const SOURCE: &str = include_str!("../../🦀️.rs");
const ANCHOR: Point = Point { x: 320.0, y: 120.0 };
const STATES: u32 = 300;

fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

fn drawn(index: u32, slot: u32, low: f64, high: f64) -> f64 {
    low + (high - low) * random_unit(&[20_261_003, index, slot])
}

fn distance(from: Point, to: Point) -> f64 {
    ((to.x - from.x) * (to.x - from.x) + (to.y - from.y) * (to.y - from.y)).sqrt()
}

fn degrees(anchor: Point, bob: Point) -> f64 {
    (bob.x - anchor.x).atan2(bob.y - anchor.y).to_degrees()
}

fn swung(bob: Point, previous: Point, ticks: usize, step: impl Fn(Point, Point) -> Point) -> Vec<Point> {
    let (mut now, mut before) = (bob, previous);
    let mut path = Vec::with_capacity(ticks);
    for _ in 0..ticks {
        let next = step(now, before);
        before = now;
        now = next;
        path.push(now);
    }
    path
}

fn amplitude(path: &[Point], anchor: Point, window: usize) -> f64 {
    path[path.len() - window..].iter().fold(0.0, |widest: f64, &bob| widest.max(degrees(anchor, bob).abs()))
}

fn group<'a>(vectors: &'a Value, name: &str) -> &'a [Value] {
    let listed = entries(&vectors[name]);
    assert!(!listed.is_empty(), "the fixture carries no {name}");
    listed
}

fn ticks_of(vector: &Value) -> Vec<usize> {
    typed(&vector["ticks"])
}

fn at<T: Copy>(path: &[T], ticks: &[usize]) -> Vec<T> {
    ticks.iter().map(|&tick| path[tick - 1]).collect()
}

fn latest(ticks: &[usize]) -> usize {
    ticks.iter().copied().max().unwrap_or_default()
}

fn walked(bob: Point, previous: Point, ticks: usize, step: impl Fn(Point, Point, usize) -> Point) -> Vec<Point> {
    let (mut now, mut before) = (bob, previous);
    let mut path = Vec::with_capacity(ticks);
    for tick in 1..=ticks {
        let next = step(now, before, tick);
        before = now;
        now = next;
        path.push(now);
    }
    path
}

fn guarded(vector: &Value) -> Value {
    let ticks = ticks_of(vector);
    let anchor: Point = typed(&vector["anchor"]);
    let (mut bob, mut previous, mut length): (Point, Point, f64) = (typed(&vector["bob"]), typed(&vector["previous"]), number(&vector["length"]));
    let mut fastest = 0.0;
    let mut states = Vec::new();
    for age in 0..latest(&ticks) {
        let reel = reel_step(anchor, bob, previous, length, number(&vector["least"]), age as Ticks);
        let speed = distance(bob, reel.bob) * 64.0;
        if speed > fastest {
            fastest = speed;
        }
        previous = bob;
        bob = reel.bob;
        length = reel.length;
        states.push(json!({ "x": bob.x, "y": bob.y, "length": length }));
    }
    json!({ "states": ticks.iter().map(|&tick| states[tick - 1].clone()).collect::<Vec<_>>(), "fastest": fastest })
}

fn dragged(vector: &Value) -> (Vec<f64>, f64, usize) {
    let length = number(&vector["length"]);
    let mut hang = hang_of(typed(&vector["feet"]), length);
    let (mut leans, mut peak, mut settled) = (Vec::new(), 0.0, 1);
    for (index, target) in typed::<Vec<Point>>(&vector["pointer"]).into_iter().enumerate() {
        hang = hang_step(hang, target, length);
        assert!((distance(point(hang.grip.x, hang.grip.y), hang.bob) - length).abs() <= 1e-6 * length, "drag {} tick {}", vector["id"], index + 1);
        let lean = lean_of(point(hang.grip.x, hang.grip.y), hang.bob, length);
        leans.push(lean);
        if lean.abs() > peak {
            peak = lean.abs();
        }
        if lean.abs() >= 1.0 / 120.0 {
            settled = index + 2;
        }
    }
    (leans, peak, settled)
}

fn descended(vector: &Value) -> Vec<Value> {
    let chute = chute_of(number(&vector["height"]));
    let feet: Point = typed(&vector["feet"]);
    let ticks = ticks_of(vector);
    let mut canopy = canopy_of(feet, number(&vector["vx"]), number(&vector["vy"]), chute);
    let mut states = Vec::new();
    for tick in 1..=latest(&ticks) {
        let wind = if vector["phase"].is_null() { 0.0 } else { chute_wind(tick as Ticks, number(&vector["phase"])) };
        canopy = chute_step(canopy, chute, number(&vector["target"]), number(&vector["remaining"]) - (canopy.bob.y - feet.y), wind);
        assert!((distance(point(canopy.x, canopy.y), canopy.bob) - chute.length).abs() <= 1e-9 * chute.length, "descent {} tick {tick}", vector["id"]);
        states.push(json!({ "x": canopy.x, "y": canopy.y, "vx": canopy.vx, "vy": canopy.vy, "bob": canopy.bob }));
    }
    ticks.iter().map(|&tick| states[tick - 1].clone()).collect()
}

fn swayed(vector: &Value) -> Vec<Point> {
    let chute = chute_of(number(&vector["height"]));
    let start: Point = typed(&vector["canopy"]);
    let ticks = ticks_of(vector);
    let mut canopy = Canopy { x: start.x, y: start.y, vx: number(&vector["vx"]), vy: chute.terminal, bob: typed(&vector["bob"]), previous: typed(&vector["previous"]) };
    let mut feet = Vec::new();
    for _ in 0..latest(&ticks) {
        canopy = chute_step(canopy, chute, number(&vector["target"]), number(&vector["remaining"]), 0.0);
        feet.push(canopy.bob);
    }
    at(&feet, &ticks)
}

fn dropped(vector: &Value) -> Value {
    let chute = chute_of(number(&vector["height"]));
    let (drop, start, opens) = (number(&vector["drop"]), number(&vector["vy"]), vector["chute"] == true);
    let (mut y, mut vy, mut opened, mut tick, mut touch): (f64, f64, Ticks, Ticks, f64) = (0.0, start, 0, 0, 0.0);
    let mut canopy: Option<Canopy> = None;
    while y < drop {
        tick += 1;
        let remaining = drop - y;
        if canopy.is_none() {
            if opened == 0 && opens && chute_opens(vy, remaining) {
                opened = tick;
            }
            if opened != 0 && tick - opened >= CHUTE_REFLEX {
                canopy = Some(canopy_of(point(0.0, y), 0.0, vy, chute));
            }
        }
        match canopy {
            None => {
                let fall = crate::terrain::fall_step(y, vy);
                y = fall.y;
                vy = fall.vy;
                touch = vy;
            }
            Some(open) => {
                let moved = chute_step(open, chute, 0.0, remaining, 0.0);
                touch = (moved.y - open.y) * 64.0;
                canopy = Some(moved);
                y = moved.bob.y;
                vy = moved.vy;
            }
        }
    }
    json!({ "opened": opened, "landed": tick, "touch": touch, "glide": vy, "plain": impact_speed(start, drop) })
}

#[test]
fn swing_leaves_a_point_that_hangs_at_rest_where_it_is() {
    let bob = point(320.0, 220.0);
    let mut now = bob;
    for _ in 0..64 {
        now = swing_step(ANCHOR, ANCHOR, now, now, 100.0, GRAVITY, 1.0, false);
        assert!((now.x - bob.x).abs() + (now.y - bob.y).abs() <= 1e-12, "{now:?}");
    }
}

#[test]
fn swing_puts_the_point_exactly_at_its_rods_length_from_an_anchor_that_moved_whenever_the_move_and_the_step_are_shorter_than_the_rod() {
    for index in 0..STATES {
        let before = point(drawn(index, 0, 0.0, 800.0), drawn(index, 1, 0.0, 600.0));
        let length = drawn(index, 2, 20.0, 160.0);
        let angle = drawn(index, 3, -3.0, 3.0);
        let bob = point(before.x + length * angle.sin(), before.y + length * angle.cos());
        let previous = point(bob.x + drawn(index, 4, -3.0, 3.0), bob.y + drawn(index, 5, -3.0, 3.0));
        let now = point(before.x + drawn(index, 6, -2.0, 2.0), before.y + drawn(index, 7, -2.0, 2.0));
        let next = swing_step(before, now, bob, previous, length, drawn(index, 8, 0.0, 7200.0), drawn(index, 9, 0.9, 1.0), false);
        assert!((distance(now, next) - length).abs() <= 1e-9 * length, "state {index}");
    }
}

#[test]
fn swing_lets_a_point_on_a_slack_rope_fly_freely_and_holds_it_on_a_taut_one() {
    let bob = point(320.0, 180.0);
    let previous = point(320.0 - 120.0 / 64.0, 180.0 + 300.0 / 64.0);
    let slack = swing_step(ANCHOR, ANCHOR, bob, previous, 100.0, GRAVITY, 1.0, true);
    assert_eq!(slack, point(bob.x + (bob.x - previous.x), bob.y + (bob.y - previous.y) + GRAVITY / 4096.0));
    let taut = swing_step(ANCHOR, ANCHOR, point(380.0, 200.0), point(378.5, 198.25), 100.0, GRAVITY, 1.0, true);
    assert!((distance(ANCHOR, taut) - 100.0).abs() <= 1e-12, "{taut:?}");
}

#[test]
fn swing_keeps_the_amplitude_of_a_free_swing_for_a_minute_where_the_shortcut_that_pulls_the_point_back_onto_the_circle_loses_it() {
    let length = 100.0;
    let start = point(ANCHOR.x + length * std::f64::consts::FRAC_1_SQRT_2, ANCHOR.y + length * std::f64::consts::FRAC_1_SQRT_2);
    let vectors = fixture("swing-dynamics");
    let gentle = group(&vectors, "rods").iter().find(|vector| vector["id"] == "gentle").unwrap_or_else(|| panic!("no gentle swing"));
    let kept = swung(typed(&gentle["bob"]), typed(&gentle["previous"]), 3840, |bob, previous| swing_step(ANCHOR, ANCHOR, bob, previous, length, GRAVITY, 1.0, false));
    let shortcut = swung(start, start, 3840, |bob, previous| {
        let free = point(bob.x + (bob.x - previous.x), bob.y + (bob.y - previous.y) + GRAVITY / 4096.0);
        let reach = distance(ANCHOR, free);
        point(ANCHOR.x + ((free.x - ANCHOR.x) * length) / reach, ANCHOR.y + ((free.y - ANCHOR.y) * length) / reach)
    });
    assert!((amplitude(&kept, ANCHOR, 128) - 45.0).abs() <= 0.05, "{}", amplitude(&kept, ANCHOR, 128));
    assert!(amplitude(&shortcut, ANCHOR, 128) < 35.0, "{}", amplitude(&shortcut, ANCHOR, 128));
}

#[test]
fn swing_keeps_the_angular_momentum_about_the_anchor_exactly_while_the_rope_shortens_without_gravity() {
    let momentum = |from: Point, to: Point| (from.x - ANCHOR.x) * (to.y - ANCHOR.y) - (from.y - ANCHOR.y) * (to.x - ANCHOR.x);
    let (mut bob, mut previous) = (point(420.0, 120.0), point(420.0, 118.0));
    let first = momentum(previous, bob);
    for tick in 1..=96 {
        let next = swing_step(ANCHOR, ANCHOR, bob, previous, 100.0 - (60.0 * f64::from(tick)) / 64.0, 0.0, 1.0, false);
        previous = bob;
        bob = next;
        assert!((momentum(previous, bob) - first).abs() <= 1e-9 * first.abs(), "tick {tick}");
    }
}

#[test]
fn swing_carries_a_hanging_point_along_with_an_anchor_in_uniform_motion_without_a_swing() {
    let drift = point(3.25, -1.5);
    let (mut anchor, mut bob) = (point(100.0, 100.0), point(100.0, 140.0));
    let mut previous = point(bob.x - drift.x, bob.y - drift.y);
    for tick in 0..128 {
        let next = point(anchor.x + drift.x, anchor.y + drift.y);
        let moved = swing_step(anchor, next, bob, previous, 40.0, GRAVITY, 1.0, false);
        previous = bob;
        bob = moved;
        anchor = next;
        assert!((bob.x - anchor.x).abs() <= 1e-9 && (bob.y - anchor.y - 40.0).abs() <= 1e-9, "tick {tick}");
    }
}

#[test]
fn cone_keeps_a_held_pet_on_its_rod_and_inside_the_cone_on_its_own_side_and_leaves_every_other_point_alone() {
    let grip = point(0.0, 0.0);
    for step in (-180_i32..=180).step_by(5) {
        let radians = f64::from(step).to_radians();
        let turned = point(50.0 * radians.sin(), 50.0 * radians.cos());
        let held = cone_clamp(grip, turned, 40.0);
        assert!((distance(grip, held) - 40.0).abs() <= 1e-9, "step {step}");
        assert!(held.y >= HANG_CONE * 40.0 - 1e-9, "step {step}");
        if turned.x != 0.0 && step.abs() < 180 {
            assert_eq!(held.x.signum(), turned.x.signum(), "step {step}");
        }
    }
    let inside = point(5.0, 20.0);
    assert_eq!(cone_clamp(grip, inside, 40.0), inside);
    assert_eq!(cone_clamp(grip, grip, 40.0), grip);
}

#[test]
fn follow_carries_the_grip_to_the_pointer_without_overshoot_half_way_in_four_ticks_and_seventy_ms_behind_a_pointer_in_steady_motion() {
    let mut grip = Grip { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0 };
    for tick in 1..=64 {
        grip = follow_step(grip, point(100.0, -50.0));
        assert!(grip.x <= 100.0 && grip.y >= -50.0, "tick {tick}");
        if tick == 3 {
            assert!(grip.x < 50.0);
        }
        if tick == 4 {
            assert!(grip.x >= 50.0);
        }
    }
    grip = Grip { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0 };
    for tick in 1..=512 {
        grip = follow_step(grip, point(10.0 * f64::from(tick), 0.0));
    }
    assert!(((10.0 * 512.0 - grip.x) / 640.0 - (FOLLOW_DAMPING / FOLLOW_STIFFNESS - 1.0 / 64.0)).abs() <= 1e-9);
}

#[test]
fn drags_lean_as_mech_measured_forty_to_fifty_one_degrees_for_a_short_drag_sixty_for_a_long_one_and_a_flick_fourteen_to_twenty_four_for_a_slow_one() {
    let vectors = fixture("swing-dynamics");
    let peak = |id: &str| {
        let vector = group(&vectors, "drags").iter().find(|vector| vector["id"] == id).unwrap_or_else(|| panic!("no drag {id}"));
        let (_, peak, settled) = dragged(vector);
        assert!((settled as f64) / 64.0 < 2.5, "drag {id} settles after {settled} ticks");
        peak * 360.0
    };
    let short = peak("short-drag");
    assert!((40.0..=51.0).contains(&short), "{short}");
    assert!((peak("short-drag-to-the-left") - short).abs() <= 1e-9);
    assert!((peak("long-drag") - 60.0).abs() < 0.001 && (peak("flick") - 60.0).abs() < 0.001);
    assert!((14.0..=24.0).contains(&peak("slow-drag")));
}

#[test]
fn lean_is_the_rotation_that_carries_a_body_hanging_straight_down_onto_its_feet() {
    for step in (-60..=60).step_by(3) {
        let radians = f64::from(step).to_radians();
        let bob = point(200.0 + 38.4 * radians.sin(), 100.0 + 38.4 * radians.cos());
        let lean = lean_of(point(200.0, 100.0), bob, 38.4);
        let turned = std::f64::consts::TAU * lean;
        let carried = point(-38.4 * turned.sin(), 38.4 * turned.cos());
        let bound = 38.4 * std::f64::consts::TAU * 2e-6;
        assert!((carried.x - (bob.x - 200.0)).abs() <= bound && (carried.y - (bob.y - 100.0)).abs() <= bound, "step {step}");
        if step > 0 {
            assert!(lean < 0.0, "step {step}");
        }
    }
    assert_eq!(lean_of(point(7.0, 9.0), point(7.0, 47.4), 38.4).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn hang_picks_a_pet_up_at_rest_with_its_grip_a_rod_above_its_feet() {
    let hang = hang_of(point(300.0, 400.0), HANG_ROD * 48.0);
    assert_eq!(hang, Hang { grip: Grip { x: 300.0, y: 400.0 - HANG_ROD * 48.0, vx: 0.0, vy: 0.0 }, bob: point(300.0, 400.0), previous: point(300.0, 400.0) });
    let rested = hang_step(hang, point(300.0, 400.0 - HANG_ROD * 48.0), HANG_ROD * 48.0);
    assert_eq!(rested.grip, hang.grip);
    assert_eq!(rested.previous, hang.bob);
    assert!(distance(rested.bob, hang.bob) <= 1e-12);
}

#[test]
fn ring_weighs_the_last_seven_samples_with_weights_that_add_up_to_nothing_and_fits_a_parabola_exactly() {
    assert_eq!(RELEASE_WEIGHTS.iter().sum::<f64>(), 0.0);
    for (velocity, thrust) in [(3.0, 0.0), (-7.5, 0.25), (12.0, -0.75)] {
        let samples: Vec<Point> = (0..7).map(f64::from).map(|tick| point(400.0 + velocity * tick + thrust * tick * tick, 50.0 - velocity * tick)).collect();
        let ring = ring_velocity(&samples);
        assert!((ring.x - (velocity + 2.0 * thrust * 6.0) * 64.0).abs() < 5e-10, "{ring:?}");
        assert!((ring.y + velocity * 64.0).abs() < 5e-10, "{ring:?}");
    }
    let resting = vec![point(123.456, -98.765_432_1); 7];
    assert_eq!(ring_velocity(&resting), point(0.0, 0.0));
    assert_eq!(ring_velocity(&[]), point(0.0, 0.0));
    let few = [point(10.0, 0.0), point(20.0, 0.0)];
    let padded = [point(10.0, 0.0), point(10.0, 0.0), point(10.0, 0.0), point(10.0, 0.0), point(10.0, 0.0), point(10.0, 0.0), point(20.0, 0.0)];
    assert_eq!(ring_velocity(&few), ring_velocity(&padded));
    assert_eq!(RELEASE_DIVISOR, 28.0);
}

#[test]
fn throw_is_nothing_below_the_least_speed_at_most_the_most_along_the_same_direction_and_never_rises_faster_than_allowed() {
    assert_eq!(throw_of(point(60.0, 30.0)), point(0.0, 0.0));
    assert_eq!(throw_of(point(70.0, 0.0)), point(70.0, 0.0));
    let fast = throw_of(point(1200.0, 500.0));
    assert!(((fast.x * fast.x + fast.y * fast.y).sqrt() - THROW_MOST).abs() <= 1e-9);
    assert!((fast.y / fast.x - 500.0 / 1200.0).abs() <= 1e-12);
    assert_eq!(throw_of(point(0.0, -630.0)), point(0.0, -THROW_RISE));
    assert_eq!(throw_of(point(0.0, 630.0)), point(0.0, 630.0));
    let stopped: Vec<Point> = [0.0, 5.0, 15.0, 30.0, 40.0, 40.0, 40.0].iter().map(|&x| point(x, 0.0)).collect();
    assert_eq!(release_velocity(&stopped), point(0.0, 0.0));
    assert_eq!(ring_velocity(&stopped).x, (45.0 * 64.0) / 28.0);
}

#[test]
fn reel_winds_the_rope_in_at_the_ramped_speed_down_to_its_least_never_lengthens_it_and_keeps_the_pet_within_the_cap_plus_what_the_reel_takes_in() {
    let mut bob = point(ANCHOR.x + 154.0 * 1.4_f64.sin(), ANCHOR.y + 154.0 * 1.4_f64.cos());
    let (mut previous, mut length) = (bob, 154.0);
    for age in 0..400 {
        let reel = reel_step(ANCHOR, bob, previous, length, 43.2, age);
        let wound = ((age + 1).min(10) as f64) * (REEL_SPEED / 10.0 / 64.0);
        assert_eq!(reel.length, (length - wound).max(43.2), "age {age}");
        assert!(distance(bob, reel.bob) * 64.0 <= REEL_CAP + REEL_SPEED, "age {age}");
        assert!(distance(ANCHOR, reel.bob) <= reel.length + 1e-9, "age {age}");
        previous = bob;
        bob = reel.bob;
        length = reel.length;
    }
    assert_eq!(length, 43.2);
    assert_eq!(reel_step(ANCHOR, point(320.0, 160.0), point(320.0, 160.0), 40.0, 43.2, 0).length, 40.0);
}

#[test]
fn impact_follows_its_closed_form_and_never_exceeds_the_terminal_speed() {
    assert_eq!(impact_speed(0.0, 100.0), HARD_LANDING);
    assert_eq!(impact_speed(0.0, 1000.0), FALL_SPEED);
    assert_eq!(impact_speed(-250.0, -5.0), 250.0);
}

#[test]
fn chute_opens_only_for_a_fast_fall_from_high_enough_that_would_land_hard_and_from_rest_for_every_drop_of_102_px_or_more() {
    assert!(!chute_opens(CHUTE_OPENING, CHUTE_HEADROOM));
    assert!(chute_opens(CHUTE_OPENING, 85.0));
    for drop in [101.0, 102.0] {
        let (mut y, mut vy, mut opened) = (0.0, 0.0, false);
        while y < drop && !opened {
            opened = chute_opens(vy, drop - y);
            vy = (vy + GRAVITY / 64.0).min(FALL_SPEED);
            y += vy / 64.0;
        }
        assert_eq!(opened, drop >= 102.0, "drop {drop}");
    }
}

#[test]
fn canopy_approaches_the_terminal_speed_by_the_exact_factor_of_the_lag() {
    let lag: f64 = 11.52;
    assert!((CHUTE_FACTOR - (1.0 - 1.0 / lag + 1.0 / (2.0 * lag * lag) - 1.0 / (6.0 * lag * lag * lag) + 1.0 / (24.0 * lag * lag * lag * lag))).abs() < 1e-6);
    assert!((CHUTE_FACTOR - (-1.0 / lag).exp()).abs() < 1e-15);
    let chute = chute_of(48.0);
    assert_eq!(chute, Chute { terminal: 96.0, reach: CHUTE_STEER_SPEED * 48.0, flare: 12.0, length: CHUTE_ROD * 48.0 });
    let canopy = canopy_of(point(300.0, 100.0), 64.0, 128.0, chute);
    assert_eq!((canopy.x, canopy.y, canopy.previous), (300.0, 100.0 - chute.length, point(299.0, 98.0)));
}

#[test]
fn flare_eases_evenly_to_half_the_speed_the_wind_blows_at_most_its_strength_and_the_canopy_sways_under_half_the_gravity_of_a_fall() {
    assert_eq!([flare_of(12.0, 12.0), flare_of(6.0, 12.0), flare_of(0.0, 12.0), flare_of(-4.0, 12.0)], [1.0, 0.75, 0.5, 0.5]);
    for tick in (0..2000).step_by(7) {
        assert!(chute_wind(tick, 0.3).abs() <= CHUTE_WIND, "tick {tick}");
    }
    assert_eq!(CHUTE_GRAVITY, GRAVITY / 2.0);
    assert_eq!(HANG_GRAVITY, 4.0 * GRAVITY);
}

#[test]
fn drops_land_as_committed_softly_with_a_parachute_under_120_px_per_second_from_150_px_and_under_60_from_200() {
    let vectors = fixture("parachute-descent");
    let touch = |id: &str| number(&group(&vectors, "drops").iter().find(|vector| vector["id"] == id).unwrap_or_else(|| panic!("no drop {id}"))["expected"]["touch"]);
    for vector in group(&vectors, "drops") {
        assert!(number(&vector["expected"]["touch"]) > 0.0, "drop {}", vector["id"]);
    }
    assert!(touch("from-150") < 120.0 && touch("from-200") < 60.0 && touch("from-150-without-a-parachute") > HARD_LANDING);
    assert_eq!(dropped(group(&vectors, "drops").iter().find(|vector| vector["id"] == "from-102").unwrap_or_else(|| panic!("no drop from-102")))["opened"], 10);
    assert_eq!(CHUTE_REFLEX, 6);
}

#[test]
fn committed_swing_dynamics_constants_rods_drifts_anchors_ropes_and_reels_are_reproduced() {
    let vectors = fixture("swing-dynamics");
    let constants = json!({ "hangRod": HANG_ROD, "hangGravity": HANG_GRAVITY, "hangDamping": HANG_DAMPING, "hangCone": HANG_CONE, "followStiffness": FOLLOW_STIFFNESS, "followDamping": FOLLOW_DAMPING, "releaseWeights": RELEASE_WEIGHTS, "releaseDivisor": RELEASE_DIVISOR, "releaseStale": RELEASE_STALE, "throwShare": THROW_SHARE, "throwLeast": THROW_LEAST, "throwMost": THROW_MOST, "throwRise": THROW_RISE, "reelSpeed": REEL_SPEED, "reelRamp": REEL_RAMP, "reelLeast": REEL_LEAST, "reelCap": REEL_CAP, "reelDamping": REEL_DAMPING });
    assert_same("constants", &constants, &vectors["constants"]);
    for vector in group(&vectors, "rods") {
        let (anchor, ticks) = (typed::<Point>(&vector["anchor"]), ticks_of(vector));
        let (length, gravity, damping) = (number(&vector["length"]), number(&vector["gravity"]), number(&vector["damping"]));
        let path = swung(typed(&vector["bob"]), typed(&vector["previous"]), latest(&ticks), |bob, previous| swing_step(anchor, anchor, bob, previous, length, gravity, damping, false));
        assert_same(&format!("rod {}", vector["id"]), &json(&at(&path, &ticks)), &vector["expected"]);
    }
    for vector in group(&vectors, "drifts") {
        let anchor: Point = typed(&vector["anchor"]);
        let (length, gravity, damping) = (number(&vector["length"]), number(&vector["gravity"]), number(&vector["damping"]));
        let (ticks, window): (usize, usize) = (typed(&vector["ticks"]), typed(&vector["window"]));
        let path = swung(typed(&vector["bob"]), typed(&vector["previous"]), ticks, |bob, previous| swing_step(anchor, anchor, bob, previous, length, gravity, damping, false));
        let lowest: Vec<f64> = path.chunks(window).map(|chunk| chunk.iter().skip(1).fold(chunk[0].y - anchor.y, |least, bob| if bob.y - anchor.y < least { bob.y - anchor.y } else { least })).collect();
        assert_same(&format!("drift {}", vector["id"]), &json!({ "lowest": lowest, "end": path[path.len() - 1] }), &vector["expected"]);
    }
    for vector in group(&vectors, "anchors") {
        let (path, ticks): (Vec<Point>, Vec<usize>) = (typed(&vector["path"]), ticks_of(vector));
        let (length, gravity) = (number(&vector["length"]), number(&vector["gravity"]));
        let start = point(path[0].x, path[0].y + length);
        let moved = walked(start, start, path.len() - 1, |bob, previous, tick| swing_step(path[tick - 1], path[tick], bob, previous, length, gravity, 1.0, false));
        assert_same(&format!("anchor {}", vector["id"]), &json(&at(&moved, &ticks)), &vector["expected"]);
    }
    for vector in group(&vectors, "ropes") {
        let anchor: Point = typed(&vector["anchor"]);
        let (length, gravity, ticks): (f64, f64, usize) = (number(&vector["length"]), number(&vector["gravity"]), typed(&vector["ticks"]));
        let path = swung(typed(&vector["bob"]), typed(&vector["previous"]), ticks, |bob, previous| swing_step(anchor, anchor, bob, previous, length, gravity, 1.0, true));
        assert_same(&format!("rope {}", vector["id"]), &json(&path), &vector["expected"]);
    }
    for vector in group(&vectors, "reels") {
        let (anchor, ticks) = (typed::<Point>(&vector["anchor"]), ticks_of(vector));
        let (length, rate, gravity, rope) = (number(&vector["length"]), number(&vector["rate"]), number(&vector["gravity"]), vector["rope"] == true);
        let path = walked(typed(&vector["bob"]), typed(&vector["previous"]), latest(&ticks), |bob, previous, tick| swing_step(anchor, anchor, bob, previous, length - (rate * tick as f64) / 64.0, gravity, 1.0, rope));
        assert_same(&format!("reel {}", vector["id"]), &json(&at(&path, &ticks)), &vector["expected"]);
    }
}

#[test]
fn committed_swing_dynamics_guards_cones_follows_drags_releases_throws_and_bit_patterns_are_reproduced() {
    let vectors = fixture("swing-dynamics");
    for vector in group(&vectors, "guards") {
        assert_same(&format!("guard {}", vector["id"]), &guarded(vector), &vector["expected"]);
    }
    for vector in group(&vectors, "cones") {
        assert_same(&format!("cone {}", vector["id"]), &json(&cone_clamp(typed(&vector["anchor"]), typed(&vector["bob"]), number(&vector["length"]))), &vector["expected"]);
    }
    for vector in group(&vectors, "follows") {
        let (target, mut grip, mut reached) = (typed::<Point>(&vector["target"]), typed::<Grip>(&vector["grip"]), 0);
        let mut states = Vec::new();
        for ticks in ticks_of(vector) {
            while reached < ticks {
                grip = follow_step(grip, target);
                reached += 1;
            }
            states.push(grip);
        }
        assert_same(&format!("follow {}", vector["id"]), &json(&states), &vector["expected"]);
    }
    for vector in group(&vectors, "drags") {
        let (leans, peak, settled) = dragged(vector);
        assert_same(&format!("drag {}", vector["id"]), &json!({ "leans": at(&leans, &ticks_of(vector)), "peak": peak, "settled": settled }), &vector["expected"]);
    }
    for vector in group(&vectors, "releases") {
        let samples: Vec<Point> = typed(&vector["samples"]);
        assert_same(&format!("release {}", vector["id"]), &json!({ "ring": ring_velocity(&samples), "release": release_velocity(&samples) }), &vector["expected"]);
    }
    for vector in group(&vectors, "throws") {
        assert_same(&format!("throw {}", vector["id"]), &json(&throw_velocity(typed(&vector["hang"]), &typed::<Vec<Point>>(&vector["samples"]))), &vector["expected"]);
    }
    for vector in group(&vectors, "steps") {
        let next = swing_step(typed(&vector["before"]), typed(&vector["now"]), typed(&vector["bob"]), typed(&vector["previous"]), number(&vector["length"]), number(&vector["gravity"]), number(&vector["damping"]), vector["rope"] == true);
        assert_eq!(json!({ "x": bits(next.x), "y": bits(next.y) }), vector["expected"], "step {}", vector["id"]);
    }
}

#[test]
fn committed_parachute_descent_vectors_are_reproduced() {
    let vectors = fixture("parachute-descent");
    let constants = json!({ "gravity": GRAVITY, "fallSpeed": FALL_SPEED, "hardLanding": HARD_LANDING, "chuteOpening": CHUTE_OPENING, "chuteHeadroom": CHUTE_HEADROOM, "chuteReflex": CHUTE_REFLEX, "chuteFactor": CHUTE_FACTOR, "chuteDescent": CHUTE_DESCENT, "chuteSteerGain": CHUTE_STEER_GAIN, "chuteSteerSpeed": CHUTE_STEER_SPEED, "chuteSteerEase": CHUTE_STEER_EASE, "chuteRod": CHUTE_ROD, "chuteGravity": CHUTE_GRAVITY, "chuteDamping": CHUTE_DAMPING, "chuteFlare": CHUTE_FLARE, "chuteWind": CHUTE_WIND, "chuteWindRate": CHUTE_WIND_RATE });
    assert_same("constants", &constants, &vectors["constants"]);
    for vector in group(&vectors, "impacts") {
        assert_same(&format!("impact {}", vector["id"]), &json!(impact_speed(number(&vector["vy"]), number(&vector["height"]))), &vector["expected"]);
    }
    for vector in group(&vectors, "triggers") {
        assert_eq!(json!(chute_opens(number(&vector["vy"]), number(&vector["height"]))), vector["expected"], "trigger {}", vector["id"]);
    }
    for vector in group(&vectors, "measures") {
        assert_same(&format!("measure {}", vector["id"]), &json(&chute_of(number(&vector["height"]))), &vector["expected"]);
    }
    for vector in group(&vectors, "flares") {
        assert_same(&format!("flare {}", vector["id"]), &json!(flare_of(number(&vector["remaining"]), number(&vector["flare"]))), &vector["expected"]);
    }
    for vector in group(&vectors, "winds") {
        assert_same(&format!("wind {}", vector["id"]), &json!(chute_wind(typed(&vector["ticks"]), number(&vector["phase"]))), &vector["expected"]);
    }
    for vector in group(&vectors, "descents") {
        assert_same(&format!("descent {}", vector["id"]), &json!(descended(vector)), &vector["expected"]);
    }
    for vector in group(&vectors, "sways") {
        assert_same(&format!("sway {}", vector["id"]), &json(&swayed(vector)), &vector["expected"]);
    }
    for vector in group(&vectors, "drops") {
        assert_same(&format!("drop {}", vector["id"]), &dropped(vector), &vector["expected"]);
    }
}

#[test]
fn source_uses_nothing_but_the_exact_operations_of_the_design() {
    let code: Vec<&str> = SOURCE.lines().filter(|line| !line.trim_start().starts_with("//")).collect();
    for banned in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt(", ".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprintln!", "dbg!"] {
        assert!(code.iter().all(|line| !line.contains(banned)), "{banned}");
    }
    for exact in [".sqrt()", "atan_turns(", "sin_turns(", "spring_step("] {
        assert!(code.iter().any(|line| line.contains(exact)), "{exact}");
    }
}
