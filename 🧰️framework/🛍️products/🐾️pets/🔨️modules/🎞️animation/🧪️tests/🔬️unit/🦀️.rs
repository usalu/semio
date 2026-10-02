//! 🎞️ Unit tests of the animation: easing, track and clip sampling, pose blending, the one-tick spring with the gaze constants and the lid of a blink in the cases of the TypeScript suite, and the committed vectors of the animation-sampling and spring-settling cases.
//!
//! @see ../../🦀️.rs — the implementation under test
//! @see ../../../../🧫️fixtures/🎞️animation-sampling/🔣️.json
//! @see ../../../../🧫️fixtures/🪀️spring-settling/🔣️.json

use super::*;
use crate::rig::solve_rig;
use crate::schema::tests::{entries, fixture, number, typed};
use crate::schema::Key;
use serde_json::Value;

const TOLERANCE: f64 = 1e-9;
const THIRD: f64 = 1.0 / 3.0;
const EASE_IN_OUT: Ease = [0.42, 0.0, 0.58, 1.0];
const SOURCE: &str = include_str!("../../🦀️.rs");

fn species() -> Species {
    typed(&fixture("animation-sampling")["species"])
}

fn group<'a>(vectors: &'a Value, name: &str) -> &'a [Value] {
    let listed = entries(&vectors[name]);
    assert!(!listed.is_empty(), "the fixture carries no {name}");
    listed
}

fn clip_of<'a>(species: &'a Species, id: &str) -> &'a Clip {
    species.clips.iter().find(|clip| clip.id == id).unwrap_or_else(|| panic!("the species has no clip {id}"))
}

fn bone_of(species: &Species, id: &str) -> usize {
    species.bones.iter().position(|bone| bone.id == id).unwrap_or_else(|| panic!("the species has no bone {id}"))
}

fn key(at: f64, value: f64) -> Key {
    Key { at, value, ease: None }
}

fn track_of(bone: &str, channel: Channel, keys: &[Key]) -> Track {
    Track { bone: bone.to_string(), channel, keys: keys.to_vec() }
}

fn lasting(seconds: f64) -> Clip {
    Clip { id: "clip".to_string(), seconds, looping: false, tracks: Vec::new() }
}

fn rest() -> BonePose {
    BonePose { x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 }
}

fn released(vector: &Value) -> (Spring, f64, f64, f64) {
    (Spring { position: number(&vector["position"]), velocity: number(&vector["velocity"]) }, number(&vector["target"]), number(&vector["stiffness"]), number(&vector["damping"]))
}

fn after(spring: Spring, target: f64, stiffness: f64, damping: f64, ticks: Ticks) -> Spring {
    (0..ticks).fold(spring, |state, _| spring_step(state.position, state.velocity, target, stiffness, damping))
}

fn settling(band: f64, horizon: Ticks, position: f64, target: f64) -> (Ticks, f64) {
    let travel = target - position;
    let mut state = Spring { position, velocity: 0.0 };
    let (mut settled, mut overshoot) = (1, 0.0);
    for tick in 1..=horizon {
        state = spring_step(state.position, state.velocity, target, GAZE_STIFFNESS, GAZE_DAMPING);
        let share = (state.position - target) / travel;
        if share.abs() > band {
            settled = tick + 1;
        }
        if share > overshoot {
            overshoot = share;
        }
    }
    (settled, overshoot)
}

fn assert_near(context: &str, produced: f64, expected: f64) {
    assert!((produced - expected).abs() <= TOLERANCE, "{context}: {produced} ≠ {expected}");
}

fn assert_pose(context: &str, produced: &[BonePose], expected: &[BonePose]) {
    assert_eq!(produced.len(), expected.len(), "{context}: bones");
    for (index, (bone, other)) in produced.iter().zip(expected).enumerate() {
        for (channel, left, right) in [("x", bone.x, other.x), ("y", bone.y, other.y), ("rotation", bone.rotation, other.rotation), ("scaleX", bone.scale_x, other.scale_x), ("scaleY", bone.scale_y, other.scale_y)] {
            assert_near(&format!("{context} bone {index} {channel}"), left, right);
        }
    }
}

