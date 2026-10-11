//! 🧬️ Replace Presence in the note.presence channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: NotePresence,
}

impl protocol::MutationKind<NotePresence, NotePresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, base: &NotePresence) -> protocol::MutationOutcome<NotePresenceDiff> {
        let diff = NotePresenceDiff {
            camera_x: (base.camera_x != self.presence.camera_x).then_some(self.presence.camera_x),
            camera_y: (base.camera_y != self.presence.camera_y).then_some(self.presence.camera_y),
            camera_zoom: (base.camera_zoom != self.presence.camera_zoom).then_some(self.presence.camera_zoom),
        };
        match protocol::DiffAlgebra::<NotePresence>::is_empty(&diff) {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Presence is unchanged."),
            false => protocol::MutationOutcome::new(diff),
        }
    }
    fn inverse(&self, base: &NotePresence) -> Result<Vec<NotePresenceMutation>, semio_framework_value::ValueError> {
    Ok(vec![NotePresenceMutation::ReplacePresence(Self { presence: base.clone() })])
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Presence", "Präsenz ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
