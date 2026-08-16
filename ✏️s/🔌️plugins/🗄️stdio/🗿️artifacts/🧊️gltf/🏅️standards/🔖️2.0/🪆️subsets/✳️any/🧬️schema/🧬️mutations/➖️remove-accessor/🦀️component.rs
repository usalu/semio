//#region 🦠️Mutation
// 🦠️ `remove-accessor` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{remap_references, remove_checked, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAccessor {
    pub index: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for RemoveAccessor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "accessor", kind: "remove-accessor", record: "RemoveAccessor" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "RemoveAccessor".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for RemoveAccessor {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        let frozen = document.clone();
        remove_checked(&mut document.accessors, IndexFamily::Accessor, self.index, &frozen, "document/accessors")?;
        remap_references(document, IndexFamily::Accessor, self.index, false);
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `remove-accessor` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::RemoveAccessor;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &RemoveAccessor, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `RemoveAccessor` semantic inverse.
    
    use super::RemoveAccessor;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &RemoveAccessor, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.accessors.get(payload.index).map(|accessor| vec![GltfMutation::InsertAccessor(InsertAccessor { index: payload.index, accessor: accessor.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

