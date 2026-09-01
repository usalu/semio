//! 🔊️ `set-data` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetData {
    pub data: WavData,
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for SetData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "data", kind: "set-data", record: "SetData" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as protocol::Mutation<WavSnapshot>>::Diff> {
        agg_diff(&WavMutation::SetData(self.clone()), base)
    }
    fn inverse(&self, base: &WavSnapshot) -> Vec<WavMutation> {
        agg_inverse(&WavMutation::SetData(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-data".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
