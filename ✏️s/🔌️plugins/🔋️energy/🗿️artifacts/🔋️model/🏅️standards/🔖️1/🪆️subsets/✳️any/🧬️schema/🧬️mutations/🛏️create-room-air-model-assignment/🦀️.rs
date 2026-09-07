//! 🛏️ Energy model mutation — `CreateRoomAirModelAssignment`: Overrides the room air model of one zone. This collection carries no id of its own — `zone_id` IS the key (vocabulary §5.5), so it is a keyed map and a second override for one zone is a duplicate.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛏️ `create-room-air-model-assignment` payload. Overrides the room air model of one zone. This collection carries no id of its own — `zone_id` IS the key (vocabulary §5.5), so it is a keyed map and a second override for one zone is a duplicate.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-room-air-model-assignment")]
pub struct CreateRoomAirModelAssignment {
    pub zone_id: crate::model::EntityId,
    pub model: crate::model::RoomAirModelType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_room_air_model_assignment(zone_id: crate::model::EntityId, model: crate::model::RoomAirModelType) -> EnergyModelMutation {
    EnergyModelMutation::CreateRoomAirModelAssignment(CreateRoomAirModelAssignment { zone_id, model })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateRoomAirModelAssignment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "room-air-model-assignment", kind: "create-room-air-model-assignment", record: "CreatedRoomAirModelAssignment" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create room air model assignment {}", self.zone_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.zone_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
