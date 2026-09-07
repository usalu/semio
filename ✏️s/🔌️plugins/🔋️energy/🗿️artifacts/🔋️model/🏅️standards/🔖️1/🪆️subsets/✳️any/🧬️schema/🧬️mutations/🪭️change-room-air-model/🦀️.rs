//! 🪭️ Energy model mutation — `ChangeRoomAirModel`: Swaps the room air model one zone's override names.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪭️ `change-room-air-model` payload. Swaps the room air model one zone's override names.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-room-air-model")]
pub struct ChangeRoomAirModel {
    pub zone_id: crate::model::EntityId,
    pub new_model: crate::model::RoomAirModelType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_room_air_model(zone_id: crate::model::EntityId, new_model: crate::model::RoomAirModelType) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRoomAirModel(ChangeRoomAirModel { zone_id, new_model })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRoomAirModel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "room-air-model-assignment", kind: "change-room-air-model", record: "ChangedRoomAirModel" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change room air model assignment {} room air model to {:?}", self.zone_id.0, self.new_model)
    }

    fn target(&self) -> Vec<String> {
        vec![self.zone_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
