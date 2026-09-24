//! 🖼️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-box`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageBox {
    pub index: usize,
    pub kind: PdfPageBox,
    pub rect: Option<PdfRect>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPageBox {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-box", kind: "set-page-box", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_box(self.index, self.kind, self.rect))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        base.pages.get(self.index).map(|page| PdfMutation::SetPageBox(SetPageBox { index: self.index, kind: self.kind, rect: match self.kind { PdfPageBox::Crop => page.crop_box, PdfPageBox::Bleed => page.bleed_box, PdfPageBox::Trim => page.trim_box, PdfPageBox::Art => page.art_box } })).into_iter().collect()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set page {} {:?} box", self.index, self.kind), &format!("Seite {}: {:?}-Box setzen", self.index, self.kind))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
