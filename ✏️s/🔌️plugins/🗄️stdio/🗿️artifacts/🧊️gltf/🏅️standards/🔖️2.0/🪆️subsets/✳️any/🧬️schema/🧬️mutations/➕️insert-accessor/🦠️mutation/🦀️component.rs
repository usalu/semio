//! 🦠️ `insert-accessor` GLTF mutation payload.

use super::super::planning::{reject, remap_references, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::mutations::GltfMutation;
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
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        super::inverse::inverse(self, base)
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
