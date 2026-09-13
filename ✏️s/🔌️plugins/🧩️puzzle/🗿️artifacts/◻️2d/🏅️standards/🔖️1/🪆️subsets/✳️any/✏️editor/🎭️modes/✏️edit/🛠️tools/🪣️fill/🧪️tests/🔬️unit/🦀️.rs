use super::*;
use crate::editor::puzzle2d::config::{Puzzle2dFillText, Puzzle2dPlayRuntime};
use crate::editor::puzzle2d::default_empty_fixture;
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::modes::edit::puzzle2d_engagement;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
use crate::editor::puzzle2d::unit_tests::context::*;

fn fill_children(runtime: Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
    let measure = measures(&scene(default_empty_fixture(), runtime, overview::utilities::select::UTILITY_ID), labels);
    let WindowMeasure::Group { children, .. } = measure else { panic!("fill group") };
    children
}

/// 🛠️ Fill's count entry is a tool measure keyed by the fill tool id, not a window utility-options group.
#[test]
fn fill_count_entry_is_a_tool_measure() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    let host = puzzle_board_host();
    let fill_runtime = Puzzle2dPlayRuntime { fill_count: 3, ..Puzzle2dPlayRuntime::default() };
    let fill_scene = scene(default_empty_fixture(), fill_runtime, overview::utilities::select::UTILITY_ID);
    let fill_measure = measures(&fill_scene, labels);
    assert!(matches!(&fill_measure, WindowMeasure::Group { id, active_utility_id: None, .. } if id == "puzzle2d-tool-options-fill"));
    assert!(!overview::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle2d-tool-options-fill")), "fill must no longer surface in window_measures");
    assert!(puzzle2d_engagement(&fill_scene, &host, overview::WINDOW_KIND_ID, labels).control.is_none(), "fill engagement HUD must no longer carry the relocated control");
}

/// ♾️ The count is a `Number` with no ceiling, and a count far past the deleted 1000-pin is carried
/// verbatim — a clamp reintroduced anywhere on this path would show up here as a lowered value.
#[test]
fn fill_count_entry_is_unbounded() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    let children = fill_children(Puzzle2dPlayRuntime { fill_count: 5_000, ..Puzzle2dPlayRuntime::default() }, labels);
    let Some(WindowMeasure::Number { id, value, min, max, step, .. }) = children.first() else { panic!("fill count entry") };
    assert_eq!(id, "puzzle2d-fill-count");
    assert_eq!(*value, 5_000.0);
    assert_eq!(*min, Some(0.0));
    assert_eq!(*max, None, "the fill count must carry no ceiling");
    assert_eq!(*step, Some(1.0));
}

/// 🎯️ A fresh board asks for 100 placements — the Rust default, the window-config default and the
/// value the entry renders all agree.
#[test]
fn fill_count_defaults_to_one_hundred() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    assert_eq!(PUZZLE2D_DEFAULT_FILL_COUNT, 100);
    assert_eq!(Puzzle2dPlayRuntime::default().fill_count, 100);
    assert_eq!(crate::editor::puzzle2d::window::Puzzle2dWindowConfig::default().fill_count, 100);
    let children = fill_children(Puzzle2dPlayRuntime::default(), labels);
    assert!(matches!(children.first(), Some(WindowMeasure::Number { value, .. }) if *value == 100.0));
}

fn toggle<'a>(children: &'a [WindowMeasure], toggle_id: &str) -> Option<&'a WindowMeasure> {
    children.iter().find(|measure| matches!(measure, WindowMeasure::Toggle { id, .. } if id == toggle_id))
}

/// 🛑️ An idle session publishes no cancel toggle; a live one publishes a cancel carrying the run's own
/// generation, with the localized stage as its text.
#[test]
fn live_fill_publishes_a_cancel_toggle_with_its_generation() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    assert!(toggle(&fill_children(Puzzle2dPlayRuntime::default(), labels), "puzzle2d-fill-cancel").is_none(), "an idle session must not offer to cancel nothing");

    let running = Puzzle2dPlayRuntime { fill_count: 9, fill_job_accepted_count: 4, fill_job_search_count: 17, fill_job_generation: 12, fill_job_lifecycle: Puzzle2dFillLifecycle::Running, ..Puzzle2dPlayRuntime::default() };
    let children = fill_children(running, labels);
    let Some(WindowMeasure::Toggle { label, text, pressed, on_change, .. }) = toggle(&children, "puzzle2d-fill-cancel") else { panic!("fill cancel toggle") };
    assert_eq!(label.as_deref(), Some("Cancel fill"));
    assert_eq!(text.as_deref(), Some("Searching"));
    assert!(!pressed);
    assert_eq!(on_change, &puzzle2d_action("brushFillSessionCancel", Some(serde_json::json!({ "generation": 12 }))));
    assert!(toggle(&children, "puzzle2d-fill-retry").is_none(), "a live run offers no retry");
}

/// 🗣️ Stage captions, the fault reason and the retry affordance stay accessible in German, and a
/// faulted run carries its machine code in the retry text rather than swallowing it.
#[test]
fn fill_toggles_localize_stage_fault_and_retry() {
    let german_view = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let labels = puzzle2d_labels(&german_view);
    let running = Puzzle2dPlayRuntime { fill_count: 9, fill_job_accepted_count: 4, fill_job_lifecycle: Puzzle2dFillLifecycle::Applying, ..Puzzle2dPlayRuntime::default() };
    let children = fill_children(running, labels);
    assert!(matches!(toggle(&children, "puzzle2d-fill-cancel"), Some(WindowMeasure::Toggle { label: Some(label), text: Some(text), .. }) if label == "Füllen abbrechen" && text == "Anwenden"));

    let faulted = Puzzle2dPlayRuntime { fill_job_lifecycle: Puzzle2dFillLifecycle::Faulted, fill_job_fault_code: Puzzle2dFillText::try_from_str("puzzle2d-fill-hostile"), ..Puzzle2dPlayRuntime::default() };
    let children = fill_children(faulted, labels);
    assert!(toggle(&children, "puzzle2d-fill-cancel").is_none(), "a stopped run must not offer to cancel nothing");
    let Some(WindowMeasure::Toggle { label: Some(label), text: Some(text), .. }) = toggle(&children, "puzzle2d-fill-retry") else { panic!("fill retry toggle") };
    assert_eq!(label, "Füllen erneut versuchen");
    assert!(text.starts_with("Füllen fehlgeschlagen"), "{text}");
    assert!(text.contains("puzzle2d-fill-hostile"), "{text}");
}
