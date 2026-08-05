//! 🧱️ Block 3D play app — world window option: per-representation visibility toggles.

use crate::apps::block3d::config::{block3d_window_view, Block3dConfig};
use crate::apps::block3d::terminology::Block3dLabels;
use crate::artifacts::block3d::Block3dDefinition;
use semio_framework_plugin::WindowMeasure;
use serde_json::json;

pub fn measure(definition: &Block3dDefinition, config: &Block3dConfig, window_id: &str, labels: &Block3dLabels) -> WindowMeasure {
    let view = block3d_window_view(config, window_id);
    let visible_set: std::collections::HashSet<&str> = if view.representation_ids.is_empty() {
        definition.representations.iter().map(|r| r.id.as_str()).collect()
    } else {
        view.representation_ids.iter().map(|s| s.as_str()).collect()
    };
    let rep_toggles: Vec<WindowMeasure> = definition
        .representations
        .iter()
        .map(|representation| WindowMeasure::Toggle {
            id: format!("block3d-rep-{}", representation.id),
            icon_id: "box".into(),
            label: Some(representation.name.clone()),
            pressed: visible_set.contains(representation.id.as_str()),
            text: None,
            on_change: crate::apps::block3d::block3d_action(
                "toggleWindowRepresentation",
                Some(json!({ "windowId": window_id, "representationId": representation.id, "visible": !visible_set.contains(representation.id.as_str()) })),
            ),
        })
        .collect();
    WindowMeasure::measure_group("block3d-representations", labels.representations.as_str(), rep_toggles)
}
