//! 📥️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-page`.

use crate::standards::v1_7::subsets::base::schema::mutations::remove_page::RemovePage;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfPage, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertPage {
    pub index: usize,
    pub page: PdfPage,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertPage
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertPage> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_page::RemovePage>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "Insert" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert page {}", self.index), &format!("Seite {} einfügen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for InsertPage
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<InsertPage> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_page::RemovePage>,
{
    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_insert_page(self.index, self.page.clone()))
    }
    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(RemovePage { index: self.index })]
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

