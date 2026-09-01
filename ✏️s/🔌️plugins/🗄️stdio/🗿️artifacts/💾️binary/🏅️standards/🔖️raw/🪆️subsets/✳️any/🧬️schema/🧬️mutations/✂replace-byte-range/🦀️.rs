//! ✂️ `replace-byte-range` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceByteRange {
    pub offset: usize,
    pub remove_len: usize,
    pub insert: Vec<u8>,
}

impl protocol::MutationKind<BinarySnapshot, BinaryMutation> for ReplaceByteRange {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "byte-range", kind: "replace-byte-range", record: "ReplaceByteRange" };

    fn diff(&self, base: &BinarySnapshot) -> protocol::MutationOutcome<<BinaryMutation as protocol::Mutation<BinarySnapshot>>::Diff> {
        agg_diff(&BinaryMutation::ReplaceByteRange(self.clone()), base)
    }
    fn inverse(&self, base: &BinarySnapshot) -> Vec<BinaryMutation> {
        agg_inverse(&BinaryMutation::ReplaceByteRange(self.clone()), base)
    }
    fn label(&self) -> String {
        "splice".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
