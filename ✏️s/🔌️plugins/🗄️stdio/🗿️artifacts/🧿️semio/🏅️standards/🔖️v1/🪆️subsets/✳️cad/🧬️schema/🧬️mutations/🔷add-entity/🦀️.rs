//! 🔷️ `add-entity` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddEntity {
    pub(crate) entity: CadEntityRecord,
}

impl protocol::MutationKind<SemioCadSnapshot, SemioCadMutation> for AddEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "entity", kind: "add-entity", record: "AddEntity" };

    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::Diff> {
        agg_diff(&SemioCadMutation::AddEntity(self.clone()), base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
        agg_inverse(&SemioCadMutation::AddEntity(self.clone()), base)
    }
    fn label(&self) -> String {
        "add-entity".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
