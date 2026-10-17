//! 🧻️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-content`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveContent {
    pub index: usize,
    pub at: usize,
    pub count: usize,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveContent
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveContent> + From<crate::standards::v1_7::subsets::base::schema::mutations::insert_content::InsertContent>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "content", kind: "remove-content", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_content(self.index, self.at, self.count))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).filter(|page| self.at + self.count <= page.content.len()).map(|page| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::insert_content::InsertContent { index: self.index, at: self.at, content: page.content[self.at..self.at + self.count].to_vec() })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove {} operators at {} on page {}", self.count, self.at, self.index), &format!("{} Operatoren an {} auf Seite {} entfernen", self.count, self.at, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string(), self.at.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

