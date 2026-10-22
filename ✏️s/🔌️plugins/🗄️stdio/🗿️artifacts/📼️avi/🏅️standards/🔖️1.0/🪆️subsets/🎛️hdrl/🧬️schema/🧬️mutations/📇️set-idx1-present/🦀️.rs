//! 🗂️ `set-idx1-present` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetIdx1Present {
    pub idx1_present: bool,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetIdx1Present {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "idx1-present", kind: "set-idx1-present", record: "SetIdx1Present" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set idx1 present", "idx1-Index vorhanden setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<AviSnapshot, AviMutation> for SetIdx1Present {
    fn diff(&self, _base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as protocol::ColdMutation<AviSnapshot>>::ReferenceDiff> {
        let Self { idx1_present } = self;
        protocol::MutationOutcome::new(AviDiff { idx1_present: Some(*idx1_present), ..AviDiff::default() })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        Ok(vec![AviMutation::SetIdx1Present(SetIdx1Present { idx1_present: base.idx1_present })])
    }
}
//#endregion 🔖️Payload
