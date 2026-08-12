//! 🔌 Puzzle3d mutation — `ReplaceObjectVortex`: whole-value swap of one vortex's presentation
//! fields (kind/label/position/direction/radius/hidden/locked together, one property-panel gesture).
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dSnapshot, Puzzle3dVortex};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔌 `replace-object-vortex` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-object-vortex")]
pub struct ReplaceObjectVortex {
    pub object_id: String,
    pub vortex_id: String,
    #[dsl(block)]
    pub new_vortex: Puzzle3dVortex,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_object_vortex(object_id: String, vortex_id: String, new_vortex: Puzzle3dVortex) -> Puzzle3dMutation {
    Puzzle3dMutation::ReplaceObjectVortex(ReplaceObjectVortex { object_id, vortex_id, new_vortex })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ReplaceObjectVortex {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "object-vortex", kind: "replace-object-vortex", record: "ReplacedObjectVortex" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace vortex \"{}\" on object \"{}\"", self.vortex_id, self.object_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone(), self.vortex_id.clone()]
    }
}
//#endregion 🔖️Mutation
