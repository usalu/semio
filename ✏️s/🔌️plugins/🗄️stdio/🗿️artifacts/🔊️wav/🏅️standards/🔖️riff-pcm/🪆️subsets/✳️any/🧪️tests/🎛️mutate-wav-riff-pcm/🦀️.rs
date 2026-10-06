//! 🦀️ WAV RIFF-PCM exhaustive mutation case — Rust adapter.
//!
//! Every scenario copies the immutable real recording into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `riff` oracle (this subset's
//! `🔮️oracles/🦀️.rs`: the RIFF container through the third-party `riff` crate, composed with its own PCM16
//! `fmt `/`data` layout), `subject` drives this repository's own decode → mutate → encode round trip, and both
//! results are read back by the SAME independent projector before the `semantic-audio-v1` profile compares them. The subject half is
//! gated behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_wav_test_oracle::standards::v_riff_pcm::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_identity_round_trip, project_wav_mutation};
use semio_repo_test_host::law;

//#region 🔖️Input
const INPUT: &str = "shared://🎙️bauen-mit-bestand-ausschnitt/🔊️.wav";


/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("input.wav"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from the untouched fixture. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to an unchanged recording's.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx)?;
    let before = project_wav_mutation(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_wav_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted rather than assumed: the `riff` oracle applies the row's kind, then the
/// reference's own computed inverse on top of that result, and the rewritten recording must project
/// back onto the pristine original. Returning the untouched original (what this used to do) asserted
/// nothing — the scenario passed whenever the oracle merely parsed the fixture.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_wav_mutation(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let projection = project_wav_mutation(&restored)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity round trip, asserted rather than assumed: the `riff` oracle decodes the `fmt `/`data` pair
/// and writes a fresh file from the decoded model alone, and the semantic projection — format
/// block, every decoded sample, every retained chunk — must survive that unchanged.
///
/// 🚫️ The "re-encoded bytes must differ from the input" half of the law does NOT hold on this side
/// and is deliberately not contrived into one that does. RIFF/WAVE 16-bit PCM has exactly ONE
/// canonical layout for a recording with no auxiliary chunks — a 44-byte `RIFF`/`fmt `/`data`
/// header followed by the samples — and `shared://🎙️bauen-mit-bestand-ausschnitt/🔊️.wav` is precisely
/// that (verified: mono, 8000 Hz, 16-bit, `data` starts at offset 44, no `LIST`/`fact`/anything
/// else). A conforming writer reproducing it byte-for-byte is the format being canonical, not the
/// input being smuggled through. What IS assertable of a canonical writer — and asserted here — is
/// that it is a fixpoint on that layout: a dropped chunk, a miscounted sample or a wrong byte rate
/// would all move the bytes. The SUBJECT side asserts the same two halves for the same reason —
/// canonicity is a property of the format, not of one writer — see `subject::identity_round_trip`.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_identity_round_trip(&input)?;
    let before = project_wav_mutation(&input)?;
    let after = project_wav_mutation(&output)?;
    law::round_trip_preserves(&after, &before)?;
    law::carrier_is_exact(&output, &input)?;
    Ok(Outcome::with_raw(output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::io::{decode_wav, encode_wav};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_wav::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation, WavMutation};
    use semio_s_artifact_stdio_wav_test_oracle::standards::v_riff_pcm::subsets::any::project_wav_mutation;
    use semio_repo_test_host::law;

    //#region 🔖️SpecReading
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<WavMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️Scenarios
    /// 🎯️ Decode → apply the declared mutation → re-encode, projected through the SAME independent
    /// reader the oracle used.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode_wav(&input).map_err(|error| format!("decode_wav failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_wav_mutation(&mut snapshot, &mutation);
        let bytes = encode_wav(&snapshot);
        let projection = project_wav_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Decode → apply the declared mutation → apply every mutation the vocabulary's own inverse returns
    /// against the pre-mutation snapshot → re-encode. The result must project back onto the pristine original —
    /// the inverse oracle's own reference claim.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let original = decode_wav(&input).map_err(|error| format!("decode_wav failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = original.clone();
        apply_wav_mutation(&mut snapshot, &mutation);
        for undo in mutation_inverse(&mutation, &original).expect("valid retained mutation inverse fixture") {
            apply_wav_mutation(&mut snapshot, &undo);
        }
        let bytes = encode_wav(&snapshot);
        let projection = project_wav_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Full parse into the typed snapshot, then re-serialize from the model ALONE — asserted
    /// through `law::carrier_is_exact`, the DOCUMENTED MIRROR of the no-byte-pass-through tripwire,
    /// for the SAME reason `round_trip_oracle` above already spells out and which turns out not to
    /// distinguish the two writers at all: RIFF/WAVE 16-bit PCM has exactly ONE canonical layout for
    /// a recording with no auxiliary chunks, and `shared://🎙️bauen-mit-bestand-ausschnitt/🔊️.wav` is
    /// precisely that layout. Canonicity is a property of the FORMAT, so a conforming writer that
    /// reproduced anything else would be the defect — this repository's `encode_wav` included.
    /// This handler used to demand the opposite of the subject in the same breath as excusing the
    /// oracle from it, and that contradiction is what the subject phase's first ever run failed on.
    /// The parse is real and stays checked elsewhere: `WavSnapshot` has no raw-byte escape hatch for
    /// what it claims to understand (this fixture decodes to typed `WavData::Pcm16` samples, one
    /// 16-bit little-endian word at a time), and the five `mutate-*` rows drive this same
    /// decode/encode pipeline and every one of them moves both the bytes and the projection.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_wav(&input).map_err(|error| format!("decode_wav failed: {error}"))?;
        let output = encode_wav(&snapshot);
        law::carrier_is_exact(&output, &input)?;
        let projection = project_wav_mutation(&output)?;
        law::round_trip_preserves(&projection, &project_wav_mutation(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Scenarios
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
