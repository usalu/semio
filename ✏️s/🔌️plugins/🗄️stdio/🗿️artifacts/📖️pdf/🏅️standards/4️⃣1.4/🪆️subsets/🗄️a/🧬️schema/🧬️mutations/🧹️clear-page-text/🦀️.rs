//! 🧹️ Direct clear-page-text payload, sparse diff, concrete inverse, and laws.

use crate::standards::v1_4::subsets::base::schema::diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff};
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
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

impl<Op> MutationKind<PdfSnapshot, Op> for ClearPageText
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<ClearPageText> + From<crate::standards::v1_4::subsets::a::schema::mutations::set_page_text::SetPageText>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "page", kind: "clear-page-text", record: "ClearedPageText" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Clear page text", "Seitentext leeren")
    }

    fn target(&self) -> Vec<String> {
        vec!["0".into()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for ClearPageText
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<ClearPageText> + From<crate::standards::v1_4::subsets::a::schema::mutations::set_page_text::SetPageText>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("mutation.target-missing", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { text: Some(String::new()), ..Default::default() } }], ..Default::default() }) })
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![Op::from(crate::standards::v1_4::subsets::a::schema::mutations::SetPageText { text: base.pages[0].text.clone() })]
    
    })())
}
}
//#endregion 🔖️Behavior

//#region 🧪️Tests

//#endregion 🧪️Tests
