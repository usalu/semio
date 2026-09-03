//! 🗑️ Playbook mutation — `RemoveBlock`: deletes a block from a step by id.

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
#[dsl(keyword = "remove-block")]
pub struct RemoveBlock {
    pub step_id: String,
    pub block_id: String,
}

/// 🏗️ Builder.
pub fn remove_block_operation(step_id: &str, block_id: &str) -> PlaybookMutation {
    PlaybookMutation::RemoveBlock(RemoveBlock { step_id: step_id.into(), block_id: block_id.into() })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for RemoveBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "block", kind: "remove-block", record: "RemovedBlock" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove block \"{}\"", self.block_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block_id.clone()]
    }
}
//#endregion 🔖️Mutation
