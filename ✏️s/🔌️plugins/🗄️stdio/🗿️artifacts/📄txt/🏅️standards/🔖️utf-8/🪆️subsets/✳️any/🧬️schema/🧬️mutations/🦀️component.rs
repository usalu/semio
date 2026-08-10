//! 🧬️ TxtMutation — document mutation dispatch.

use crate::artifacts::txt::schema::diff::{diff_set_snapshot, TxtDiff};
use crate::artifacts::txt::TxtSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.txt`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TxtMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: TxtSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_txt_mutation(snapshot: &mut TxtSnapshot, mutation: &TxtMutation) {
    match mutation {
        TxtMutation::NoMutation => {}
        TxtMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TxtSnapshot> for TxtMutation {
    type Diff = TxtDiff;

    fn diff(&self, _base: &TxtSnapshot) -> Self::Diff {
        match self {
            TxtMutation::NoMutation => TxtDiff::default(),
            TxtMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<Self> {
        match self {
            TxtMutation::NoMutation => vec![TxtMutation::NoMutation],
            TxtMutation::SetSnapshot { .. } => vec![TxtMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for TxtMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for TxtMutation {
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
