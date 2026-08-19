//! ✂️ Puzzle3d mutation — `DisconnectVortices`: removes an attraction between two vortices.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✂️ `disconnect-vortices` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-vortices")]
pub struct DisconnectVortices {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn disconnect_vortices(id: String) -> Puzzle3dMutation {
    Puzzle3dMutation::DisconnectVortices(DisconnectVortices { id })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for DisconnectVortices {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "vortices", kind: "disconnect-vortices", record: "DisconnectedVortices" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Disconnect \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
