//! 🔷️ `insert-shape` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertShape {
    pub(crate) slide_index: usize,
    pub(crate) shape_index: usize,
    pub(crate) shape: PptxShape,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for InsertShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "shape", kind: "insert-shape", record: "InsertShape" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as protocol::Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::InsertShape(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Vec<PptxMutation> {
        agg_inverse(&PptxMutation::InsertShape(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-shape".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
