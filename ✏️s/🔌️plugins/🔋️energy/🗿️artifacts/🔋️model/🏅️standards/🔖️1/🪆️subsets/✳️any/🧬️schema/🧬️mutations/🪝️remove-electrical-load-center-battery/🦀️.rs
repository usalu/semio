//! 🪝️ Energy model mutation — `RemoveElectricalLoadCenterBattery`: Detaches one battery from a load centre; the battery itself survives, unattached.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪝️ `remove-electrical-load-center-battery` payload. Detaches one battery from a load centre; the battery itself survives, unattached.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-electrical-load-center-battery")]
pub struct RemoveElectricalLoadCenterBattery {
    pub id: crate::model::EntityId,
    pub battery_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_electrical_load_center_battery(id: crate::model::EntityId, battery_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::RemoveElectricalLoadCenterBattery(RemoveElectricalLoadCenterBattery { id, battery_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveElectricalLoadCenterBattery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "electrical-load-center", kind: "remove-electrical-load-center-battery", record: "RemovedElectricalLoadCenterBattery" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove battery {} from electrical load center {}", self.battery_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
