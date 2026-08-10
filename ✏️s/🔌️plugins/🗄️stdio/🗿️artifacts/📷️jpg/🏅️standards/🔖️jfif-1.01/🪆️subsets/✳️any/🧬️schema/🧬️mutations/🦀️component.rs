//! 🧬️ JpgMutation — document mutation dispatch.

use crate::artifacts::jpg::schema::diff::{diff_set_snapshot, JpgDiff};
use crate::artifacts::jpg::JpgSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.jpg`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum JpgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: JpgSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_jpg_mutation(snapshot: &mut JpgSnapshot, mutation: &JpgMutation) -> JpgDiff {
    let __diff = <JpgMutation as protocol::Mutation<JpgSnapshot>>::diff(mutation, snapshot);
    match mutation {
        JpgMutation::NoMutation => {}
        JpgMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<JpgSnapshot> for JpgMutation {
    type Diff = JpgDiff;

    fn diff(&self, _base: &JpgSnapshot) -> Self::Diff {
        match self {
            JpgMutation::NoMutation => JpgDiff::default(),
            JpgMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &JpgSnapshot) -> Vec<Self> {
        match self {
            JpgMutation::NoMutation => vec![JpgMutation::NoMutation],
            JpgMutation::SetSnapshot { .. } => vec![JpgMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for JpgMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for JpgMutation {
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
