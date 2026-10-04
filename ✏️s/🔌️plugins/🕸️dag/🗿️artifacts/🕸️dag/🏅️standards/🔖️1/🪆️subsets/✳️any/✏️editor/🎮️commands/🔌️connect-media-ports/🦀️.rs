//! 🕸️ 🕸️ DAG play app commands command — `connect-media-ports`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    match crate::schema::connect_edge(&crate::dag_scene(doc)?, &payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id) {
        Ok(edge) => Ok(crate::dag_child_emit(doc.snapshot, &[crate::create_edge_leaf(&edge)])),
        Err(_) => Ok(Emit::default()),
    }
}
