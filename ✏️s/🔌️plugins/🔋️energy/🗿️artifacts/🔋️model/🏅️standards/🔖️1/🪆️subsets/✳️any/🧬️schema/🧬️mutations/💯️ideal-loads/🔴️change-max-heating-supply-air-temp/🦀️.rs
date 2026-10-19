//! 🔴️ Energy model mutation — `ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp`: Sets the warmest air the ideal-loads system may deliver, in °C.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔴️ `change-ideal-loads-system-max-heating-supply-air-temp` payload. Sets the warmest air the ideal-loads system may deliver, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-heating-supply-air-temp")]
pub struct ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp {
    pub id: crate::model::EntityId,
    pub new_max_heating_supply_air_temp_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_max_heating_supply_air_temp<Op: From<ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp>>(id: crate::model::EntityId, new_max_heating_supply_air_temp_c: f64) -> Op {
    ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp { id, new_max_heating_supply_air_temp_c }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-max-heating-supply-air-temp", record: "ChangedIdealLoadsSystemMaxHeatingSupplyAirTemp" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change ideal loads system {} maximum heating supply air temperature to {:?}", self.id.0, self.new_max_heating_supply_air_temp_c), &format!("Maximale Heizzulufttemperatur von Ideallastsystem {} auf {:?} ändern", self.id.0, self.new_max_heating_supply_air_temp_c))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp>,
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
