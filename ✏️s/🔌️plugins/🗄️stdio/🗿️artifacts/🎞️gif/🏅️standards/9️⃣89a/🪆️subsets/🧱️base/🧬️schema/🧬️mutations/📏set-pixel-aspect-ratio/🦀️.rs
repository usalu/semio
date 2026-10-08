//! 📏️ `set-pixel-aspect-ratio` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-pixel-aspect-ratio")]
pub struct SetPixelAspectRatio {
    pub(crate) ratio: u8,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetPixelAspectRatio {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "pixel-aspect-ratio", kind: "set-pixel-aspect-ratio", record: "SetPixelAspectRatio" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { ratio } = self;
        protocol::MutationOutcome::new(GifDiff { pixel_aspect_ratio: (*ratio != base.pixel_aspect_ratio).then_some(*ratio), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        Ok(vec![GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: base.pixel_aspect_ratio })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set pixel aspect ratio", "Pixel-Seitenverhältnis setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
