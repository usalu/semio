//! 🪜️ `set-image-interlace` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-image-interlace")]
pub struct SetImageInterlace {
    pub(crate) index: usize,
    pub(crate) interlace: bool,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetImageInterlace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "image-interlace", kind: "set-image-interlace", record: "SetImageInterlace" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index, interlace } = self;
        protocol::MutationOutcome::new({
            let d = GifImageDiff { interlace: Some(*interlace), ..Default::default() };
            GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.images.get(*index) {
            Some(img) => vec![GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: *index, interlace: img.interlace })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set image interlace", "Zeilensprung des Bilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
