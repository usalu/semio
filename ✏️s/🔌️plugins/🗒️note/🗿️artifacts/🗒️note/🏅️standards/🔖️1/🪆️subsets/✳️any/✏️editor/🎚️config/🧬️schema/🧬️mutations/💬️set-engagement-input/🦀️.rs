//! 🧬️ Set Engagement Input in the note.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<NoteConfig, NoteConfigMutation> for SetEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "engagement-input", kind: "set-engagement-input", record: "SetEngagementInput" };
    fn diff(&self, base: &NoteConfig) -> protocol::MutationOutcome<NoteConfig> {
        let mut next = base.clone();
        next.engagement_input = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &NoteConfig) -> Vec<NoteConfigMutation> {
        vec![NoteConfigMutation::SetEngagementInput(Self { value: base.engagement_input.clone() })]
    }
    fn label(&self) -> String {
        "Set Engagement Input".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["engagement_input".into()]
    }
}
