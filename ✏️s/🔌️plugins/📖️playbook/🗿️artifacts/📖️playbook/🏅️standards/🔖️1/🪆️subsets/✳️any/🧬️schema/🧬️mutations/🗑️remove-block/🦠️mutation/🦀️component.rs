//! 🗑️ Playbook mutation — `RemoveBlock`: deletes a block from a step by id.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-block")]
pub struct RemoveBlock {
    pub step_id: String,
    pub block_id: String,
}

/// 🏗️ Builder.
pub async fn remove_block_operation(step_id: &str, block_id: &str) -> PlaybookMutation {
    PlaybookMutation::RemoveBlock(RemoveBlock { step_id: step_id.into(), block_id: block_id.into() })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for RemoveBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "block", kind: "remove-block", record: "RemovedBlock" };

    async fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove block \"{}\"", self.block_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block_id.clone()]
    }
}
//#endregion 🔖️Mutation
