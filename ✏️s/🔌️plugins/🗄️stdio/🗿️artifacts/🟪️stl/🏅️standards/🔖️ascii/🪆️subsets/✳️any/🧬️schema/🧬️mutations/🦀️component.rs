//! 🧬️ StlMutation — document mutation dispatch.

use crate::artifacts::stl::schema::diff::{diff_set_snapshot, StlDiff};
use crate::artifacts::stl::StlSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.stl`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StlMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: StlSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_stl_mutation(snapshot: &mut StlSnapshot, mutation: &StlMutation) -> StlDiff {
    let __diff = <StlMutation as protocol::Mutation<StlSnapshot>>::diff(mutation, snapshot);
    match mutation {
        StlMutation::NoMutation => {}
        StlMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StlSnapshot> for StlMutation {
    type Diff = StlDiff;

    fn diff(&self, _base: &StlSnapshot) -> Self::Diff {
        match self {
            StlMutation::NoMutation => StlDiff::default(),
            StlMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &StlSnapshot) -> Vec<Self> {
        match self {
            StlMutation::NoMutation => vec![StlMutation::NoMutation],
            StlMutation::SetSnapshot { .. } => vec![StlMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for StlMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for StlMutation {
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
