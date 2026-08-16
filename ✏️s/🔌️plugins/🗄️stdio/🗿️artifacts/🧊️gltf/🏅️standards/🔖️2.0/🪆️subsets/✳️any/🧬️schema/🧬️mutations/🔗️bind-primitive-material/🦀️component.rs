//#region 🦠️Mutation
// 🦠️ `bind-primitive-material` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindPrimitiveMaterial {
    pub mesh: usize,
    pub primitive: usize,
    pub material: Option<usize>,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for BindPrimitiveMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "primitive-material", kind: "bind-primitive-material", record: "BindPrimitiveMaterial" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "BindPrimitiveMaterial".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.mesh.to_string(), self.primitive.to_string()]
    }
}

impl GltfSemanticMutation for BindPrimitiveMaterial {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/meshes", self.mesh, snapshot.document.meshes.len())?;
        check_index(format!("document/meshes/{}/primitives", self.mesh), self.primitive, snapshot.document.meshes[self.mesh].primitives.len())?;
        if let Some(material) = self.material {
            check_index("document/materials", material, snapshot.document.materials.len())?;
        }
        snapshot.document.meshes[self.mesh].primitives[self.primitive].material = self.material;
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `bind-primitive-material` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::BindPrimitiveMaterial;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &BindPrimitiveMaterial, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `BindPrimitiveMaterial` semantic inverse.
    
    use super::BindPrimitiveMaterial;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &BindPrimitiveMaterial, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document
            .meshes
            .get(payload.mesh)
            .and_then(|mesh| mesh.primitives.get(payload.primitive))
            .map(|primitive| vec![GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterial { mesh: payload.mesh, primitive: payload.primitive, material: primitive.material })])
            .unwrap_or_default()
    }
}
//#endregion ↩️Inverse

