//! 🧱 Playbook mutation — `AddBlock`: inserts a new block into a step, positioned at `index`
//! (final-state) or appended when absent.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookBlock, PlaybookDiff, PlaybookSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-block")]
pub struct AddBlock {
    pub step_id: String,
    #[dsl(block)]
    pub block: PlaybookBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
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
