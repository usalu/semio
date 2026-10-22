//! 🔁️ `set-points-by-return` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 🔁️ Sets the Number of Points by Return histogram (return channels 1..=5).
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPointsByReturn {
    pub counts: [u32; 5],
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetPointsByReturn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "points-by-return", kind: "set-points-by-return", record: "SetPointsByReturn" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set points by return", "Punktanzahl pro Echo setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<LasSnapshot, LasMutation> for SetPointsByReturn {
    fn diff(&self, _base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::ColdMutation<LasSnapshot>>::ReferenceDiff> {
        let Self { counts } = self;
        protocol::MutationOutcome::new(diff::diff_set_points_by_return(*counts))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok(vec![LasMutation::SetPointsByReturn(SetPointsByReturn { counts: base.header.points_by_return })])
    }
}
//#endregion 🔖️Payload
