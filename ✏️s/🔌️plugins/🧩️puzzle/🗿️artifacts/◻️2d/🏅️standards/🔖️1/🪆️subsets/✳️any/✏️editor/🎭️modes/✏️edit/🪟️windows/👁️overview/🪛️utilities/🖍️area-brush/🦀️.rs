//! 🖍️ Overview-window utility — Area Brush: alt+click paints grid-snapped target regions that
//! constrain where the Fill tool may place. Its Utility Options are the width/height steppers that
//! size each painted region (in grid cells) — the 2d twin of puzzle3d's `🧊️volume-brush`, whose
//! voxel W/D/H trio collapses to a W/H pair on a flat board.

use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{LabelText, LocalizedLabel, UtilityCategory, UtilityDefinition, WindowMeasure};
use serde_json::json;

pub const UTILITY_ID: &str = "areaBrush";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(UTILITY_ID, label, "square-dashed") }
}

/// 🖍️ Width/height measures for the Area Brush utility, in whole grid cells.
pub fn extent_measures(runtime: &Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
    let axis_slider = |axis: &str, label: LabelText, value: f64| WindowMeasure::Slider {
        id: format!("puzzle2d-area-brush-{axis}"),
        label: Some(format!("{} {} {value}", labels.area_brush.as_str(), label.as_str())),
        value,
        min: 1.0,
        max: 64.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        on_change: puzzle2d_action("setAreaBrushSize", Some(json!({ "axis": axis }))),
    };
    vec![axis_slider("w", labels.width, runtime.area_brush_width), axis_slider("h", labels.height, runtime.area_brush_height)]
}

/// 🖍️ Utility Options for the Area Brush — the extent steppers for alt+click painting.
pub fn options(runtime: &Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-utility-options-area-brush"),
        label: labels.area_brush.into(),
        default_open: Some(true),
        active_utility_id: Some(UTILITY_ID.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: extent_measures(runtime, labels),
    }
}
