//! 🧬️ PlyMutation — document mutation dispatch.

use crate::artifacts::ply::schema::diff::{diff_set_snapshot, PlyDiff};
use crate::artifacts::ply::PlySnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ply`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PlyMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PlySnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_ply_mutation(snapshot: &mut PlySnapshot, mutation: &PlyMutation) -> PlyDiff {
    let __diff = <PlyMutation as protocol::Mutation<PlySnapshot>>::diff(mutation, snapshot);
    match mutation {
        PlyMutation::NoMutation => {}
        PlyMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PlySnapshot> for PlyMutation {
    type Diff = PlyDiff;

    fn diff(&self, _base: &PlySnapshot) -> Self::Diff {
        match self {
            PlyMutation::NoMutation => PlyDiff::default(),
            PlyMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &PlySnapshot) -> Vec<Self> {
        match self {
            PlyMutation::NoMutation => vec![PlyMutation::NoMutation],
            PlyMutation::SetSnapshot { .. } => vec![PlyMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for PlyMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for PlyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs
