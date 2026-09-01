//! 📷️ `set-viewpoint-camera` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetViewpointCamera {
    pub(crate) topic_guid: String,
    pub(crate) guid: String,
    pub(crate) camera: Option<BcfCamera>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetViewpointCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "viewpoint-camera", kind: "set-viewpoint-camera", record: "SetViewpointCamera" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as protocol::Mutation<BcfSnapshot>>::Diff> {
        agg_diff(&BcfMutation::SetViewpointCamera(self.clone()), base)
    }
    fn inverse(&self, base: &BcfSnapshot) -> Vec<BcfMutation> {
        agg_inverse(&BcfMutation::SetViewpointCamera(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-viewpoint-camera".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
