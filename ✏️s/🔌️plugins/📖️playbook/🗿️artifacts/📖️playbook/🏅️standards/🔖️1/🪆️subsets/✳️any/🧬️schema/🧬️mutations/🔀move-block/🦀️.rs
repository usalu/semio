//! 🔀 Playbook mutation — `MoveBlock`: repositions a block to `index` (final-state) within its
//! step, or relocates it into a different step (same shape covers both — `from_step_id ==
//! to_step_id` is the same-step reorder case).

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
