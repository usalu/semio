//! 🔊️ `apply-audio` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Audio subset: `diff`/`inverse` delegate straight through to
//! `SemioAudioMutation`'s own already-real `Mutation` impl (via `agg_diff`/`agg_inverse`, lifted
//! verbatim from the former hand-rolled `impl Mutation`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyAudio {
    pub(crate) mutation: SemioAudioMutation,
}

impl protocol::MutationKind<SemioSnapshot, SemioMutation> for ApplyAudio {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "audio", kind: "apply-audio", record: "ApplyAudio" };

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<SemioMutation as protocol::Mutation<SemioSnapshot>>::Diff> {
        agg_diff(&SemioMutation::ApplyAudio(self.clone()), base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Vec<SemioMutation> {
        agg_inverse(&SemioMutation::ApplyAudio(self.clone()), base)
    }
    fn label(&self) -> String {
        "audio".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
