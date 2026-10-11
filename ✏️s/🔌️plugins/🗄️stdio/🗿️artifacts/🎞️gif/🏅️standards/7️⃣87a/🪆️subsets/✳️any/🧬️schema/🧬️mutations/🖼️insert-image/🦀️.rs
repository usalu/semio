//! 🖼️ `insert-image` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "insert-image")]
pub struct InsertImage {
    pub(crate) index: usize,
    #[dsl(block)]
    pub(crate) image: GifImage,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for InsertImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "image", kind: "insert-image", record: "InsertImage" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let screen = (base.width, base.height);
        let Self { index, image } = self;
        if let Some((message, target)) = {
            let at = (*index).min(base.images.len());
            image_fits(at, image, screen).or_else(|| image_covers(at, image)).or_else(|| image_colored(at, image, base.gct.as_ref()))
        } {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new({
            GifDiff { images: Some(GifImagesDiff { added: vec![GifImageAdded { index: (*index).min(base.images.len()), image: image.clone() }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(vec![GifMutation::RemoveImage(remove_image::RemoveImage { index: (*index).min(base.images.len()) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert image", "Bild einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
