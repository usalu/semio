//! 🎞️ `set-image-pixels` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-image-pixels")]
pub struct SetImagePixels {
    pub(crate) index: usize,
    #[dsl(base64)]
    pub(crate) indices: Vec<u8>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetImagePixels {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "image-pixels", kind: "set-image-pixels", record: "SetImagePixels" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let image_at = |index: usize| base.images.get(index).map(|image| (index, image));
        let Self { index, indices } = self;
        if let Some((message, target)) = image_at(*index).and_then(|(index, image)| {
            let repainted = GifImage { indices: indices.clone(), ..image.clone() };
            image_covers(index, &repainted).or_else(|| image_colored(index, &repainted, base.gct.as_ref()))
        }) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new({
            let d = GifImageDiff { indices: Some(indices.clone()), ..Default::default() };
            GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.images.get(*index) {
            Some(img) => vec![GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: *index, indices: img.indices.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set image pixels", "Bildpixel setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
