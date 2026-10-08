//! 📍️ `set-image-geometry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-image-geometry")]
pub struct SetImageGeometry {
    pub(crate) index: usize,
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetImageGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "image-geometry", kind: "set-image-geometry", record: "SetImageGeometry" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let screen = (base.width, base.height);
        let image_at = |index: usize| base.images.get(index).map(|image| (index, image));
        let Self { index, left, top, width, height } = self;
        if let Some((message, target)) = image_at(*index).and_then(|(index, image)| {
            let moved = GifImage { left: *left, top: *top, width: *width, height: *height, ..image.clone() };
            image_covers(index, &moved).or_else(|| image_fits(index, &moved, screen))
        }) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new({
            let d = GifImageDiff { left: Some(*left), top: Some(*top), width: Some(*width), height: Some(*height), ..Default::default() };
            GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.images.get(*index) {
            Some(img) => vec![GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: *index, left: img.left, top: img.top, width: img.width, height: img.height })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set image geometry", "Bildgeometrie setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
