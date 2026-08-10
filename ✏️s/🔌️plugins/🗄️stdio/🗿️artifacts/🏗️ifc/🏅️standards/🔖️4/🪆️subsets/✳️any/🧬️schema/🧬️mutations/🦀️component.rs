//! 🧬️ IfcMutation — document mutation dispatch.

use crate::artifacts::ifc::schema::diff::{diff_set_snapshot, IfcDiff};
use crate::artifacts::ifc::IfcSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ifc`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum IfcMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: IfcSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_ifc_mutation(snapshot: &mut IfcSnapshot, mutation: &IfcMutation) {
    match mutation {
        IfcMutation::NoMutation => {}
        IfcMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<IfcSnapshot> for IfcMutation {
    type Diff = IfcDiff;

    fn diff(&self, _base: &IfcSnapshot) -> Self::Diff {
        match self {
            IfcMutation::NoMutation => IfcDiff::default(),
            IfcMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &IfcSnapshot) -> Vec<Self> {
        match self {
            IfcMutation::NoMutation => vec![IfcMutation::NoMutation],
            IfcMutation::SetSnapshot { .. } => vec![IfcMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for IfcMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for IfcMutation {
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