fn assert_polynomial(ease: Ease, polynomial: fn(f64) -> f64) {
    for step in 0..=1024 {
        let amount = f64::from(step) / 1024.0;
        assert!((ease_bezier(ease, amount) - polynomial(amount)).abs() <= 1e-13, "{ease:?} {amount}");
    }
}

fn assert_spring(context: &str, produced: Spring, expected: &Value) {
    assert_near(&format!("{context} position"), produced.position, number(&expected["position"]));
    assert_near(&format!("{context} velocity"), produced.velocity, number(&expected["velocity"]));
}

#[test]
fn ease_is_0_at_and_below_0_and_1_at_and_above_1_whatever_the_curve() {
    for ease in [EASE_IN_OUT, [0.34, 1.56, 0.64, 1.0], [0.0, 1.0, 1.0, 0.0], [1.0, 0.0, 0.0, 1.0]] {
        for amount in [-1.0, -1e-300, 0.0, -0.0] {
            assert_eq!(ease_bezier(ease, amount).to_bits(), 0.0_f64.to_bits(), "{ease:?} {amount}");
        }
        for amount in [1.0, 1.0 + 1e-12, 7.0] {
            assert_eq!(ease_bezier(ease, amount), 1.0, "{ease:?} {amount}");
        }
    }
}

#[test]
fn ease_is_the_polynomial_whose_bezier_has_a_linear_abscissa() {
    assert_polynomial([THIRD, THIRD, 2.0 * THIRD, 2.0 * THIRD], |amount| amount);
    assert_polynomial([THIRD, 0.0, 2.0 * THIRD, 0.0], |amount| amount * amount * amount);
    assert_polynomial([THIRD, 1.0, 2.0 * THIRD, 1.0], |amount| (amount - 1.0) * (amount - 1.0) * (amount - 1.0) + 1.0);
    assert_polynomial([THIRD, 0.0, 2.0 * THIRD, THIRD], |amount| amount * amount);
    assert_polynomial([THIRD, 2.0 * THIRD, 2.0 * THIRD, 1.0], |amount| amount * (2.0 - amount));
    assert_polynomial([THIRD, 0.0, 2.0 * THIRD, 1.0], smoothstep);
}

#[test]
fn ease_in_out_of_css_is_symmetric_and_rising() {
    let mut previous = 0.0;
    for step in 1..=256 {
        let amount = f64::from(step) / 256.0;
        let eased = ease_bezier(EASE_IN_OUT, amount);
        assert!(eased > previous, "{amount}");
        assert!((eased + ease_bezier(EASE_IN_OUT, 1.0 - amount) - 1.0).abs() <= 1e-13, "{amount}");
        if step % 128 != 0 {
            assert_eq!(eased < amount, step < 128, "{amount}");
        }
        previous = eased;
    }
}

#[test]
fn ease_leaves_the_unit_range_with_curves_that_anticipate_or_overshoot_and_stays_on_a_curve_that_stands_still() {
    assert!(ease_bezier([0.34, 1.56, 0.64, 1.0], 0.7) > 1.0);
    assert!(ease_bezier([0.36, 0.0, 0.66, -0.56], 0.3) < 0.0);
    assert!((ease_bezier([1.0, 0.0, 0.0, 1.0], 0.5) - 0.5).abs() < 1e-5);
    assert!(ease_bezier(EASE_IN_OUT, f64::NAN).abs() < 1e-12);
}

#[test]
fn track_yields_each_key_exactly_on_its_phase_and_holds_the_end_values_outside_the_clip() {
    let keys = [key(0.0, 2.0), Key { at: 0.25, value: -6.0, ease: Some(EASE_IN_OUT) }, key(0.75, 10.0), key(1.0, 2.0)];
    let track = track_of("body", Channel::X, &keys);
    for key in keys {
        assert_eq!(sample_track(&track, key.at), key.value);
    }
    assert_eq!(sample_track(&track, -3.0), 2.0);
    assert_eq!(sample_track(&track, 4.0), 2.0);
    assert_eq!(sample_track(&track, 0.125), lerp(2.0, -6.0, 0.5));
    assert_eq!(sample_track(&track, 0.375), lerp(-6.0, 10.0, ease_bezier(EASE_IN_OUT, 0.25)));
    assert!((sample_track(&track, 0.5) - 2.0).abs() < 1e-12);
    assert_eq!(sample_track(&track, 0.875), lerp(10.0, 2.0, 0.5));
    assert!(sample_track(&track, f64::NAN).is_nan());
}

