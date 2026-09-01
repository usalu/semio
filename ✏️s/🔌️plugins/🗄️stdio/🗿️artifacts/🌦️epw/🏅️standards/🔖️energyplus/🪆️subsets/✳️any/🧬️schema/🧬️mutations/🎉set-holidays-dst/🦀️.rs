//! 🎉️ `set-holidays-dst` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetHolidaysDst {
    pub value: String,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for SetHolidaysDst {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "holidays-dst", kind: "set-holidays-dst", record: "SetHolidaysDst" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as protocol::Mutation<EpwSnapshot>>::Diff> {
        agg_diff(&EpwMutation::SetHolidaysDst(self.clone()), base)
    }
    fn inverse(&self, base: &EpwSnapshot) -> Vec<EpwMutation> {
        agg_inverse(&EpwMutation::SetHolidaysDst(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-holidays-dst".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
