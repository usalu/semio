//! 🧬️ GlbMutation — document mutation dispatch.

use crate::artifacts::glb::schema::diff::{diff_set_snapshot, GlbDiff};
use crate::artifacts::glb::GlbSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.glb`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GlbMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: GlbSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_glb_mutation(snapshot: &mut GlbSnapshot, mutation: &GlbMutation) {
    match mutation {
        GlbMutation::NoMutation => {}
        GlbMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<GlbSnapshot> for GlbMutation {
    type Diff = GlbDiff;

    fn diff(&self, _base: &GlbSnapshot) -> Self::Diff {
        match self {
            GlbMutation::NoMutation => GlbDiff::default(),
            GlbMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &GlbSnapshot) -> Vec<Self> {
        match self {
            GlbMutation::NoMutation => vec![GlbMutation::NoMutation],
            GlbMutation::SetSnapshot { .. } => vec![GlbMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for GlbMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for GlbMutation {
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
