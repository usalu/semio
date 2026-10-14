//! 🧺️ Energy model mutation — `DeleteRoomAirModelAssignment`: Drops one zone's room air model override, so the zone falls back to the engine's own well-mixed default.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧺️ `delete-room-air-model-assignment` payload. Drops one zone's room air model override, so the zone falls back to the engine's own well-mixed default.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-room-air-model-assignment")]
pub struct DeleteRoomAirModelAssignment {
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_room_air_model_assignment(zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteRoomAirModelAssignment(DeleteRoomAirModelAssignment { zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteRoomAirModelAssignment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "room-air-model-assignment", kind: "delete-room-air-model-assignment", record: "DeletedRoomAirModelAssignment" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete room air model assignment {}", self.zone_id.0), &format!("Raumluftmodellzuordnung {} löschen", self.zone_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.zone_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
