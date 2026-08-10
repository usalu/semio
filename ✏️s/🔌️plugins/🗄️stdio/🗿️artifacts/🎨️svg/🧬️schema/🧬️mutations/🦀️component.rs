//! 🧬️ SvgMutation — document mutation dispatch.

use crate::artifacts::svg::schema::diff::{diff_set_snapshot, SvgDiff};
use crate::artifacts::svg::SvgSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.svg`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SvgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SvgSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) {
    match mutation {
        SvgMutation::NoMutation => {}
        SvgMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SvgSnapshot> for SvgMutation {
    type Diff = SvgDiff;

    fn diff(&self, _base: &SvgSnapshot) -> Self::Diff {
        match self {
            SvgMutation::NoMutation => SvgDiff::default(),
            SvgMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<Self> {
        match self {
            SvgMutation::NoMutation => vec![SvgMutation::NoMutation],
            SvgMutation::SetSnapshot { .. } => vec![SvgMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for SvgMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for SvgMutation {
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
