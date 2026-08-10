//! 🧬️ GltfMutation — document mutation dispatch.

use crate::artifacts::gltf::schema::diff::{diff_set_snapshot, GltfDiff};
use crate::artifacts::gltf::GltfSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gltf`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GltfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: GltfSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_gltf_mutation(snapshot: &mut GltfSnapshot, mutation: &GltfMutation) {
    match mutation {
        GltfMutation::NoMutation => {}
        GltfMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<GltfSnapshot> for GltfMutation {
    type Diff = GltfDiff;

    fn diff(&self, _base: &GltfSnapshot) -> Self::Diff {
        match self {
            GltfMutation::NoMutation => GltfDiff::default(),
            GltfMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<Self> {
        match self {
            GltfMutation::NoMutation => vec![GltfMutation::NoMutation],
            GltfMutation::SetSnapshot { .. } => vec![GltfMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for GltfMutation {
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
