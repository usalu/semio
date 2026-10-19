//! 🌂️ Energy model mutation — `CreateHumidistat`: Creates the humidity control of one zone, against two setpoint schedules the model already defines.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌂️ `create-humidistat` payload. Creates the humidity control of one zone, against two setpoint schedules the model already defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-humidistat")]
pub struct CreateHumidistat {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub humidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub dehumidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    pub humidifying_throttle_range: f64,
    pub dehumidifying_throttle_range: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_humidistat<Op: From<CreateHumidistat>>(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    humidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    dehumidifying_setpoint_schedule_id: crate::schedule::ScheduleId,
    humidifying_throttle_range: f64,
    dehumidifying_throttle_range: f64,
    index: Option<u32>,
) -> Op {
    CreateHumidistat { id, zone_id, humidifying_setpoint_schedule_id, dehumidifying_setpoint_schedule_id, humidifying_throttle_range, dehumidifying_throttle_range, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateHumidistat
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateHumidistat> + From<crate::DeleteHumidistat>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create humidistat {}", self.id.0), &format!("Feuchteregler {} erstellen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateHumidistat
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateHumidistat> + From<crate::DeleteHumidistat>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "humidistat", kind: "create-humidistat", record: "CreatedHumidistat" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
