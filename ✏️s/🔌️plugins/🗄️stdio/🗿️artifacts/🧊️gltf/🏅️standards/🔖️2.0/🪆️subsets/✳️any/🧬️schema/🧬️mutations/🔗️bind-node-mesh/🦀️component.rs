//#region 🦠️Mutation
// 🦠️ `bind-node-mesh` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
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
    async fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    async fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "BindNodeMesh".into()
    }
    async fn target(&self) -> Vec<String> {
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
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `bind-node-mesh` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::BindNodeMesh;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn diff(payload: &BindNodeMesh, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `BindNodeMesh` semantic inverse.
    
    use super::BindNodeMesh;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(payload: &BindNodeMesh, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::BindNodeMesh(BindNodeMesh { index: payload.index, mesh: node.mesh })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

