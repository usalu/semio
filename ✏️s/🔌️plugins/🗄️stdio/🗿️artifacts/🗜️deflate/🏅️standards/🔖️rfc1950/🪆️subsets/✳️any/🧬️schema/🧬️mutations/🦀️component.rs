//! 🧬️ DeflateMutation — document mutation dispatch over the typed RFC1950 container fields.

use crate::artifacts::deflate::schema::diff::{
    diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, diff_set_snapshot,
    DeflateDiff,
};
use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.deflate`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DeflateMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: DeflateSnapshot,
    },
    /// 🧮️ Sets CMF's compression method/window bits and FLG's compression-level hint together
    /// (they're written to the same two-byte header, so one mutation covers all three).
    SetCompressionParams {
        method: u8,
        window_bits: u8,
        level_hint: DeflateLevelHint,
    },
    /// 📖️ Sets or clears (via `None`) the preset-dictionary id (FLG.FDICT + DICTID).
    SetPresetDictionary {
        dict_id: Option<u32>,
    },
    /// 📦️ Replaces the decompressed payload wholesale.
    SetPayload {
        payload: Vec<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`; the diff is the single semantics source (never
/// apply-and-capture).
pub fn apply_deflate_mutation(snapshot: &mut DeflateSnapshot, mutation: &DeflateMutation) -> DeflateDiff {
    let d = <DeflateMutation as protocol::Mutation<DeflateSnapshot>>::diff(mutation, &*snapshot);
    *snapshot = <DeflateDiff as protocol::MutationDiff<DeflateSnapshot>>::apply(&d, snapshot);
    d
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<DeflateSnapshot> for DeflateMutation {
    type Diff = DeflateDiff;

    fn diff(&self, base: &DeflateSnapshot) -> Self::Diff {
        match self {
            DeflateMutation::NoMutation => DeflateDiff::default(),
            DeflateMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            DeflateMutation::SetCompressionParams { method, window_bits, level_hint } => {
                diff_set_compression_params(*method, *window_bits, *level_hint)
            }
            DeflateMutation::SetPresetDictionary { dict_id } => diff_set_preset_dictionary(*dict_id),
            DeflateMutation::SetPayload { payload } => diff_set_payload(payload.clone()),
        }
    }

    fn inverse(&self, base: &DeflateSnapshot) -> Vec<Self> {
        match self {
            DeflateMutation::NoMutation => vec![DeflateMutation::NoMutation],
            DeflateMutation::SetSnapshot { .. } => vec![DeflateMutation::SetSnapshot { snapshot: base.clone() }],
            DeflateMutation::SetCompressionParams { .. } => vec![DeflateMutation::SetCompressionParams {
                method: base.compression_method,
                window_bits: base.window_bits,
                level_hint: base.compression_level_hint,
            }],
            DeflateMutation::SetPresetDictionary { .. } => {
                vec![DeflateMutation::SetPresetDictionary { dict_id: base.dict_id }]
            }
            DeflateMutation::SetPayload { .. } => vec![DeflateMutation::SetPayload { payload: base.payload.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for DeflateMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for DeflateMutation {
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
