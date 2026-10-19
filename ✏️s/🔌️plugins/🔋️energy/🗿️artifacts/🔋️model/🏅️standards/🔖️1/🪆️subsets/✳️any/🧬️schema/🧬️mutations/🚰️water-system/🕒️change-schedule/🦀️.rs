//! 🕒️ Energy model mutation — `ChangeWaterSystemSchedule`: Re-points one water system's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕒️ `change-water-system-schedule` payload. Re-points one water system's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-schedule")]
pub struct ChangeWaterSystemSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_water_system_schedule<Op: From<ChangeWaterSystemSchedule>>(id: crate::model::EntityId, new_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeWaterSystemSchedule { id, new_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeWaterSystemSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeWaterSystemSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "water-system", kind: "change-water-system-schedule", record: "ChangedWaterSystemSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Water System Schedule of water system {}", self.id.0), &format!("Zeitplan von Wasseranlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeWaterSystemSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeWaterSystemSchedule>,
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
