//! 🧊️ Main-window utility — Volume Brush: Alt+click paints grid-snapped target volumes that constrain
//! where the Fill tool may place. Its Utility Options are the voxel width/depth/height steppers that
//! size each painted volume (in grid-spacing units).

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{puzzle3d_action, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{LabelText, LocalizedLabel, UtilityDefinition, WindowMeasure};
use dsl::json;

pub const UTILITY_ID: &str = "volumeBrush";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, label, "volume-brush")
}

/// 🧊️ Voxel width/depth/height measures for the Volume Brush utility.
pub fn voxel_dim_measures(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    let [w, d, h] = runtime.voxel_dims;
    let axis_slider = |axis: &str, label: LabelText, value: u32| WindowMeasure::Slider {
        id: format!("puzzle3d-voxel-{axis}"),
        label: Some(format!("{} {} {value}", labels.voxel.as_str(), label.as_str())),
        value: value as f64,
        min: 1.0,
        max: 64.0,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: puzzle3d_action("setVoxelDims", Some(json!({ "axis": axis }))),
    };
    vec![axis_slider("w", labels.width, w), axis_slider("d", labels.depth, d), axis_slider("h", labels.height, h)]
}

/// 🧊️ Utility Options for the Volume Brush utility — the voxel dimension sliders for Alt+click painting.
pub fn options(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush"),
        label: labels.volume_brush.into(),
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
        children: voxel_dim_measures(runtime, labels),
    }
}
