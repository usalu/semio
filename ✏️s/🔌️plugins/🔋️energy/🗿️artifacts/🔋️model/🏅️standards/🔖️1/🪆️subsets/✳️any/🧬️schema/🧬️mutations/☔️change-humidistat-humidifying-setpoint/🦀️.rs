//! ☔️ Energy model mutation — `ChangeHumidistatHumidifyingSetpointSchedule`: Repoints the humidifying setpoint at another schedule the model defines.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☔️ `change-humidistat-humidifying-setpoint-schedule` payload. Repoints the humidifying setpoint at another schedule the model defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-humidifying-setpoint-schedule")]
pub struct ChangeHumidistatHumidifyingSetpointSchedule {
    pub id: crate::model::EntityId,
    pub new_humidifying_setpoint_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_humidistat_humidifying_setpoint_schedule(id: crate::model::EntityId, new_humidifying_setpoint_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeHumidistatHumidifyingSetpointSchedule(ChangeHumidistatHumidifyingSetpointSchedule { id, new_humidifying_setpoint_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeHumidistatHumidifyingSetpointSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-humidifying-setpoint-schedule", record: "ChangedHumidistatHumidifyingSetpointSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change humidistat {} humidifying setpoint schedule to {}", self.id.0, self.new_humidifying_setpoint_schedule_id.0), &format!("Befeuchtungs-Sollwertzeitplan von Feuchteregler {} auf {} ändern", self.id.0, self.new_humidifying_setpoint_schedule_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