#[test]
fn track_of_one_key_answers_that_key_and_a_track_without_keys_the_rest_of_its_channel() {
    assert_eq!(sample_track(&track_of("body", Channel::X, &[key(0.0, 7.0)]), 0.4), 7.0);
    for channel in [Channel::X, Channel::Y, Channel::Rotation] {
        assert_eq!(sample_track(&track_of("body", channel, &[]), 0.4), 0.0);
    }
    for channel in [Channel::ScaleX, Channel::ScaleY] {
        assert_eq!(sample_track(&track_of("body", channel, &[]), 0.4), 1.0);
    }
}

#[test]
fn track_never_divides_by_an_empty_stretch_between_two_keys_on_one_phase() {
    let track = track_of("body", Channel::X, &[key(0.0, 0.0), key(0.5, 4.0), key(0.5, 8.0), key(1.0, 0.0)]);
    for step in 0..=64 {
        assert!(sample_track(&track, f64::from(step) / 64.0).is_finite(), "{step}");
    }
    assert_eq!(sample_track(&track, 0.5), 8.0);
}

#[test]
fn clip_ticks_round_half_up_and_never_fall_below_one_tick() {
    for (seconds, ticks) in [(3.0, 192), (0.6, 38), (1.4, 90), (0.0390625, 3), (0.0234375, 2), (0.001, 1), (0.0, 1), (-2.0, 1), (f64::NAN, 1)] {
        assert_eq!(clip_ticks(&lasting(seconds)), ticks, "{seconds}");
    }
}

#[test]
fn clip_leaves_every_channel_without_a_track_at_rest_and_starts_the_breathing_loop_in_the_rest_pose() {
    let species = species();
    let wave = clip_of(&species, "wave");
    let pose = sample_clip(&species, wave, 20);
    let arm = bone_of(&species, "arm-right");
    assert_eq!(pose.len(), species.bones.len());
    for (index, bone) in pose.iter().enumerate() {
        let expected = if index == arm { BonePose { rotation: sample_track(&wave.tracks[0], 20.0 / 90.0), ..rest() } } else { rest() };
        assert_eq!(*bone, expected, "bone {index}");
    }
    assert_eq!(sample_clip(&species, clip_of(&species, "breathe"), 0), rest_pose(&species));
}

#[test]
fn clip_wraps_when_it_loops_and_counts_ticks_before_its_beginning_as_its_beginning() {
    let species = species();
    let clip = clip_of(&species, "stroll");
    let length = clip_ticks(clip);
    for tick in [0, 1, 7, 19, 37] {
        assert_eq!(sample_clip(&species, clip, tick + length), sample_clip(&species, clip, tick), "{tick}");
        assert_eq!(sample_clip(&species, clip, tick + 5 * length), sample_clip(&species, clip, tick), "{tick}");
    }
    assert_eq!(sample_clip(&species, clip, -9), sample_clip(&species, clip, 0));
}

#[test]
fn clip_holds_its_last_key_when_it_plays_once() {
    let species = species();
    let clip = clip_of(&species, "lunge");
    let length = clip_ticks(clip);
    let held = sample_clip(&species, clip, length);
    assert_eq!(held[0].x, 3.0);
    assert_eq!(held[bone_of(&species, "tuft")].rotation, 12.0);
    for tick in [length + 1, 2 * length, 100_000] {
        assert_eq!(sample_clip(&species, clip, tick), held, "{tick}");
    }
    assert_eq!(sample_clip(&species, clip, -4), sample_clip(&species, clip, 0));
}

