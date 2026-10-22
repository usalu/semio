//! ➕️ `insert-run` — places a new text run at a FINAL-state index in the run sequence (an
//! intrinsically ordered, anonymous collection — no stable id on `SemioTextRun`).

use crate::schema::mutations::SemioTextMutation;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextRun;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertRun {
    pub index: usize,
    pub run: SemioTextRun,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for InsertRun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "run", kind: "insert-run", record: "InsertedRun" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert run at #{}", self.index), &format!("Textlauf an #{} einfügen", self.index))
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl protocol::ColdMutationKind<SemioTextSnapshot, SemioTextMutation> for InsertRun {
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
