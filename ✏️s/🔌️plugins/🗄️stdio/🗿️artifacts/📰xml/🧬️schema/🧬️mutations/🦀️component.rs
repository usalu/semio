//! 🧬️ XmlMutation — document mutation dispatch.

use crate::artifacts::xml::schema::diff::{diff_set_snapshot, XmlDiff};
use crate::artifacts::xml::XmlSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.xml`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XmlMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: XmlSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_xml_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlMutation) {
    match mutation {
        XmlMutation::NoMutation => {}
        XmlMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<XmlSnapshot> for XmlMutation {
    type Diff = XmlDiff;

    fn diff(&self, _base: &XmlSnapshot) -> Self::Diff {
        match self {
            XmlMutation::NoMutation => XmlDiff::default(),
            XmlMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &XmlSnapshot) -> Vec<Self> {
        match self {
            XmlMutation::NoMutation => vec![XmlMutation::NoMutation],
            XmlMutation::SetSnapshot { .. } => vec![XmlMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for XmlMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for XmlMutation {
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