#[test]
fn clip_skips_a_track_on_a_bone_the_species_lacks_and_lets_the_later_of_two_tracks_on_one_channel_win() {
    let species = species();
    let constant = |bone: &str, channel: Channel, value: f64| track_of(bone, channel, &[key(0.0, value), key(1.0, value)]);
    let clip = Clip { id: "odd".to_string(), seconds: 1.0, looping: false, tracks: vec![constant("body", Channel::X, 1.0), constant("wing", Channel::Y, 9.0), constant("body", Channel::X, 5.0)] };
    let pose = sample_clip(&species, &clip, 10);
    assert_eq!(pose.len(), species.bones.len());
    assert_eq!(pose[1], BonePose { x: 5.0, ..rest() });
    assert!(pose.iter().all(|bone| bone.y == 0.0));
}

#[test]
fn clip_poses_feed_the_rig_solver() {
    let species = species();
    let resting = solve_rig(&species, &rest_pose(&species));
    assert_eq!(solve_rig(&species, &sample_clip(&species, clip_of(&species, "breathe"), 0)), resting);
    let solved = solve_rig(&species, &sample_clip(&species, clip_of(&species, "stroll"), 9));
    assert_eq!(solved.len(), species.bones.len() * 6);
    assert!(solved.iter().all(|entry| entry.is_finite()));
    assert_ne!(solved, resting);
}

#[test]
fn blend_is_the_first_pose_at_and_below_0_the_second_at_and_above_1_and_linear_per_channel_in_between() {
    let species = species();
    let from = rest_pose(&species);
    let to = sample_clip(&species, clip_of(&species, "stroll"), 9);
    for amount in [0.0, -2.0, -0.0] {
        assert_eq!(blend_pose(&from, &to, amount), from, "{amount}");
    }
    for amount in [1.0, 3.0] {
        assert_eq!(blend_pose(&from, &to, amount), to, "{amount}");
    }
    let blended = blend_pose(&from, &to, 0.25);
    let blend = |from: f64, to: f64| lerp(from, to, 0.25);
    assert_eq!(blended.len(), from.len());
    for (index, bone) in blended.iter().enumerate() {
        let (left, right) = (from[index], to[index]);
        assert_eq!(*bone, BonePose { x: blend(left.x, right.x), y: blend(left.y, right.y), rotation: blend(left.rotation, right.rotation), scale_x: blend(left.scale_x, right.scale_x), scale_y: blend(left.scale_y, right.scale_y) }, "bone {index}");
    }
    assert_eq!(blend_pose(&to, &to, 0.6), to);
}

#[test]
fn spring_moves_the_velocity_first_and_the_position_with_the_new_velocity() {
    assert_eq!(spring_step(0.0, 0.0, 1.0, 512.0, 32.0), Spring { position: 0.125, velocity: 8.0 });
    assert_eq!(spring_step(4.0, 3.0, 9.0, 0.0, 0.0), Spring { position: 4.046875, velocity: 3.0 });
    assert_eq!(spring_step(2.0, -1.0, 0.0, 100.0, 0.0), Spring { position: 1.935546875, velocity: -4.125 });
}

#[test]
fn spring_of_the_gaze_applies_its_exact_one_tick_matrix_and_rests_exactly_on_its_target() {
    assert_eq!((GAZE_STIFFNESS, GAZE_DAMPING), (512.0, 32.0));
    for (offset, velocity) in [(1.0, 0.0), (0.0, 1.0), (-0.375, 2.5), (0.25, -7.0)] {
        assert_eq!(spring_step(offset, velocity, 0.0, GAZE_STIFFNESS, GAZE_DAMPING), Spring { position: 0.875 * offset + 0.0078125 * velocity, velocity: -8.0 * offset + 0.5 * velocity });
    }
    for target in [0.0, 0.1, -37.25, 1e-9, 123_456.789] {
        assert_eq!(after(Spring { position: target, velocity: 0.0 }, target, GAZE_STIFFNESS, GAZE_DAMPING, 500), Spring { position: target, velocity: 0.0 });
    }
}

