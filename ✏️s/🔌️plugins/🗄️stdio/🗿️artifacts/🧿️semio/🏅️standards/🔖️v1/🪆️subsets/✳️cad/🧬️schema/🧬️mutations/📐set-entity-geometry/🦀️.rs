//! 📐️ `set-entity-geometry` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEntityGeometry {
    pub(crate) handle: String,
    pub(crate) entity: CadEntity,
}

impl protocol::MutationKind<SemioCadSnapshot, SemioCadMutation> for SetEntityGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "entity-geometry", kind: "set-entity-geometry", record: "SetEntityGeometry" };

    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::Diff> {
        agg_diff(&SemioCadMutation::SetEntityGeometry(self.clone()), base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
        agg_inverse(&SemioCadMutation::SetEntityGeometry(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-entity-geometry".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
