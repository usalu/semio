//! ⌛️ Energy model mutation — `ChangeEquipmentGainSchedule`: Re-points one equipment gain's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⌛️ `change-equipment-gain-schedule` payload. Re-points one equipment gain's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-schedule")]
pub struct ChangeEquipmentGainSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_equipment_gain_schedule(id: crate::model::EntityId, new_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeEquipmentGainSchedule(ChangeEquipmentGainSchedule { id, new_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeEquipmentGainSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-schedule", record: "ChangedEquipmentGainSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Equipment Gain Schedule of equipment gain {}", self.id.0), &format!("Zeitplan von Gerätewärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
