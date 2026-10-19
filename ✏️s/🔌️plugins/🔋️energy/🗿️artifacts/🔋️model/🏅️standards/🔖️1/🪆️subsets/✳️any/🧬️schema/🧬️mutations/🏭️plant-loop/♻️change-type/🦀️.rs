//! ♻️ Energy model mutation — `ChangePlantLoopType`: Swaps which fluid duty the loop carries.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♻️ `change-plant-loop-type` payload. Swaps which fluid duty the loop carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-type")]
pub struct ChangePlantLoopType {
    pub id: crate::model::EntityId,
    pub new_loop_type: crate::model::PlantLoopType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_plant_loop_type<Op: From<ChangePlantLoopType>>(id: crate::model::EntityId, new_loop_type: crate::model::PlantLoopType) -> Op {
    ChangePlantLoopType { id, new_loop_type }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangePlantLoopType
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangePlantLoopType>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "plant-loop", kind: "change-plant-loop-type", record: "ChangedPlantLoopType" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change plant loop {} loop type to {:?}", self.id.0, self.new_loop_type), &format!("Kreislauftyp von Anlagenkreislauf {} auf {:?} ändern", self.id.0, self.new_loop_type))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangePlantLoopType
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangePlantLoopType>,
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
