//! 🩺️ Energy model mutation — `CreateThermostat`: Creates the setpoint control of one zone. The two setpoint schedules must already be defined by the model's own `ScheduleSet` and the zone must exist, so a thermostat can never be born dangling.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
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
    pub heating_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub cooling_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_thermostat<Op: From<CreateThermostat>>(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    heating_setpoint_schedule_id: crate::schedule::ScheduleId,
    cooling_setpoint_schedule_id: crate::schedule::ScheduleId,
    heating_throttle_range_k: f64,
    cooling_throttle_range_k: f64,
    index: Option<u32>,
) -> Op {
    CreateThermostat { id, zone_id, heating_setpoint_schedule_id, cooling_setpoint_schedule_id, heating_throttle_range_k, cooling_throttle_range_k, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateThermostat
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateThermostat> + From<crate::DeleteThermostat>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create thermostat {}", self.id.0), &format!("Thermostat {} erstellen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateThermostat
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateThermostat> + From<crate::DeleteThermostat>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "thermostat", kind: "create-thermostat", record: "CreatedThermostat" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
