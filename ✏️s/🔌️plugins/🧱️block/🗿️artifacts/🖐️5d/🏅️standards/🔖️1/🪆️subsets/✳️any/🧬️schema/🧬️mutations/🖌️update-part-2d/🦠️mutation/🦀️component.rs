//! 🖌️ Block5d mutation — `UpdatePart2d`: the whole 2D-projection presentation facet atomically — mirrors block2d's `update-presentation` (same shape, same no-identity-field reasoning).
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖌️ `update-part-2d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-part-2d")]
pub struct UpdatePart2d {
    pub new_shape: Option<String>,
    pub new_radius: Option<f64>,
    pub new_width: Option<f64>,
    pub new_height: Option<f64>,
    pub new_color: Option<String>,
    pub new_icon_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn update_part_2d(new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>, new_color: Option<String>, new_icon_kind: Option<String>) -> Block5dMutation {
    Block5dMutation::UpdatePart2d(UpdatePart2d { new_shape, new_radius, new_width, new_height, new_color, new_icon_kind })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for UpdatePart2d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "part-2d", kind: "update-part2d", record: "UpdatedPart2d" };

    fn diff(&self, base: &Block5dSnapshot) -> Block5dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update part 2D presentation".to_string()
    }
}
//#endregion 🔖️Mutation
