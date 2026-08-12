//! 🎯️ S Studio app — Workflow window's "active app instance" measure (a `Select` over every node).

use crate::apps::space::config::SpaceConfig;
use crate::apps::space::terminology::SStudioLabels;
use semio_framework_os::WorkflowNode;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub fn measure(config: &SpaceConfig, nodes: &[WorkflowNode], labels: &SStudioLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: "s-media-active-instance".into(),
        label: Some(labels.active_app.into()),
        value: config.active_node_id.clone().unwrap_or_default(),
        items: nodes.iter().map(|node| MeasureSelectItem { id: node.id.clone(), value: node.id.clone(), label: node.label.clone() }).collect(),
        on_change: crate::apps::space::s_play_action("selectInstance", None),
    }
}
//#endregion 🔖️Measure
