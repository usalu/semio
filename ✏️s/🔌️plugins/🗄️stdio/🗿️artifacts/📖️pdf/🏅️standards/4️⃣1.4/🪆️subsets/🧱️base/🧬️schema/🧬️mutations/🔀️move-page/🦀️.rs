//! 🔀️ Direct move-page payload, sparse diff, concrete inverse, and laws.

use crate::standards::v1_4::subsets::base::schema::diff::{PdfDiff, PdfPageAdded, PdfPagesDiff};
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct MovePage {
    pub from: usize,
    pub to: usize,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl MovePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        self.from < base.pages.len() && self.to < base.pages.len()
    }
}

impl<Op> MutationKind<PdfSnapshot, Op> for MovePage
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<MovePage>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "page", kind: "move-page", record: "MovedPage" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move page", "Seite verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec![self.from.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for MovePage
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<MovePage>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("mutation.target-missing", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        if self.from == self.to {
            return MutationOutcome::new(PdfDiff::default());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { removed: vec![self.from], added: vec![PdfPageAdded { index: self.to, page: base.pages[self.from].clone() }], ..Default::default() }) })
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if !self.valid(base) || self.from == self.to {
            return Vec::new();
        }
        vec![Op::from(MovePage { from: self.to, to: self.from })]
    
    })())
}
}
//#endregion 🔖️Behavior

//#region 🔖️Codecs
//#endregion 🔖️Codecs

//#region 🧪️Tests



//#endregion 🧪️Tests
