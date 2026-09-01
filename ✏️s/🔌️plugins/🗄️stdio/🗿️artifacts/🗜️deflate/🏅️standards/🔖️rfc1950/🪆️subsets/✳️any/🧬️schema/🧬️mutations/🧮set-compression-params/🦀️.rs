//! 🧮️ `set-compression-params` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by construction
//! rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCompressionParams {
    pub method: u8,
    pub window_bits: u8,
    pub level_hint: DeflateLevelHint,
}

impl protocol::MutationKind<DeflateSnapshot, DeflateMutation> for SetCompressionParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "compression-params", kind: "set-compression-params", record: "SetCompressionParams" };

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<<DeflateMutation as protocol::Mutation<DeflateSnapshot>>::Diff> {
        agg_diff(&DeflateMutation::SetCompressionParams(self.clone()), base)
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Vec<DeflateMutation> {
        agg_inverse(&DeflateMutation::SetCompressionParams(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-compression-params".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
