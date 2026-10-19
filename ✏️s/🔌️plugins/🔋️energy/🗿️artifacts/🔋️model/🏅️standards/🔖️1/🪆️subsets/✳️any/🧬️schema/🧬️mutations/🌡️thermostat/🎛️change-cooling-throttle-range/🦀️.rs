//! 🎛️ Energy model mutation — `ChangeThermostatCoolingThrottleRange`: Sets the proportional band the cooling setpoint is approached over, in kelvin.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎛️ `change-thermostat-cooling-throttle-range` payload. Sets the proportional band the cooling setpoint is approached over, in kelvin.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-cooling-throttle-range")]
pub struct ChangeThermostatCoolingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_cooling_throttle_range_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_thermostat_cooling_throttle_range<Op: From<ChangeThermostatCoolingThrottleRange>>(id: crate::model::EntityId, new_cooling_throttle_range_k: f64) -> Op {
    ChangeThermostatCoolingThrottleRange { id, new_cooling_throttle_range_k }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeThermostatCoolingThrottleRange
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeThermostatCoolingThrottleRange>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change thermostat {} cooling throttle range to {:?}", self.id.0, self.new_cooling_throttle_range_k), &format!("Kühl-Proportionalbereich von Thermostat {} auf {:?} ändern", self.id.0, self.new_cooling_throttle_range_k))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeThermostatCoolingThrottleRange
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeThermostatCoolingThrottleRange>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-cooling-throttle-range", record: "ChangedThermostatCoolingThrottleRange" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
