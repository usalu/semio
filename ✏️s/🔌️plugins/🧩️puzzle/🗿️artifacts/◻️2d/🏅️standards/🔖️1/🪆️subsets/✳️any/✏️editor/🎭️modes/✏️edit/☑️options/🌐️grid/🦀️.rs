//! 🌐️ Edit-mode window option — the grid group: the snap toggle and the grid factor slider the board
//! engine snaps node drags and brush placements onto. Per window instance (`Puzzle2dWindowConfig`).

use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

pub const PUZZLE2D_GRID_FACTOR_MIN: f64 = 0.25;
pub const PUZZLE2D_GRID_FACTOR_MAX: f64 = 8.0;

pub fn measure(runtime: &Puzzle2dPlayRuntime, labels: &Puzzle2dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-grid"),
        label: labels.grid.into(),
        default_open: Some(false),
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
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-grid-visible"),
                icon_id: "layout-grid".into(),
                label: Some(labels.visible.into()),
                pressed: runtime.grid_visible,
                text: None,
                on_change: puzzle2d_action("setGridVisible", None),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-grid-snap"),
                icon_id: "magnet".into(),
                label: Some(labels.grid_snap.into()),
                pressed: runtime.grid_snap_enabled,
                text: None,
                on_change: puzzle2d_action("setGridSnapEnabled", None),
            },
            WindowMeasure::Slider {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-grid-factor"),
                label: Some(format!("{} {:.2}", labels.grid_factor.as_str(), runtime.grid_factor)),
                value: runtime.grid_factor,
                min: PUZZLE2D_GRID_FACTOR_MIN,
                max: PUZZLE2D_GRID_FACTOR_MAX,
                step: Some(0.25),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: puzzle2d_action("setGridFactor", None),
            },
        ],
    }
}
