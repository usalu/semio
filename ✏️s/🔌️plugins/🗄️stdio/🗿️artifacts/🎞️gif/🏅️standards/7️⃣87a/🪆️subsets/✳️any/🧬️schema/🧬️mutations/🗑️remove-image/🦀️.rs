//! 🗑️ `remove-image` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-image")]
pub struct RemoveImage {
    pub(crate) index: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for RemoveImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "image", kind: "remove-image", record: "RemoveImage" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(GifDiff { images: Some(GifImagesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok(match base.images.get(*index) {
            Some(image) => vec![GifMutation::InsertImage(insert_image::InsertImage { index: *index, image: image.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove image", "Bild entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
