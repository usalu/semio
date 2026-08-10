//! 🧬️ ObjMutation — document mutation dispatch.

use crate::artifacts::obj::schema::diff::{diff_set_snapshot, ObjDiff};
use crate::artifacts::obj::ObjSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.obj`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum ObjMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: ObjSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_obj_mutation(snapshot: &mut ObjSnapshot, mutation: &ObjMutation) {
    match mutation {
        ObjMutation::NoMutation => {}
        ObjMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<ObjSnapshot> for ObjMutation {
    type Diff = ObjDiff;

    fn diff(&self, _base: &ObjSnapshot) -> Self::Diff {
        match self {
            ObjMutation::NoMutation => ObjDiff::default(),
            ObjMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &ObjSnapshot) -> Vec<Self> {
        match self {
            ObjMutation::NoMutation => vec![ObjMutation::NoMutation],
            ObjMutation::SetSnapshot { .. } => vec![ObjMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for ObjMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for ObjMutation {
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
