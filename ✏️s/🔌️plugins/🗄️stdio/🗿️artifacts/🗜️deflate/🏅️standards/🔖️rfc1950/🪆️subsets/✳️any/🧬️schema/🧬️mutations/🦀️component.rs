//! 🧬️ DeflateMutation — document mutation dispatch over the typed RFC1950 container fields.

use crate::artifacts::deflate::schema::diff::{
    diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, diff_set_snapshot,
    DeflateDiff,
};
use crate::artifacts::deflate::schema::snapshot::DeflateLevelHint;
use crate::artifacts::deflate::DeflateSnapshot;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.deflate`.
///
/// 🧪️ F6: `dsl::DslOps` derive — `DeflateSnapshot`'s field tree has no data-carrying enum
/// anywhere (`DeflateLevelHint` is unit-variant-only, `dsl::DslScalar`-derived), so every
/// variant's payload binds cleanly (confirmed via real `cargo check`, no mirror-enum needed).
/// `#[dsl(block)]` on the struct-valued `snapshot` payload matches the `SpaceMutation`/
/// `GifMutation` framework precedent's formatting convention; `#[dsl(base64)]` on the two bare
/// `Vec<u8>`/`insert`-shaped payloads keeps the printed op compact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DeflateMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
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
        #[dsl(base64)]
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
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only, never `OpText`/
/// `OpBinary` themselves) — the same ~15-line body every `DslOps`-derived enum's `OpText` impl
/// uses (`FlowMutationDsl`/`SpaceMutation`/`BinaryMutation`/`GifMutation` precedent). Replaces
/// the prior `serde_json` stub.
impl protocol::OpText for DeflateMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`. Replaces the prior
/// `serde_json` stub.
impl protocol::OpBinary for DeflateMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

    fn base_snapshot() -> DeflateSnapshot {
        DeflateSnapshot {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 8,
            window_bits: 7,
            compression_level_hint: DeflateLevelHint::Fastest,
            dict_id: None,
            payload: b"op-text-binary-fixture".to_vec(),
        }
    }

    /// 🧪️ `op_text_binary_roundtrip_law`: every variant (incl. both `SetPresetDictionary` arms,
    /// `Some`/`None`, and the `SetSnapshot` struct-payload variant) round-trips through
    /// `print_op`/`parse_op` (one line, no `\n`) AND `encode_op`/`decode_op`.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in [
            DeflateMutation::NoMutation,
            DeflateMutation::SetSnapshot {
                snapshot: DeflateSnapshot { dict_id: Some(0xDEAD_BEEF), ..base.clone() },
            },
            DeflateMutation::SetCompressionParams {
                method: 8,
                window_bits: 5,
                level_hint: DeflateLevelHint::Maximum,
            },
            DeflateMutation::SetPresetDictionary { dict_id: Some(7) },
            DeflateMutation::SetPresetDictionary { dict_id: None },
            DeflateMutation::SetPayload { payload: b"mutation-op-text-binary".to_vec() },
            DeflateMutation::SetPayload { payload: Vec::new() },
        ] {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = DeflateMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = DeflateMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
}
//#endregion Tests
