//! 🧬️ PdfMutation — document mutation dispatch.

use crate::artifacts::pdf::schema::diff::{diff_set_snapshot, PdfDiff};
use crate::artifacts::pdf::PdfSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pdf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) {
    match mutation {
        PdfMutation::NoMutation => {}
        PdfMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfMutation {
    type Diff = PdfDiff;

    fn diff(&self, _base: &PdfSnapshot) -> Self::Diff {
        match self {
            PdfMutation::NoMutation => PdfDiff::default(),
            PdfMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        match self {
            PdfMutation::NoMutation => vec![PdfMutation::NoMutation],
            PdfMutation::SetSnapshot { .. } => vec![PdfMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for PdfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for PdfMutation {
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
