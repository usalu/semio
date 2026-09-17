//! 🌐️ 3D-window option — the grid group: the visibility and snap toggles plus the spacing slider
//! (the pitch snapping rounds world placements onto). Per window instance, like every other pane
//! chrome measure.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_GRID_SPACING_MAX, PUZZLE5D_GRID_SPACING_MIN, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

/// 🌐️ `puzzle5d-play-world-grid` — visible / snap / spacing.
pub fn measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-grid"),
        label: labels.grid.into(),
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
        children: vec![
            WindowMeasure::Toggle {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-grid-visible"),
                icon_id: "layout-grid".into(),
                label: Some(labels.visible.into()),
                pressed: runtime.grid_visible,
                text: None,
                on_change: puzzle5d_action("setGridVisible", None),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-grid-snap"),
                icon_id: "magnet".into(),
                label: Some(labels.snap.into()),
                pressed: runtime.grid_snap_enabled,
                text: None,
                on_change: puzzle5d_action("setGridSnapEnabled", None),
            },
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-grid-spacing"),
                label: Some(format!("{} {:.1}", labels.spacing.as_str(), runtime.grid_spacing)),
                value: runtime.grid_spacing,
                min: PUZZLE5D_GRID_SPACING_MIN,
                max: PUZZLE5D_GRID_SPACING_MAX,
                step: Some(0.5),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: puzzle5d_action("setGridSpacing", None),
            },
        ],
    }
}
