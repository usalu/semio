//#region 🦠️Mutation
// 🦠️ `no-mutation` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoMutation {}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for NoMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "no", entity: "mutation", kind: "no-mutation", record: "NoMutation" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "NoMutation".into()
    }
    fn target(&self) -> Vec<String> {
        vec![]
    }
}

impl GltfSemanticMutation for NoMutation {
    fn apply(&self, _snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `no-mutation` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::NoMutation;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &NoMutation, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `NoMutation` semantic inverse.
    
    use super::NoMutation;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(_payload: &NoMutation, _base: &GltfSnapshot) -> Vec<GltfMutation> {
        Vec::new()
    }
}
//#endregion ↩️Inverse

