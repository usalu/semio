//! 🔩️ `set-main-namespace` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMainNamespace {
        pub(crate) namespace: String,
    }

impl protocol::MutationKind<PptxSnapshot, PptxStrictMutation> for SetMainNamespace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "main-namespace", kind: "set-main-namespace", record: "SetMainNamespace" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxStrictMutation as protocol::Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxStrictMutation::SetMainNamespace(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Vec<PptxStrictMutation> {
        agg_inverse(&PptxStrictMutation::SetMainNamespace(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-main-namespace".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
