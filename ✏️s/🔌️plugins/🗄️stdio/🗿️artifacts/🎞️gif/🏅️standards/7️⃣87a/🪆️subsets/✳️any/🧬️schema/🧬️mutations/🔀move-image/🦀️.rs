//! 🔀️ `move-image` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "move-image")]
pub struct MoveImage {
    pub(crate) from: usize,
    pub(crate) to: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for MoveImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "image", kind: "move-image", record: "MoveImage" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { from, to } = self;
        let Some(image) = base.images.get(*from) else { return protocol::MutationOutcome::new(GifDiff::default()) };
        let at = (*to).min(base.images.len() - 1);
        if at == *from {
            return protocol::MutationOutcome::new(GifDiff::default());
        }
        protocol::MutationOutcome::new(GifDiff { images: Some(GifImagesDiff { removed: vec![*from], added: vec![GifImageAdded { index: at, image: image.clone() }], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { from, to } = self;
        if *from >= base.images.len() {
            return Ok(Vec::new());
        }
        let at = (*to).min(base.images.len() - 1);
        Ok(if at == *from { Vec::new() } else { vec![GifMutation::MoveImage(move_image::MoveImage { from: at, to: *from })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move image", "Bild verschieben")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
