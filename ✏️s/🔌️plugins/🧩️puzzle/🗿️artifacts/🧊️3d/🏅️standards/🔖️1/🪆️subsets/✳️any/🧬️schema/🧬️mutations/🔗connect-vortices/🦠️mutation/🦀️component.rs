//! 🔗 Puzzle3d mutation — `ConnectVortices`: creates an attraction between two full vortex ids
//! (`object_id:vortex_id`), full initial connection-parameterization payload included.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-vortices` payload — attraction `id`, both endpoint full vortex ids, and the full
/// initial connection-parameter payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "connect-vortices")]
pub struct ConnectVortices {
    pub id: String,
    pub attracting: String,
    pub attracted: String,
    pub gap: f64,
    pub shift: f64,
    pub rise: f64,
    pub rotation: f64,
    pub turn: f64,
    pub tilt: f64,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
#[allow(clippy::too_many_arguments)]
pub fn connect_vortices(id: String, attracting: String, attracted: String, gap: f64, shift: f64, rise: f64, rotation: f64, turn: f64, tilt: f64, x: f64, y: f64) -> Puzzle3dMutation {
    Puzzle3dMutation::ConnectVortices(ConnectVortices { id, attracting, attracted, gap, shift, rise, rotation, turn, tilt, x, y })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ConnectVortices {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "vortices", kind: "connect-vortices", record: "ConnectedVortices" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.attracting, self.attracted)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
