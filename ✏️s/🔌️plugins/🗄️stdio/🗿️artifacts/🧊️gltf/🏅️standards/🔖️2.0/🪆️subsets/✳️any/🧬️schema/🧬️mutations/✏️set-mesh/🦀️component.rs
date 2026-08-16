//#region 🦠️Mutation
// 🦠️ `set-mesh` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMesh {
    pub index: usize,
    pub mesh: GltfMesh,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "mesh", kind: "set-mesh", record: "SetMesh" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "SetMesh".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for SetMesh {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/meshes", self.index, snapshot.document.meshes.len())?;
        snapshot.document.meshes[self.index] = self.mesh.clone();
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `set-mesh` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::SetMesh;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &SetMesh, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `SetMesh` semantic inverse.
    
    use super::SetMesh;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &SetMesh, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.meshes.get(payload.index).map(|mesh| vec![GltfMutation::SetMesh(SetMesh { index: payload.index, mesh: mesh.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

