//! 🪡️ `set-linetype` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLinetype {
    pub name: String,
    pub linetype: DxfLinetype,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetLinetype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "linetype", kind: "set-linetype", record: "SetLinetype" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as protocol::Mutation<DxfSnapshot>>::Diff> {
        agg_diff(&DxfMutation::SetLinetype(self.clone()), base)
    }
    fn inverse(&self, base: &DxfSnapshot) -> Vec<DxfMutation> {
        agg_inverse(&DxfMutation::SetLinetype(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-linetype".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
