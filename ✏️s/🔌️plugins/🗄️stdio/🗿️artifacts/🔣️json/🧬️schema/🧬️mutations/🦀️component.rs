//! 🧬️ JsonMutation — document mutation dispatch.

use crate::artifacts::json::schema::diff::{diff_set_snapshot, JsonDiff};
use crate::artifacts::json::JsonSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.json`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum JsonMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: JsonSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_json_mutation(snapshot: &mut JsonSnapshot, mutation: &JsonMutation) {
    match mutation {
        JsonMutation::NoMutation => {}
        JsonMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<JsonSnapshot> for JsonMutation {
    type Diff = JsonDiff;

    fn diff(&self, _base: &JsonSnapshot) -> Self::Diff {
        match self {
            JsonMutation::NoMutation => JsonDiff::default(),
            JsonMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<Self> {
        match self {
            JsonMutation::NoMutation => vec![JsonMutation::NoMutation],
            JsonMutation::SetSnapshot { .. } => vec![JsonMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for JsonMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for JsonMutation {
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
