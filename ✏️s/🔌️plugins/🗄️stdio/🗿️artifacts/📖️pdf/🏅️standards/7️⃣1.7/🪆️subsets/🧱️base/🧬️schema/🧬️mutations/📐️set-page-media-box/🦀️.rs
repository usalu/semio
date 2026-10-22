//! 📐️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-media-box`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageMediaBox {
    pub index: usize,
    pub media_box: [f64; 4],
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetPageMediaBox
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetPageMediaBox>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-media-box", kind: "set-page-media-box", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set page {} media box", self.index), &format!("MediaBox von Seite {} setzen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetPageMediaBox
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetPageMediaBox>,
{
    fn diff(&self, _base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_page_media_box(self.index, self.media_box))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        base.pages.get(self.index).map(|page| Op::from(SetPageMediaBox { index: self.index, media_box: page.media_box })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

