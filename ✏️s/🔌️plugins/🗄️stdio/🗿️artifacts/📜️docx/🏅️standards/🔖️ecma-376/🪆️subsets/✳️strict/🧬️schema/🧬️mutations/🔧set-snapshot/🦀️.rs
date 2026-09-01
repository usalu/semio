//! 🔧️ `set-snapshot` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
        pub(crate) snapshot: DocxSnapshot,
    }

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxStrictMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxStrictMutation::SetSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
        agg_inverse(&DocxStrictMutation::SetSnapshot(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-snapshot".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
