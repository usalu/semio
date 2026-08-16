//#region 🦠️Mutation
// 🦠️ `set-scene` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetScene {
    pub index: usize,
    pub scene: GltfScene,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetScene {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "scene", kind: "set-scene", record: "SetScene" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "SetScene".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for SetScene {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/scenes", self.index, snapshot.document.scenes.len())?;
        snapshot.document.scenes[self.index] = self.scene.clone();
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `set-scene` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::SetScene;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &SetScene, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `SetScene` semantic inverse.
    
    use super::SetScene;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &SetScene, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.scenes.get(payload.index).map(|scene| vec![GltfMutation::SetScene(SetScene { index: payload.index, scene: scene.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

