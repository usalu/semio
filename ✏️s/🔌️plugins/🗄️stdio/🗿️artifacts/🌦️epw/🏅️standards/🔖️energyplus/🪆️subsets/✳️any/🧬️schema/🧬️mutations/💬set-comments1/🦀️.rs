//! 💬️ `set-comments1` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetComments1 {
    pub value: String,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for SetComments1 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "comments1", kind: "set-comments1", record: "SetComments1" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as protocol::Mutation<EpwSnapshot>>::Diff> {
        agg_diff(&EpwMutation::SetComments1(self.clone()), base)
    }
    fn inverse(&self, base: &EpwSnapshot) -> Vec<EpwMutation> {
        agg_inverse(&EpwMutation::SetComments1(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-comments1".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
