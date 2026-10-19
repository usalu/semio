//! ⏰️ Energy model mutation — `ChangePeopleGainSchedule`: Re-points one people gain's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏰️ `change-people-gain-schedule` payload. Re-points one people gain's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-people-gain-schedule")]
pub struct ChangePeopleGainSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_people_gain_schedule<Op: From<ChangePeopleGainSchedule>>(id: crate::model::EntityId, new_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangePeopleGainSchedule { id, new_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePeopleGainSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "people-gain", kind: "change-people-gain-schedule", record: "ChangedPeopleGainSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change People Gain Schedule of people gain {}", self.id.0), &format!("Zeitplan von Personenwärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePeopleGainSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePeopleGainSchedule>,
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
