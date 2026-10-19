//! ☕️ Energy model mutation — `ChangePlantLoopSupplyTemperature`: Sets the temperature the loop supplies at, in °C.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☕️ `change-plant-loop-supply-temperature` payload. Sets the temperature the loop supplies at, in °C.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-supply-temperature")]
pub struct ChangePlantLoopSupplyTemperature {
    pub id: crate::model::EntityId,
    pub new_supply_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_plant_loop_supply_temperature<Op: From<ChangePlantLoopSupplyTemperature>>(id: crate::model::EntityId, new_supply_temperature_c: f64) -> Op {
    ChangePlantLoopSupplyTemperature { id, new_supply_temperature_c }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePlantLoopSupplyTemperature
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePlantLoopSupplyTemperature>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change plant loop {} supply temperature to {:?}", self.id.0, self.new_supply_temperature_c), &format!("Vorlauftemperatur von Anlagenkreislauf {} auf {:?} ändern", self.id.0, self.new_supply_temperature_c))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePlantLoopSupplyTemperature
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePlantLoopSupplyTemperature>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "plant-loop", kind: "change-plant-loop-supply-temperature", record: "ChangedPlantLoopSupplyTemperature" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
