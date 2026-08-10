//! 🧬️ StepMutation — document mutation dispatch.

use crate::artifacts::step::schema::diff::{diff_set_snapshot, StepDiff};
use crate::artifacts::step::StepSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.step`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StepMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: StepSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_step_mutation(snapshot: &mut StepSnapshot, mutation: &StepMutation) -> StepDiff {
    let __diff = <StepMutation as protocol::Mutation<StepSnapshot>>::diff(mutation, snapshot);
    match mutation {
        StepMutation::NoMutation => {}
        StepMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StepSnapshot> for StepMutation {
    type Diff = StepDiff;

    fn diff(&self, _base: &StepSnapshot) -> Self::Diff {
        match self {
            StepMutation::NoMutation => StepDiff::default(),
            StepMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &StepSnapshot) -> Vec<Self> {
        match self {
            StepMutation::NoMutation => vec![StepMutation::NoMutation],
            StepMutation::SetSnapshot { .. } => vec![StepMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for StepMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for StepMutation {
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
