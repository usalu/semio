//! 🕝️ Energy model mutation — `ChangeConstantScheduleValue`: Sets the one value a constant schedule returns at every timestep.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕝️ `change-constant-schedule-value` payload. Sets the one value a constant schedule returns at every timestep.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-constant-schedule-value")]
pub struct ChangeConstantScheduleValue {
    pub id: crate::schedule::ScheduleId,
    pub new_value: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_constant_schedule_value<Op: From<ChangeConstantScheduleValue>>(id: crate::schedule::ScheduleId, new_value: f64) -> Op {
    ChangeConstantScheduleValue { id, new_value }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeConstantScheduleValue
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeConstantScheduleValue>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "constant-schedule", kind: "change-constant-schedule-value", record: "ChangedConstantScheduleValue" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Constant Schedule Value of constant schedule {}", self.id.0), &format!("Wert von konstantem Zeitplan {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeConstantScheduleValue
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeConstantScheduleValue>,
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
