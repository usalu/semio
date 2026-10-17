//! 📍️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-annotation`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveAnnotation {
    pub index: usize,
    pub at: usize,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveAnnotation
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveAnnotation> + From<crate::standards::v1_7::subsets::base::schema::mutations::insert_annotation::InsertAnnotation>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "annotation", kind: "remove-annotation", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_annotation(self.index, self.at))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).and_then(|page| page.annotations.get(self.at)).map(|annotation| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::insert_annotation::InsertAnnotation { index: self.index, at: self.at, annotation: annotation.clone() })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove annotation {} on page {}", self.at, self.index), &format!("Anmerkung {} auf Seite {} entfernen", self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

