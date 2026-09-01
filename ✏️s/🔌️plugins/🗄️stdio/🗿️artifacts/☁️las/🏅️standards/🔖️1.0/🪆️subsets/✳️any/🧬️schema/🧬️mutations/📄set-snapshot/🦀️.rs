//! 📄️ `set-snapshot` — its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value
//! and delegates, so the semantics are preserved by construction rather than re-derived.
//!
//! The sibling `↩️inverse/` and `🔺️diff/` files beside this one predate this leaf shape (contract
//! D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION) and are kept as committed, untouched by this
//! migration — they are not `mod`-wired into this leaf and stay free-standing.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub snapshot: LasSnapshot,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::Mutation<LasSnapshot>>::Diff> {
        agg_diff(&LasMutation::SetSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &LasSnapshot) -> Vec<LasMutation> {
        agg_inverse(&LasMutation::SetSnapshot(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-snapshot".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
