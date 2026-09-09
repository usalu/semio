//! 🧬️ DeflateMutation — document mutation dispatch over the typed RFC1950 container fields.

use crate::schema::diff::{diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, diff_set_snapshot, DeflateDiff};
use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
#[path = "🧮set-compression-params/🦀️.rs"]
pub mod set_compression_params;
#[path = "📦set-payload/🦀️.rs"]
pub mod set_payload;
#[path = "📖set-preset-dictionary/🦀️.rs"]
pub mod set_preset_dictionary;
/// 📐️ Typed content mutation for `stdio.deflate`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 🧭️ `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly
/// one leaf payload (a unit variant wraps none) and asserts `is_approved_verb(SEMANTICS.verb)`,
/// and `no` is not an approved verb.
///
/// 🧪️ `#[derive(dsl::DslOps)]` is kept ALONGSIDE `#[derive(dsl::Mutations)]`: every variant below
/// is a single-field newtype wrapping its own mutation leaf, and `dsl_variants_codegen`'s
/// "single-field tuple variant" branch (`✨️derive/🦀️.rs`) delegates `DslVariants`
/// straight through to that leaf's own `#[derive(dsl::DslRecord)]`-provided `DslField` impl — the
/// SAME `record_codegen` output the fields produced when they lived inline in the enum, so the
/// committed `mutations::text::COMPONENT_GRAMMAR_SEMIO`/`mutations::binary::COMPONENT_PROTOCOL_SEMIO`
/// facets and this `OpText`/`OpBinary` pair are unaffected by the leaf split.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = DeflateSnapshot, diff = DeflateDiff, schema = "DeflateMutation")]
pub enum DeflateMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
pub const KINDS: &[&str] = &["set-snapshot", "set-compression-params", "set-preset-dictionary", "set-payload"];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`; the diff is the single semantics source (never
/// apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_deflate_mutation(snapshot: &mut DeflateSnapshot, mutation: &DeflateMutation) -> protocol::MutationOutcome<DeflateDiff> {
    let outcome = <DeflateMutation as Mutation<DeflateSnapshot>>::diff(mutation, &*snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &DeflateMutation, base: &DeflateSnapshot) -> protocol::MutationOutcome<DeflateDiff> {
    protocol::MutationOutcome::new(match this {
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method, window_bits, level_hint }) => diff_set_compression_params(*method, *window_bits, *level_hint),
        DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id }) => diff_set_preset_dictionary(*dict_id),
        DeflateMutation::SetPayload(set_payload::SetPayload { payload }) => diff_set_payload(payload.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DeflateMutation, base: &DeflateSnapshot) -> Vec<DeflateMutation> {
    match this {
        DeflateMutation::SetSnapshot(_) => vec![DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        DeflateMutation::SetCompressionParams(_) => vec![DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: base.compression_method, window_bits: base.window_bits, level_hint: base.compression_level_hint })],
        DeflateMutation::SetPresetDictionary(_) => {
            vec![DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: base.dict_id })]
        }
        DeflateMutation::SetPayload(_) => vec![DeflateMutation::SetPayload(set_payload::SetPayload { payload: base.payload.clone() })],
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only, never `OpText`/
/// `OpBinary` themselves) — the same ~15-line body every `DslOps`-derived enum's `OpText` impl
/// uses (`GifMutation`, `FlowMutationDsl`, `SpaceMutation` precedent).
impl OpText for DeflateMutation {
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

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for DeflateMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
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

    let snapshot =
        DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::Default, dict_id: Some(0x1234_5678), payload: b"demo-mutation-snapshot-payload".to_vec() };
    vec![
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() }),
        DeflateMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: DeflateSnapshot { dict_id: None, compression_level_hint: DeflateLevelHint::Fastest, ..snapshot } }),
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

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/📈️raises-the-af4398/🦀️.rs"]
mod set_snapshot_raises_the_flevel_hint_and_extends_the_payload;
//#endregion 🧪️FixtureCases
