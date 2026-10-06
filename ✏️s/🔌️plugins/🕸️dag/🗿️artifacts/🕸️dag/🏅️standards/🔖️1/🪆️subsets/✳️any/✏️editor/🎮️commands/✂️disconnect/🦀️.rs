//! 🕸️ 🕸️ DAG play app commands command — `disconnect`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "disconnect")]
pub struct Disconnect {
    pub edge_id: String,
}

pub fn handle(payload: &Disconnect, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let present = crate::dag_scene(doc)?.edges.iter().any(|edge| edge.id == payload.edge_id);
    Ok(if present { crate::dag_child_emit(doc.snapshot, vec![crate::delete_edge_leaf(&payload.edge_id)]) } else { Emit::default() })
}
