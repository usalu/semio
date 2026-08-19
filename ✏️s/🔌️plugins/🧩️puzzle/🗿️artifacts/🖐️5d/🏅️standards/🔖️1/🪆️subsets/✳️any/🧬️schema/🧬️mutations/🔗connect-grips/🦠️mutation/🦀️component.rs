//! 🔗 Puzzle5d mutation — `ConnectGrips`: creates a fastener between two full grip ids
//! (`part_id:grip_id`), full initial connection-parameterization payload included.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-grips` payload — fastener `id`, both endpoint full grip ids, and the full initial
/// connection-parameter payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "connect-grips")]
pub struct ConnectGrips {
    pub id: String,
    pub source: String,
    pub target: String,
    pub fastener_kind: Option<String>,
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
pub async fn connect_grips(id: String, source: String, target: String, fastener_kind: Option<String>, gap: f64, shift: f64, rise: f64, rotation: f64, turn: f64, tilt: f64, x: f64, y: f64) -> Puzzle5dMutation {
    Puzzle5dMutation::ConnectGrips(ConnectGrips { id, source, target, fastener_kind, gap, shift, rise, rotation, turn, tilt, x, y })
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ConnectGrips {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "grips", kind: "connect-grips", record: "ConnectedGrips" };

    async fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.source, self.target)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
