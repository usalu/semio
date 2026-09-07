//! ⏬️ Energy model mutation — `ChangeBatteryMaxDischarge`: Sets maximum discharge power (W) on one battery, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏬️ `change-battery-max-discharge` payload. Sets maximum discharge power (W) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-max-discharge")]
pub struct ChangeBatteryMaxDischarge {
    pub id: crate::model::EntityId,
    pub new_max_discharge_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_battery_max_discharge(id: crate::model::EntityId, new_max_discharge_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeBatteryMaxDischarge(ChangeBatteryMaxDischarge { id, new_max_discharge_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeBatteryMaxDischarge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-max-discharge", record: "ChangedBatteryMaxDischarge" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Battery Max Discharge of battery {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
