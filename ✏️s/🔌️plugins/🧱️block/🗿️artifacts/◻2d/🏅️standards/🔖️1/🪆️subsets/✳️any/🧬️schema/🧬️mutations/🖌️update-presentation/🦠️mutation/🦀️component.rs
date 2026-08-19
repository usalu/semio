//! 🖌️ Block2d mutation — `UpdatePresentation`: the whole rim-presentation facet atomically (shape/radius/width/height/color/iconKind are edited together in the shape inspector — see report).
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖌️ `update-presentation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "update-presentation")]
pub struct UpdatePresentation {
    pub new_shape: Option<String>,
    pub new_radius: Option<f64>,
    pub new_width: Option<f64>,
    pub new_height: Option<f64>,
    pub new_color: Option<String>,
    pub new_icon_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn update_presentation(new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>, new_color: Option<String>, new_icon_kind: Option<String>) -> Block2dMutation {
    Block2dMutation::UpdatePresentation(UpdatePresentation { new_shape, new_radius, new_width, new_height, new_color, new_icon_kind })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for UpdatePresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "presentation", kind: "update-presentation", record: "UpdatedPresentation" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        "Update presentation".to_string()
    }
}
//#endregion 🔖️Mutation
