//! 👻️ `set-frame-transparency` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-transparency")]
#[value(rename_all = "camelCase")]
pub struct SetFrameTransparency {
    pub(crate) index: usize,
    pub(crate) transparent_index: Option<u8>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFrameTransparency {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-transparency", kind: "set-frame-transparency", record: "SetFrameTransparency" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index, transparent_index } = self;
        protocol::MutationOutcome::new({
            let d = GifFrameDiff { transparent_index: Some(*transparent_index), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: *index, transparent_index: f.transparent_index })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set frame transparency", "Transparenz des Einzelbilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
