//! 📏 Block3d mutation — `ResizeVortex`: a vortex's `radius`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📏 `resize-vortex` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-vortex")]
pub struct ResizeVortex {
    pub id: String,
    pub new_radius: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_vortex(id: String, new_radius: f64) -> Block3dMutation {
    Block3dMutation::ResizeVortex(ResizeVortex { id, new_radius })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for ResizeVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "vortex", kind: "resize-vortex", record: "ResizedVortex" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Resize vortex \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
