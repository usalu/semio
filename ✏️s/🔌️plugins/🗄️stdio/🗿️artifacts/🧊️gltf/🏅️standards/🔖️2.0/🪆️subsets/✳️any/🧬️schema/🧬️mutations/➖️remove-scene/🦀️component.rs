//#region 🦠️Mutation
// 🦠️ `remove-scene` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{remap_references, remove_checked, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScene {
    pub index: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for RemoveScene {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "scene", kind: "remove-scene", record: "RemoveScene" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "RemoveScene".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for RemoveScene {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        let frozen = document.clone();
        remove_checked(&mut document.scenes, IndexFamily::Scene, self.index, &frozen, "document/scenes")?;
        remap_references(document, IndexFamily::Scene, self.index, false);
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `remove-scene` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::RemoveScene;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &RemoveScene, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `RemoveScene` semantic inverse.
    
    use super::RemoveScene;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &RemoveScene, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.scenes.get(payload.index).map(|scene| vec![GltfMutation::InsertScene(InsertScene { index: payload.index, scene: scene.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

