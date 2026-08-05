//! 🪣️ Edit-mode tool — Fill: a whole-document generator (not a window utility), so its count slider
//! is a mode-level *tool* measure keyed by the tool id rather than a window utility-options group.

use crate::apps::puzzle2d::terminology::Puzzle2dLabels;
use crate::apps::puzzle2d::{puzzle2d_action, Puzzle2dScene};
use semio_framework_plugin::{LocalizedLabel, ToolDefinition, WindowMeasure};

//#region 🔖️Constants
pub const TOOL_ID: &str = "fill";
pub const PUZZLE2D_FILL_COUNT_MAX: u32 = 1000;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> ToolDefinition {
    ToolDefinition::new(TOOL_ID, label, "paint-bucket")
}

/// 🎚️ The fill-count slider, surfaced in the mode-level tool panel while the fill tool is active.
pub fn measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
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
        children: vec![WindowMeasure::Slider {
            id: "puzzle2d-fill-count".into(),
            label: Some(labels.count.into()),
            value: envelope.runtime.fill_count as f64,
            min: 0.0,
            max: PUZZLE2D_FILL_COUNT_MAX as f64,
            step: Some(1.0),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle2d_action("setFillCount", None),
        }],
    }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle2d::config::Puzzle2dPlayRuntime;
    use crate::apps::puzzle2d::modes::edit::puzzle2d_engagement;
    use crate::apps::puzzle2d::modes::edit::windows::overview;
    use crate::apps::puzzle2d::terminology::puzzle2d_labels;
    use crate::apps::puzzle2d::testkit::*;
    use crate::apps::puzzle2d::config::Puzzle2dConfig;
    use crate::apps::puzzle2d::default_empty_fixture;
    use crate::artifacts::puzzle2d::engine::board_host::puzzle_board_host;

    /// 🛠️ Fill's count slider is a tool measure keyed by the fill tool id, not a window utility-options group.
    #[test]
    fn fill_count_slider_is_a_tool_measure() {
        let labels = puzzle2d_labels(&Puzzle2dConfig::default());
        let host = puzzle_board_host();
        let fill_runtime = Puzzle2dPlayRuntime { fill_count: 3, ..Puzzle2dPlayRuntime::default() };
        let fill_scene = scene(default_empty_fixture(), fill_runtime, overview::utilities::select::UTILITY_ID);
        let fill_measure = measures(&fill_scene, labels);
        assert!(matches!(&fill_measure, WindowMeasure::Group { id, active_utility_id: None, .. } if id == "puzzle2d-tool-options-fill"));
        assert!(
            !overview::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle2d-tool-options-fill")),
            "fill must no longer surface in window_measures"
        );
        assert!(puzzle2d_engagement(&fill_scene, &host, overview::WINDOW_KIND_ID, labels).control.is_none(), "fill engagement HUD must no longer carry the relocated control");
    }
}
//#endregion 🧪️Tests
