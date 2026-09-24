//! 🔏 Puzzle2d mutation — `ChangeTargetRegionLocked`: changes a region's locked flag.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Mutation
/// 🔏 `change-target-region-locked` payload — changes a region's locked flag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "change-target-region-locked")]
pub struct ChangeTargetRegionLocked {
    pub id: String,
    pub new_locked: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_target_region_locked(id: String, new_locked: bool) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeTargetRegionLocked(ChangeTargetRegionLocked { id, new_locked })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeTargetRegionLocked {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "target region", kind: "change-target-region-locked", record: "ChangedTargetRegionLocked" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change target region locked \"{}\"", self.id), &format!("Sperre von Zielregion \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
