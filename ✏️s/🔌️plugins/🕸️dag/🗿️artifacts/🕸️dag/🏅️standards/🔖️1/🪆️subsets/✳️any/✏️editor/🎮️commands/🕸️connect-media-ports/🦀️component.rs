//! 🕸️ 🕸️ DAG play app commands command — `connect-media-ports`.

use crate::editor::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::artifacts::dag::schema;
use crate::artifacts::dag::mutations::{connect_nodes, dag_snapshot_mutations, disconnect_nodes, move_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::{dag_document_from_fixture, dag_fixture_from_document, DagFixture, DagHost, DagLayoutOptions};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    match crate::artifacts::dag::schema::connect_edge(document, &payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id) {
        Ok(edge) => Ok(Emit::mutations(vec![connect_nodes(edge.id, edge.source, edge.target, edge.route_style, edge.properties)])),
        Err(_) => Ok(Emit::default()),
    }
}
