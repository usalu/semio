//! 🔀️ `reorder-runs` — repositions one run within the sequence (never spatial — `SemioTextRun`
//! carries no position of its own, only sequence order).

use crate::schema::mutations::SemioTextMutation;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReorderRuns {
    pub from: usize,
    pub to: usize,
}

impl protocol::MutationKind<SemioTextSnapshot, SemioTextMutation> for ReorderRuns {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "runs", kind: "reorder-runs", record: "ReorderedRuns" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move run #{} to #{}", self.from, self.to), &format!("Textlauf #{} nach #{} verschieben", self.from, self.to))
    }
    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}

impl protocol::ColdMutationKind<SemioTextSnapshot, SemioTextMutation> for ReorderRuns {
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
