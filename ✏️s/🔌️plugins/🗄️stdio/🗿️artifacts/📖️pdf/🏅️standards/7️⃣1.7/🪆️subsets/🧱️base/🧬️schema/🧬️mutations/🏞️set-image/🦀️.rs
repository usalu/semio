//! 🏞️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-image`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetImage {
    pub image: PdfImage,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetImage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "image", kind: "set-image", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_image(base, self.image.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.images.iter().find(|item| item.id == self.image.id) { Some(previous) => vec![PdfMutation::SetImage(SetImage { image: previous.clone(), index: None })], None => vec![PdfMutation::RemoveImage(super::remove_image::RemoveImage { id: self.image.id.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set image {}", self.image.id), &format!("Bild {} setzen", self.image.id))
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

