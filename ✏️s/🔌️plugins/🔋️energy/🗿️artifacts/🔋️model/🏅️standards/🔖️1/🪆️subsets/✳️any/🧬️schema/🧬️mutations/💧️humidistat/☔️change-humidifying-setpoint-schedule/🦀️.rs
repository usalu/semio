//! ☔️ Energy model mutation — `ChangeHumidistatHumidifyingSetpointSchedule`: Repoints the humidifying setpoint at another schedule the model defines.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
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
    pub new_humidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_humidistat_humidifying_setpoint_schedule<Op: From<ChangeHumidistatHumidifyingSetpointSchedule>>(id: crate::model::EntityId, new_humidifying_setpoint_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeHumidistatHumidifyingSetpointSchedule { id, new_humidifying_setpoint_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeHumidistatHumidifyingSetpointSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeHumidistatHumidifyingSetpointSchedule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change humidistat {} humidifying setpoint schedule to {}", self.id.0, self.new_humidifying_setpoint_schedule_id.0), &format!("Befeuchtungs-Sollwertzeitplan von Feuchteregler {} auf {} ändern", self.id.0, self.new_humidifying_setpoint_schedule_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeHumidistatHumidifyingSetpointSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeHumidistatHumidifyingSetpointSchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-humidifying-setpoint-schedule", record: "ChangedHumidistatHumidifyingSetpointSchedule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
