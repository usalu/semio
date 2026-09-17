//! 🧊️ 3D-window utility — Volume Brush: Alt+click paints grid-snapped target volumes that constrain
//! where the Fill tool may place. Its Utility Options are the voxel width/depth/height steppers that
//! size each painted volume (in grid-spacing units). Bound only by the 3D world window — a target
//! volume is a pose-space box, and the flat board paints its projection rather than painting it.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_VOXEL_DIM_MAX, PUZZLE5D_VOXEL_DIM_MIN};
use dsl::os_pack::json::object;
use semio_framework_plugin::{LabelText, LocalizedLabel, UtilityDefinition, WindowMeasure};

pub const UTILITY_ID: &str = "volumeBrush";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, label, "volume-brush")
}

/// 🧊️ Voxel width/depth/height measures for the Volume Brush utility.
pub fn voxel_dim_measures(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let [w, d, h] = runtime.voxel_dims;
    let axis_slider = |axis: &str, label: LabelText, value: u32| WindowMeasure::Slider {
        id: format!("puzzle5d-voxel-{axis}"),
        label: Some(format!("{} {} {value}", labels.voxel.as_str(), label.as_str())),
        value: f64::from(value),
        min: PUZZLE5D_VOXEL_DIM_MIN,
        max: PUZZLE5D_VOXEL_DIM_MAX,
        step: Some(1.0),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        on_change: puzzle5d_action("setVoxelDims", Some(object([("axis".to_string(), axis.into())]))),
    };
    vec![axis_slider("w", labels.width, w), axis_slider("d", labels.depth, d), axis_slider("h", labels.height, h)]
}

/// 🧊️ Utility Options for the Volume Brush utility — the voxel dimension sliders for Alt+click painting.
pub fn options(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-volume-brush"),
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
