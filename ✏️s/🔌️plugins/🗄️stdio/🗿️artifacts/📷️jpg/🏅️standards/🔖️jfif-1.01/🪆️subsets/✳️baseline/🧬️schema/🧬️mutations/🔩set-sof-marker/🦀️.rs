//! 🔩️ `set-sof-marker` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSofMarker {
        pub(crate) marker: u8,
    }

impl protocol::MutationKind<JpgSnapshot, JpgBaselineMutation> for SetSofMarker {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sof-marker", kind: "set-sof-marker", record: "SetSofMarker" };

    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<<JpgBaselineMutation as protocol::Mutation<JpgSnapshot>>::Diff> {
        agg_diff(&JpgBaselineMutation::SetSofMarker(self.clone()), base)
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgBaselineMutation> {
        agg_inverse(&JpgBaselineMutation::SetSofMarker(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-sof-marker".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
