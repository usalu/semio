//! 📆️ `set-typical-extreme-periods` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTypicalExtremePeriods {
    pub value: String,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for SetTypicalExtremePeriods {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "typical-extreme-periods", kind: "set-typical-extreme-periods", record: "SetTypicalExtremePeriods" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as protocol::Mutation<EpwSnapshot>>::Diff> {
        agg_diff(&EpwMutation::SetTypicalExtremePeriods(self.clone()), base)
    }
    fn inverse(&self, base: &EpwSnapshot) -> Vec<EpwMutation> {
        agg_inverse(&EpwMutation::SetTypicalExtremePeriods(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-typical-extreme-periods".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
