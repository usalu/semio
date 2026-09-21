//! 🌍 Puzzle2d mutation — `CreateTargetRegion`: brings a new id-keyed fill target region into existence.

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::{Puzzle2dSnapshot, Puzzle2dTargetRegion};

//#region 🔖️Mutation
/// 🌍 `create-target-region` payload — full initial payload at an optional FINAL-state `index`
/// (`None` appends). A duplicate `target_region.id` is fatal (an id-keyed entity that already
/// exists cannot be re-created).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-target-region")]
pub struct CreateTargetRegion {
    #[dsl(block)]
    pub target_region: Puzzle2dTargetRegion,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_target_region(target_region: Puzzle2dTargetRegion, index: Option<usize>) -> Puzzle2dMutation {
    Puzzle2dMutation::CreateTargetRegion(CreateTargetRegion { target_region, index })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for CreateTargetRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "target region", kind: "create-target-region", record: "CreatedTargetRegion" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create target region \"{}\"", self.target_region.id), &format!("Zielregion \"{}\" erstellen", self.target_region.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.target_region.id.clone()]
    }
}
//#endregion 🔖️Mutation
