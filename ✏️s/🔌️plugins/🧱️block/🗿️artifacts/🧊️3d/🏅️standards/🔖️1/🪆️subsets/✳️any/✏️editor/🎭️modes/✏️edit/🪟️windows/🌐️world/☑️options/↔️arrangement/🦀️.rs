//! ↔️ Block 3D play app — world window option: multi-representation arrangement axis.

use semio_s_artifact_block_3d_editor_config::{block3d_window_view, Block3dConfig};
use semio_s_artifact_block_3d_editor_terminology::Block3dLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

pub fn measure(config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> WindowMeasure {
    let view = block3d_window_view(config, window_id);
    WindowMeasure::Select {
        id: "block3d-arrangement".into(),
        label: Some(labels.arrangement.as_str().to_string()),
        value: view.arrangement,
        items: vec![
            MeasureSelectItem { id: "overlap".into(), value: "overlap".into(), label: "Overlap".into() },
            MeasureSelectItem { id: "x".into(), value: "x".into(), label: "X".into() },
            MeasureSelectItem { id: "y".into(), value: "y".into(), label: "Y".into() },
            MeasureSelectItem { id: "z".into(), value: "z".into(), label: "Z".into() },
        ],
        on_change: crate::block3d_window_action("setWindowArrangement", Some(semio_framework_value::DslValue::object([("windowId".to_string(), semio_framework_value::DslValue::String(window_id.to_string()))]))),
    }
}
