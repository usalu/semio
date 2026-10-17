//! 📌️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-annotation`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertAnnotation {
    pub index: usize,
    pub at: usize,
    pub annotation: PdfAnnotation,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertAnnotation
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertAnnotation> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_annotation::RemoveAnnotation>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "annotation", kind: "insert-annotation", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_insert_annotation(self.index, self.at, self.annotation.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_annotation::RemoveAnnotation { index: self.index, at: self.at })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert annotation at {} on page {}", self.at, self.index), &format!("Anmerkung an {} auf Seite {} einfügen", self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

