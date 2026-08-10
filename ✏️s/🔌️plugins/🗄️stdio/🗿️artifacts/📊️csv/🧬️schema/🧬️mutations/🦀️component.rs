//! 🧬️ CsvMutation — document mutation dispatch.

use crate::artifacts::csv::schema::diff::{diff_set_snapshot, CsvDiff};
use crate::artifacts::csv::CsvSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.csv`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum CsvMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: CsvSnapshot,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`.
pub fn apply_csv_mutation(snapshot: &mut CsvSnapshot, mutation: &CsvMutation) {
    match mutation {
        CsvMutation::NoMutation => {}
        CsvMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<CsvSnapshot> for CsvMutation {
    type Diff = CsvDiff;

    fn diff(&self, _base: &CsvSnapshot) -> Self::Diff {
        match self {
            CsvMutation::NoMutation => CsvDiff::default(),
            CsvMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, base: &CsvSnapshot) -> Vec<Self> {
        match self {
            CsvMutation::NoMutation => vec![CsvMutation::NoMutation],
            CsvMutation::SetSnapshot { .. } => vec![CsvMutation::SetSnapshot { snapshot: base.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for CsvMutation {
    fn print_op(&self) -> String {
        serde_csv::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_csv::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for CsvMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_csv::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_csv::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs
