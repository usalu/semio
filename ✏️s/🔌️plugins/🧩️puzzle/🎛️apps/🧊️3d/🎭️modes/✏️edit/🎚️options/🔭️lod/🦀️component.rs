//! 🔭️ Edit-mode window option — the level-of-detail group: automatic-zoom and depth-variable
//! toggles plus the manual LOD slider whose range `setLodManual` clamps against.

use crate::apps::puzzle3d::config::Puzzle3dRuntime;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Constants
pub const PUZZLE3D_LOD_SLIDER_MIN: f64 = 0.0;
pub const PUZZLE3D_LOD_SLIDER_MAX: f64 = 1000.0;
//#endregion 🔖️Constants

pub fn measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod"),
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
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto"),
                icon_id: "zoom-in".into(),
                label: Some(labels.auto_zoom.into()),
                pressed: runtime.lod_automatic,
                text: None,
                on_change: puzzle3d_action("setLodAutomatic", None),
            },
            WindowMeasure::Toggle {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-depth-variable"),
                icon_id: "lod-depth".into(),
                label: Some(labels.depth_variable.into()),
                pressed: runtime.lod_depth_variable,
                text: None,
                on_change: puzzle3d_action("setLodDepthVariable", None),
            },
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-value"),
                label: Some(format!("{} {:.0}", labels.lod.as_str(), runtime.lod_manual)),
                value: runtime.lod_manual,
                min: PUZZLE3D_LOD_SLIDER_MIN,
                max: PUZZLE3D_LOD_SLIDER_MAX,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: puzzle3d_action("setLodManual", None),
            },
        ],
    }
}
