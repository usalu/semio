//! 🧬️ LasMutation — document mutation dispatch.

use crate::artifacts::las::schema::diff::{diff_set_snapshot, LasDiff};
use crate::artifacts::las::LasSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.las`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum LasMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: LasSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_las_mutation(snapshot: &mut LasSnapshot, mutation: &LasMutation) -> LasDiff {
    let __diff = <LasMutation as protocol::Mutation<LasSnapshot>>::diff(mutation, snapshot);
    match mutation {
        LasMutation::NoMutation => {}
        LasMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<LasSnapshot> for LasMutation {
    type Diff = LasDiff;

    fn diff(&self, _base: &LasSnapshot) -> Self::Diff {
        match self {
            LasMutation::NoMutation => LasDiff::default(),
            LasMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &LasSnapshot) -> Vec<Self> {
        match self {
            LasMutation::NoMutation => vec![LasMutation::NoMutation],
            LasMutation::SetSnapshot { .. } => vec![LasMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for LasMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for LasMutation {
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
