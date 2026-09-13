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

/// ⏳️ An idle session publishes no progress row; a live one publishes stage, accepted-of-requested,
/// its counters and a cancel action carrying the run's own generation.
#[test]
fn live_fill_publishes_a_progress_row_with_cancel() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    assert!(!fill_children(Puzzle2dPlayRuntime::default(), labels).iter().any(|measure| matches!(measure, WindowMeasure::Progress { .. })), "an idle session must not claim progress");

    let running = Puzzle2dPlayRuntime { fill_count: 9, fill_job_accepted_count: 4, fill_job_search_count: 17, fill_job_generation: 12, fill_job_lifecycle: Puzzle2dFillLifecycle::Running, ..Puzzle2dPlayRuntime::default() };
    let children = fill_children(running, labels);
    let Some(WindowMeasure::Progress { id, stage, completed, total, steps, cancel, loading, .. }) = children.iter().find(|measure| matches!(measure, WindowMeasure::Progress { .. })) else { panic!("fill progress row") };
    assert_eq!(id, "puzzle2d-fill-progress");
    assert_eq!(stage.as_deref(), Some("Searching"));
    assert_eq!(*completed, 4.0);
    assert_eq!(*total, Some(9.0));
    assert_eq!(*loading, Some(true));
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].kind, MeasureProgressStepKind::Success);
    assert_eq!(steps[0].text, "accepted · 4");
    assert_eq!(steps[1].text, "tested · 17");
    let cancel = cancel.as_ref().expect("a live run must be cancellable");
    assert_eq!(cancel, &puzzle2d_action("brushFillSessionCancel", Some(serde_json::json!({ "generation": 12 }))));
}

/// 🗣️ Stage captions, the fault reason and the retry affordance stay accessible in German, and a
/// faulted run publishes its machine code as a danger step rather than swallowing it.
#[test]
fn fill_progress_localizes_stage_fault_and_retry() {
    let german_view = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let labels = puzzle2d_labels(&german_view);
    let running = Puzzle2dPlayRuntime { fill_count: 9, fill_job_accepted_count: 4, fill_job_lifecycle: Puzzle2dFillLifecycle::Applying, ..Puzzle2dPlayRuntime::default() };
    let children = fill_children(running, labels);
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Progress { label: Some(label), stage: Some(stage), .. } if label == "Füllfortschritt" && stage == "Anwenden")));

    let faulted = Puzzle2dPlayRuntime { fill_job_lifecycle: Puzzle2dFillLifecycle::Faulted, fill_job_fault_code: Puzzle2dFillText::try_from_str("puzzle2d-fill-hostile"), ..Puzzle2dPlayRuntime::default() };
    let children = fill_children(faulted, labels);
    let Some(WindowMeasure::Progress { stage: Some(stage), steps, cancel, .. }) = children.iter().find(|measure| matches!(measure, WindowMeasure::Progress { .. })) else { panic!("fill progress row") };
    assert!(stage.starts_with("Füllen fehlgeschlagen"), "{stage}");
    assert!(stage.contains("puzzle2d-fill-hostile"), "{stage}");
    assert!(cancel.is_none(), "a stopped run must not offer to cancel nothing");
    assert!(steps.iter().any(|step| step.kind == MeasureProgressStepKind::Danger && step.text == "puzzle2d-fill-hostile"));
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, label: Some(label), .. } if id == "puzzle2d-fill-retry" && label == "Füllen erneut versuchen")));
}
