//! 📖️ `set-preset-dictionary` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by construction
//! rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPresetDictionary {
    pub dict_id: Option<u32>,
}

impl protocol::MutationKind<DeflateSnapshot, DeflateMutation> for SetPresetDictionary {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preset-dictionary", kind: "set-preset-dictionary", record: "SetPresetDictionary" };

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<<DeflateMutation as protocol::Mutation<DeflateSnapshot>>::Diff> {
        agg_diff(&DeflateMutation::SetPresetDictionary(self.clone()), base)
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Vec<DeflateMutation> {
        agg_inverse(&DeflateMutation::SetPresetDictionary(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-preset-dictionary".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
