//! 💔️ Energy model mutation — `DisconnectSurfaces`: Removes the adjacency relationship between two surfaces, addressed by the unordered pair. Refused when the two are not adjacent, so an undo chain can never invent a disconnect that had no partner.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 💔️ `disconnect-surfaces` payload. Removes the adjacency relationship between two surfaces, addressed by the unordered pair. Refused when the two are not adjacent, so an undo chain can never invent a disconnect that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-surfaces")]
pub struct DisconnectSurfaces {
    pub surface_a_id: crate::model::EntityId,
    pub surface_b_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_surfaces(surface_a_id: crate::model::EntityId, surface_b_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DisconnectSurfaces(DisconnectSurfaces { surface_a_id, surface_b_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DisconnectSurfaces {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "surfaces", kind: "disconnect-surfaces", record: "DisconnectedSurfaces" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Disconnect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
