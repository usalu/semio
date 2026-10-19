//! 🏃️ Energy model mutation — `ChangePeopleGainActivitySchedule`: Re-points one people gain's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏃️ `change-people-gain-activity-schedule` payload. Re-points one people gain's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-activity-schedule")]
pub struct ChangePeopleGainActivitySchedule {
    pub id: crate::model::EntityId,
    pub new_activity_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_activity_schedule<Op: From<ChangePeopleGainActivitySchedule>>(id: crate::model::EntityId, new_activity_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangePeopleGainActivitySchedule { id, new_activity_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainActivitySchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePeopleGainActivitySchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-activity-schedule", record: "ChangedPeopleGainActivitySchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change People Gain Activity Schedule of people gain {}", self.id.0), &format!("Aktivitätszeitplan von Personenwärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainActivitySchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePeopleGainActivitySchedule>,
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
