//! 🖋️ Authoritative PDF mutation payload, diff, inverse, and tests for `insert-content`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertContent {
    pub index: usize,
    pub at: usize,
    pub content: Vec<PdfOp>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertContent
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertContent> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_content::RemoveContent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "content", kind: "insert-content", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_insert_content(self.index, self.at, &self.content))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_content::RemoveContent { index: self.index, at: self.at, count: self.content.len() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert {} operators at {} on page {}", self.content.len(), self.at, self.index), &format!("{} Operatoren an {} auf Seite {} einfügen", self.content.len(), self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

