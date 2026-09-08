//! 🧬️ Set Locale in the note.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-locale")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLocale {
    pub value: String,
}

impl protocol::MutationKind<NoteConfig, NoteConfigMutation> for SetLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &NoteConfig) -> protocol::MutationOutcome<NoteConfig> {
        let mut next = base.clone();
        next.locale = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &NoteConfig) -> Vec<NoteConfigMutation> { vec![NoteConfigMutation::SetLocale(Self { value: base.locale.clone() })] }
    fn label(&self) -> String { "Set Locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
