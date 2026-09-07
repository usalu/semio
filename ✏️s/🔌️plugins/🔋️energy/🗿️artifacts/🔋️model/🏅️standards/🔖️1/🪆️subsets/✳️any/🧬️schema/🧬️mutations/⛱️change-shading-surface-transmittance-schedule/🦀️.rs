//! ⛱️ Energy model mutation — `ChangeShadingSurfaceTransmittanceSchedule`: Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛱️ `change-shading-surface-transmittance-schedule` payload. Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shading-surface-transmittance-schedule")]
pub struct ChangeShadingSurfaceTransmittanceSchedule {
    pub id: crate::model::EntityId,
    pub new_transmittance_schedule_id: Option<crate::model::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_shading_surface_transmittance_schedule(id: crate::model::EntityId, new_transmittance_schedule_id: Option<crate::model::ScheduleId>) -> EnergyModelMutation {
    EnergyModelMutation::ChangeShadingSurfaceTransmittanceSchedule(ChangeShadingSurfaceTransmittanceSchedule { id, new_transmittance_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeShadingSurfaceTransmittanceSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "shading-surface", kind: "change-shading-surface-transmittance-schedule", record: "ChangedShadingSurfaceTransmittanceSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change shading surface {} transmittance schedule", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
