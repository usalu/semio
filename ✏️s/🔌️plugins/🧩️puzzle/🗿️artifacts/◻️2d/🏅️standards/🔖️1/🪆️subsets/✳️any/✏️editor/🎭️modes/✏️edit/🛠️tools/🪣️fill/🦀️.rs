//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count slider
//! is a mode-level *tool* measure keyed by the tool id rather than a window utility-options group.

use crate::editor::puzzle2d::config::{Puzzle2dFillLifecycle, Puzzle2dFillText};
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, Puzzle2dScene};
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
pub const PUZZLE2D_FILL_COUNT_MAX: u32 = 1000;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    semio_framework::io::resolve_ready(ToolDefinition::new(TOOL_ID, label, "paint-bucket"))
}

/// 🎚️ The fill-count slider, surfaced in the mode-level tool panel while the fill tool is active.
pub fn measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let lifecycle = envelope.runtime.fill_job_lifecycle;
    let accepted = envelope.runtime.fill_job_accepted_count as f64;
    let running = matches!(
        lifecycle,
        Puzzle2dFillLifecycle::Capturing
            | Puzzle2dFillLifecycle::Queued
            | Puzzle2dFillLifecycle::Running
            | Puzzle2dFillLifecycle::CheckpointReady
            | Puzzle2dFillLifecycle::Applying
            | Puzzle2dFillLifecycle::AwaitingAdoption
            | Puzzle2dFillLifecycle::Closing
    );
    let waiting = matches!(lifecycle, Puzzle2dFillLifecycle::Capturing | Puzzle2dFillLifecycle::Queued | Puzzle2dFillLifecycle::CheckpointReady | Puzzle2dFillLifecycle::AwaitingAdoption | Puzzle2dFillLifecycle::Closing);
    let status = match lifecycle {
        Puzzle2dFillLifecycle::Faulted => match envelope.runtime.fill_job_fault_code.as_deref() {
            Some(code) => format!("{}: {}", labels.fill_fault.as_str(), code),
            None => labels.fill_fault.into(),
        },
        Puzzle2dFillLifecycle::Completed => format!("{}: {}", labels.fill_result.as_str(), envelope.runtime.fill_job_accepted_count),
        _ => format!("{}: {}/{}", labels.fill_progress.as_str(), envelope.runtime.fill_job_accepted_count, envelope.runtime.fill_count),
    };
    let mut children = vec![WindowMeasure::Slider {
        id: "puzzle2d-fill-count".into(),
        label: Some(status),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE2D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        ready: running.then_some(accepted),
        loading: running.then_some(!waiting),
        waiting: running.then_some(waiting),
        disabled: None,
        reveal: None,
        on_change: puzzle2d_action("setFillCount", None),
    }];
    if running {
        children.push(WindowMeasure::Toggle {
            id: "puzzle2d-fill-cancel".into(),
            icon_id: "x".into(),
            label: Some(labels.fill_cancel.into()),
            pressed: false,
            text: None,
            on_change: puzzle2d_action("brushFillSessionCancel", Some(serde_json::json!({ "generation": envelope.runtime.fill_job_generation }))),
        });
    }
    if matches!(lifecycle, Puzzle2dFillLifecycle::Faulted | Puzzle2dFillLifecycle::Cancelled) {
        children.push(WindowMeasure::Toggle { id: "puzzle2d-fill-retry".into(), icon_id: "refresh-cw".into(), label: Some(labels.fill_retry.into()), pressed: false, text: None, on_change: puzzle2d_action("brushFillSessionRetry", None) });
    }
    WindowMeasure::Group {
        id: "puzzle2d-tool-options-fill".into(),
        label: labels.fill.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children,
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle2d::config::Puzzle2dConfig;
    use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
    use crate::editor::puzzle2d::default_empty_fixture;
    use crate::editor::puzzle2d::engine::board_host::puzzle_board_host;
    use crate::editor::puzzle2d::modes::edit::puzzle2d_engagement;
    use crate::editor::puzzle2d::modes::edit::windows::overview;
    use crate::editor::puzzle2d::terminology::puzzle2d_labels;
    use crate::editor::puzzle2d::testkit::*;

    /// 🛠️ Fill's count slider is a tool measure keyed by the fill tool id, not a window utility-options group.
    #[test]
    fn fill_count_slider_is_a_tool_measure() {
        let labels = puzzle2d_labels(&Puzzle2dConfig::default());
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
        let mut running_config = Puzzle2dConfig::default();
        running_config.locale = "de-DE".into();
        running_config.fill_count = 9;
        running_config.fill_job_accepted_count = 4;
        running_config.fill_job_generation = 12;
        running_config.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
        let running_labels = puzzle2d_labels(&running_config);
        let running_scene = scene(default_empty_fixture(), running_config, overview::utilities::select::UTILITY_ID);
        let running_measure = measures(&running_scene, running_labels);
        let WindowMeasure::Group { children, .. } = running_measure else { panic!("fill group") };
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Slider { label: Some(label), ready: Some(4.0), loading: Some(true), .. } if label.contains("Füllfortschritt"))));
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, label: Some(label), .. } if id == "puzzle2d-fill-cancel" && label == "Füllen abbrechen")));

        let mut fault_config = Puzzle2dConfig::default();
        fault_config.locale = "de-DE".into();
        fault_config.fill_job_lifecycle = Puzzle2dFillLifecycle::Faulted;
        fault_config.fill_job_fault_code = Puzzle2dFillText::try_from_str("puzzle2d-fill-hostile");
        let fault_labels = puzzle2d_labels(&fault_config);
        let fault_scene = scene(default_empty_fixture(), fault_config, overview::utilities::select::UTILITY_ID);
        let fault_measure = measures(&fault_scene, fault_labels);
        let WindowMeasure::Group { children, .. } = fault_measure else { panic!("fill group") };
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Slider { label: Some(label), .. } if label.contains("Füllen fehlgeschlagen"))));
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, label: Some(label), .. } if id == "puzzle2d-fill-retry" && label == "Füllen erneut versuchen")));
    }
}
//#endregion 🧪️Tests
