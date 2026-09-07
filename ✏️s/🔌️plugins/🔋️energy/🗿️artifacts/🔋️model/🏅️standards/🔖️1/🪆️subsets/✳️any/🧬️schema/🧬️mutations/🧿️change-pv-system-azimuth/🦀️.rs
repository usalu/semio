//! 🧿️ Energy model mutation — `ChangePvSystemAzimuth`: Sets azimuth on one pv system, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧿️ `change-pv-system-azimuth` payload. Sets azimuth on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-azimuth")]
pub struct ChangePvSystemAzimuth {
    pub id: crate::model::EntityId,
    pub new_azimuth_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pv_system_azimuth(id: crate::model::EntityId, new_azimuth_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePvSystemAzimuth(ChangePvSystemAzimuth { id, new_azimuth_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePvSystemAzimuth {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-azimuth", record: "ChangedPvSystemAzimuth" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Pv System Azimuth of pv system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
