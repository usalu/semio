//! 🔖️ `insert-tile-tags` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTileTags {
        pub(crate) tile_width: u32,
        pub(crate) tile_length: u32,
    }

impl protocol::MutationKind<TiffSnapshot, TiffBaselineMutation> for InsertTileTags {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "tile-tags", kind: "insert-tile-tags", record: "InsertTileTags" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffBaselineMutation as protocol::Mutation<TiffSnapshot>>::Diff> {
        agg_diff(&TiffBaselineMutation::InsertTileTags(self.clone()), base)
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffBaselineMutation> {
        agg_inverse(&TiffBaselineMutation::InsertTileTags(self.clone()), base)
    }
    fn label(&self) -> String {
        "insert-tile-tags".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
