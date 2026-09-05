//! 🔗️ `set-relation` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRelation {
    pub(crate) id: String,
    #[value(default)]
    pub(crate) kind: Option<RelationKind>,
    #[value(default)]
    pub(crate) from: Option<String>,
    #[value(default)]
    pub(crate) to: Option<String>,
}

impl protocol::MutationKind<SemioModelSnapshot, SemioModelMutation> for SetRelation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relation", kind: "set-relation", record: "SetRelation" };

    fn diff(&self, base: &SemioModelSnapshot) -> protocol::MutationOutcome<<SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::Diff> {
        agg_diff(&SemioModelMutation::SetRelation(self.clone()), base)
    }
    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
        agg_inverse(&SemioModelMutation::SetRelation(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-relation".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
