//! 🔧️ Energy model mutation — `ChangeZoneEquipmentType`: Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔧️ `change-zone-equipment-type` payload. Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-type")]
pub struct ChangeZoneEquipmentType {
    pub id: crate::model::EntityId,
    pub new_equipment_type: crate::model::ZoneEquipmentType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_equipment_type<Op: From<ChangeZoneEquipmentType>>(id: crate::model::EntityId, new_equipment_type: crate::model::ZoneEquipmentType) -> Op {
    ChangeZoneEquipmentType { id, new_equipment_type }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeZoneEquipmentType
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeZoneEquipmentType>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-equipment", kind: "change-zone-equipment-type", record: "ChangedZoneEquipmentType" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change zone equipment {} equipment type to {:?}", self.id.0, self.new_equipment_type), &format!("Gerätetyp von Zonengerät {} auf {:?} ändern", self.id.0, self.new_equipment_type))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeZoneEquipmentType
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeZoneEquipmentType>,
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
