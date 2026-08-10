//! 🧬️ XlsxMutation — document mutation dispatch.

use crate::artifacts::xlsx::schema::diff::{diff_set_snapshot, XlsxDiff};
use crate::artifacts::xlsx::XlsxSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.xlsx`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XlsxMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: XlsxSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_xlsx_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> XlsxDiff {
    let __diff = <XlsxMutation as protocol::Mutation<XlsxSnapshot>>::diff(mutation, snapshot);
    match mutation {
        XlsxMutation::NoMutation => {}
        XlsxMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<XlsxSnapshot> for XlsxMutation {
    type Diff = XlsxDiff;

    fn diff(&self, _base: &XlsxSnapshot) -> Self::Diff {
        match self {
            XlsxMutation::NoMutation => XlsxDiff::default(),
            XlsxMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Vec<Self> {
        match self {
            XlsxMutation::NoMutation => vec![XlsxMutation::NoMutation],
            XlsxMutation::SetSnapshot { .. } => vec![XlsxMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for XlsxMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for XlsxMutation {
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
