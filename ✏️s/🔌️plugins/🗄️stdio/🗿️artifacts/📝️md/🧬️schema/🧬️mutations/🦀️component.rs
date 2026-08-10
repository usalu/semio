//! 🧬️ MdMutation — document mutation dispatch.

use crate::artifacts::md::schema::diff::{diff_set_snapshot, MdDiff};
use crate::artifacts::md::MdSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.md`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum MdMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: MdSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_md_mutation(snapshot: &mut MdSnapshot, mutation: &MdMutation) {
    match mutation {
        MdMutation::NoMutation => {}
        MdMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<MdSnapshot> for MdMutation {
    type Diff = MdDiff;

    fn diff(&self, _base: &MdSnapshot) -> Self::Diff {
        match self {
            MdMutation::NoMutation => MdDiff::default(),
            MdMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &MdSnapshot) -> Vec<Self> {
        match self {
            MdMutation::NoMutation => vec![MdMutation::NoMutation],
            MdMutation::SetSnapshot { .. } => vec![MdMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for MdMutation {
    fn print_op(&self) -> String {
        serde_md::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_md::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for MdMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_md::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_md::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs
