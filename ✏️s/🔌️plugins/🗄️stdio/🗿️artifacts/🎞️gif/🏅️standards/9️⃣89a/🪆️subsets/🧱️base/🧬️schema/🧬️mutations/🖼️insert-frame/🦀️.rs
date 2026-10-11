//! 🖼️ `insert-frame` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "insert-frame")]
pub struct InsertFrame {
    pub(crate) index: usize,
    #[dsl(block)]
    pub(crate) frame: GifFrame,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for InsertFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "frame", kind: "insert-frame", record: "InsertFrame" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let screen = (base.width, base.height);
        let Self { index, frame } = self;
        if let Some((message, target)) = {
            let at = (*index).min(base.frames.len());
            frame_fits(at, frame, screen).or_else(|| frame_covers(at, frame)).or_else(|| frame_colored(at, frame, base.gct.as_ref()))
        } {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new({
            GifDiff { frames: Some(GifFramesDiff { added: vec![GifFrameAdded { index: (*index).min(base.frames.len()), frame: frame.clone() }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(vec![GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: (*index).min(base.frames.len()) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert frame", "Einzelbild einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
