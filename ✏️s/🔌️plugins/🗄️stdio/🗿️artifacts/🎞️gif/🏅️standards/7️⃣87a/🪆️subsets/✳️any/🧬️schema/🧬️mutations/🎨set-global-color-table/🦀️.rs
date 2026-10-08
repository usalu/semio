//! 🎨️ `set-global-color-table` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from
//! its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-global-color-table")]
pub struct SetGlobalColorTable {
    #[dsl(block)]
    pub(crate) gct: Option<GifColorTable>,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetGlobalColorTable {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "global-color-table", kind: "set-global-color-table", record: "SetGlobalColorTable" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
        let Self { gct } = self;
        if let Some((message, target)) = base.images.iter().enumerate().filter(|(_, image)| image.lct.is_none()).find_map(|(index, image)| image_colored(index, image, gct.as_ref())) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, target);
        }
        protocol::MutationOutcome::new(GifDiff { gct: (*gct != base.gct).then_some(gct.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &GifSnapshot) -> Result<Vec<GifMutation>, semio_framework_value::ValueError> {
        Ok(vec![GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: base.gct.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set global color table", "Globale Farbtabelle setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
