//! 🕜️ Energy model mutation — `CreateConstantSchedule`: Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕜️ `create-constant-schedule` payload. Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-constant-schedule")]
pub struct CreateConstantSchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub value: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_constant_schedule<Op: From<CreateConstantSchedule>>(index: u32, id: crate::schedule::ScheduleId, value: f64) -> Op {
    CreateConstantSchedule { index, id, value }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateConstantSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateConstantSchedule> + From<crate::DeleteConstantSchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "constant-schedule", kind: "create-constant-schedule", record: "CreatedConstantSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Constant Schedule {} at index {}", self.id.0, self.index), &format!("Konstanten Zeitplan {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateConstantSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateConstantSchedule> + From<crate::DeleteConstantSchedule>,
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
