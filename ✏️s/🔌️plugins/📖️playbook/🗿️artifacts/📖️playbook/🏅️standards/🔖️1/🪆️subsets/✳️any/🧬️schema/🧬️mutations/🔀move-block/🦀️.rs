//! 🔀 Playbook mutation — `MoveBlock`: repositions a block to `index` (final-state) within its
//! step, or relocates it into a different step (same shape covers both — `from_step_id ==
//! to_step_id` is the same-step reorder case).

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "move-block")]
pub struct MoveBlock {
    pub block_id: String,
    pub from_step_id: String,
    pub to_step_id: String,
    pub index: usize,
}

/// 🏗️ Builder.
pub fn move_block_operation(block_id: &str, from_step_id: &str, to_step_id: &str, index: usize) -> PlaybookMutation {
    PlaybookMutation::MoveBlock(MoveBlock { block_id: block_id.into(), from_step_id: from_step_id.into(), to_step_id: to_step_id.into(), index })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for MoveBlock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "block", kind: "move-block", record: "MovedBlock" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move block \"{}\"", self.block_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.from_step_id.clone(), self.block_id.clone()]
    }
}
//#endregion 🔖️Mutation
