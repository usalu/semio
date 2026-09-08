//! 📈️ Energy model mutation — `ChangePvSystemTilt`: Sets tilt on one pv system, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📈️ `change-pv-system-tilt` payload. Sets tilt on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-tilt")]
pub struct ChangePvSystemTilt {
    pub id: crate::model::EntityId,
    pub new_tilt_deg: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pv_system_tilt(id: crate::model::EntityId, new_tilt_deg: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangePvSystemTilt(ChangePvSystemTilt { id, new_tilt_deg })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangePvSystemTilt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "pv-system", kind: "change-pv-system-tilt", record: "ChangedPvSystemTilt" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Pv System Tilt of pv system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
