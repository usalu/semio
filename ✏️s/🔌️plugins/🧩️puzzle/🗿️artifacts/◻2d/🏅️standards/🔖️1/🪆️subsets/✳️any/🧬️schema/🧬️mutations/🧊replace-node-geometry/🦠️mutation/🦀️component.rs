//! 🧊 Puzzle2d mutation — `ReplaceNodeGeometry`: whole-value swap of a node's shape/extent —
//! `shape`+`radius`+`width`+`height` together are the node's one geometric representation, the
//! same grouping cad's `replace-object-geometry` uses for `mesh_url`+`extent`+`solid_handle`.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧊 `replace-node-geometry` payload — new shape/extent, whichever fields the shape uses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-node-geometry")]
pub struct ReplaceNodeGeometry {
    pub id: String,
    pub new_shape: Option<String>,
    pub new_radius: Option<f64>,
    pub new_width: Option<f64>,
    pub new_height: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_node_geometry(id: String, new_shape: Option<String>, new_radius: Option<f64>, new_width: Option<f64>, new_height: Option<f64>) -> Puzzle2dMutation {
    Puzzle2dMutation::ReplaceNodeGeometry(ReplaceNodeGeometry { id, new_shape, new_radius, new_width, new_height })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ReplaceNodeGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "node", kind: "replace-node-geometry", record: "ReplacedNodeGeometry" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace node \"{}\" geometry", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
