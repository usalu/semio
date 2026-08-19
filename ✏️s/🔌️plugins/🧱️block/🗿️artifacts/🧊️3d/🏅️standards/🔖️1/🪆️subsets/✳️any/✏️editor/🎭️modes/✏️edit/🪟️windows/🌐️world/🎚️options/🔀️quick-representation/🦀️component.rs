//! 🔀️ Block 3D play app — world window option: the single-pick representation quick-select.

use crate::editor::block3d::config::{block3d_window_view, Block3dConfig};
use crate::editor::block3d::terminology::Block3dLabels;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};
use serde_json::json;

pub async fn measure(definition: &Block3dSnapshot, config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> WindowMeasure {
    let view = block3d_window_view(config, window_id);
    let mut quick_items = vec![MeasureSelectItem { id: "all".into(), value: String::new(), label: labels.show_all.as_str().to_string() }];
    quick_items.extend(definition.representations.iter().map(|representation| MeasureSelectItem { id: representation.id.clone(), value: representation.id.clone(), label: representation.name.clone() }));
    let quick_value = view.representation_ids.first().cloned().unwrap_or_default();
    WindowMeasure::Select {
        id: "block3d-rep-quick".into(),
        label: Some(labels.representation.as_str().to_string()),
        value: quick_value,
        items: quick_items,
        on_change: crate::editor::block3d::block3d_action("setWindowRepresentations", Some(json!({ "windowId": window_id }))),
    }
}
