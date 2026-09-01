//! 🎯️ S Studio app — Workflow window's "active app instance" measure (a `Select` over every node).

use crate::engine::space::config::SpaceConfig;
use crate::engine::space::terminology::SStudioLabels;
use semio_framework_os::WorkflowNode;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};

//#region 🔖️Measure
pub async fn measure(config: &SpaceConfig, nodes: &[WorkflowNode], labels: &SStudioLabels) -> WindowMeasure {
    let value = config.active_node_id.clone().unwrap_or_default();
    WindowMeasure::Select {
        id: "s-media-active-instance".into(),
        label: Some(labels.active_app.into()),
        // 🕹️ Selection is the framework's `graph` interaction domain now (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `on_change` dispatches
        // `interactionSelect` instead of the deleted `selectInstance` action. Discovered gap: the
        // renderer's generic `Select` measure only ever merges a flat `value` key into `on_change.args`
        // (see `ShellHelpers`'s `onValueChange`), never the `targets` JSON blob `interactionSelect`
        // requires — this measure's on_change is therefore only correct for reselecting the CURRENT
        // value until a future wave teaches the renderer to build `targets` for measure selects too.
        on_change: crate::engine::space::space_interaction_select("instance", &value).await,
        value,
        items: nodes.iter().map(|node| MeasureSelectItem { id: node.id.clone(), value: node.id.clone(), label: node.label.clone() }).collect(),
    }
}
//#endregion 🔖️Measure
