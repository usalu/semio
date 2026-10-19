//! 🪭️ Energy model mutation — `ChangeRoomAirModel`: Swaps the room air model one zone's override names.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪭️ `change-room-air-model` payload. Swaps the room air model one zone's override names.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-room-air-model")]
pub struct ChangeRoomAirModel {
    pub zone_id: crate::model::EntityId,
    pub new_model: crate::model::RoomAirModelType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_room_air_model<Op: From<ChangeRoomAirModel>>(zone_id: crate::model::EntityId, new_model: crate::model::RoomAirModelType) -> Op {
    ChangeRoomAirModel { zone_id, new_model }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeRoomAirModel
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeRoomAirModel>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change room air model assignment {} room air model to {:?}", self.zone_id.0, self.new_model), &format!("Raumluftmodell von Raumluftmodellzuordnung {} auf {:?} ändern", self.zone_id.0, self.new_model))
    }

    fn target(&self) -> Vec<String> {
        vec![self.zone_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeRoomAirModel
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeRoomAirModel>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "room-air-model-assignment", kind: "change-room-air-model", record: "ChangedRoomAirModel" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
