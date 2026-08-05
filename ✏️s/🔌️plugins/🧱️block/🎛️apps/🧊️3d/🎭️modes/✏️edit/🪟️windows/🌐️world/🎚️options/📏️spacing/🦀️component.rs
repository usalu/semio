//! 📏️ Block 3D play app — world window option: multi-representation arrangement spacing.

use crate::apps::block3d::config::{block3d_window_view, Block3dConfig};
use crate::apps::block3d::terminology::Block3dLabels;
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

pub fn measure(config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> WindowMeasure {
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
        on_change: crate::apps::block3d::block3d_action("setWindowSpacing", Some(json!({ "windowId": window_id }))),
    }
}
