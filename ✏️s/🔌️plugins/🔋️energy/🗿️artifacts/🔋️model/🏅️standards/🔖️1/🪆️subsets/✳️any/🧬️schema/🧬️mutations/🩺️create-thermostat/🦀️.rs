//! 🩺️ Energy model mutation — `CreateThermostat`: Creates the setpoint control of one zone. The two setpoint schedules must already be defined by the model's own `ScheduleSet` and the zone must exist, so a thermostat can never be born dangling.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🩺️ `create-thermostat` payload. Creates the setpoint control of one zone. The two setpoint schedules must already be defined by the model's own `ScheduleSet` and the zone must exist, so a thermostat can never be born dangling.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-thermostat")]
pub struct CreateThermostat {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub heating_setpoint_schedule_id: crate::model::ScheduleId,
    pub cooling_setpoint_schedule_id: crate::model::ScheduleId,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_thermostat(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    heating_setpoint_schedule_id: crate::model::ScheduleId,
    cooling_setpoint_schedule_id: crate::model::ScheduleId,
    heating_throttle_range_k: f64,
    cooling_throttle_range_k: f64,
    index: Option<u32>,
) -> EnergyModelMutation {
    EnergyModelMutation::CreateThermostat(CreateThermostat { id, zone_id, heating_setpoint_schedule_id, cooling_setpoint_schedule_id, heating_throttle_range_k, cooling_throttle_range_k, index })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateThermostat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "thermostat", kind: "create-thermostat", record: "CreatedThermostat" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create thermostat {}", self.id.0), &format!("Thermostat {} erstellen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
