//! 🖌️ `set-background-color-index` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-background-color-index")]
pub struct SetBackgroundColorIndex {
    pub(crate) index: u8,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetBackgroundColorIndex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "background-color-index", kind: "set-background-color-index", record: "SetBackgroundColorIndex" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(GifDiff { background_color_index: (*index != base.background_color_index).then_some(*index), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        Ok(vec![GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: base.background_color_index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set background color index", "Hintergrundfarbindex setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
