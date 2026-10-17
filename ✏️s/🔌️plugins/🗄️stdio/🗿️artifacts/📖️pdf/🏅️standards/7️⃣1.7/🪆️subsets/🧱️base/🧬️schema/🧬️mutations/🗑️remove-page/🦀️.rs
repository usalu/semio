//! 🗑️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-page`.

use crate::standards::v1_7::subsets::base::schema::mutations::insert_page::InsertPage;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemovePage {
    pub index: usize,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemovePage
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemovePage> + From<crate::standards::v1_7::subsets::base::schema::mutations::insert_page::InsertPage>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "page", kind: "remove-page", record: "Remove" };

    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_remove_page(self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        base.pages.get(self.index).cloned().map(|page| Op::from(InsertPage { index: self.index, page })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove page {}", self.index), &format!("Seite {} entfernen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

