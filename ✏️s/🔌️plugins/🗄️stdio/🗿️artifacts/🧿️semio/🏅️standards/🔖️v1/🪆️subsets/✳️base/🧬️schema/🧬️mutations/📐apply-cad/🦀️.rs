//! 📐️ `apply-cad` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Cad subset: `diff`/`inverse` delegate straight through to
//! `SemioCadMutation`'s own already-real `Mutation` impl (via `agg_diff`/`agg_inverse`, lifted
//! verbatim from the former hand-rolled `impl Mutation`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyCad {
    pub(crate) mutation: SemioCadMutation,
}

impl protocol::MutationKind<SemioSnapshot, SemioMutation> for ApplyCad {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "cad", kind: "apply-cad", record: "ApplyCad" };

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<SemioMutation as protocol::Mutation<SemioSnapshot>>::Diff> {
        agg_diff(&SemioMutation::ApplyCad(self.clone()), base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Vec<SemioMutation> {
        agg_inverse(&SemioMutation::ApplyCad(self.clone()), base)
    }
    fn label(&self) -> String {
        "cad".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
