//! ➖️ `remove-stream` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveStream {
    pub index: usize,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for RemoveStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "stream", kind: "remove-stream", record: "RemoveStream" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as protocol::Mutation<AviSnapshot>>::Diff> {
        agg_diff(&AviMutation::RemoveStream(self.clone()), base)
    }
    fn inverse(&self, base: &AviSnapshot) -> Vec<AviMutation> {
        agg_inverse(&AviMutation::RemoveStream(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-stream".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
