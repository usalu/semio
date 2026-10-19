//! 📆️ Energy model mutation — `ChangeMechanicalVentilationSchedule`: Re-points one mechanical ventilation's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📆️ `change-mechanical-ventilation-schedule` payload. Re-points one mechanical ventilation's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-mechanical-ventilation-schedule")]
pub struct ChangeMechanicalVentilationSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_mechanical_ventilation_schedule<Op: From<ChangeMechanicalVentilationSchedule>>(id: crate::model::EntityId, new_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeMechanicalVentilationSchedule { id, new_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeMechanicalVentilationSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeMechanicalVentilationSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mechanical-ventilation", kind: "change-mechanical-ventilation-schedule", record: "ChangedMechanicalVentilationSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Mechanical Ventilation Schedule of mechanical ventilation {}", self.id.0), &format!("Zeitplan von mechanischer Lüftung {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeMechanicalVentilationSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeMechanicalVentilationSchedule>,
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
