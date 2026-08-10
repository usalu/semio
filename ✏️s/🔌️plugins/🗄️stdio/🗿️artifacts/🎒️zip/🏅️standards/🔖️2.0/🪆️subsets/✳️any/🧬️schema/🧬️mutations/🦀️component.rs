//! 🧬️ ZipMutation — document mutation dispatch.

use crate::artifacts::zip::schema::diff::{diff_set_snapshot, ZipDiff};
use crate::artifacts::zip::ZipSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.zip`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum ZipMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: ZipSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_zip_mutation(snapshot: &mut ZipSnapshot, mutation: &ZipMutation) {
    match mutation {
        ZipMutation::NoMutation => {}
        ZipMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<ZipSnapshot> for ZipMutation {
    type Diff = ZipDiff;

    fn diff(&self, _base: &ZipSnapshot) -> Self::Diff {
        match self {
            ZipMutation::NoMutation => ZipDiff::default(),
            ZipMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &ZipSnapshot) -> Vec<Self> {
        match self {
            ZipMutation::NoMutation => vec![ZipMutation::NoMutation],
            ZipMutation::SetSnapshot { .. } => vec![ZipMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for ZipMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for ZipMutation {
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
