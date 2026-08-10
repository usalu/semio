//! 🧬️ BcfMutation — document mutation dispatch.

use crate::artifacts::bcf::schema::diff::{diff_set_snapshot, BcfDiff};
use crate::artifacts::bcf::BcfSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.bcf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum BcfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: BcfSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_bcf_mutation(snapshot: &mut BcfSnapshot, mutation: &BcfMutation) -> BcfDiff {
    let __diff = <BcfMutation as protocol::Mutation<BcfSnapshot>>::diff(mutation, snapshot);
    match mutation {
        BcfMutation::NoMutation => {}
        BcfMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<BcfSnapshot> for BcfMutation {
    type Diff = BcfDiff;

    fn diff(&self, _base: &BcfSnapshot) -> Self::Diff {
        match self {
            BcfMutation::NoMutation => BcfDiff::default(),
            BcfMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &BcfSnapshot) -> Vec<Self> {
        match self {
            BcfMutation::NoMutation => vec![BcfMutation::NoMutation],
            BcfMutation::SetSnapshot { .. } => vec![BcfMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for BcfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for BcfMutation {
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
