//! 🛢️ Energy model mutation — `ChangeShwSystemStorageVolume`: Sets storage volume (m³) on one service hot water system, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛢️ `change-shw-system-storage-volume` payload. Sets storage volume (m³) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-storage-volume")]
pub struct ChangeShwSystemStorageVolume {
    pub id: crate::model::EntityId,
    pub new_storage_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_shw_system_storage_volume(id: crate::model::EntityId, new_storage_volume_m3: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeShwSystemStorageVolume(ChangeShwSystemStorageVolume { id, new_storage_volume_m3 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeShwSystemStorageVolume {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "service-hot-water-system", kind: "change-shw-system-storage-volume", record: "ChangedShwSystemStorageVolume" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Shw System Storage Volume of service hot water system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
