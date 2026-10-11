//! ♻️ `set-frame-disposal` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-disposal")]
pub struct SetFrameDisposal {
    pub(crate) index: usize,
    pub(crate) disposal: GifDisposal,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFrameDisposal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-disposal", kind: "set-frame-disposal", record: "SetFrameDisposal" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index, disposal } = self;
        protocol::MutationOutcome::new({
            let d = GifFrameDiff { disposal: Some(*disposal), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: *index, disposal: f.disposal })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set frame disposal", "Entsorgungsmethode des Einzelbilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
