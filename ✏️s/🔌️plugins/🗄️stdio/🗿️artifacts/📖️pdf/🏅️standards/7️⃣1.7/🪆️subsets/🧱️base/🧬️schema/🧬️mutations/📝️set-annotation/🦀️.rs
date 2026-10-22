//! 📝️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-annotation`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAnnotation {
    pub index: usize,
    pub at: usize,
    pub annotation: PdfAnnotation,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetAnnotation
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetAnnotation>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "annotation", kind: "set-annotation", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set annotation {} on page {}", self.at, self.index), &format!("Anmerkung {} auf Seite {} setzen", self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetAnnotation
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetAnnotation>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_annotation(self.index, self.at, self.annotation.clone()))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).and_then(|page| page.annotations.get(self.at)).map(|annotation| Op::from(SetAnnotation { index: self.index, at: self.at, annotation: annotation.clone() })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

