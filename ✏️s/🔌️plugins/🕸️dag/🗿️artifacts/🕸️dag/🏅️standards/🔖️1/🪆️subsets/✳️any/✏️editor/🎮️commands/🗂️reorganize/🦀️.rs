//! 🕸️ 🕸️ DAG play app commands command — `reorganize`.

use crate::mutations::dag_snapshot_mutations;
use crate::op::DagMutation;
use crate::DagSnapshot;
use crate::editor::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use infinite_board_port_directed_dag::{DagHost, DagLayoutOptions};
use semio_framework_artifact_infinite_dag::{dag_fixture_from_document, DagFixture};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[cfg(test)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let camera = dag_config_camera(config);
    if let Ok(mut host) = DagHost::load_fixture_json(&dsl::json::to_json_string(&dag_fixture_from_document(&semio_framework_artifact_infinite_dag::DagSnapshot::from(document), camera))) {
        let _ = host.reorganize(&DagLayoutOptions::default());
        if let Ok(json) = host.fixture_json() {
            if let Ok(fixture) = dsl::json::from_json_str::<DagFixture>(&json) {
                // 🎯️ Reorganize only ever moves EXISTING nodes (same ids/edges) — the generic
                // differ correctly narrows that down to a `move-node` per node whose position
                // actually changed, never a whole-collection replace.
                let content = crate::dag_content_child_with_owner(fixture.nodes, document.edges());
                let recomputed = DagSnapshot { schema: document.schema.clone(), content };
                return Ok(Emit::mutations(dag_snapshot_mutations(document, &recomputed)));
            }
        }
    }
    Ok(Emit::default())
}
