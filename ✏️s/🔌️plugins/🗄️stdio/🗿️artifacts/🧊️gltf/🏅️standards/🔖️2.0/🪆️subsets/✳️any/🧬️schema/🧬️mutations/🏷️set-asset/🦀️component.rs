//#region 🦠️Mutation
// 🦠️ `set-asset` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAsset {
    pub asset: GltfAsset,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetAsset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "asset", kind: "set-asset", record: "SetAsset" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "SetAsset".into()
    }
    fn target(&self) -> Vec<String> {
        vec![]
    }
}

impl GltfSemanticMutation for SetAsset {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        snapshot.document.asset = self.asset.clone();
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `set-asset` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::SetAsset;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &SetAsset, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `SetAsset` semantic inverse.
    
    use super::SetAsset;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(_payload: &SetAsset, base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::SetAsset(SetAsset { asset: base.document.asset.clone() })]
    }
}
//#endregion ↩️Inverse

