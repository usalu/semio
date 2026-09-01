//! ⚙️ `set-photometric-interpretation` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPhotometricInterpretation {
        pub(crate) photometric: u16,
    }

impl protocol::MutationKind<TiffSnapshot, TiffBaselineMutation> for SetPhotometricInterpretation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "photometric-interpretation", kind: "set-photometric-interpretation", record: "SetPhotometricInterpretation" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffBaselineMutation as protocol::Mutation<TiffSnapshot>>::Diff> {
        agg_diff(&TiffBaselineMutation::SetPhotometricInterpretation(self.clone()), base)
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffBaselineMutation> {
        agg_inverse(&TiffBaselineMutation::SetPhotometricInterpretation(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-photometric-interpretation".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
