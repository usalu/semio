//! 🌫️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-image`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveImage {
    pub id: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveImage
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveImage> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_image::SetImage>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "image", kind: "remove-image", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove image {}", self.id), &format!("Bild {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveImage
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveImage> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_image::SetImage>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_image(base, &self.id))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.images.iter().position(|item| item.id == self.id).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_image::SetImage { image: base.images[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

