//! 🧬️ DocxMutation — document mutation dispatch.

use crate::artifacts::docx::schema::diff::{diff_set_snapshot, DocxDiff};
use crate::artifacts::docx::DocxSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.docx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DocxMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: DocxSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_docx_mutation(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) {
    match mutation {
        DocxMutation::NoMutation => {}
        DocxMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<DocxSnapshot> for DocxMutation {
    type Diff = DocxDiff;

    fn diff(&self, _base: &DocxSnapshot) -> Self::Diff {
        match self {
            DocxMutation::NoMutation => DocxDiff::default(),
            DocxMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &DocxSnapshot) -> Vec<Self> {
        match self {
            DocxMutation::NoMutation => vec![DocxMutation::NoMutation],
            DocxMutation::SetSnapshot { .. } => vec![DocxMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for DocxMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for DocxMutation {
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
