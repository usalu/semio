//! 🔺️ `set-block-entity-geometry` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetBlockEntityGeometry {
    pub(crate) block_name: String,
    pub(crate) handle: String,
    pub(crate) entity: CadEntity,
}

impl protocol::MutationKind<SemioCadSnapshot, SemioCadMutation> for SetBlockEntityGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "block-entity-geometry", kind: "set-block-entity-geometry", record: "SetBlockEntityGeometry" };

    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<SemioCadMutation as protocol::Mutation<SemioCadSnapshot>>::Diff> {
        agg_diff(&SemioCadMutation::SetBlockEntityGeometry(self.clone()), base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Vec<SemioCadMutation> {
        agg_inverse(&SemioCadMutation::SetBlockEntityGeometry(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-block-entity-geometry".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
