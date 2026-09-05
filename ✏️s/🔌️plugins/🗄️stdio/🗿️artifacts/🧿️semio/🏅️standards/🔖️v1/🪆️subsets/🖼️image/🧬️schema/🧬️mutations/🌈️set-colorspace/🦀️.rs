//! 🌈️️ `set-colorspace` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetColorspace {
    pub(crate) colorspace: SemioColorspace,
}

impl protocol::MutationKind<SemioImageSnapshot, SemioImageMutation> for SetColorspace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "colorspace", kind: "set-colorspace", record: "SetColorspace" };

    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<SemioImageMutation as protocol::Mutation<SemioImageSnapshot>>::Diff> {
        agg_diff(&SemioImageMutation::SetColorspace(self.clone()), base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
        agg_inverse(&SemioImageMutation::SetColorspace(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-colorspace".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
