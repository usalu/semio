//! ❄️ Energy model mutation — `CreateRefrigerationSystem`: Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ❄️ `create-refrigeration-system` payload. Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-refrigeration-system")]
pub struct CreateRefrigerationSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub case_count: u32,
    pub design_load_w: f64,
    pub defrost_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_refrigeration_system<Op: From<CreateRefrigerationSystem>>(index: u32, id: crate::model::EntityId, case_count: u32, design_load_w: f64, defrost_schedule_id: crate::schedule::ScheduleId) -> Op {
    CreateRefrigerationSystem { index, id, case_count, design_load_w, defrost_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateRefrigerationSystem
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateRefrigerationSystem> + From<crate::DeleteRefrigerationSystem>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "refrigeration-system", kind: "create-refrigeration-system", record: "CreatedRefrigerationSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Refrigeration System {} at index {}", self.id.0, self.index), &format!("Kälteanlage {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
