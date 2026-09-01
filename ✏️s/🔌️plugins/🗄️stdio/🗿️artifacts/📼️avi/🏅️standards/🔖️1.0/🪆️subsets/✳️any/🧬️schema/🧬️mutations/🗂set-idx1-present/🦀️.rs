//! 🗂️ `set-idx1-present` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetIdx1Present {
    pub idx1_present: bool,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for SetIdx1Present {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "idx1-present", kind: "set-idx1-present", record: "SetIdx1Present" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as protocol::Mutation<AviSnapshot>>::Diff> {
        agg_diff(&AviMutation::SetIdx1Present(self.clone()), base)
    }
    fn inverse(&self, base: &AviSnapshot) -> Vec<AviMutation> {
        agg_inverse(&AviMutation::SetIdx1Present(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-idx1-present".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
