//! 🧪 Chrome motion laws over the shared language-agnostic fixture.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../🎨️styling/🧪️tests/🧫️fixtures/🎞️chrome-motion/🔣️.json")).unwrap()
}

fn points(value: &Value) -> [f32; 4] {
    let array = value.as_array().unwrap();
    [array[0].as_f64().unwrap() as f32, array[1].as_f64().unwrap() as f32, array[2].as_f64().unwrap() as f32, array[3].as_f64().unwrap() as f32]
}

#[test]
fn cubic_bezier_matches_the_independent_samples() {
    for (name, curve) in fixture()["curves"].as_object().unwrap() {
        for sample in curve["samples"].as_array().unwrap() {
            let (t, expect) = (sample[0].as_f64().unwrap() as f32, sample[1].as_f64().unwrap() as f32);
            assert!((cubic_bezier(points(&curve["points"]), t) - expect).abs() < 1e-4, "{name} at {t}");
        }
    }
}

#[test]
fn transitions_follow_the_fixture_scenarios() {
    for case in fixture()["transitions"].as_array().unwrap() {
        let mut transitions = Transitions::default();
        let duration = case["durationSeconds"].as_f64().unwrap() as f32;
        let easing = points(&case["easing"]);
        let reduced = case["reduced"].as_bool().unwrap();
        for step in case["steps"].as_array().unwrap() {
            let value = transitions.value(7, step["target"].as_f64().unwrap() as f32, step["at"].as_f64().unwrap() as f32, duration, easing, reduced);
            assert!((value - step["expect"].as_f64().unwrap() as f32).abs() < 1e-4, "{} at {}", case["name"], step["at"]);
        }
    }
}

#[test]
fn enter_progress_follows_the_fixture_vectors() {
    for case in fixture()["enter"].as_array().unwrap() {
        let value = enter_progress(case["elapsedSeconds"].as_f64().unwrap() as f32, case["durationSeconds"].as_f64().unwrap() as f32, points(&case["easing"]), case["reduced"].as_bool().unwrap());
        assert!((value - case["expect"].as_f64().unwrap() as f32).abs() < 1e-4);
    }
}

#[test]
fn introduced_pulse_opacity_follows_the_fixture_vectors() {
    for case in fixture()["introducedPulse"].as_array().unwrap() {
        assert!((introduced_pulse_opacity(case["phase"].as_f64().unwrap() as f32) - case["expect"].as_f64().unwrap() as f32).abs() < 1e-5);
    }
}

#[test]
fn the_tokens_equal_the_fixture() {
    use ui_styling::metrics::chrome;
    let tokens = &fixture()["tokens"];
    assert_eq!(chrome::TRANSITION_DURATION_MS, tokens["transitionDurationMs"].as_f64().unwrap());
    assert_eq!(chrome::DIALOG_ENTER_DURATION_MS, tokens["dialogEnterDurationMs"].as_f64().unwrap());
    assert_eq!(chrome::DIALOG_ENTER_SCALE_FROM, tokens["dialogEnterScaleFrom"].as_f64().unwrap());
    assert_eq!(chrome::TRANSITION_EASING, &[0.4, 0.0, 0.2, 1.0][..]);
    assert_eq!(chrome::DIALOG_ENTER_EASING, &[0.25, 0.1, 0.25, 1.0][..]);
    assert_eq!(chrome::LOADING_BORDER_DURATION_SECONDS, tokens["loadingBorderDurationSeconds"].as_f64().unwrap());
    assert_eq!(chrome::WAITING_BORDER_DURATION_SECONDS, tokens["waitingBorderDurationSeconds"].as_f64().unwrap());
    assert_eq!(chrome::INTRODUCED_BORDER_DURATION_SECONDS, tokens["introducedBorderDurationSeconds"].as_f64().unwrap());
    assert_eq!(chrome::CELEBRATE_BORDER_DURATION_SECONDS, tokens["celebrateBorderDurationSeconds"].as_f64().unwrap());
}

#[test]
fn reduced_motion_paints_every_animated_border_as_the_static_ring() {
    use crate::{DrawList, SharedBorderState, KIND_FINISHED_BORDER, KIND_INTRODUCING_BORDER, KIND_LOADING_BORDER, KIND_WAITING_BORDER};
    use semio_ui_theme::{geometry::Rect, Rgba};
    for (state, animated) in [(SharedBorderState::Loading, KIND_LOADING_BORDER), (SharedBorderState::Waiting, KIND_WAITING_BORDER), (SharedBorderState::Introduced, KIND_INTRODUCING_BORDER)] {
        for (reduced, expected) in [(false, animated), (true, KIND_FINISHED_BORDER)] {
            let mut draw = DrawList::default();
            draw.set_reduced_motion(reduced);
            draw.push_shared_border_effect(Rect::new(0.0, 0.0, 10.0, 10.0), Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 }, 0.0, 1.0, state);
            while !draw.finalize_shared_borders_step().unwrap() {}
            let kinds: Vec<f32> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).map(|item| item.params[2]).collect();
            assert!(!kinds.is_empty() && kinds.iter().all(|kind| *kind == expected), "{state:?} reduced={reduced}");
        }
    }
    let mut draw = DrawList::default();
    draw.push_shared_border_effect(Rect::new(0.0, 0.0, 10.0, 10.0), Rgba::TRANSPARENT, 0.0, 1.0, SharedBorderState::Celebrated);
    while !draw.finalize_shared_borders_step().unwrap() {}
    assert!(draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0).all(|item| item.params[2] == KIND_FINISHED_BORDER), "celebrate is a colour cycle, never a pulse");
}

#[test]
fn an_enter_starts_at_first_sight_and_a_forgotten_one_starts_again() {
    let mut transitions = Transitions::default();
    let easing = [0.25, 0.1, 0.25, 1.0];
    assert_eq!(transitions.enter(1 << 32, 5.0, 0.2, easing, false), 0.0);
    assert!((transitions.enter(1 << 32, 5.1, 0.2, easing, false) - 0.802403).abs() < 1e-4);
    assert_eq!(transitions.enter(1 << 32, 5.3, 0.2, easing, false), 1.0);
    transitions.forget_enters(1);
    assert_eq!(transitions.enter(1 << 32, 6.0, 0.2, easing, false), 0.0);
    assert_eq!(transitions.enter(1 << 32, 6.0, 0.2, easing, true), 1.0);
}

#[test]
fn dialog_zoom_runs_from_the_enter_scale_to_full_and_back() {
    assert_eq!(dialog_zoom(0.0, 0.95), 0.95);
    assert_eq!(dialog_zoom(1.0, 0.95), 1.0);
    assert!((dialog_zoom(0.5, 0.95) - 0.975).abs() < 1e-6);
    assert_eq!(dialog_zoom(2.0, 0.95), 1.0);
    assert_eq!(dialog_zoom(-1.0, 0.95), 0.95);
}
