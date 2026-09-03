//! 🧱 Playbook mutation — `AddBlock`: inserts a new block into a step, positioned at `index`
//! (final-state) or appended when absent.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookBlock, PlaybookDiff, PlaybookSnapshot};
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
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub step_id: String,
    #[dsl(block)]
    pub block: PlaybookBlock,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

/// 🏗️ Builder.
pub fn add_block_operation(step_id: &str, block: PlaybookBlock, index: Option<usize>) -> PlaybookMutation {
    PlaybookMutation::AddBlock(AddBlock { step_id: step_id.into(), block, index })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for AddBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "block", kind: "add-block", record: "AddedBlock" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add block \"{}\"", self.block.label)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone(), self.block.id.clone()]
    }
}
//#endregion 🔖️Mutation
