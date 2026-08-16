//#region 🦠️Mutation
// 🦠️ `remove-animation` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
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
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
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
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `remove-animation` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::RemoveAnimation;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &RemoveAnimation, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `RemoveAnimation` semantic inverse.
    
    use super::RemoveAnimation;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &RemoveAnimation, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.animations.get(payload.index).map(|animation| vec![GltfMutation::InsertAnimation(InsertAnimation { index: payload.index, animation: animation.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

