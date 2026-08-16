//#region 🦠️Mutation
// 🦠️ `set-node` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNode {
    pub index: usize,
    pub node: GltfNode,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "node", kind: "set-node", record: "SetNode" };
    fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "SetNode".into()
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for SetNode {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        check_index("document/nodes", self.index, snapshot.document.nodes.len())?;
        snapshot.document.nodes[self.index] = self.node.clone();
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `set-node` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::SetNode;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn diff(payload: &SetNode, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `SetNode` semantic inverse.
    
    use super::SetNode;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    pub fn inverse(payload: &SetNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
        base.document.nodes.get(payload.index).map(|node| vec![GltfMutation::SetNode(SetNode { index: payload.index, node: node.clone() })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

