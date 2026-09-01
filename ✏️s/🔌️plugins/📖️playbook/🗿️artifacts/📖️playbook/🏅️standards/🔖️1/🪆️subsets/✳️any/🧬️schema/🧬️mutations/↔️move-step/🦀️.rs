//! ↔️ Playbook mutation — `MoveStep`: repositions a step to `index` (final-state) within the
//! ordered step list.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "move-step")]
pub struct MoveStep {
    pub step_id: String,
    pub index: usize,
}

/// 🏗️ Builder.
pub fn move_step_operation(step_id: &str, index: usize) -> PlaybookMutation {
    PlaybookMutation::MoveStep(MoveStep { step_id: step_id.into(), index })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for MoveStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "step", kind: "move-step", record: "MovedStep" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move step \"{}\"", self.step_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step_id.clone()]
    }
}
//#endregion 🔖️Mutation
