//! 🚫️ `remove-comment` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveComment {
    pub(crate) index: usize,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for RemoveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "comment", kind: "remove-comment", record: "RemoveComment" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as protocol::Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::RemoveComment(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::RemoveComment(self.clone()), base)
    }
    fn label(&self) -> String {
        "remove-comment".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
