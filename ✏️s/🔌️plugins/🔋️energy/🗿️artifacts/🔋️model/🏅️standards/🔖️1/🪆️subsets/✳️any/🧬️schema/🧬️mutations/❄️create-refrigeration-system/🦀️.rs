//! ❄️ Energy model mutation — `CreateRefrigerationSystem`: Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ❄️ `create-refrigeration-system` payload. Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-refrigeration-system")]
pub struct CreateRefrigerationSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub case_count: u32,
    pub design_load_w: f64,
    pub defrost_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_refrigeration_system(index: u32, id: crate::model::EntityId, case_count: u32, design_load_w: f64, defrost_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::CreateRefrigerationSystem(CreateRefrigerationSystem { index, id, case_count, design_load_w, defrost_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateRefrigerationSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "refrigeration-system", kind: "create-refrigeration-system", record: "CreatedRefrigerationSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Refrigeration System {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
