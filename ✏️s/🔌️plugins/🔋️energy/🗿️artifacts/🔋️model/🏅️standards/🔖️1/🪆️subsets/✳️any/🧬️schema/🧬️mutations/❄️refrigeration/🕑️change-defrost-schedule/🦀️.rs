//! 🕑️ Energy model mutation — `ChangeRefrigerationSystemDefrostSchedule`: Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕑️ `change-refrigeration-system-defrost-schedule` payload. Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-defrost-schedule")]
pub struct ChangeRefrigerationSystemDefrostSchedule {
    pub id: crate::model::EntityId,
    pub new_defrost_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_refrigeration_system_defrost_schedule<Op: From<ChangeRefrigerationSystemDefrostSchedule>>(id: crate::model::EntityId, new_defrost_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeRefrigerationSystemDefrostSchedule { id, new_defrost_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeRefrigerationSystemDefrostSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeRefrigerationSystemDefrostSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "refrigeration-system", kind: "change-refrigeration-system-defrost-schedule", record: "ChangedRefrigerationSystemDefrostSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Refrigeration System Defrost Schedule of refrigeration system {}", self.id.0), &format!("Abtauzeitplan von Kälteanlage {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
