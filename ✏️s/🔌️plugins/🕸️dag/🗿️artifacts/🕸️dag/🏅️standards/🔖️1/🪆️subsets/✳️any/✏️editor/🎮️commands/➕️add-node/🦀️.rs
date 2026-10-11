//! 🔧️ 🔧️ DAG play app commands command — `add-node`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// 🕹️ No longer auto-selects the newly-added node — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let scene = crate::dag_scene(doc)?;
    let id = crate::schema::next_node_id(&scene);
    let node = crate::schema::default_node_for_kind(&payload.kind, &id, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0));
    Ok(crate::dag_child_emit(doc.snapshot, vec![crate::create_node_leaf(&node)]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
