//! 🔭️ 3D-window option — the level-of-detail group: the automatic-zoom and depth-variable toggles
//! plus the manual LOD slider whose range `setLodManual` clamps against. The board pane's LOD is a
//! tier SELECT over the board engine's own scale table (`◻️2d/☑️options/🔭️lod`), a different control
//! over different state, so neither file is a copy of the other.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_LOD_SLIDER_MAX, PUZZLE5D_LOD_SLIDER_MIN, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

/// 🔭️ `puzzle5d-play-world-lod` — auto / depth-variable / manual value.
pub fn measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-lod"),
        label: labels.lod.into(),
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
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-lod-auto"),
                icon_id: "zoom-in".into(),
                label: Some(labels.auto_zoom.into()),
                pressed: runtime.lod_automatic,
                text: None,
                on_change: puzzle5d_action("setLodAutomatic", None),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-lod-depth-variable"),
                icon_id: "lod-depth".into(),
                label: Some(labels.depth_variable.into()),
                pressed: runtime.lod_depth_variable,
                text: None,
                on_change: puzzle5d_action("setLodDepthVariable", None),
            },
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-world-lod-value"),
                label: Some(format!("{} {:.0}", labels.lod.as_str(), runtime.lod_manual)),
                value: runtime.lod_manual,
                min: PUZZLE5D_LOD_SLIDER_MIN,
                max: PUZZLE5D_LOD_SLIDER_MAX,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: puzzle5d_action("setLodManual", None),
            },
        ],
    }
}
