//! 🥫️ Energy model mutation — `ChangeBatteryCapacity`: Sets storage capacity (kWh) on one battery, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥫️ `change-battery-capacity` payload. Sets storage capacity (kWh) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-capacity")]
pub struct ChangeBatteryCapacity {
    pub id: crate::model::EntityId,
    pub new_capacity_kwh: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_battery_capacity<Op: From<ChangeBatteryCapacity>>(id: crate::model::EntityId, new_capacity_kwh: f64) -> Op {
    ChangeBatteryCapacity { id, new_capacity_kwh }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeBatteryCapacity
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeBatteryCapacity>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-capacity", record: "ChangedBatteryCapacity" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Battery Capacity of battery {}", self.id.0), &format!("Kapazität von Batterie {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeBatteryCapacity
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeBatteryCapacity>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
