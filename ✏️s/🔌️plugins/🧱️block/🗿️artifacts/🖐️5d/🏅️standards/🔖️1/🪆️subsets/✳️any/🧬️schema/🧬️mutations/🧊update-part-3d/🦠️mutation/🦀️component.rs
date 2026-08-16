//! 🧊 Block5d mutation — `UpdatePart3d`: the whole 2-field 3D-projection pose facet atomically (orientation quaternion + scale vector, always edited together in a 3D pose gizmo).
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧊 `update-part-3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-part-3d")]
pub struct UpdatePart3d {
    pub new_orientation: Option<[f64; 4]>,
    pub new_scale: Option<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_part_3d(new_orientation: Option<[f64; 4]>, new_scale: Option<[f64; 3]>) -> Block5dMutation {
    Block5dMutation::UpdatePart3d(UpdatePart3d { new_orientation, new_scale })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for UpdatePart3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "part-3d", kind: "update-part3d", record: "UpdatedPart3d" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update part 3D pose".to_string()
    }
}
//#endregion 🔖️Mutation
