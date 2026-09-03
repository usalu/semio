//! ➖ Playbook mutation — `RemoveStep`: deletes a step by id.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle — committed
// `🧪️tests/<fixture>/🦀️.rs` fixture vectors decode/re-encode through them — never a production
// dependency of this crate.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-step")]
pub struct RemoveStep {
    pub step_id: String,
}

/// 🏗️ Builder.
pub fn remove_step_operation(step_id: &str) -> PlaybookMutation {
    PlaybookMutation::RemoveStep(RemoveStep { step_id: step_id.into() })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for RemoveStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "step", kind: "remove-step", record: "RemovedStep" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove step \"{}\"", self.step_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone()]
    }
}
//#endregion 🔖️Mutation
