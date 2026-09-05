//! 🔢️️ `set-bit-depth` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBitDepth {
    pub(crate) bit_depth: u8,
}

impl protocol::MutationKind<SemioImageSnapshot, SemioImageMutation> for SetBitDepth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "bit-depth", kind: "set-bit-depth", record: "SetBitDepth" };

    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<SemioImageMutation as protocol::Mutation<SemioImageSnapshot>>::Diff> {
        agg_diff(&SemioImageMutation::SetBitDepth(self.clone()), base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
        agg_inverse(&SemioImageMutation::SetBitDepth(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-bit-depth".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
