//! 🔀️ `move-frame` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "move-frame")]
pub struct MoveFrame {
    pub(crate) from: usize,
    pub(crate) to: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for MoveFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "frame", kind: "move-frame", record: "MoveFrame" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { from, to } = self;
        let Some(frame) = base.frames.get(*from) else { return protocol::MutationOutcome::new(GifDiff::default()) };
        let at = (*to).min(base.frames.len() - 1);
        if at == *from {
            return protocol::MutationOutcome::new(GifDiff::default());
        }
        protocol::MutationOutcome::new(GifDiff { frames: Some(GifFramesDiff { removed: vec![*from], added: vec![GifFrameAdded { index: at, frame: frame.clone() }], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { from, to } = self;
        if *from >= base.frames.len() {
            return Ok(Vec::new());
        }
        let at = (*to).min(base.frames.len() - 1);
        Ok(if at == *from { Vec::new() } else { vec![GifMutation::MoveFrame(move_frame::MoveFrame { from: at, to: *from })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move frame", "Einzelbild verschieben")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
