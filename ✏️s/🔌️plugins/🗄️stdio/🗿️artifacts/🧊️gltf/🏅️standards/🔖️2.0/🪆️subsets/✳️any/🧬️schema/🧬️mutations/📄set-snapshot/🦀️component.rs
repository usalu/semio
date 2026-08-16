//#region 🦠️Mutation
// 🦠️ `set-snapshot` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub snapshot: GltfSnapshot,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "SetSnapshot".into()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl GltfSemanticMutation for SetSnapshot {
    fn apply(&self, target: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        *target = self.snapshot.clone();
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `set-snapshot` sparse diff.
    
    use super::SetSnapshot;
    use crate::artifacts::gltf::schema::diff::{diff_set_snapshot, GltfDiff};
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &SetSnapshot, base: &GltfSnapshot) -> GltfDiff {
        diff_set_snapshot(base, &payload.snapshot)
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `set-snapshot` semantic inverse.
    
    use super::SetSnapshot;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(_payload: &SetSnapshot, base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
    }
}
//#endregion ↩️Inverse
