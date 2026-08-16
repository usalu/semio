//#region 🦠️Mutation
// 🦠️ `insert-accessor` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertAccessor {
    pub index: usize,
    pub accessor: GltfAccessor,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for InsertAccessor {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "accessor", kind: "insert-accessor", record: "InsertAccessor" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "InsertAccessor".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for InsertAccessor {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        if self.index > document.accessors.len() {
            return Err(reject("gltf.mutation.insert-out-of-range", "document/accessors", format!("index {}, length {}", self.index, document.accessors.len())));
        }
        remap_references(document, IndexFamily::Accessor, self.index, true);
        document.accessors.insert(self.index, self.accessor.clone());
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `insert-accessor` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::InsertAccessor;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &InsertAccessor, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `InsertAccessor` semantic inverse.
    
    use super::InsertAccessor;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &InsertAccessor, _base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::RemoveAccessor(RemoveAccessor { index: payload.index })]
    }
}
//#endregion ↩️Inverse

