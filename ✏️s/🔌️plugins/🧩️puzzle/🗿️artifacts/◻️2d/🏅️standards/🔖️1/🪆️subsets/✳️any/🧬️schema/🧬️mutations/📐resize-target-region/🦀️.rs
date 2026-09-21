//! 📐 Puzzle2d mutation — `ResizeTargetRegion`: absolute FINAL-state extent of a region.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 📐 `resize-target-region` payload — absolute FINAL-state extent of a region.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "resize-target-region")]
pub struct ResizeTargetRegion {
    pub id: String,
    pub new_width: f64,
    pub new_height: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn resize_target_region(id: String, new_width: f64, new_height: f64) -> Puzzle2dMutation {
    Puzzle2dMutation::ResizeTargetRegion(ResizeTargetRegion { id, new_width, new_height })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ResizeTargetRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "resize", entity: "target region", kind: "resize-target-region", record: "ResizedTargetRegion" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Resize target region \"{}\"", self.id), &format!("Zielregion \"{}\" skalieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
