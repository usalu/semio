//! ⚙️ `set-relationships-namespace` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRelationshipsNamespace {
        pub(crate) namespace: String,
    }

impl protocol::MutationKind<XlsxSnapshot, XlsxStrictMutation> for SetRelationshipsNamespace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relationships-namespace", kind: "set-relationships-namespace", record: "SetRelationshipsNamespace" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<<XlsxStrictMutation as protocol::Mutation<XlsxSnapshot>>::Diff> {
        agg_diff(&XlsxStrictMutation::SetRelationshipsNamespace(self.clone()), base)
    }
    fn inverse(&self, base: &XlsxSnapshot) -> Vec<XlsxStrictMutation> {
        agg_inverse(&XlsxStrictMutation::SetRelationshipsNamespace(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-relationships-namespace".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
