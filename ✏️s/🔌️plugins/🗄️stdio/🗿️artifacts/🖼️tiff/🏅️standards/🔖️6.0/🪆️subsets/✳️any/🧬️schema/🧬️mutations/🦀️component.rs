//! 🧬️ TiffMutation — document mutation dispatch.

use crate::artifacts::tiff::schema::diff::{diff_set_snapshot, TiffDiff};
use crate::artifacts::tiff::TiffSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.tiff`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TiffMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: TiffSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_tiff_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffMutation) -> TiffDiff {
    let __diff = <TiffMutation as protocol::Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    match mutation {
        TiffMutation::NoMutation => {}
        TiffMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TiffSnapshot> for TiffMutation {
    type Diff = TiffDiff;

    fn diff(&self, _base: &TiffSnapshot) -> Self::Diff {
        match self {
            TiffMutation::NoMutation => TiffDiff::default(),
            TiffMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &TiffSnapshot) -> Vec<Self> {
        match self {
            TiffMutation::NoMutation => vec![TiffMutation::NoMutation],
            TiffMutation::SetSnapshot { .. } => vec![TiffMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for TiffMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for TiffMutation {
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
