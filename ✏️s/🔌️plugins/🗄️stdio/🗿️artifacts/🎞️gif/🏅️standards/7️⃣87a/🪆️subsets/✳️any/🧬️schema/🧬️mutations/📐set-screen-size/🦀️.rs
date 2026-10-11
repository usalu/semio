//! 📐️ `set-screen-size` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-screen-size")]
pub struct SetScreenSize {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetScreenSize {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "screen-size", kind: "set-screen-size", record: "SetScreenSize" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { width, height } = self;
        if let Some((message, target)) = base.images.iter().enumerate().find_map(|(index, image)| image_fits(index, image, (*width, *height))) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new(GifDiff { width: (*width != base.width).then_some(*width), height: (*height != base.height).then_some(*height), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        Ok(vec![GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: base.width, height: base.height })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set screen size", "Bildschirmgröße setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
