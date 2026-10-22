//! 🧬️ SemioAudioMutation — the full named-variant vocabulary replacing the W1b `SetSnapshot`-only
//! scaffold: scalar setters (`SetSampleRate`/`SetFormat`), channel insert/remove/set-samples, and
//! tag insert/remove/set-value. Every variant's `diff()`/`inverse()` is handcrafted directly
//! against the sparse `SemioAudioDiff` shape — never apply-and-capture (per the schema-design
//! recipe's own svg infinite-recursion warning).
//!
//! `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
//! leaf payload, and its sentinel verb `no` is not in `APPROVED_VERBS` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`. Every variant is now a
//! tuple variant wrapping its own mutation leaf (`./*/🦀️.rs`), and this file's `agg_diff`/
//! `agg_inverse` carry the handcrafted semantics every leaf's `MutationKind` impl delegates back to.

use semio_s_artifact_stdio_semio_audio_model::schema::diff::SemioAudioDiff;














use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioChannel;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioFormat;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioTag;
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioAudioMutation` below's `encode_op`/
/// `decode_op` are now real production code (binary upgrade, this wave), not test-only.
use protocol::{ColdMutation, Mutation};

//#region 🔖️Mutations
//#region 🔖️Leaves
use semio_s_artifact_stdio_semio_audio_mut_family::schema::mutations::{insert_channel, insert_tag, remove_channel, remove_tag, set_channel_samples, set_format, set_sample_rate, set_tag_value};
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `s.stdio.semio.audio`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioAudioSnapshot, diff = SemioAudioDiff, schema = "SemioAudioMutation", reference = SemioAudioSnapshot, reference_diff = SemioAudioDiff)]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAudioMutation {
    SetSampleRate(set_sample_rate::SetSampleRate),
    SetFormat(set_format::SetFormat),
    InsertChannel(insert_channel::InsertChannel),
    RemoveChannel(remove_channel::RemoveChannel),
    SetChannelSamples(set_channel_samples::SetChannelSamples),
    InsertTag(insert_tag::InsertTag),
    RemoveTag(remove_tag::RemoveTag),
    SetTagValue(set_tag_value::SetTagValue),
}

/// 🏷️ The declared kebab-case mutation vocabulary of `s.stdio.semio.audio`, in enum declaration
/// order — what the `🔊️mutate-semio-audio` case's completeness gate counts against and what
/// `../../🔣️oracle.json`'s catalog repeats. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this declaration honest.
pub const KINDS: &[&str] = &["set-sample-rate", "set-format", "insert-channel", "remove-channel", "set-channel-samples", "insert-tag", "remove-tag", "set-tag-value"];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_audio_mutation(mutation: &SemioAudioMutation, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    <SemioAudioMutation as ColdMutation<SemioAudioSnapshot>>::diff(mutation, base)
}


/// ↩️ Free-function face of [`SemioAudioMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with `diff_semio_*_mutation` it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_audio_mutation(mutation: &SemioAudioMutation, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioAudioMutation as ColdMutation<SemioAudioSnapshot>>::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply




//#endregion 🔖️MutationTrait

//#region OpCodecs












//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioAudioMutation` cases, one per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs` and this
/// file's own `op_text_binary_roundtrip_law`.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_mutation_cases() -> Vec<SemioAudioMutation> {
    fn channel(seed: f32) -> SemioAudioChannel {
        SemioAudioChannel { samples: vec![seed, seed + 1.0, seed + 2.0] }
    }
    fn fixture() -> SemioAudioSnapshot {
        SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, channels: vec![channel(1.0), channel(2.0), channel(3.0)], tags: vec![SemioAudioTag { key: "title".into(), value: "t0".into() }], ..SemioAudioSnapshot::default() }
    }
    vec![
        SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: 48_000 }),
        SemioAudioMutation::SetFormat(set_format::SetFormat { format: SemioAudioFormat::Float32 }),
        SemioAudioMutation::InsertChannel(insert_channel::InsertChannel { index: 1, channel: channel(9.0) }),
        SemioAudioMutation::RemoveChannel(remove_channel::RemoveChannel { index: 1 }),
        SemioAudioMutation::SetChannelSamples(set_channel_samples::SetChannelSamples { index: 0, samples: vec![0.25, 0.5, 0.75] }),
        SemioAudioMutation::InsertTag(insert_tag::InsertTag { index: 0, tag: SemioAudioTag { key: "artist".into(), value: "a".into() } }),
        SemioAudioMutation::RemoveTag(remove_tag::RemoveTag { index: 0 }),
        SemioAudioMutation::SetTagValue(set_tag_value::SetTagValue { index: 0, value: "changed".into() }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
