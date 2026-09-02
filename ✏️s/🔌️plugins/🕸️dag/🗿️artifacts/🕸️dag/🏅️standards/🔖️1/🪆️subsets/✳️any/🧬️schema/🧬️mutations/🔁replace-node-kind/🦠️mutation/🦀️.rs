//! 🔁 DAG mutation — `ReplaceNodeKind`: whole-value swap of the node's tagged `kind` (the 11-variant
//! `DagNodeKind` — every kind-specific field, e.g. a Slider's `value`/`min`/`max` or a Note's
//! `text`, changes through this one mutation; see `deviations` in this ticket's report for why no
//! finer per-field granularity was minted).
use serde::{Deserialize, Serialize};
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{DagNodeKind, DagSnapshot};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct ReplaceNodeKind {
    pub id: String,
    pub new_kind: DagNodeKind,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_node_kind(id: String, new_kind: DagNodeKind) -> DagMutation {
    DagMutation::ReplaceNodeKind(ReplaceNodeKind { id, new_kind })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for ReplaceNodeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node", kind: "replace-node-kind", record: "ReplacedNodeKind" };

    async fn diff(&self, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace node \"{}\" kind", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
