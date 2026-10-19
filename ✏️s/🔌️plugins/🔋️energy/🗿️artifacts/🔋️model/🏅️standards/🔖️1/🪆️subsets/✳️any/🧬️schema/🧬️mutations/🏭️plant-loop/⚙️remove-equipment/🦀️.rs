//! ⚙️ Energy model mutation — `RemovePlantLoopEquipment`: Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚙️ `remove-plant-loop-equipment` payload. Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-plant-loop-equipment")]
pub struct RemovePlantLoopEquipment {
    pub id: crate::model::EntityId,
    pub equipment_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_plant_loop_equipment<Op: From<RemovePlantLoopEquipment>>(id: crate::model::EntityId, equipment_id: crate::model::EntityId) -> Op {
    RemovePlantLoopEquipment { id, equipment_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemovePlantLoopEquipment
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemovePlantLoopEquipment> + From<crate::AddPlantLoopEquipment>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove plant equipment {} from plant loop {}", self.equipment_id.0, self.id.0), &format!("Anlagenkomponente {} aus Anlagenkreislauf {} entfernen", self.equipment_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.equipment_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemovePlantLoopEquipment
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemovePlantLoopEquipment> + From<crate::AddPlantLoopEquipment>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "plant-loop", kind: "remove-plant-loop-equipment", record: "RemovedPlantLoopEquipment" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
