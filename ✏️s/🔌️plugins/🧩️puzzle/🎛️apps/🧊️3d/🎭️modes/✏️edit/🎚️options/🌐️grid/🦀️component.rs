//! 🌐️ Edit-mode window option — the grid group: visibility and snap toggles plus the spacing
//! slider (the same spacing `addTargetVolume` snaps new voxel volumes onto).

use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

pub fn measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid"),
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
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible"),
                icon_id: "layout-grid".into(),
                label: Some(labels.visible.into()),
                pressed: runtime.grid_visible,
                text: None,
                on_change: puzzle3d_action("setGridVisible", None),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-snap"),
                icon_id: "magnet".into(),
                label: Some(labels.snap.into()),
                pressed: runtime.grid_snap_enabled,
                text: None,
                on_change: puzzle3d_action("setGridSnapEnabled", None),
            },
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing"),
                label: Some(format!("{} {:.1}", labels.spacing.as_str(), runtime.grid_spacing)),
                value: runtime.grid_spacing,
                min: 0.5,
                max: 50.0,
                step: Some(0.5),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: puzzle3d_action("setGridSpacing", None),
            },
        ],
    }
}
