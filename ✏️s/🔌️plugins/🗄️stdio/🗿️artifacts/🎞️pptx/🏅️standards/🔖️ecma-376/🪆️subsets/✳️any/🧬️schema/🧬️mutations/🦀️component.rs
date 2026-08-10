//! 🧬️ PptxMutation — document mutation dispatch.

use crate::artifacts::pptx::schema::diff::{diff_set_snapshot, PptxDiff};
use crate::artifacts::pptx::PptxSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pptx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PptxMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PptxSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_pptx_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxMutation) {
    match mutation {
        PptxMutation::NoMutation => {}
        PptxMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PptxSnapshot> for PptxMutation {
    type Diff = PptxDiff;

    fn diff(&self, _base: &PptxSnapshot) -> Self::Diff {
        match self {
            PptxMutation::NoMutation => PptxDiff::default(),
            PptxMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &PptxSnapshot) -> Vec<Self> {
        match self {
            PptxMutation::NoMutation => vec![PptxMutation::NoMutation],
            PptxMutation::SetSnapshot { .. } => vec![PptxMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for PptxMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for PptxMutation {
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
