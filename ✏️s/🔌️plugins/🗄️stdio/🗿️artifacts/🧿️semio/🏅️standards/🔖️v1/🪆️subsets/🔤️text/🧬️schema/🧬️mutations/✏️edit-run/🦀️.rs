//! ✏️ `edit-run` — replaces one run's authored `content` body, addressed by BASE-state index.

use crate::schema::mutations::SemioTextMutation;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct EditRun {
    pub index: usize,
    pub new_text: String,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for EditRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "run", kind: "edit-run", record: "EditedRun" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit run #{}", self.index), &format!("Textlauf #{} bearbeiten", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl protocol::ColdMutationKind<SemioTextSnapshot, SemioTextMutation> for EditRun {
    fn diff(&self, base: &SemioTextSnapshot) -> protocol::MutationOutcome<<SemioTextMutation as protocol::ColdMutation<SemioTextSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioTextSnapshot) -> Result<Vec<SemioTextMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
