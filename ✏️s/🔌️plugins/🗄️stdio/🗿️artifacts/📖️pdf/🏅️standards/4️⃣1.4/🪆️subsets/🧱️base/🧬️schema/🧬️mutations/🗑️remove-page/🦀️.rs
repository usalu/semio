//! 🗑️ Direct remove-page payload, sparse diff, concrete inverse, and laws.

use crate::standards::v1_4::subsets::base::schema::diff::{PdfDiff, PdfPagesDiff};
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovePage {
    pub index: usize,
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl RemovePage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        base.pages.len() > 1 && self.index < base.pages.len()
    }
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemovePage
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemovePage> + From<crate::standards::v1_4::subsets::base::schema::mutations::insert_page::InsertPage>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "page", kind: "remove-page", record: "RemovedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error(if self.index < base.pages.len() { "mutation.target-mismatch" } else { "mutation.target-missing" }, "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { removed: vec![self.index], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![Op::from(crate::standards::v1_4::subsets::base::schema::mutations::component::InsertPage { index: self.index, page: base.pages[self.index].clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove page", "Seite entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Behavior

//#region 🔖️Codecs
//#endregion 🔖️Codecs

//#region 🧪️Tests



//#endregion 🧪️Tests
