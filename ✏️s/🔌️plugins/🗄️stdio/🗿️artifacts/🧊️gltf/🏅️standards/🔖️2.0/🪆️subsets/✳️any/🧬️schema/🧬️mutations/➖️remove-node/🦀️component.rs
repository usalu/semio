//#region 🦠️Mutation
// 🦠️ `remove-node` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{remap_references, remove_checked, GltfMutationRejection, GltfSemanticMutation, IndexFamily};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveNode {
    pub index: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for RemoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "node", kind: "remove-node", record: "RemoveNode" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "RemoveNode".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for RemoveNode {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        let frozen = document.clone();
        remove_checked(&mut document.nodes, IndexFamily::Node, self.index, &frozen, "document/nodes")?;
        remap_references(document, IndexFamily::Node, self.index, false);
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `remove-node` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::RemoveNode;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &RemoveNode, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `RemoveNode` semantic inverse.
    
    use super::RemoveNode;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &RemoveNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::InsertNode(InsertNode { index: payload.index, node: node.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

