//! 🏷️ `insert-vml-part` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertVmlPart {
        pub(crate) path: String,
        pub(crate) markup: String,
    }

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for InsertVmlPart {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vml-part", kind: "insert-vml-part", record: "InsertVmlPart" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxStrictMutation as protocol::Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxStrictMutation::InsertVmlPart(self.clone()), base)
    }
    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
        agg_inverse(&DocxStrictMutation::InsertVmlPart(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-vml-part".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
