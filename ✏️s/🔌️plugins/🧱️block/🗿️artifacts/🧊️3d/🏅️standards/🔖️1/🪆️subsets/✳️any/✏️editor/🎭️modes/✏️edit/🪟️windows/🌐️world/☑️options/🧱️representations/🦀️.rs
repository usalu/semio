//! 🧱️ Block 3D play app — world window option: per-representation visibility toggles.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::editor::block3d::config::{block3d_window_view, Block3dConfig};
use crate::editor::block3d::terminology::Block3dLabels;
use semio_framework_plugin::WindowMeasure;

pub fn measure(definition: &Block3dSnapshot, config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> WindowMeasure {
    let view = block3d_window_view(config, window_id);
    let visible_set: std::collections::HashSet<&str> = if view.representation_ids.is_empty() { definition.representations.iter().map(|r| r.id.as_str()).collect() } else { view.representation_ids.iter().map(|s| s.as_str()).collect() };
    let rep_toggles: Vec<WindowMeasure> = definition
        .representations
        .iter()
        .map(|representation| WindowMeasure::Toggle {
            id: format!("block3d-rep-{}", representation.id),
            icon_id: "box".into(),
            label: Some(representation.name.clone()),
            pressed: visible_set.contains(representation.id.as_str()),
            text: None,
            on_change: crate::editor::block3d::block3d_window_action(
                "toggleWindowRepresentation",
                Some(dsl::DslValue::object([
                    ("windowId".to_string(), dsl::DslValue::String(window_id.to_string())),
                    ("representationId".to_string(), dsl::DslValue::String(representation.id.clone())),
                    ("visible".to_string(), dsl::DslValue::Bool(!visible_set.contains(representation.id.as_str()))),
                ])),
            ),
        })
        .collect();
    WindowMeasure::measure_group("block3d-representations", labels.representations.as_str(), rep_toggles)
}
