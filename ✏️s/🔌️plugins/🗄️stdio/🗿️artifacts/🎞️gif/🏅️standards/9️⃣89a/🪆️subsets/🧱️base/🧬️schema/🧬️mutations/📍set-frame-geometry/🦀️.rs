//! 📍️ `set-frame-geometry` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-geometry")]
pub struct SetFrameGeometry {
    pub(crate) index: usize,
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFrameGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-geometry", kind: "set-frame-geometry", record: "SetFrameGeometry" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let screen = (base.width, base.height);
        let frame_at = |index: usize| base.frames.get(index).map(|frame| (index, frame));
        let Self { index, left, top, width, height } = self;
        if let Some((message, target)) = frame_at(*index).and_then(|(index, frame)| {
            let moved = GifFrame { left: *left, top: *top, width: *width, height: *height, ..frame.clone() };
            frame_covers(index, &moved).or_else(|| frame_fits(index, &moved, screen))
        }) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new({
            let d = GifFrameDiff { left: Some(*left), top: Some(*top), width: Some(*width), height: Some(*height), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: *index, left: f.left, top: f.top, width: f.width, height: f.height })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set frame geometry", "Geometrie des Einzelbilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
