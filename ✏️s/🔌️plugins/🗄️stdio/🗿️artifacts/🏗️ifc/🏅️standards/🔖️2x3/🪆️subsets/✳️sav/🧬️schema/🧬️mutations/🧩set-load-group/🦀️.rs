//! 🧩️ `set-load-group` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLoadGroup {
        pub(crate) id: u64,
        pub(crate) group: Option<SavLoadGroup>,
    }

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3SavMutation> for SetLoadGroup {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "load-group", kind: "set-load-group", record: "SetLoadGroup" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<<Ifc2x3SavMutation as protocol::Mutation<Ifc2x3Snapshot>>::Diff> {
        agg_diff(&Ifc2x3SavMutation::SetLoadGroup(self.clone()), base)
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3SavMutation> {
        agg_inverse(&Ifc2x3SavMutation::SetLoadGroup(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-load-group".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
