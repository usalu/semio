//! 📉️ Direct collapse-page-size payload, sparse diff, concrete inverse, and laws.

use crate::standards::v1_4::subsets::base::schema::diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff};
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollapsePageSize {}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl CollapsePageSize {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty()
    }
}

impl<Op> MutationKind<PdfSnapshot, Op> for CollapsePageSize
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<CollapsePageSize> + From<crate::standards::v1_4::subsets::x::schema::mutations::set_page_size::SetPageSize>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "page", kind: "collapse-page-size", record: "ResizedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("mutation.target-missing", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { width: Some(0.0), ..Default::default() } }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![Op::from(crate::standards::v1_4::subsets::x::schema::mutations::SetPageSize { width: base.pages[0].width, height: base.pages[0].height })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Collapse page width to zero", "Seitenbreite auf null setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["0".into()]
    }
}
//#endregion 🔖️Behavior

//#region 🧪️Tests

//#endregion 🧪️Tests
