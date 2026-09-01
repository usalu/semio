//! 🖼️ `apply-image` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Image subset: `diff`/`inverse` delegate straight through to
//! `SemioImageMutation`'s own already-real `Mutation` impl (via `agg_diff`/`agg_inverse`, lifted
//! verbatim from the former hand-rolled `impl Mutation`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyImage {
    pub(crate) mutation: SemioImageMutation,
}

impl protocol::MutationKind<SemioSnapshot, SemioMutation> for ApplyImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "image", kind: "apply-image", record: "ApplyImage" };

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<SemioMutation as protocol::Mutation<SemioSnapshot>>::Diff> {
        agg_diff(&SemioMutation::ApplyImage(self.clone()), base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Vec<SemioMutation> {
        agg_inverse(&SemioMutation::ApplyImage(self.clone()), base)
    }
    fn label(&self) -> String {
        "image".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
