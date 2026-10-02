//! 🏞️ Unit tests of the terrain: perches, strides, falls, landings and hops in the cases of the TypeScript suite, the extremes of JavaScript, and the committed vectors of the terrain-walking and hop-ballistics cases.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🏞️terrain-walking/🔣️.json
//! @see ../../../../🧫️fixtures/🦘️hop-ballistics/🔣️.json

use super::*;
use crate::schema::tests::{entries, fixture, number, typed};
use serde_json::Value;

const TOLERANCE: f64 = 1e-9;
const SOURCE: &str = include_str!("../../🦀️.rs");

fn surface(id: &str, x0: f64, x1: f64, y: f64) -> Surface {
    Surface { id: id.to_string(), x0, x1, y }
}

fn keepout(x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect { x, y, width, height }
}

fn perch(id: &str, x0: f64, x1: f64, y: f64) -> Perch {
    Perch { surface: id.to_string(), x0, x1, y }
}

fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

fn index_of(perches: &[Perch], found: Option<&Perch>) -> Option<usize> {
    found.map(|found| perches.iter().position(|candidate| std::ptr::eq(candidate, found)).unwrap_or_else(|| panic!("{found:?} is not an element of the list")))
}

fn group<'a>(vectors: &'a Value, name: &str) -> &'a [Value] {
    let listed = entries(&vectors[name]);
    assert!(!listed.is_empty(), "the fixture carries no {name}");
    listed
}

fn whole(value: &Value) -> Ticks {
    typed(value)
}

fn index(value: &Value) -> Option<usize> {
    typed(value)
}

fn assert_near(context: &str, produced: f64, expected: f64) {
    assert!((produced - expected).abs() <= TOLERANCE, "{context}: {produced} ≠ {expected}");
}

fn assert_same(context: &str, produced: f64, expected: &Value) {
    assert!(produced == number(expected), "{context}: {produced} ≠ {expected}");
}

fn stream(seed: u32) -> impl FnMut(u32) -> f64 {
    let mut state = seed;
    move |bound| {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((f64::from(state) / 4_294_967_296.0) * f64::from(bound)).floor()
    }
}

