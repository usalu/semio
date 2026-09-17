//! 🌐️ 2D-window option — the board pane's grid group: the visibility and snap toggles plus the snap
//! FACTOR slider the board engine multiplies its LOD-derived snap step by. The world pane's grid
//! slider is an absolute spacing in metres instead (`🧊️3d/☑️options/🌐️grid`) — different state,
//! different verb, so neither file is a copy of the other.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_GRID_FACTOR_MAX, PUZZLE5D_GRID_FACTOR_MIN, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

/// 🌐️ `puzzle5d-play-board-grid` — visible / snap / factor.
pub fn measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-board-grid"),
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
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-board-grid-visible"),
                icon_id: "layout-grid".into(),
                label: Some(labels.visible.into()),
                pressed: runtime.grid_visible,
                text: None,
                on_change: puzzle5d_action("setGridVisible", None),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-board-grid-snap"),
                icon_id: "magnet".into(),
                label: Some(labels.snap.into()),
                pressed: runtime.grid_snap_enabled,
                text: None,
                on_change: puzzle5d_action("setGridSnapEnabled", None),
            },
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-board-grid-factor"),
                label: Some(format!("{} {:.2}", labels.factor.as_str(), runtime.grid_factor)),
                value: runtime.grid_factor,
                min: PUZZLE5D_GRID_FACTOR_MIN,
                max: PUZZLE5D_GRID_FACTOR_MAX,
                step: Some(0.25),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: puzzle5d_action("setGridFactor", None),
            },
        ],
    }
}
