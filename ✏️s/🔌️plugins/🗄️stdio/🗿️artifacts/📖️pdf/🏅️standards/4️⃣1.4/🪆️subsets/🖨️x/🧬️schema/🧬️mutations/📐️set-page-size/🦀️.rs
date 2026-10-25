//! 📐️ Direct set-page-size payload, sparse diff, concrete inverse, and laws.

use crate::standards::v1_4::subsets::base::schema::diff::{PdfDiff, PdfPageDiff, PdfPageModified, PdfPagesDiff};
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
/// 📐️ A4 geometry used by the independent PDF/X conformance oracle.
pub const CONFORMANT_WIDTH: f64 = 595.276;
pub const CONFORMANT_HEIGHT: f64 = 841.89;
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetPageSize {
    pub width: f64,
    pub height: f64,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl SetPageSize {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty() && self.width.is_finite() && self.height.is_finite()
    }
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetPageSize
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetPageSize>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page", kind: "set-page-size", record: "SetPage" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set page size", "Seitengröße setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["0".into()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetPageSize
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetPageSize> + protocol::Mutation<PdfSnapshot, Diff = PdfDiff>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("mutation.target-missing", "Page target or geometry is outside the PDF 1.4 domain", <Self as protocol::MutationKind<PdfSnapshot, Op>>::target(self));
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { width: Some(self.width), height: Some(self.height), text: None } }], ..Default::default() }) })
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![Op::from(SetPageSize { width: base.pages[0].width, height: base.pages[0].height })]
    
    })())
}
}
//#endregion 🔖️Behavior

//#region 🧪️Tests

//#endregion 🧪️Tests
