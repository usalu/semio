//#region 🦠️Mutation
// 🦠️ `remove-material` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{remap_references, remove_checked, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveMaterial {
    pub index: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for RemoveMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "material", kind: "remove-material", record: "RemoveMaterial" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "RemoveMaterial".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for RemoveMaterial {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        let frozen = document.clone();
        remove_checked(&mut document.materials, IndexFamily::Material, self.index, &frozen, "document/materials")?;
        remap_references(document, IndexFamily::Material, self.index, false);
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `remove-material` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::RemoveMaterial;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &RemoveMaterial, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `RemoveMaterial` semantic inverse.
    
    use super::RemoveMaterial;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &RemoveMaterial, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.materials.get(payload.index).map(|material| vec![GltfMutation::InsertMaterial(InsertMaterial { index: payload.index, material: material.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

