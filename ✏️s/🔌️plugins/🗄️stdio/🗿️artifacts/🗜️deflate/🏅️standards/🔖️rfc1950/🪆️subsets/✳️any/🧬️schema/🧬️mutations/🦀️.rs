//! 🧬️ DeflateMutation — document mutation dispatch over the typed RFC1950 container fields.

use crate::schema::diff::{diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, DeflateDiff};
use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::Mutation;


//#region 🔖️Mutations
#[path = "🧮set-compression-params/🦀️.rs"]
pub mod set_compression_params;
#[path = "📦set-payload/🦀️.rs"]
pub mod set_payload;
#[path = "📖set-preset-dictionary/🦀️.rs"]
pub mod set_preset_dictionary;
/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
///
/// 🧪️ `#[derive(dsl::DslOps)]` is kept ALONGSIDE `#[derive(dsl::Mutations)]`: every variant below
/// is a single-field newtype wrapping its own mutation leaf, and `dsl_variants_codegen`'s
/// "single-field tuple variant" branch (`✨️derive/🦀️.rs`) delegates `DslVariants`
/// straight through to that leaf's own `#[derive(dsl::DslRecord)]`-provided `DslField` impl — the
/// SAME `record_codegen` output the fields produced when they lived inline in the enum, so the
/// committed `crate::standards::v_rfc1950::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO`/`crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO`
/// facets and this `OpText`/`OpBinary` pair are unaffected by the leaf split.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = DeflateSnapshot, diff = DeflateDiff, schema = "DeflateMutation")]
pub enum DeflateMutation {
    /// 🧮️ Sets CMF's compression method/window bits and FLG's compression-level hint together
    /// (they're written to the same two-byte header, so one mutation covers all three).
    SetCompressionParams(set_compression_params::SetCompressionParams),
    /// 📖️ Sets or clears (via `None`) the preset-dictionary id (FLG.FDICT + DICTID).
    SetPresetDictionary(set_preset_dictionary::SetPresetDictionary),
    /// 📦️ Replaces the decompressed payload wholesale.
    SetPayload(set_payload::SetPayload),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🗂️ Kebab-case spelling of every `DeflateMutation` variant, declaration order, mirrored by this
/// subset's `🔣️oracle.json` mutation catalog (`deflate-rfc1950-any`). The completeness
/// gate reads that JSON catalog, never this enum, so `kinds_match_enum_variants_and_catalog` below
/// is what keeps the two lists honest.
pub const KINDS: &[&str] = &["set-compression-params", "set-preset-dictionary", "set-payload"];
//#endregion 🔖️Kinds


//#endregion 🔖️Apply

//#region OpCodecs



//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `DeflateMutation` values (every variant, incl. both
/// `SetPresetDictionary` arms and both `SetPayload` empty/non-empty arms) — the single source of
/// truth reused by `op_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DeflateMutation> {
    use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;

    vec![
        DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: 8, window_bits: 5, level_hint: DeflateLevelHint::Maximum }),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: Some(7) }),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: None }),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload: b"demo-payload".to_vec() }),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload: Vec::new() }),
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
