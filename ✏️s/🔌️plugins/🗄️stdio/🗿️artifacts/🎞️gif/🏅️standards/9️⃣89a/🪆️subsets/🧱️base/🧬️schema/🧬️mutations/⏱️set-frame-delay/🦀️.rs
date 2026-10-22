//! ⏱️ `set-frame-delay` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-delay")]
#[value(rename_all = "camelCase")]
pub struct SetFrameDelay {
    pub index: usize,
    pub delay_cs: u16,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFrameDelay {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-delay", kind: "set-frame-delay", record: "SetFrameDelay" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set frame delay", "Verzögerung des Einzelbilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<GifSnapshot, GifMutation> for SetFrameDelay {
    fn diff(&self, _base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index, delay_cs } = self;
        protocol::MutationOutcome::new({
            let d = GifFrameDiff { delay_cs: Some(*delay_cs), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameDelay(SetFrameDelay { index: *index, delay_cs: f.delay_cs })],
            None => Vec::new(),
        })
    }
}
//#endregion 🔖️Payload
