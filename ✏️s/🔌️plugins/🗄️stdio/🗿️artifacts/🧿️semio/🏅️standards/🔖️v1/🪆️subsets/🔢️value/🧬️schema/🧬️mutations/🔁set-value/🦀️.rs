//! 🔁️ `set-value` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetValue {
    pub(crate) path: SemioValuePath,
    pub(crate) value: SemioValue,
}

impl protocol::MutationKind<SemioValueSnapshot, SemioValueMutation> for SetValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "value", kind: "set-value", record: "SetValue" };

    fn diff(&self, base: &SemioValueSnapshot) -> protocol::MutationOutcome<<SemioValueMutation as protocol::Mutation<SemioValueSnapshot>>::Diff> {
        agg_diff(&SemioValueMutation::SetValue(self.clone()), base)
    }
    fn inverse(&self, base: &SemioValueSnapshot) -> Vec<SemioValueMutation> {
        agg_inverse(&SemioValueMutation::SetValue(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-value".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
