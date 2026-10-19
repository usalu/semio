//! 🎯️ S Studio app — Workflow window's "active app instance" measure (a `Select` over every node).

use semio_hub_space_foundation::config::SpaceConfig;
use semio_hub_space_foundation::terminology::SStudioLabels;
use semio_framework_os::WorkflowNode;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub async fn measure(config: &SpaceConfig, nodes: &[WorkflowNode], labels: &SStudioLabels) -> WindowMeasure {
    let value = config.active_node_id.clone().unwrap_or_default();
    WindowMeasure::Select {
        id: "s-media-active-instance".into(),
        label: Some(labels.active_app.into()),
        on_change: crate::space_interaction_select("instance", &value).await,
        value,
        items: nodes.iter().map(|node| MeasureSelectItem { id: node.id.clone(), value: node.id.clone(), label: node.label.clone() }).collect(),
    }
}
//#endregion 🔖️Measure
