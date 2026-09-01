//! 📏️ `remove-strip-offsets` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveStripOffsets {}

impl protocol::MutationKind<TiffSnapshot, TiffBaselineMutation> for RemoveStripOffsets {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "strip-offsets", kind: "remove-strip-offsets", record: "RemoveStripOffsets" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffBaselineMutation as protocol::Mutation<TiffSnapshot>>::Diff> {
        agg_diff(&TiffBaselineMutation::RemoveStripOffsets(self.clone()), base)
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffBaselineMutation> {
        agg_inverse(&TiffBaselineMutation::RemoveStripOffsets(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-strip-offsets".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
