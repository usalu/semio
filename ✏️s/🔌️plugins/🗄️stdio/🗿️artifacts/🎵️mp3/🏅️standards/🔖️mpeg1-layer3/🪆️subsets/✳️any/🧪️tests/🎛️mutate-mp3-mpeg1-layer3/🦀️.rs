//! 🦀️ MP3 mpeg1-layer3 mutation case — Rust adapter.
//!
//! Every scenario copies the committed 193,275-byte real stream into the case work
//! directory first; the committed file is never written to. `oracle` drives the registered `id3`
//! 1.17 reference composed with a hand-written ISO/IEC 11172-3 frame walker
//! (`../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🦀️oracle.rs`), `subject` drives
//! this repository's own decode/apply/encode round trip, and both results are read back through the
//! same independent projection before `semantic-mp3-mpeg1-layer3-v1` compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles
//! the local implementation.
//!
//! The `inverse-<kind>` and `identity-round-trip` handlers ASSERT their laws here rather than
//! deferring them to the parity phase: both laws are checkable by one role alone, and a handler
//! that merely projects and returns passes whenever the reference did not error.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_mp3};
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable, reparsed_not_copied, round_trip_preserves_within};


//#region 🔖️Profile
/// 📏️ `semantic-mp3-mpeg1-layer3-v1`'s own declared writer freedom
/// (`../../🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🔣️oracle.json`), mirrored here so
/// an in-handler law check is exactly as strict as the profile the case is measured by, never
/// stricter. Every projected value is an exact integer, a boolean or a string, so the tolerance is
/// genuinely zero rather than a nominal one.
const MP3_WRITER_FREEDOM: &[&str] = &["flags", "tagSize", "paddingLength", "fileSize"];
const MP3_TOLERANCE: f64 = 0.0;
//#endregion 🔖️Profile

//#region 🔖️Input
const INPUT: &str = "shared://🔊️.mp3";

/// 🧫️ Copies the immutable committed asset into the work directory and returns the copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.mp3"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from the untouched fixture. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to an unchanged stream's.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_mp3(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_mp3(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies `<id>` forward, then its independently computed inverse — the layer(s) it replaced restored
/// from the SAME untouched input, matching `Mp3Mutation::inverse()`'s own base-relative semantics — and
/// ASSERTS the law in role: the restored stream's projection must equal the real original's own. Without
/// the check the scenario would pass for any inverse `id3` merely tolerated.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let projection = project_mp3(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role. `id3`'s writer chooses its own ID3v2 padding
/// and re-derives the tag region wholesale (57 committed bytes come back as 86), so bit-identical
/// output could only mean the bytes were copied rather than parsed — the reference is bound by
/// [`reparsed_not_copied`]. The SUBJECT is bound by the opposite law and asserts it below.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_mp3(&bytes)?;
    round_trip_preserves_within(&projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, MP3_TOLERANCE, MP3_WRITER_FREEDOM};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_mp3::standards::mpeg1_layer3::subsets::any::io::{decode_mp3, encode_mp3};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_mp3::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::{apply_mp3_mutation, Mp3Mutation};
    use semio_s_artifact_stdio_mp3_test_oracle::standards::v_mpeg1_layer3::subsets::any::project_mp3;
    use semio_repo_test_host::law::{carrier_is_exact, inverse_restores_within, round_trip_preserves_within};

    //#region 🔖️Mutation
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_of(spec: &Json) -> Result<Mp3Mutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️Mutation

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode_mp3(&input).map_err(|error| format!("decode_mp3 failed: {error}"))?;
        let mutation = mutation_of(&ctx.doc_json()?)?;
        apply_mp3_mutation(&mut snapshot, &mutation);
        let bytes = encode_mp3(&snapshot);
        let projection = project_mp3(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_mp3(&input).map_err(|error| format!("decode_mp3 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let forward = mutation_of(&spec)?;
        let mut snapshot = base.clone();
        apply_mp3_mutation(&mut snapshot, &forward);
        for backward in mutation_inverse(&forward, &base) {
            apply_mp3_mutation(&mut snapshot, &backward);
        }
        let bytes = encode_mp3(&snapshot);
        let projection = project_mp3(&bytes)?;
        inverse_restores_within(&spec.str("kind"), &projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔁️ The identity law for the SUBJECT's own encoder, which is bound by the opposite byte law
    /// to the reference's: `encode_mp3` re-emits each frame's retained payload verbatim and
    /// recomputes the ID3v2 sizes from the frame data, and this fixture's tag is already canonical
    /// under that rule (169-byte body = TSSE's 10+47 plus TIT2's 10+63 plus TPE1's 10+13 plus
    /// TLEN's 10+6, no trailing padding — LAME wrote it tight), so its own
    /// `codec_retention_law` says the output reproduces the input exactly. Demanding a byte
    /// DIFFERENCE here would be a fabricated law; [`carrier_is_exact`] is the one that actually
    /// binds, and it still fails loudly the moment either half drifts.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_mp3(&input).map_err(|error| format!("decode_mp3 failed: {error}"))?;
        let bytes = encode_mp3(&snapshot);
        carrier_is_exact(&bytes, &input)?;
        let projection = project_mp3(&bytes)?;
        round_trip_preserves_within(&projection, &project_mp3(&input)?, MP3_WRITER_FREEDOM, MP3_TOLERANCE)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
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
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
