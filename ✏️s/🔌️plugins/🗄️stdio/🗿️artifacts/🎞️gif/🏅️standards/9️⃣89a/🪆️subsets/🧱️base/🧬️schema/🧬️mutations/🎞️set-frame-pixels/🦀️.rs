//! 🎞️ `set-frame-pixels` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-pixels")]
pub struct SetFramePixels {
    pub(crate) index: usize,
    #[dsl(base64)]
    pub(crate) indices: Vec<u8>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFramePixels {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-pixels", kind: "set-frame-pixels", record: "SetFramePixels" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let frame_at = |index: usize| base.frames.get(index).map(|frame| (index, frame));
        let Self { index, indices } = self;
        if let Some((message, target)) = frame_at(*index).and_then(|(index, frame)| {
            let repainted = GifFrame { indices: indices.clone(), ..frame.clone() };
            frame_covers(index, &repainted).or_else(|| frame_colored(index, &repainted, base.gct.as_ref()))
        }) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new({
            let d = GifFrameDiff { indices: Some(indices.clone()), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: *index, indices: f.indices.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set frame pixels", "Pixel des Einzelbilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
