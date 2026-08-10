//! 🧬️ BinaryMutation — document mutation dispatch.

use crate::artifacts::binary::schema::diff::{diff_set_snapshot, BinaryDiff};
use crate::artifacts::binary::BinarySnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.binary`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum BinaryMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: BinarySnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_binary_mutation(snapshot: &mut BinarySnapshot, mutation: &BinaryMutation) {
    match mutation {
        BinaryMutation::NoMutation => {}
        BinaryMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<BinarySnapshot> for BinaryMutation {
    type Diff = BinaryDiff;

    fn diff(&self, _base: &BinarySnapshot) -> Self::Diff {
        match self {
            BinaryMutation::NoMutation => BinaryDiff::default(),
            BinaryMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &BinarySnapshot) -> Vec<Self> {
        match self {
            BinaryMutation::NoMutation => vec![BinaryMutation::NoMutation],
            BinaryMutation::SetSnapshot { .. } => vec![BinaryMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for BinaryMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for BinaryMutation {
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
