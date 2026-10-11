//! 🗑️ `remove-frame` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-frame")]
pub struct RemoveFrame {
    pub(crate) index: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for RemoveFrame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "frame", kind: "remove-frame", record: "RemoveFrame" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(GifDiff { frames: Some(GifFramesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok(match base.frames.get(*index) {
            Some(frame) => vec![GifMutation::InsertFrame(insert_frame::InsertFrame { index: *index, frame: frame.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove frame", "Einzelbild entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
