//! 🏷️ `set-solid-name` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
/// 🏷️ Sets the `solid`/`endsolid` header/trailer name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSolidName {
    pub(crate) name: String,
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for SetSolidName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solid-name", kind: "set-solid-name", record: "SetSolidName" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as protocol::Mutation<StlSnapshot>>::Diff> {
        agg_diff(&StlMutation::SetSolidName(self.clone()), base)
    }
    fn inverse(&self, base: &StlSnapshot) -> Vec<StlMutation> {
        agg_inverse(&StlMutation::SetSolidName(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-solid-name".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
