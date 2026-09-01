//! 🧩️ `set-project-units` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProjectUnits {
        pub(crate) project: u64,
        pub(crate) units: Option<u64>,
    }

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Cv20Mutation> for SetProjectUnits {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "project-units", kind: "set-project-units", record: "SetProjectUnits" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<<Ifc2x3Cv20Mutation as protocol::Mutation<Ifc2x3Snapshot>>::Diff> {
        agg_diff(&Ifc2x3Cv20Mutation::SetProjectUnits(self.clone()), base)
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3Cv20Mutation> {
        agg_inverse(&Ifc2x3Cv20Mutation::SetProjectUnits(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-project-units".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
