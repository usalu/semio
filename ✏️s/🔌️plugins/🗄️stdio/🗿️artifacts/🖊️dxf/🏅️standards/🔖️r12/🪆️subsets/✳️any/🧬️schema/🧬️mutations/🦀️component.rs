//! 🧬️ DxfMutation — document mutation dispatch.

use crate::artifacts::dxf::schema::diff::{diff_set_snapshot, DxfDiff};
use crate::artifacts::dxf::DxfSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.dxf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DxfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: DxfSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_dxf_mutation(snapshot: &mut DxfSnapshot, mutation: &DxfMutation) -> DxfDiff {
    let __diff = <DxfMutation as protocol::Mutation<DxfSnapshot>>::diff(mutation, snapshot);
    match mutation {
        DxfMutation::NoMutation => {}
        DxfMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<DxfSnapshot> for DxfMutation {
    type Diff = DxfDiff;

    fn diff(&self, _base: &DxfSnapshot) -> Self::Diff {
        match self {
            DxfMutation::NoMutation => DxfDiff::default(),
            DxfMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &DxfSnapshot) -> Vec<Self> {
        match self {
            DxfMutation::NoMutation => vec![DxfMutation::NoMutation],
            DxfMutation::SetSnapshot { .. } => vec![DxfMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for DxfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for DxfMutation {
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
