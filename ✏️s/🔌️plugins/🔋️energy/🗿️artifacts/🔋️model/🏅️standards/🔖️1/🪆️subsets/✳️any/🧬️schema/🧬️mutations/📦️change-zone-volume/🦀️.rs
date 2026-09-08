//! 📦️ Energy model mutation — `ChangeZoneVolume`: Sets one zone's air volume in cubic metres — the capacitance the zone air heat balance integrates over.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📦️ `change-zone-volume` payload. Sets one zone's air volume in cubic metres — the capacitance the zone air heat balance integrates over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-volume")]
pub struct ChangeZoneVolume {
    pub id: crate::model::EntityId,
    pub new_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_zone_volume(id: crate::model::EntityId, new_volume_m3: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeZoneVolume(ChangeZoneVolume { id, new_volume_m3 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeZoneVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone", kind: "change-zone-volume", record: "ChangedZoneVolume" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change zone {} volume to {} m³", self.id.0, self.new_volume_m3)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
