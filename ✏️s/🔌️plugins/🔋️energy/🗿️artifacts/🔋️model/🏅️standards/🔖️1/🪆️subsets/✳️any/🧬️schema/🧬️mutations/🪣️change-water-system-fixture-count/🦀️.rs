//! 🪣️ Energy model mutation — `ChangeWaterSystemFixtureCount`: Sets how many fixtures one water system carries.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪣️ `change-water-system-fixture-count` payload. Sets how many fixtures one water system carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-fixture-count")]
pub struct ChangeWaterSystemFixtureCount {
    pub id: crate::model::EntityId,
    pub new_fixture_count: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_water_system_fixture_count(id: crate::model::EntityId, new_fixture_count: u32) -> EnergyModelMutation {
    EnergyModelMutation::ChangeWaterSystemFixtureCount(ChangeWaterSystemFixtureCount { id, new_fixture_count })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeWaterSystemFixtureCount {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "water-system", kind: "change-water-system-fixture-count", record: "ChangedWaterSystemFixtureCount" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Water System Fixture Count of water system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
