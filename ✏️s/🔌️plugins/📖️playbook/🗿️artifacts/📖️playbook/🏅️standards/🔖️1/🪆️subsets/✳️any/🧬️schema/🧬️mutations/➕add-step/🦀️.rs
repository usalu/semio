//! ➕ Playbook mutation — `AddStep`: inserts a new step, positioned at `index` (final-state) or
//! appended when absent. A duplicate `step.id` is Warning `mutation.no-op`.

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::schema::diff::text::diff_replace_content;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot, PlaybookStep};
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
#[dsl(keyword = "add-step")]
pub struct AddStep {
    #[dsl(block)]
    pub step: PlaybookStep,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[value(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

/// 🏗️ Builder from operation-owned scalar identity and title; it never reads or clones the document.
pub fn add_step_operation(step_id: String, title: String) -> PlaybookMutation {
    PlaybookMutation::AddStep(AddStep { step: PlaybookStep { id: step_id, title, description: None, blocks: Vec::new() }, index: None })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for AddStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "step", kind: "add-step", record: "AddedStep" };

    fn diff(&self, base: &PlaybookSnapshot) -> protocol::MutationOutcome<crate::artifacts::playbook::PlaybookDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add step \"{}\"", self.step.title)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
    }
}
//#endregion 🔖️Mutation