#[test]
fn spring_of_the_gaze_settles_within_a_quarter_of_a_second_and_barely_overshoots() {
    let gaze = &fixture("spring-settling")["gaze"];
    assert_eq!((number(&gaze["stiffness"]), number(&gaze["damping"])), (GAZE_STIFFNESS, GAZE_DAMPING));
    for jump in group(gaze, "jumps") {
        let (settled, overshoot) = settling(number(&gaze["band"]), typed(&gaze["horizon"]), number(&jump["position"]), number(&jump["target"]));
        assert_eq!(settled, typed::<Ticks>(&jump["expected"]["settled"]), "{}", jump["id"]);
        assert!(settled <= typed::<Ticks>(&gaze["settleTicks"]), "{}", jump["id"]);
        assert!(overshoot > 0.0 && overshoot <= number(&gaze["overshootBound"]), "{}", jump["id"]);
        assert_near(&format!("gaze {}", jump["id"]), overshoot, number(&jump["expected"]["overshoot"]));
    }
}

#[test]
fn spring_dies_out_inside_the_stated_stability_region_and_keeps_moving_outside_it() {
    let vectors = fixture("spring-settling");
    let region = |stiffness: f64, damping: f64| stiffness > 0.0 && damping > 0.0 && stiffness / 4096.0 + damping / 32.0 < 4.0;
    for vector in group(&vectors, "stables") {
        let (spring, target, stiffness, damping) = released(vector);
        assert!(region(stiffness, damping), "{}", vector["id"]);
        let faded = after(spring, target, stiffness, damping, 4096);
        assert!((faded.position - target).abs() < 1e-6 && faded.velocity.abs() < 1e-6, "{}: {faded:?}", vector["id"]);
    }
    for vector in group(&vectors, "unstables") {
        let (stiffness, damping) = (number(&vector["stiffness"]), number(&vector["damping"]));
        assert!(!region(stiffness, damping), "{}", vector["id"]);
        let kept = after(Spring { position: 1.0, velocity: 1.0 }, 0.0, stiffness, damping, 256);
        assert!(kept.position.abs() + kept.velocity.abs() > 0.1, "{}: {kept:?}", vector["id"]);
    }
}

#[test]
fn lid_is_open_before_at_the_beginning_and_from_the_end_on_and_shut_at_the_4th_and_5th_tick() {
    assert_eq!(BLINK_TICKS, 12);
    for tick in [-5, 0, BLINK_TICKS, BLINK_TICKS + 1, 1000] {
        assert_eq!(lid_at(tick), 0.0, "{tick}");
    }
    assert_eq!((lid_at(4), lid_at(5)), (1.0, 1.0));
    for parity in [0, 1] {
        assert!((0..8).any(|index| lid_at(parity + 2 * index) == 1.0), "{parity}");
    }
}

#[test]
fn lid_closes_faster_than_it_opens_each_way_by_the_hermite_ease() {
    for tick in 1..=4 {
        assert!(lid_at(tick) > lid_at(tick - 1), "{tick}");
        assert_eq!(lid_at(tick), smoothstep(tick as f64 / 4.0), "{tick}");
    }
    for tick in 6..=BLINK_TICKS {
        assert!(lid_at(tick) < lid_at(tick - 1), "{tick}");
        if tick < BLINK_TICKS {
            assert_eq!(lid_at(tick), 1.0 - smoothstep((tick as f64 - 5.0) / 7.0), "{tick}");
        }
    }
    assert_eq!(lid_at(2), 0.5);
    assert!(lid_at(7) > 0.5);
}

