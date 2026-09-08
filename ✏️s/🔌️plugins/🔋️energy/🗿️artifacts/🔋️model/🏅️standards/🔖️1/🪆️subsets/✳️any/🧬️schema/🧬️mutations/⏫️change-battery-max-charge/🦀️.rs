//! ⏫️ Energy model mutation — `ChangeBatteryMaxCharge`: Sets maximum charge power (W) on one battery, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏫️ `change-battery-max-charge` payload. Sets maximum charge power (W) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-max-charge")]
pub struct ChangeBatteryMaxCharge {
    pub id: crate::model::EntityId,
    pub new_max_charge_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_battery_max_charge(id: crate::model::EntityId, new_max_charge_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeBatteryMaxCharge(ChangeBatteryMaxCharge { id, new_max_charge_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeBatteryMaxCharge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-max-charge", record: "ChangedBatteryMaxCharge" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Battery Max Charge of battery {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
