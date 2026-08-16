//#region 🦠️Mutation
// 🦠️ `insert-mesh` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertMesh {
    pub index: usize,
    pub mesh: GltfMesh,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "mesh", kind: "insert-mesh", record: "InsertMesh" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertMesh".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertMesh {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        if self.index > document.meshes.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/meshes", format!("index {}, length {}", self.index, document.meshes.len())));
        }
        remap_references(document, IndexFamily::Mesh, self.index, true);
        document.meshes.insert(self.index, self.mesh.clone());
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `insert-mesh` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::InsertMesh;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &InsertMesh, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `InsertMesh` semantic inverse.
    
    use super::InsertMesh;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &InsertMesh, _base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::RemoveMesh(RemoveMesh { index: payload.index })]
    }
}
//#endregion ↩️Inverse

