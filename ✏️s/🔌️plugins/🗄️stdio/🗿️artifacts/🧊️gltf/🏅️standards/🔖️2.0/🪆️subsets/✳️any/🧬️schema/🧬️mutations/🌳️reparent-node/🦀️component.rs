//#region 🦠️Mutation
// 🦠️ `reparent-node` GLTF mutation payload.

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{check_index, locate_node_owner, reject, GltfMutationRejection, GltfSemanticMutation};
use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReparentNode {
    pub index: usize,
    pub parent: Option<usize>,
    pub scene: Option<usize>,
    pub position: usize,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for ReparentNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reparent", entity: "node", kind: "reparent-node", record: "ReparentNode" };
    async fn diff(&self, base: &GltfSnapshot) -> <GltfMutation as protocol::Mutation<GltfSnapshot>>::Diff {
        diff::diff(self, base)
    }
    async fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "ReparentNode".into()
    }
    async fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl GltfSemanticMutation for ReparentNode {
    fn apply(&self, snapshot: &mut GltfSnapshot) -> Result<(), GltfMutationRejection> {
        let document = &mut snapshot.document;
        check_index("document/nodes", self.index, document.nodes.len())?;
        if self.parent.is_some() && self.scene.is_some() {
            return Err(reject("gltf.node.owner-exclusive", format!("document/nodes/{}", self.index), "parent and scene cannot both be selected"));
        }
        if let Some(parent) = self.parent {
            check_index("document/nodes", parent, document.nodes.len())?;
            if parent == self.index {
                return Err(reject("gltf.node.self-parent", format!("document/nodes/{}", self.index), "node cannot parent itself"));
            }
        }
        if let Some(scene) = self.scene {
            check_index("document/scenes", scene, document.scenes.len())?;
        }
        locate_node_owner(document, self.index)?;
        for node in &mut document.nodes {
            node.children.retain(|child| *child != self.index);
        }
        for scene in &mut document.scenes {
            scene.nodes.retain(|node| *node != self.index);
        }
        if let Some(parent) = self.parent {
            if self.position > document.nodes[parent].children.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", format!("document/nodes/{parent}/children"), format!("position {}, length {}", self.position, document.nodes[parent].children.len())));
            }
            document.nodes[parent].children.insert(self.position, self.index);
        } else if let Some(scene) = self.scene {
            if self.position > document.scenes[scene].nodes.len() {
                return Err(reject("gltf.mutation.insert-out-of-range", format!("document/scenes/{scene}/nodes"), format!("position {}, length {}", self.position, document.scenes[scene].nodes.len())));
            }
            document.scenes[scene].nodes.insert(self.position, self.index);
        }
        Ok(())
    }
}
//#endregion 🦠️Mutation

//#region 🔺️Diff
mod diff {
    // 🔺️ `reparent-node` validated sparse diff.
    
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfSemanticMutation;
    use super::ReparentNode;
    use crate::artifacts::gltf::schema::diff::GltfDiff;
    use crate::artifacts::gltf::GltfSnapshot;
    
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn diff(payload: &ReparentNode, base: &GltfSnapshot) -> GltfDiff {
        payload.plan(base).unwrap_or_default()
    }
}
//#endregion 🔺️Diff

//#region ↩️Inverse
mod inverse {
    // ↩️ `ReparentNode` semantic inverse.
    
    use super::ReparentNode;
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::*;
    use crate::artifacts::gltf::GltfSnapshot;
    
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn inverse(payload: &ReparentNode, base: &GltfSnapshot) -> Vec<GltfMutation> {
        locate_node_owner(&base.document, payload.index).map(|(parent, scene, position)| vec![GltfMutation::ReparentNode(ReparentNode { index: payload.index, parent, scene, position })]).unwrap_or_default()
    }
}
//#endregion ↩️Inverse

