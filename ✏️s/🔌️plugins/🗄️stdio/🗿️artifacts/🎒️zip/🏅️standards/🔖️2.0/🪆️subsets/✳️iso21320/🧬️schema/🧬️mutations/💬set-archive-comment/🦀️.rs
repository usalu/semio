//! 💬️ `set-archive-comment` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by
//! construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetArchiveComment {
    pub(crate) comment: String,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for SetArchiveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "archive-comment", kind: "set-archive-comment", record: "SetArchiveComment" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipIso21320Mutation::SetArchiveComment(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Vec<ZipIso21320Mutation> {
        agg_inverse(&ZipIso21320Mutation::SetArchiveComment(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-archive-comment".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
