//! 🕸️ 🕸️ DAG play app commands command — `reorganize`.

use crate::apps::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::artifacts::dag::schema;
use crate::artifacts::dag::mutations::{connect_nodes, dag_snapshot_mutations, disconnect_nodes, move_node};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::{dag_document_from_fixture, dag_fixture_from_document, DagFixture, DagHost, DagLayoutOptions};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let camera = dag_config_camera(config);
    if let Ok(mut host) = DagHost::load_fixture_json(&serde_json::to_string(&dag_fixture_from_document(&infinite_board_port_directed_dag::DagSnapshot::from(document), camera)).unwrap_or_default()) {
        let _ = host.reorganize(&DagLayoutOptions::default());
        if let Ok(json) = host.fixture_json() {
            if let Ok(fixture) = serde_json::from_str::<DagFixture>(&json) {
                // 🎯️ Reorganize only ever moves EXISTING nodes (same ids/edges) — the generic
                // differ correctly narrows that down to a `move-node` per node whose position
                // actually changed, never a whole-collection replace.
                let content = crate::artifacts::dag::dag_content_child_handle_and_cache(fixture.nodes, document.edges());
                let recomputed = DagSnapshot { schema: document.schema.clone(), content };
                return Ok(Emit::mutations(dag_snapshot_mutations(document, &recomputed)));
            }
        }
    }
    Ok(Emit::default())
}
