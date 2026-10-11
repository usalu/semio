//! 🧹️ Direct clear-page-text payload, sparse diff, concrete inverse, and laws.

use super::PdfA1Mutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff},
    snapshot::PdfSnapshot,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClearPageText {}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl ClearPageText {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty()
    }
}

impl MutationKind<PdfSnapshot, PdfA1Mutation> for ClearPageText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "page", kind: "clear-page-text", record: "ClearedPageText" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("mutation.target-missing", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { text: Some(String::new()), ..Default::default() } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfA1Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfA1Mutation::SetPageText(super::SetPageText { text: base.pages[0].text.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Clear page text", "Seitentext leeren")
    }

    fn target(&self) -> Vec<String> {
        vec!["0".into()]
    }
}
//#endregion 🔖️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
