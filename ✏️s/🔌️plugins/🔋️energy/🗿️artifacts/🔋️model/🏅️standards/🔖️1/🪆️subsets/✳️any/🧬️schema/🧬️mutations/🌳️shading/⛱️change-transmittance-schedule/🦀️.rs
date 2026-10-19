//! ⛱️ Energy model mutation — `ChangeShadingSurfaceTransmittanceSchedule`: Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛱️ `change-shading-surface-transmittance-schedule` payload. Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shading-surface-transmittance-schedule")]
pub struct ChangeShadingSurfaceTransmittanceSchedule {
    pub id: crate::model::EntityId,
    pub new_transmittance_schedule_id: Option<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shading_surface_transmittance_schedule<Op: From<ChangeShadingSurfaceTransmittanceSchedule>>(id: crate::model::EntityId, new_transmittance_schedule_id: Option<crate::schedule::ScheduleId>) -> Op {
    ChangeShadingSurfaceTransmittanceSchedule { id, new_transmittance_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeShadingSurfaceTransmittanceSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeShadingSurfaceTransmittanceSchedule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change shading surface {} transmittance schedule", self.id.0), &format!("Transmissionszeitplan von Verschattungsfläche {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeShadingSurfaceTransmittanceSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeShadingSurfaceTransmittanceSchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shading-surface", kind: "change-shading-surface-transmittance-schedule", record: "ChangedShadingSurfaceTransmittanceSchedule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
