//! ✂️ `replace-byte-range` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed. The variant was renamed
//! `ReplaceByteRange` (`#[value(rename = "splice")]` on the aggregate variant), but the DSL
//! keyword stays `splice` — that is what the committed grammar/protocol facets and the catalog
//! still speak.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, dsl::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "splice")]
pub struct ReplaceByteRange {
    pub offset: usize,
    pub remove_len: usize,
    #[dsl(base64)]
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