fn flown(from: Point, to: Point, hop: Hop) -> Vec<Point> {
    let mut path = Vec::new();
    let mut flight = Flight { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
    for left in (1..=hop.ticks).rev() {
        flight = hop_step(flight.x, flight.y, flight.vx, flight.vy, to, left);
        path.push(point(flight.x, flight.y));
    }
    path
}

fn summed(from: Point, hop: Hop) -> (f64, Fall) {
    let mut x = from.x;
    let mut fall = Fall { y: from.y, vy: hop.vy };
    for _ in 0..hop.ticks {
        x += hop.vx / 64.0;
        fall = fall_step(fall.y, fall.vy);
    }
    (x, fall)
}

fn dropped(perches: &[Perch], x: f64, start: Fall, limit: Ticks) -> (Option<usize>, Ticks, f64) {
    let mut fall = start;
    for tick in 1..=limit {
        let next = fall_step(fall.y, fall.vy);
        if let Some(landing) = landing_of(perches, x, fall.y, next.y) {
            return (index_of(perches, Some(landing)), tick, landing.y);
        }
        fall = next;
    }
    (None, limit, fall.y)
}

fn granted(from: Point, to: Point) -> Hop {
    hop_of(from, to).unwrap_or_else(|| panic!("the hop from {from:?} to {to:?} is out of reach"))
}

fn ended(perches: &[Perch], from: Point, to: Point) -> Option<&str> {
    hop_landing(perches, from, to, granted(from, to)).map(|landing| landing.surface.as_str())
}

fn index_of_route(perches: &[Perch], from: Point, to: Point) -> Option<usize> {
    index_of(perches, hop_landing(perches, from, to, granted(from, to)))
}

#[test]
fn extremes_follow_javascript_for_signed_zeros_and_nan() {
    assert_eq!(larger(-0.0, 0.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(larger(0.0, -0.0).to_bits(), 0.0_f64.to_bits());
    assert_eq!(larger(-0.0, -0.0).to_bits(), (-0.0_f64).to_bits());
    assert_eq!(smaller(-0.0, 0.0).to_bits(), (-0.0_f64).to_bits());
    assert_eq!(smaller(0.0, -0.0).to_bits(), (-0.0_f64).to_bits());
    assert_eq!(smaller(0.0, 0.0).to_bits(), 0.0_f64.to_bits());
    for other in [0.0, -3.5, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
        assert!(larger(f64::NAN, other).is_nan() && larger(other, f64::NAN).is_nan());
        assert!(smaller(f64::NAN, other).is_nan() && smaller(other, f64::NAN).is_nan());
    }
    assert_eq!(larger(2.0, 7.5), 7.5);
    assert_eq!(larger(7.5, 2.0), 7.5);
    assert_eq!(smaller(2.0, 7.5), 2.0);
    assert_eq!(smaller(7.5, 2.0), 2.0);
    assert_eq!(larger(f64::NEG_INFINITY, f64::INFINITY), f64::INFINITY);
    assert_eq!(smaller(f64::NEG_INFINITY, f64::INFINITY), f64::NEG_INFINITY);
}

#[test]
fn perches_clip_a_negative_zero_to_the_positive_zero_of_the_viewport() {
    let perches = perches_of(&[surface("card", -0.0, 100.0, 200.0)], &[], 640.0, 480.0, 56.0, 0.0);
    assert_eq!(perches.len(), 1);
    assert_eq!(perches[0].x0.to_bits(), 0.0_f64.to_bits());
    assert_eq!(stride_to(1.0, 5.0, -0.0), 1.0);
}

#[test]
fn perches_keep_free_surfaces_whole_in_surface_order() {
    let surfaces = [surface("b", 300.0, 500.0, 200.0), surface("a", 20.0, 220.0, 120.0), surface("floor", 0.0, 640.0, 480.0)];
    assert_eq!(perches_of(&surfaces, &[], 640.0, 480.0, 56.0, 72.0), [perch("b", 300.0, 500.0, 200.0), perch("a", 20.0, 220.0, 120.0), perch("floor", 0.0, 640.0, 480.0)]);
}

#[test]
fn perches_clip_a_surface_to_the_viewport_and_drop_what_lies_outside_or_only_touches_it() {
    let surfaces = [surface("left", -80.0, 120.0, 200.0), surface("right", 560.0, 720.0, 200.0), surface("wide", -50.0, 700.0, 300.0), surface("out", 640.0, 800.0, 200.0), surface("gone", -200.0, 0.0, 200.0), surface("far", 900.0, 1000.0, 200.0)];
    assert_eq!(perches_of(&surfaces, &[], 640.0, 480.0, 56.0, 72.0), [perch("left", 0.0, 120.0, 200.0), perch("right", 560.0, 640.0, 200.0), perch("wide", 0.0, 640.0, 300.0)]);
}

#[test]
fn perches_skip_surfaces_above_the_clearance_line_and_below_the_viewport_and_keep_both_limits() {
    let surfaces = [surface("high", 0.0, 200.0, 55.5), surface("line", 0.0, 200.0, 56.0), surface("floor", 0.0, 640.0, 480.0), surface("below", 0.0, 640.0, 480.5)];
    assert_eq!(perches_of(&surfaces, &[], 640.0, 480.0, 56.0, 72.0), [perch("line", 0.0, 200.0, 56.0), perch("floor", 0.0, 640.0, 480.0)]);
}

#[test]
fn perches_lose_a_keep_out_only_when_it_reaches_into_the_band_above_the_surface() {
    let card = [surface("card", 100.0, 400.0, 200.0)];
    let cut = [perch("card", 100.0, 200.0, 200.0), perch("card", 260.0, 400.0, 200.0)];
    let whole = [perch("card", 100.0, 400.0, 200.0)];
    assert_eq!(perches_of(&card, &[keepout(200.0, 150.0, 60.0, 20.0)], 640.0, 480.0, 56.0, 0.0), cut);
    assert_eq!(perches_of(&card, &[keepout(200.0, 100.0, 60.0, 44.0)], 640.0, 480.0, 56.0, 0.0), whole);
    assert_eq!(perches_of(&card, &[keepout(200.0, 100.0, 60.0, 44.5)], 640.0, 480.0, 56.0, 0.0), cut);
    assert_eq!(perches_of(&card, &[keepout(200.0, 200.0, 60.0, 80.0)], 640.0, 480.0, 56.0, 0.0), whole);
    assert_eq!(perches_of(&card, &[keepout(200.0, 199.5, 60.0, 80.0)], 640.0, 480.0, 56.0, 0.0), cut);
    assert_eq!(perches_of(&card, &[keepout(200.0, 20.0, 60.0, 400.0)], 640.0, 480.0, 56.0, 0.0), cut);
    assert_eq!(perches_of(&card, &[keepout(200.0, 20.0, 60.0, 400.0)], 640.0, 480.0, 0.0, 0.0), whole);
}

#[test]
fn perches_lose_nothing_to_keep_outs_that_only_touch_an_end_or_have_no_area() {
    let card = [surface("card", 100.0, 400.0, 200.0)];
    let whole = [perch("card", 100.0, 400.0, 200.0)];
    assert_eq!(perches_of(&card, &[keepout(40.0, 150.0, 60.0, 40.0), keepout(400.0, 150.0, 60.0, 40.0)], 640.0, 480.0, 56.0, 0.0), whole);
    assert_eq!(perches_of(&card, &[keepout(250.0, 150.0, 0.0, 40.0), keepout(250.0, 170.0, 40.0, 0.0), keepout(250.0, 170.0, -40.0, 20.0), keepout(250.0, 190.0, 40.0, -20.0)], 640.0, 480.0, 56.0, 0.0), whole);
    assert_eq!(perches_of(&card, &[keepout(40.0, 150.0, 61.0, 40.0), keepout(399.0, 150.0, 60.0, 40.0)], 640.0, 480.0, 56.0, 0.0), [perch("card", 101.0, 399.0, 200.0)]);
}

#[test]
fn perches_merge_nested_overlapping_and_abutting_keep_outs_in_any_order() {
    let card = [surface("card", 0.0, 600.0, 200.0)];
    let keepouts = [keepout(100.0, 160.0, 200.0, 30.0), keepout(150.0, 170.0, 50.0, 10.0), keepout(250.0, 150.0, 100.0, 60.0), keepout(350.0, 190.0, 50.0, 5.0), keepout(500.0, 180.0, 40.0, 40.0)];
    let expected = [perch("card", 0.0, 100.0, 200.0), perch("card", 400.0, 500.0, 200.0), perch("card", 540.0, 600.0, 200.0)];
    let reversed: Vec<Rect> = keepouts.iter().rev().copied().collect();
    let shuffled = [2, 4, 0, 3, 1].map(|at| keepouts[at]);
    assert_eq!(perches_of(&card, &keepouts, 640.0, 480.0, 56.0, 0.0), expected);
    assert_eq!(perches_of(&card, &reversed, 640.0, 480.0, 56.0, 0.0), expected);
    assert_eq!(perches_of(&card, &shuffled, 640.0, 480.0, 56.0, 0.0), expected);
}

#[test]
fn perches_keep_stretches_of_exactly_the_minimum_drop_narrower_ones_and_never_have_zero_width() {
    let card = [surface("card", 0.0, 300.0, 200.0)];
    let keepouts = [keepout(72.0, 180.0, 28.0, 10.0), keepout(171.5, 180.0, 28.5, 10.0), keepout(200.0, 180.0, 100.0, 10.0)];
    assert_eq!(perches_of(&card, &keepouts, 640.0, 480.0, 56.0, 72.0), [perch("card", 0.0, 72.0, 200.0)]);
    assert_eq!(perches_of(&card, &keepouts, 640.0, 480.0, 56.0, 71.5), [perch("card", 0.0, 72.0, 200.0), perch("card", 100.0, 171.5, 200.0)]);
    assert!(perches_of(&card, &[keepout(0.0, 180.0, 150.0, 10.0), keepout(150.0, 180.0, 150.0, 10.0)], 640.0, 480.0, 56.0, 0.0).is_empty());
    assert!(perches_of(&[surface("edge", -100.0, 0.0, 200.0), surface("point", 50.0, 50.0, 200.0)], &[], 640.0, 480.0, 56.0, 0.0).is_empty());
    assert_eq!(perches_of(&card, &[], 640.0, 480.0, 56.0, 300.0), [perch("card", 0.0, 300.0, 200.0)]);
    assert!(perches_of(&card, &[], 640.0, 480.0, 56.0, 300.5).is_empty());
}

#[test]
fn perch_at_finds_the_perch_of_a_surface_that_carries_x_ends_included_the_first_where_two_touch() {
    let perches = [perch("card", 0.0, 100.0, 200.0), perch("card", 100.0, 180.0, 200.0), perch("other", 0.0, 300.0, 200.0), perch("card", 240.0, 300.0, 200.0)];
    assert_eq!(index_of(&perches, perch_at(&perches, "card", 0.0)), Some(0));
    assert_eq!(index_of(&perches, perch_at(&perches, "card", 100.0)), Some(0));
    assert_eq!(index_of(&perches, perch_at(&perches, "card", 100.5)), Some(1));
    assert_eq!(index_of(&perches, perch_at(&perches, "card", 300.0)), Some(3));
    assert_eq!(index_of(&perches, perch_at(&perches, "other", 200.0)), Some(2));
    assert_eq!(perch_at(&perches, "card", 200.0), None);
    assert_eq!(perch_at(&perches, "card", -0.5), None);
    assert_eq!(perch_at(&perches, "card", 300.5), None);
    assert_eq!(perch_at(&perches, "none", 50.0), None);
    assert_eq!(perch_at(&perches, "card", f64::NAN), None);
    assert_eq!(perch_at(&[], "card", 50.0), None);
}

#[test]
fn nearest_perch_measures_to_the_nearest_point_of_each_perch_and_prefers_the_first_among_equals() {
    let perches = [perch("a", 0.0, 100.0, 200.0), perch("b", 200.0, 300.0, 120.0), perch("floor", 0.0, 640.0, 480.0), perch("c", 200.0, 300.0, 280.0)];
    assert_eq!(index_of(&perches, nearest_perch(&perches, 50.0, 190.0)), Some(0));
    assert_eq!(index_of(&perches, nearest_perch(&perches, 150.0, 200.0)), Some(0));
    assert_eq!(index_of(&perches, nearest_perch(&perches, 190.0, 130.0)), Some(1));
    assert_eq!(index_of(&perches, nearest_perch(&perches, 600.0, 300.0)), Some(2));
    assert_eq!(index_of(&perches, nearest_perch(&perches, 250.0, 201.0)), Some(3));
    assert_eq!(index_of(&perches, nearest_perch(&perches, -400.0, -400.0)), Some(0));
    assert_eq!(index_of(&perches, nearest_perch(&perches, 250.0, 200.0)), Some(1));
    let swapped = [perches[3].clone(), perches[1].clone()];
    assert_eq!(index_of(&swapped, nearest_perch(&swapped, 250.0, 200.0)), Some(0));
    assert_eq!(index_of(&perches, nearest_perch(&perches, 150.0, 160.0)), Some(0));
    assert_eq!(nearest_perch(&perches, f64::NAN, 200.0), None);
    assert_eq!(nearest_perch(&[], 10.0, 10.0), None);
}

#[test]
fn stride_advances_speed_over_64_pixels_per_tick_in_either_direction() {
    assert_eq!(stride_to(10.0, 100.0, 64.0), 11.0);
    assert_eq!(stride_to(10.0, -100.0, 64.0), 9.0);
    assert_eq!(stride_to(10.0, 100.0, 48.0), 10.75);
    assert_eq!(stride_to(-3.5, -100.0, 40.0), -4.125);
}

#[test]
fn stride_never_overshoots_and_stands_still_without_a_positive_speed() {
    assert_eq!(stride_to(99.5, 100.0, 64.0), 100.0);
    assert_eq!(stride_to(99.0, 100.0, 64.0), 100.0);
    assert_eq!(stride_to(100.25, 100.0, 64.0), 100.0);
    assert_eq!(stride_to(100.0, 100.0, 64.0), 100.0);
    assert_eq!(stride_to(98.5, 100.0, 64.0), 99.5);
    assert_eq!(stride_to(10.0, 100.0, 0.0), 10.0);
    assert_eq!(stride_to(10.0, 100.0, -64.0), 10.0);
    assert_eq!(stride_to(10.0, 10.0, 0.0), 10.0);
}

#[test]
fn stride_arrives_after_the_ceiling_of_distance_over_step_and_follows_the_closed_form() {
    let mut next = stream(7);
    for walk in 0..200 {
        let start = next(1280) / 2.0 - 160.0;
        let goal = next(1280) / 2.0 - 160.0;
        let speed = 8.0 + next(120);
        let step = speed / 64.0;
        let distance = (goal - start).abs();
        let ticks = (distance / step).ceil();
        let mut x = start;
        let mut tick = 1.0;
        while tick <= ticks + 2.0 {
            x = stride_to(x, goal, speed);
            assert_near(&format!("walk {walk} tick {tick}"), x, start + (goal - start).signum() * (tick * step).min(distance));
            assert_eq!(x == goal, tick >= ticks, "walk {walk} tick {tick}");
            tick += 1.0;
        }
    }
}

#[test]
fn fall_adds_gravity_to_the_speed_first_and_moves_by_the_new_speed() {
    assert_eq!(GRAVITY / 64.0, 28.125);
    assert_eq!(fall_step(100.0, 0.0), Fall { y: 100.439453125, vy: 28.125 });
    assert_eq!(fall_step(100.0, -225.0), Fall { y: 96.923828125, vy: -196.875 });
}

#[test]
fn fall_follows_its_closed_form_below_the_terminal_speed() {
    for (y, vy) in [(100.0, 0.0), (320.5, -410.25), (12.0, 33.3), (0.0, -550.0)] {
        let mut fall = Fall { y, vy };
        let mut tick = 1.0;
        while tick <= 30.0 && vy + tick * 28.125 <= FALL_SPEED {
            fall = fall_step(fall.y, fall.vy);
            assert_near("speed", fall.vy, vy + tick * 28.125);
            assert_near("height", fall.y, y + (vy * tick) / 64.0 + (GRAVITY * tick * (tick + 1.0)) / 8192.0);
            tick += 1.0;
        }
    }
}

#[test]
fn fall_reaches_the_terminal_speed_after_32_ticks_from_rest_and_keeps_it() {
    let mut fall = Fall { y: 0.0, vy: 0.0 };
    for _ in 0..32 {
        fall = fall_step(fall.y, fall.vy);
    }
    assert_eq!(fall, Fall { y: (GRAVITY * 32.0 * 33.0) / 8192.0, vy: FALL_SPEED });
    assert_eq!(fall_step(fall.y, fall.vy), Fall { y: fall.y + FALL_SPEED / 64.0, vy: FALL_SPEED });
    assert_eq!(fall_step(0.0, 5000.0), Fall { y: FALL_SPEED / 64.0, vy: FALL_SPEED });
}

#[test]
fn landing_is_the_highest_perch_crossed_at_x_the_first_among_equals_with_heights_and_ends_included() {
    let perches = [perch("floor", 0.0, 640.0, 480.0), perch("low", 100.0, 300.0, 300.0), perch("high", 150.0, 250.0, 200.0), perch("twin", 0.0, 640.0, 200.0), perch("side", 400.0, 500.0, 250.0)];
    let landed = |x: f64, from_y: f64, to_y: f64| index_of(&perches, landing_of(&perches, x, from_y, to_y));
    assert_eq!(landed(200.0, 100.0, 500.0), Some(2));
    assert_eq!(landed(120.0, 100.0, 500.0), Some(3));
    assert_eq!(landed(200.0, 201.0, 500.0), Some(1));
    assert_eq!(landed(200.0, 301.0, 500.0), Some(0));
    assert_eq!(landed(450.0, 210.0, 260.0), Some(4));
    assert_eq!(landed(200.0, 300.0, 310.0), Some(1));
    assert_eq!(landed(200.0, 290.0, 300.0), Some(1));
    assert_eq!(landed(100.0, 290.0, 310.0), Some(1));
    assert_eq!(landed(300.0, 290.0, 310.0), Some(1));
    assert_eq!(landed(300.5, 290.0, 310.0), None);
    assert_eq!(landed(99.5, 290.0, 310.0), None);
    assert_eq!(landed(200.0, 300.0, 300.0), Some(1));
    assert_eq!(landed(200.0, 210.0, 290.0), None);
    assert_eq!(landed(200.0, 481.0, 600.0), None);
    assert_eq!(landed(700.0, 0.0, 600.0), None);
    assert_eq!(landed(200.0, 310.0, 190.0), None);
    assert_eq!(landed(f64::NAN, 0.0, 600.0), None);
    assert_eq!(landing_of(&[], 200.0, 0.0, 600.0), None);
}

#[test]
fn landing_stops_a_fall_on_the_first_perch_below_however_fast_and_lets_it_pass_the_ones_beside() {
    let perches = [perch("floor", 0.0, 640.0, 480.0), perch("low", 100.0, 300.0, 300.0), perch("side", 400.0, 500.0, 250.0)];
    let mut fall = Fall { y: 40.0, vy: 0.0 };
    let mut landing = None;
    let mut ticks = 0;
    while landing.is_none() && ticks < 200 {
        let next = fall_step(fall.y, fall.vy);
        landing = landing_of(&perches, 350.0, fall.y, next.y);
        fall = next;
        ticks += 1;
    }
    assert_eq!(index_of(&perches, landing), Some(0));
    assert_eq!(fall.vy, FALL_SPEED);
    assert_eq!(ticks, 47);
    assert!(fall.y > 480.0 && fall.y < 480.0 + FALL_SPEED / 64.0, "{}", fall.y);
}

#[test]
fn hop_on_the_spot_launches_with_the_clearance_as_its_apex() {
    assert_eq!(hop_of(point(50.0, 200.0), point(50.0, 200.0)), Some(Hop { vx: 0.0, vy: -225.0, ticks: 15 }));
}

#[test]
fn hop_is_out_of_reach_when_too_high_too_far_too_long_or_landing_faster_than_a_fall() {
    let from = point(100.0, 300.0);
    assert!(hop_of(from, point(100.0, 300.0 - (HOP_HEIGHT - HOP_CLEARANCE))).is_some());
    assert_eq!(hop_of(from, point(100.0, 300.0 - (HOP_HEIGHT - HOP_CLEARANCE) - 0.5)), None);
    assert!(hop_of(from, point(100.0 + HOP_DISTANCE, 300.0)).is_some());
    assert!(hop_of(from, point(100.0 - HOP_DISTANCE, 300.0)).is_some());
    assert_eq!(hop_of(from, point(100.0 + HOP_DISTANCE + 0.5, 300.0)), None);
    assert_eq!(hop_of(from, point(100.0 - HOP_DISTANCE - 0.5, 300.0)), None);
    assert_eq!(granted(from, point(260.0, 455.0)).ticks, HOP_TICKS);
    assert_eq!(hop_of(from, point(260.0, 465.0)), None);
    let fast = granted(from, point(100.0, 510.0));
    assert!(fast.ticks < HOP_TICKS);
    assert!(fast.vy + fast.ticks as f64 * 28.125 <= FALL_SPEED);
    assert_eq!(hop_of(from, point(100.0, 520.0)), None);
}

#[test]
fn hop_is_out_of_reach_for_points_that_are_not_finite_numbers() {
    let from = point(100.0, 300.0);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(hop_of(from, point(value, 300.0)), None);
        assert_eq!(hop_of(from, point(100.0, value)), None);
        assert_eq!(hop_of(point(value, 300.0), from), None);
        assert_eq!(hop_of(point(100.0, value), from), None);
    }
}

#[test]
fn hop_mirrors_with_the_direction_and_does_not_depend_on_where_it_starts() {
    let right = granted(point(100.0, 300.0), point(196.0, 276.0));
    let left = granted(point(100.0, 300.0), point(4.0, 276.0));
    let moved = granted(point(612.0, 44.0), point(708.0, 20.0));
    assert_eq!(left, Hop { vx: -right.vx, vy: right.vy, ticks: right.ticks });
    assert_eq!(moved, right);
}

#[test]
fn hop_lands_every_granted_hop_on_its_target_after_its_ticks_with_an_apex_above_both_ends() {
    let from = point(217.3, 301.7);
    let mut hops = 0;
    let mut refused = 0;
    for column in 0..52 {
        for row in 0..60 {
            let (dx, dy) = (-168.0 + 6.5 * f64::from(column), -80.0 + 5.25 * f64::from(row));
            let to = point(from.x + dx, from.y + dy);
            let label = format!("dx {dx} dy {dy}");
            let Some(hop) = hop_of(from, to) else {
                refused += 1;
                continue;
            };
            hops += 1;
            assert!(hop.ticks >= 15 && hop.ticks <= HOP_TICKS, "{label}");
            assert!(dx.abs() <= HOP_DISTANCE, "{label}");
            let path = flown(from, to, hop);
            assert_eq!(path.len() as Ticks, hop.ticks, "{label}");
            assert_eq!(path[path.len() - 1], to, "{label}");
            let (x, fall) = summed(from, hop);
            assert_near(&label, x, to.x);
            assert_near(&label, fall.y, to.y);
            assert!(fall.vy <= FALL_SPEED + TOLERANCE, "{label}");
            assert_near(&label, fall.vy, hop.vy + hop.ticks as f64 * 28.125);
            let apex = path.iter().fold(f64::INFINITY, |least, feet| least.min(feet.y));
            let rise = (HOP_CLEARANCE - dy.min(0.0)).max(dx.abs() * HOP_STEEPNESS);
            assert!(from.y.min(to.y) - apex > HOP_CLEARANCE - 3.0, "{label}");
            assert!((from.y - apex - rise).abs() < 3.0, "{label}");
            assert!(from.y - apex < HOP_HEIGHT + 3.0, "{label}");
            assert!(path[0].y < from.y, "{label}");
            assert!(path[path.len() - 2].y < to.y, "{label}");
            for tick in 1..path.len() {
                assert_eq!((path[tick].x - path[tick - 1].x).partial_cmp(&0.0), dx.partial_cmp(&0.0), "{label}");
            }
        }
    }
    assert!(hops > 1500, "{hops}");
    assert!(refused > 500, "{refused}");
}

#[test]
fn hop_step_moves_by_the_fall_step_and_vx_over_64_and_puts_the_last_tick_on_the_target() {
    let rising = fall_step(200.0, -225.0);
    let to = point(80.0, 200.0);
    assert_eq!(hop_step(50.0, 200.0, 128.0, -225.0, to, 15), Flight { x: 52.0, y: rising.y, vx: 128.0, vy: rising.vy });
    assert_eq!(hop_step(50.0, 200.0, 128.0, -225.0, to, 2), Flight { x: 52.0, y: rising.y, vx: 128.0, vy: rising.vy });
    let sinking = fall_step(190.0, 170.0);
    assert_eq!(hop_step(78.0, 190.0, 128.0, 170.0, to, 1), Flight { x: 80.0, y: 200.0, vx: 128.0, vy: sinking.vy });
    assert_eq!(hop_step(78.0, 190.0, 128.0, 170.0, to, 0), Flight { x: 80.0, y: 200.0, vx: 128.0, vy: sinking.vy });
    assert_eq!(hop_step(78.0, 190.0, 128.0, 170.0, to, -3), Flight { x: 80.0, y: 200.0, vx: 128.0, vy: sinking.vy });
}

#[test]
fn hop_landing_ends_on_the_perch_of_the_target_when_nothing_lies_in_the_way() {
    let from = point(100.0, 300.0);
    let launch = perch("launch", 0.0, 120.0, 300.0);
    assert_eq!(ended(&[launch.clone(), perch("level", 150.0, 300.0, 300.0)], from, point(180.0, 300.0)), Some("level"));
    assert_eq!(ended(&[launch.clone(), perch("higher", 150.0, 300.0, 240.0)], from, point(180.0, 240.0)), Some("higher"));
    assert_eq!(ended(&[launch.clone(), perch("lower", 150.0, 300.0, 420.0)], from, point(180.0, 420.0)), Some("lower"));
    assert_eq!(ended(&[launch], from, point(40.0, 300.0)), Some("launch"));
    assert_eq!(index_of_route(&[perch("level", 150.0, 300.0, 300.0), perch("end", 150.0, 180.0, 300.0)], from, point(180.0, 300.0)), Some(0));
}

#[test]
fn hop_landing_ends_on_whatever_is_crossed_first_on_the_way_down_and_passes_a_perch_from_below() {
    let from = point(100.0, 300.0);
    let launch = perch("launch", 0.0, 120.0, 300.0);
    let lower = perch("lower", 90.0, 300.0, 420.0);
    let shelf = perch("shelf", 170.0, 195.0, 360.0);
    assert_eq!(ended(&[launch.clone(), lower.clone()], from, point(110.0, 420.0)), Some("launch"));
    assert_eq!(ended(&[launch.clone(), lower.clone(), shelf], from, point(200.0, 420.0)), Some("shelf"));
    assert_eq!(ended(&[launch.clone(), lower], from, point(200.0, 420.0)), Some("lower"));
    assert_eq!(ended(&[launch.clone(), perch("roof", 0.0, 300.0, 290.0), perch("higher", 150.0, 300.0, 240.0)], from, point(180.0, 240.0)), Some("higher"));
    assert_eq!(ended(&[launch], from, point(180.0, 300.0)), None);
    assert_eq!(ended(&[], from, from), None);
}

#[test]
fn committed_terrain_walking_vectors_are_reproduced() {
    let vectors = fixture("terrain-walking");
    for layout in group(&vectors, "layouts") {
        let found = perches_of(&typed::<Vec<Surface>>(&layout["surfaces"]), &typed::<Vec<Rect>>(&layout["keepouts"]), number(&layout["width"]), number(&layout["height"]), number(&layout["clearance"]), number(&layout["minimum"]));
        assert_eq!(found, typed::<Vec<Perch>>(&layout["expected"]), "layout {}", layout["id"]);
    }
    for stand in group(&vectors, "standings") {
        let perches: Vec<Perch> = typed(&stand["perches"]);
        let found: Vec<Option<usize>> = group(stand, "queries").iter().map(|query| index_of(&perches, perch_at(&perches, query["surface"].as_str().unwrap_or_default(), number(&query["x"])))).collect();
        assert_eq!(found, typed::<Vec<Option<usize>>>(&stand["expected"]), "standing {}", stand["id"]);
    }
    for near in group(&vectors, "nearests") {
        let perches: Vec<Perch> = typed(&near["perches"]);
        let found: Vec<Option<usize>> = group(near, "points").iter().map(|feet| index_of(&perches, nearest_perch(&perches, number(&feet["x"]), number(&feet["y"])))).collect();
        assert_eq!(found, typed::<Vec<Option<usize>>>(&near["expected"]), "nearest {}", near["id"]);
    }
    for walk in group(&vectors, "strides") {
        let expected = group(walk, "expected");
        assert_eq!(expected.len() as Ticks, whole(&walk["ticks"]), "stride {}", walk["id"]);
        let mut x = number(&walk["x"]);
        for (tick, expected) in expected.iter().enumerate() {
            x = stride_to(x, number(&walk["goal"]), number(&walk["speed"]));
            assert_same(&format!("stride {} tick {tick}", walk["id"]), x, expected);
        }
    }
}

#[test]
fn committed_hop_ballistics_constants_falls_landings_and_drops_are_reproduced() {
    let vectors = fixture("hop-ballistics");
    let constants = &vectors["constants"];
    assert_eq!([GRAVITY, FALL_SPEED, HOP_CLEARANCE, HOP_STEEPNESS, HOP_HEIGHT, HOP_DISTANCE], ["gravity", "fallSpeed", "hopClearance", "hopSteepness", "hopHeight", "hopDistance"].map(|name| number(&constants[name])));
    assert_eq!(HOP_TICKS, whole(&constants["hopTicks"]));
    for vector in group(&vectors, "falls") {
        let mut fall = Fall { y: number(&vector["y"]), vy: number(&vector["vy"]) };
        let (heights, speeds) = (entries(&vector["expected"]["heights"]), entries(&vector["expected"]["speeds"]));
        assert_eq!((heights.len() as Ticks, speeds.len() as Ticks), (whole(&vector["ticks"]), whole(&vector["ticks"])), "fall {}", vector["id"]);
        for (height, speed) in heights.iter().zip(speeds) {
            fall = fall_step(fall.y, fall.vy);
            assert_same(&format!("fall {} height", vector["id"]), fall.y, height);
            assert_same(&format!("fall {} speed", vector["id"]), fall.vy, speed);
        }
    }
    for vector in group(&vectors, "landings") {
        let perches: Vec<Perch> = typed(&vector["perches"]);
        let found: Vec<Option<usize>> = group(vector, "sweeps").iter().map(|sweep| index_of(&perches, landing_of(&perches, number(&sweep["x"]), number(&sweep["fromY"]), number(&sweep["toY"])))).collect();
        assert_eq!(found, typed::<Vec<Option<usize>>>(&vector["expected"]), "landing {}", vector["id"]);
    }
    for vector in group(&vectors, "drops") {
        let perches: Vec<Perch> = typed(&vector["perches"]);
        let start = &vector["start"];
        let (landed, ticks, y) = dropped(&perches, number(&start["x"]), Fall { y: number(&start["y"]), vy: number(&start["vy"]) }, whole(&vector["limit"]));
        let expected = &vector["expected"];
        assert_eq!((landed, ticks), (index(&expected["perch"]), whole(&expected["ticks"])), "drop {}", vector["id"]);
        assert_same(&format!("drop {}", vector["id"]), y, &expected["y"]);
    }
}

#[test]
fn committed_hop_ballistics_hops_flights_and_routes_are_reproduced() {
    let vectors = fixture("hop-ballistics");
    for vector in group(&vectors, "hops") {
        let hop = hop_of(typed(&vector["from"]), typed(&vector["to"]));
        let expected = &vector["expected"];
        assert_eq!(hop.is_some(), vector["verdict"] == "granted", "hop {}", vector["id"]);
        assert_eq!(hop.is_none(), expected.is_null(), "hop {}", vector["id"]);
        if let Some(hop) = hop {
            assert_eq!(hop.ticks, whole(&expected["ticks"]), "hop {}", vector["id"]);
            assert_same(&format!("hop {} vx", vector["id"]), hop.vx, &expected["vx"]);
            assert_same(&format!("hop {} vy", vector["id"]), hop.vy, &expected["vy"]);
        }
    }
    for vector in group(&vectors, "flights") {
        let (from, to): (Point, Point) = (typed(&vector["from"]), typed(&vector["to"]));
        let hop = granted(from, to);
        let path = flown(from, to, hop);
        let expected = &vector["expected"];
        assert_eq!((hop.ticks, path.len()), (whole(&expected["ticks"]), entries(&expected["path"]).len()), "flight {}", vector["id"]);
        for (feet, expected) in path.iter().zip(entries(&expected["path"])) {
            assert_same(&format!("flight {} x", vector["id"]), feet.x, &expected[0]);
            assert_same(&format!("flight {} y", vector["id"]), feet.y, &expected[1]);
        }
        assert_same(&format!("flight {} apex", vector["id"]), path.iter().fold(f64::INFINITY, |least, feet| least.min(feet.y)), &expected["apex"]);
    }
    for vector in group(&vectors, "routes") {
        let perches: Vec<Perch> = typed(&vector["perches"]);
        assert_eq!(index_of_route(&perches, typed(&vector["from"]), typed(&vector["to"])), index(&vector["expected"]), "route {}", vector["id"]);
    }
}

#[test]
fn source_uses_nothing_but_the_exact_operations_of_the_design() {
    let code: Vec<&str> = SOURCE.lines().filter(|line| !line.trim_start().starts_with("//")).collect();
    for banned in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt(", ".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprintln!", "dbg!"] {
        assert!(code.iter().all(|line| !line.contains(banned)), "{banned}");
    }
    for exact in [".sqrt()", ".abs()", ".floor()", "larger(", "smaller("] {
        assert!(code.iter().any(|line| line.contains(exact)), "{exact}");
    }
}
