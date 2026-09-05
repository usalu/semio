//! 📏️ Block 3D play app — world window option: multi-representation arrangement spacing.

use crate::editor::block3d::config::{block3d_window_view, Block3dConfig};
use crate::editor::block3d::terminology::Block3dLabels;
use semio_framework_plugin::WindowMeasure;

pub async fn measure(config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> WindowMeasure {
    let view = block3d_window_view(config, window_id);
    WindowMeasure::Slider {
        id: "block3d-spacing".into(),
        label: Some(labels.spacing.as_str().to_string()),
        value: view.spacing,
        min: 0.0,
        max: 40.0,
        step: Some(0.5),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::block3d::block3d_action(
            "setWindowSpacing",
            Some(crate::editor::block3d::ui_value_map([("windowId", crate::editor::block3d::ui_value_text(window_id).expect("window id fits ui text capacity"))]).expect("single-entry args fit ui map capacity")),
        ),
    }
}
