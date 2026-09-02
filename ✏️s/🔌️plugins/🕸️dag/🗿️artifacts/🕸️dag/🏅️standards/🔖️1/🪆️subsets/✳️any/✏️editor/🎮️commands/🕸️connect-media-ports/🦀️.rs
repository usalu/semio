//! 🕸️ 🕸️ DAG play app commands command — `connect-media-ports`.

use crate::artifacts::dag::mutations::connect_nodes;
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

pub async fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    match crate::artifacts::dag::schema::connect_edge(document, &payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id) {
        Ok(edge) => Ok(Emit::mutations(vec![connect_nodes(edge.id, edge.source, edge.target, edge.route_style, edge.properties)])),
        Err(_) => Ok(Emit::default()),
    }
}
