//#region 🦠️Mutation
// 🦠️ `insert-animation` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{reject, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertAnimation {
    pub index: usize,
    pub animation: GltfAnimation,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertAnimation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "animation", kind: "insert-animation", record: "InsertAnimation" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertAnimation".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertAnimation {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        if self.index > snapshot.document.animations.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/animations", format!("index {}, length {}", self.index, snapshot.document.animations.len())));
        }
        snapshot.document.animations.insert(self.index, self.animation.clone());
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `insert-animation` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::InsertAnimation;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &InsertAnimation, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `InsertAnimation` semantic inverse.
    
    use super::InsertAnimation;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &InsertAnimation, _base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::RemoveAnimation(RemoveAnimation { index: payload.index })]
    }
}
//#endregion ↩️Inverse

