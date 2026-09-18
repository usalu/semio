//! 🏞️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-image`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetImage {
    pub image: PdfImage,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetImage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "image", kind: "set-image", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_image(base, self.image.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.images.iter().find(|item| item.id == self.image.id) { Some(previous) => vec![PdfMutation::SetImage(SetImage { image: previous.clone() })], None => vec![PdfMutation::RemoveImage(super::remove_image::RemoveImage { id: self.image.id.clone() })] }
    }

    fn label(&self) -> String {
        format!("Set image {}", self.image.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.image.id.clone()]
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
