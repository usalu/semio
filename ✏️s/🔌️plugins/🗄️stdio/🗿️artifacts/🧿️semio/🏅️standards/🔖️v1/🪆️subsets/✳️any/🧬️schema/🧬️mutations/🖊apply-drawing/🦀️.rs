//! 🖊️ `apply-drawing` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Drawing subset: `diff`/`inverse` delegate straight through to
//! `SemioDrawingMutation`'s own already-real `Mutation` impl (via `agg_diff`/`agg_inverse`, lifted
//! verbatim from the former hand-rolled `impl Mutation`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyDrawing {
    pub(crate) mutation: SemioDrawingMutation,
}

impl protocol::MutationKind<SemioSnapshot, SemioMutation> for ApplyDrawing {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "drawing", kind: "apply-drawing", record: "ApplyDrawing" };

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<SemioMutation as protocol::Mutation<SemioSnapshot>>::Diff> {
        agg_diff(&SemioMutation::ApplyDrawing(self.clone()), base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Vec<SemioMutation> {
        agg_inverse(&SemioMutation::ApplyDrawing(self.clone()), base)
    }
    fn label(&self) -> String {
        "drawing".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
