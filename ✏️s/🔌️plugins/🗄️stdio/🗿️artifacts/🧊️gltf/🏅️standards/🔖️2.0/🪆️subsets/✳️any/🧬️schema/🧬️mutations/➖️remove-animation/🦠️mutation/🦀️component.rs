//! 🦠️ `remove-animation` GLTF mutation payload.

use super::super::planning::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAnimation {
    pub index: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for RemoveAnimation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "animation", kind: "remove-animation", record: "RemoveAnimation" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "RemoveAnimation".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for RemoveAnimation {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/animations", self.index, snapshot.document.animations.len())?;
        snapshot.document.animations.remove(self.index);
        Ok(())
    }
}
