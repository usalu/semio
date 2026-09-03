//! 🏛️ `apply-model` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Model subset: `diff`/`inverse` delegate straight through to
//! `SemioModelMutation`'s own already-real `Mutation` impl (via `agg_diff`/`agg_inverse`, lifted
//! verbatim from the former hand-rolled `impl Mutation`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyModel {
    pub(crate) mutation: SemioModelMutation,
}

impl protocol::MutationKind<SemioSnapshot, SemioMutation> for ApplyModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "model", kind: "apply-model", record: "ApplyModel" };

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<SemioMutation as protocol::Mutation<SemioSnapshot>>::Diff> {
        agg_diff(&SemioMutation::ApplyModel(self.clone()), base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Vec<SemioMutation> {
        agg_inverse(&SemioMutation::ApplyModel(self.clone()), base)
    }
    fn label(&self) -> String {
        "model".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
