//! 🧩️ `set-arithmetic` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetArithmetic {
        pub(crate) arithmetic: bool,
    }

impl protocol::MutationKind<JpgSnapshot, JpgBaselineMutation> for SetArithmetic {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "arithmetic", kind: "set-arithmetic", record: "SetArithmetic" };

    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<<JpgBaselineMutation as protocol::Mutation<JpgSnapshot>>::Diff> {
        agg_diff(&JpgBaselineMutation::SetArithmetic(self.clone()), base)
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgBaselineMutation> {
        agg_inverse(&JpgBaselineMutation::SetArithmetic(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-arithmetic".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
