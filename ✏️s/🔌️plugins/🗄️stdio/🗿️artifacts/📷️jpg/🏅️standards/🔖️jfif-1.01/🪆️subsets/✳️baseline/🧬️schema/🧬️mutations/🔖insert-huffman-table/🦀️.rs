//! 🔖️ `insert-huffman-table` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertHuffmanTable {
        pub(crate) index: usize,
        pub(crate) table: JpgHuffmanTable,
    }

impl protocol::MutationKind<JpgSnapshot, JpgBaselineMutation> for InsertHuffmanTable {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "huffman-table", kind: "insert-huffman-table", record: "InsertHuffmanTable" };

    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<<JpgBaselineMutation as protocol::Mutation<JpgSnapshot>>::Diff> {
        agg_diff(&JpgBaselineMutation::InsertHuffmanTable(self.clone()), base)
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgBaselineMutation> {
        agg_inverse(&JpgBaselineMutation::InsertHuffmanTable(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-huffman-table".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
