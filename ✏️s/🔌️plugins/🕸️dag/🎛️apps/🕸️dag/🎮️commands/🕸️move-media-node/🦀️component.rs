//! 🕸️ 🕸️ DAG play app commands command — `move-media-node`.

use crate::apps::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::artifacts::dag::schema;
use crate::artifacts::dag::mutations::{connect_nodes, dag_snapshot_mutations, disconnect_nodes, move_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::{dag_document_from_fixture, dag_fixture_from_document, DagFixture, DagHost, DagLayoutOptions};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-media-node")]
pub struct MoveMediaNode {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    if document.nodes().iter().any(|node| node.id == payload.node_id) {
        Ok(Emit::amend(vec![move_node(payload.node_id.clone(), payload.x, payload.y)], format!("move-{}", payload.node_id)))
    } else {
        Ok(Emit::default())
    }
}
