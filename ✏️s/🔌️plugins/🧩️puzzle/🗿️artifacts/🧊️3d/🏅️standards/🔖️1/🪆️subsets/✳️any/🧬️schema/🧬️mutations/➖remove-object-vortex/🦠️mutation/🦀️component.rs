//! ➖ Puzzle3d mutation — `RemoveObjectVortex`: detaches a rim vortex from an object (captures
//! cascade — any attraction whose `attracting`/`attracted` referenced this vortex is severed too).
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➖ `remove-object-vortex` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-object-vortex")]
pub struct RemoveObjectVortex {
    pub object_id: String,
    pub vortex_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_object_vortex(object_id: String, vortex_id: String) -> Puzzle3dMutation {
    Puzzle3dMutation::RemoveObjectVortex(RemoveObjectVortex { object_id, vortex_id })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for RemoveObjectVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "object-vortex", kind: "remove-object-vortex", record: "RemovedObjectVortex" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove vortex \"{}\" from object \"{}\"", self.vortex_id, self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone(), self.vortex_id.clone()]
    }
}
//#endregion 🔖️Mutation
