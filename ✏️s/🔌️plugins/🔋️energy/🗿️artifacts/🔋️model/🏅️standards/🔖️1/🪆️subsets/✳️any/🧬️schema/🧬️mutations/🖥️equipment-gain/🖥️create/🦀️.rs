//! 🖥️ Energy model mutation — `CreateEquipmentGain`: Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖥️ `create-equipment-gain` payload. Adds one electric equipment gain to a zone — ANSI/ASHRAE 140 §5.2's 200 W internal load is exactly this entity at 60 % radiative.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-equipment-gain")]
pub struct CreateEquipmentGain {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub schedule_id: crate::schedule::ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub latent_fraction: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_equipment_gain<Op: From<CreateEquipmentGain>>(index: u32, id: crate::model::EntityId, zone_id: crate::model::EntityId, schedule_id: crate::schedule::ScheduleId, watts_per_area: f64, radiant_fraction: f64, latent_fraction: f64) -> Op {
    CreateEquipmentGain { index, id, zone_id, schedule_id, watts_per_area, radiant_fraction, latent_fraction }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateEquipmentGain
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateEquipmentGain> + From<crate::DeleteEquipmentGain>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "equipment-gain", kind: "create-equipment-gain", record: "CreatedEquipmentGain" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Equipment Gain {} at index {}", self.id.0, self.index), &format!("Gerätewärmegewinn {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateEquipmentGain
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateEquipmentGain> + From<crate::DeleteEquipmentGain>,
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
