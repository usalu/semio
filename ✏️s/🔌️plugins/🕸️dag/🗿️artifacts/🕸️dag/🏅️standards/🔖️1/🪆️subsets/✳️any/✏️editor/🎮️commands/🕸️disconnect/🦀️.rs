//! 🕸️ 🕸️ DAG play app commands command — `disconnect`.

use crate::artifacts::dag::mutations::disconnect_nodes;
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "disconnect")]
pub struct Disconnect {
    pub edge_id: String,
}

pub async fn handle(payload: &Disconnect, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let document = doc.snapshot;
    if document.edges().iter().any(|edge| edge.id == payload.edge_id) {
        Ok(Emit::mutations(vec![disconnect_nodes(payload.edge_id.clone())]))
    } else {
        Ok(Emit::default())
    }
}
