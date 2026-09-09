use super::*;
use crate::editor::puzzle2d::config::{Puzzle2dFillText, Puzzle2dPlayRuntime};
use crate::editor::puzzle2d::default_empty_fixture;
use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
use crate::editor::puzzle2d::modes::edit::puzzle2d_engagement;
use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::terminology::puzzle2d_labels;
use crate::editor::puzzle2d::testkit::*;

/// 🛠️ Fill's count slider is a tool measure keyed by the fill tool id, not a window utility-options group.
#[test]
fn fill_count_slider_is_a_tool_measure() {
    let labels = puzzle2d_labels(&semio_framework_plugin::ViewModel::default());
    let host = puzzle_board_host();
    let fill_runtime = Puzzle2dPlayRuntime { fill_count: 3, ..Puzzle2dPlayRuntime::default() };
    let fill_scene = scene(default_empty_fixture(), fill_runtime, overview::utilities::select::UTILITY_ID);
    let fill_measure = measures(&fill_scene, labels);
    assert!(matches!(&fill_measure, WindowMeasure::Group { id, active_utility_id: None, .. } if id == "puzzle2d-tool-options-fill"));
    assert!(!overview::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle2d-tool-options-fill")), "fill must no longer surface in window_measures");
    assert!(puzzle2d_engagement(&fill_scene, &host, overview::WINDOW_KIND_ID, labels).control.is_none(), "fill engagement HUD must no longer carry the relocated control");
}

/// 🗣️ Mounted fill progress, cancellation, faults, and retry remain accessible in German.
#[test]
fn mounted_fill_measure_localizes_progress_cancel_fault_and_retry() {
    let mut running_config = Puzzle2dPlayRuntime::default();
    running_config.fill_count = 9;
    running_config.fill_job_accepted_count = 4;
    running_config.fill_job_generation = 12;
    running_config.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
    let german_view = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let running_labels = puzzle2d_labels(&german_view);
    let running_scene = scene(default_empty_fixture(), running_config, overview::utilities::select::UTILITY_ID);
    let running_measure = measures(&running_scene, running_labels);
    let WindowMeasure::Group { children, .. } = running_measure else { panic!("fill group") };
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Slider { label: Some(label), ready: Some(4.0), loading: Some(true), .. } if label.contains("Füllfortschritt"))));
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, label: Some(label), .. } if id == "puzzle2d-fill-cancel" && label == "Füllen abbrechen")));

    let mut fault_config = Puzzle2dPlayRuntime::default();
    fault_config.fill_job_lifecycle = Puzzle2dFillLifecycle::Faulted;
    fault_config.fill_job_fault_code = Puzzle2dFillText::try_from_str("puzzle2d-fill-hostile");
    let fault_labels = puzzle2d_labels(&german_view);
    let fault_scene = scene(default_empty_fixture(), fault_config, overview::utilities::select::UTILITY_ID);
    let fault_measure = measures(&fault_scene, fault_labels);
    let WindowMeasure::Group { children, .. } = fault_measure else { panic!("fill group") };
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Slider { label: Some(label), .. } if label.contains("Füllen fehlgeschlagen"))));
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, label: Some(label), .. } if id == "puzzle2d-fill-retry" && label == "Füllen erneut versuchen")));
}
