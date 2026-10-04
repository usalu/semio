//! 🧬️ Replace Presence in the note.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: NotePresence,
}

impl protocol::MutationKind<NotePresence, NotePresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &NotePresence) -> protocol::MutationOutcome<NotePresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &NotePresence) -> Result<Vec<NotePresenceMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![NotePresenceMutation::ReplacePresence(Self { presence: base.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
