//! 🔗 Puzzle2d mutation — `ConnectHandles`: creates a directed link between two handles, full
//! initial connection-parameterization payload included (rule 4: `connect-<nouns>{endpoints,
//! payload}`).
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗 `connect-handles` payload — edge `id`, both endpoint handle ids, and the full initial
/// connection-parameter payload (`edge_kind`/`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt`/`x`/`y`/
/// `source_tip`/`target_tip`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "connect-handles")]
pub struct ConnectHandles {
    pub id: String,
    #[dsl(refs = "handle")]
    pub source: String,
    #[dsl(refs = "handle")]
    pub target: String,
    pub edge_kind: Option<String>,
    pub gap: f64,
    pub shift: f64,
    pub rise: f64,
    pub rotation: f64,
    pub turn: f64,
    pub tilt: f64,
    pub x: f64,
    pub y: f64,
    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
#[allow(clippy::too_many_arguments)]
pub fn connect_handles(
    id: String,
    source: String,
    target: String,
    edge_kind: Option<String>,
    gap: f64,
    shift: f64,
    rise: f64,
    rotation: f64,
    turn: f64,
    tilt: f64,
    x: f64,
    y: f64,
    source_tip: Option<String>,
    target_tip: Option<String>,
) -> Puzzle2dMutation {
    Puzzle2dMutation::ConnectHandles(ConnectHandles { id, source, target, edge_kind, gap, shift, rise, rotation, turn, tilt, x, y, source_tip, target_tip })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ConnectHandles {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "handles", kind: "connect-handles", record: "ConnectedHandles" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect \"{}\" to \"{}\"", self.source, self.target)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
