//! 🧱️ `upsert-instance` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpsertInstance {
    pub(crate) instance: Part21Instance,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation> for UpsertInstance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "instance", kind: "upsert-instance", record: "UpsertInstance" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<<Ifc2x3Mutation as protocol::Mutation<Ifc2x3Snapshot>>::Diff> {
        agg_diff(&Ifc2x3Mutation::UpsertInstance(self.clone()), base)
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3Mutation> {
        agg_inverse(&Ifc2x3Mutation::UpsertInstance(self.clone()), base)
    }
    fn label(&self) -> String {
        "upsert-instance".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
