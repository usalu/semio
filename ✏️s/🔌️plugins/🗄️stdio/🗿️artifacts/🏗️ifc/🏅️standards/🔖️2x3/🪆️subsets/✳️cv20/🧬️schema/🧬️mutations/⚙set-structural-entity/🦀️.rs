//! ⚙️ `set-structural-entity` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStructuralEntity {
        pub(crate) id: u64,
        pub(crate) entity: Option<Cv20StructuralEntity>,
    }

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Cv20Mutation> for SetStructuralEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "structural-entity", kind: "set-structural-entity", record: "SetStructuralEntity" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<<Ifc2x3Cv20Mutation as protocol::Mutation<Ifc2x3Snapshot>>::Diff> {
        agg_diff(&Ifc2x3Cv20Mutation::SetStructuralEntity(self.clone()), base)
    }
    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3Cv20Mutation> {
        agg_inverse(&Ifc2x3Cv20Mutation::SetStructuralEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-structural-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
