//! 📃️ `apply-document` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Document subset: `diff`/`inverse` delegate straight through to
//! `SemioDocumentMutation`'s own already-real `Mutation` impl (via `agg_diff`/`agg_inverse`, lifted
//! verbatim from the former hand-rolled `impl Mutation`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyDocument {
    pub(crate) mutation: SemioDocumentMutation,
}

impl protocol::MutationKind<SemioSnapshot, SemioMutation> for ApplyDocument {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "document", kind: "apply-document", record: "ApplyDocument" };

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<SemioMutation as protocol::Mutation<SemioSnapshot>>::Diff> {
        agg_diff(&SemioMutation::ApplyDocument(self.clone()), base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Vec<SemioMutation> {
        agg_inverse(&SemioMutation::ApplyDocument(self.clone()), base)
    }
    fn label(&self) -> String {
        "document".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
