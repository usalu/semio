//! ⏱️ Energy model mutation — `ChangeLightingGainSchedule`: Re-points one lighting gain's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏱️ `change-lighting-gain-schedule` payload. Re-points one lighting gain's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-lighting-gain-schedule")]
pub struct ChangeLightingGainSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_lighting_gain_schedule<Op: From<ChangeLightingGainSchedule>>(id: crate::model::EntityId, new_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeLightingGainSchedule { id, new_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeLightingGainSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeLightingGainSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "lighting-gain", kind: "change-lighting-gain-schedule", record: "ChangedLightingGainSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Lighting Gain Schedule of lighting gain {}", self.id.0), &format!("Zeitplan von Beleuchtungswärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeLightingGainSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeLightingGainSchedule>,
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