#[test]
fn committed_animation_sampling_vectors_are_reproduced() {
    let vectors = fixture("animation-sampling");
    let species: Species = typed(&vectors["species"]);
    for vector in group(&vectors, "easings") {
        let ease: Ease = typed(&vector["ease"]);
        assert_eq!(entries(&vector["amounts"]).len(), entries(&vector["expected"]).len(), "easing {}", vector["id"]);
        for (amount, expected) in group(vector, "amounts").iter().zip(entries(&vector["expected"])) {
            assert_near(&format!("easing {} at {amount}", vector["id"]), ease_bezier(ease, number(amount)), number(expected));
        }
    }
    for vector in group(&vectors, "tracks") {
        let track: Track = typed(&vector["track"]);
        assert_eq!(entries(&vector["phases"]).len(), entries(&vector["expected"]).len(), "track {}", vector["id"]);
        for (phase, expected) in group(vector, "phases").iter().zip(entries(&vector["expected"])) {
            assert_near(&format!("track {} at {phase}", vector["id"]), sample_track(&track, number(phase)), number(expected));
        }
    }
    for vector in group(&vectors, "lengths") {
        assert_eq!(clip_ticks(&lasting(number(&vector["seconds"]))), typed::<Ticks>(&vector["expected"]), "length {}", vector["id"]);
    }
    for vector in group(&vectors, "poses") {
        let clip = clip_of(&species, vector["clip"].as_str().unwrap_or_default());
        assert_eq!(entries(&vector["ticks"]).len(), entries(&vector["expected"]).len(), "pose {}", vector["id"]);
        for (tick, expected) in group(vector, "ticks").iter().zip(entries(&vector["expected"])) {
            assert_pose(&format!("pose {} tick {tick}", vector["id"]), &sample_clip(&species, clip, typed(tick)), &typed::<Pose>(expected));
        }
    }
    for vector in group(&vectors, "blends") {
        let (from, to): (Pose, Pose) = (typed(&vector["from"]), typed(&vector["to"]));
        assert_eq!(entries(&vector["amounts"]).len(), entries(&vector["expected"]).len(), "blend {}", vector["id"]);
        for (amount, expected) in group(vector, "amounts").iter().zip(entries(&vector["expected"])) {
            assert_pose(&format!("blend {} at {amount}", vector["id"]), &blend_pose(&from, &to, number(amount)), &typed::<Pose>(expected));
        }
    }
    let lids = &vectors["lids"];
    assert_eq!(BLINK_TICKS, typed::<Ticks>(&lids["expected"]["blinkTicks"]));
    assert_eq!(entries(&lids["ticks"]).len(), entries(&lids["expected"]["closures"]).len());
    for (tick, expected) in group(lids, "ticks").iter().zip(entries(&lids["expected"]["closures"])) {
        assert_near(&format!("lid at {tick}"), lid_at(typed(tick)), number(expected));
    }
}

#[test]
fn committed_spring_settling_vectors_are_reproduced_by_single_ticks() {
    let vectors = fixture("spring-settling");
    for vector in group(&vectors, "steps") {
        let (spring, target, stiffness, damping) = released(vector);
        assert_spring(&format!("step {}", vector["id"]), after(spring, target, stiffness, damping, 1), &vector["expected"]);
    }
    for vector in group(&vectors, "runs") {
        let (spring, target, stiffness, damping) = released(vector);
        assert_eq!(entries(&vector["ticks"]).len(), entries(&vector["expected"]).len(), "run {}", vector["id"]);
        for (ticks, expected) in group(vector, "ticks").iter().zip(entries(&vector["expected"])) {
            assert_spring(&format!("run {} after {ticks}", vector["id"]), after(spring, target, stiffness, damping, typed(ticks)), expected);
        }
    }
    for vector in group(&vectors, "rests") {
        let target = number(&vector["target"]);
        let rested = after(Spring { position: target, velocity: 0.0 }, target, number(&vector["stiffness"]), number(&vector["damping"]), typed(&vector["ticks"]));
        assert_eq!(rested, Spring { position: target, velocity: 0.0 }, "rest {}", vector["id"]);
        assert_spring(&format!("rest {}", vector["id"]), rested, &vector["expected"]);
    }
    for vector in group(&vectors, "stables") {
        let (spring, target, stiffness, damping) = released(vector);
        assert_spring(&format!("stable {}", vector["id"]), after(spring, target, stiffness, damping, typed(&vector["ticks"])), &vector["expected"]);
    }
}

#[test]
fn source_uses_nothing_but_the_exact_operations_of_the_design() {
    let code: Vec<&str> = SOURCE.lines().filter(|line| !line.trim_start().starts_with("//")).collect();
    for banned in [".sin(", ".cos(", ".tan(", ".atan2(", ".exp(", ".ln(", ".log", ".powf(", ".powi(", ".mul_add(", ".hypot(", ".cbrt(", ".sqrt(", ".max(", ".min(", ".clamp(", "f64::max", "f64::min", "std::time", "println!", "eprintln!", "dbg!"] {
        assert!(code.iter().all(|line| !line.contains(banned)), "{banned}");
    }
    for exact in [".floor()", "lerp(", "smoothstep(", "0.015625"] {
        assert!(code.iter().any(|line| line.contains(exact)), "{exact}");
    }
}
