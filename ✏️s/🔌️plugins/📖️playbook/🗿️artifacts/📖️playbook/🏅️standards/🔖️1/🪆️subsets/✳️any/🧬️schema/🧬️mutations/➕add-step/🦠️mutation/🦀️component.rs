//! ➕ Playbook mutation — `AddStep`: inserts a new step, positioned at `index` (final-state) or
//! appended when absent.
use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::{PlaybookSnapshot, PlaybookStep};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-step")]
pub struct AddStep {
    #[dsl(block)]
    pub step: PlaybookStep,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

/// 🏗️ Builder — names the new step by its position in `spec` (the app's own step-add gesture never
/// prompts for a title up front).
pub fn add_step_operation(spec: &PlaybookSnapshot, step_id: String) -> PlaybookMutation {
    PlaybookMutation::AddStep(AddStep { step: PlaybookStep { id: step_id, title: format!("Step {}", spec.steps.len() + 1), description: None, blocks: Vec::new() }, index: None })
}

impl protocol::MutationKind<PlaybookSnapshot, PlaybookMutation> for AddStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "step", kind: "add-step", record: "AddedStep" };

    fn diff(&self, base: &PlaybookSnapshot) -> crate::artifacts::playbook::PlaybookDiff {
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
