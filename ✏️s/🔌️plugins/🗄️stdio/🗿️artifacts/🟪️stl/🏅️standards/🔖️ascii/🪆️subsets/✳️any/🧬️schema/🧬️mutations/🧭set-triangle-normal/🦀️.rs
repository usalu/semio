//! 🧭️ `set-triangle-normal` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs
//! its aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

use super::*;

//#region 🔖️Payload
/// 🧭️ Replaces one triangle's facet normal.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTriangleNormal {
    pub(crate) index: usize,
    pub(crate) normal: [f64; 3],
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for SetTriangleNormal {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "triangle-normal", kind: "set-triangle-normal", record: "SetTriangleNormal" };

    fn diff(&self, base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as protocol::Mutation<StlSnapshot>>::Diff> {
        agg_diff(&StlMutation::SetTriangleNormal(self.clone()), base)
    }
    fn inverse(&self, base: &StlSnapshot) -> Vec<StlMutation> {
        agg_inverse(&StlMutation::SetTriangleNormal(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-triangle-normal".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
