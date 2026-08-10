//! 🧬️ DeflateMutation — document mutation dispatch.

use crate::artifacts::deflate::schema::diff::{diff_set_snapshot, DeflateDiff};
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.deflate`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DeflateMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: DeflateSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_deflate_mutation(snapshot: &mut DeflateSnapshot, mutation: &DeflateMutation) {
    match mutation {
        DeflateMutation::NoMutation => {}
        DeflateMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<DeflateSnapshot> for DeflateMutation {
    type Diff = DeflateDiff;

    fn diff(&self, _base: &DeflateSnapshot) -> Self::Diff {
        match self {
            DeflateMutation::NoMutation => DeflateDiff::default(),
            DeflateMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &DeflateSnapshot) -> Vec<Self> {
        match self {
            DeflateMutation::NoMutation => vec![DeflateMutation::NoMutation],
            DeflateMutation::SetSnapshot { .. } => vec![DeflateMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for DeflateMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for DeflateMutation {
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
