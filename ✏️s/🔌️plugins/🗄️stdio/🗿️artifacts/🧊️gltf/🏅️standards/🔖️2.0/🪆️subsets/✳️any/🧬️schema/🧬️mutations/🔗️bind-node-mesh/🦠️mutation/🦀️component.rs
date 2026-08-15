//! 🦠️ `bind-node-mesh` GLTF mutation payload.

use super::super::planning::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindNodeMesh {
    pub index: usize,
    pub mesh: Option<usize>,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for BindNodeMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "node-mesh", kind: "bind-node-mesh", record: "BindNodeMesh" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "BindNodeMesh".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for BindNodeMesh {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/nodes", self.index, snapshot.document.nodes.len())?;
        if let Some(mesh) = self.mesh {
            check_index("document/meshes", mesh, snapshot.document.meshes.len())?;
        }
        snapshot.document.nodes[self.index].mesh = self.mesh;
        Ok(())
    }
}
