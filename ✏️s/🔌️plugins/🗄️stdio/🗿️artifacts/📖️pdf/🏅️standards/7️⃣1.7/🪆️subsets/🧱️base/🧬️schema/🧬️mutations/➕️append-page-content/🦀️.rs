//! ➕️ Authoritative PDF mutation payload, diff, inverse, and tests for `append-page-content`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AppendPageContent {
    pub index: usize,
    pub content: Vec<PdfOp>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for AppendPageContent
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<AppendPageContent> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_content::RemoveContent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "append", entity: "page-content", kind: "append-page-content", record: "Append" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Append {} operators to page {}", self.content.len(), self.index), &format!("{} Operatoren an Seite {} anhängen", self.content.len(), self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for AppendPageContent
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<AppendPageContent> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_content::RemoveContent>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_append_page_content(base, self.index, &self.content))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).map(|page| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_content::RemoveContent { index: self.index, at: page.content.len(), count: self.content.len() })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

