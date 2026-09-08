//! 🤝️ Energy model mutation — `ConnectSurfaces`: Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🤝️ `connect-surfaces` payload. Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-surfaces")]
pub struct ConnectSurfaces {
    pub surface_a_id: crate::model::EntityId,
    pub surface_b_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_surfaces(surface_a_id: crate::model::EntityId, surface_b_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ConnectSurfaces(ConnectSurfaces { surface_a_id, surface_b_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ConnectSurfaces {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "surfaces", kind: "connect-surfaces", record: "ConnectedSurfaces" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Connect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
