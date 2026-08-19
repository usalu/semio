//! ➖ Playbook mutation — `RemoveStep`: deletes a step by id.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub step_id: String,
}

/// 🏗️ Builder.
pub async fn remove_step_operation(step_id: &str) -> PlaybookMutation {
    PlaybookMutation::RemoveStep(RemoveStep { step_id: step_id.into() })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for RemoveStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "step", kind: "remove-step", record: "RemovedStep" };

    async fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove step \"{}\"", self.step_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step_id.clone()]
    }
}
//#endregion 🔖️Mutation
