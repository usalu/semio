//! 🎬️ `set-main-header` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetMainHeader {
    pub main_header: AviMainHeader,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetMainHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "main-header", kind: "set-main-header", record: "SetMainHeader" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { main_header } = self;
        protocol::MutationOutcome::new(AviDiff { main_header: Some(main_header.clone()), ..AviDiff::default() })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        Ok(vec![AviMutation::SetMainHeader(set_main_header::SetMainHeader { main_header: base.main_header.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set main header", "Haupt-Header setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
