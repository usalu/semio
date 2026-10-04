//! 🦀️ MP4 ISO-BMFF exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR wave 7.
//!
//! Every scenario copies the immutable real 1.5s H.264 excerpt into the case work directory first;
//! the committed fixture is never written to. `oracle` drives the registered `mp4` 0.14 reference
//! implementation (`../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🦀️oracle.rs`'s own
//! `oracle_apply_mutation`); `subject` drives this repository's own `decode_mp4`/`encode_mp4`/
//! `apply_mp4_mutation` over the full 9-kind `Mp4Mutation` vocabulary. Both results are read back by
//! the SAME independent `project_mp4_mutation` (`mp4`) before the `semantic-mp4-mutate-v1` profile
//! compares them. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_mp4_test_oracle::standards::v_isobmff::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_identity_round_trip, project_mp4_mutation};
use semio_repo_test_host::law;


//#region 🔖️Input
const INPUT: &str = "shared://🎬️.mp4";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.mp4"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the registered reference implementation and ASSERTS
/// the result is distinguishable from the untouched fixture. The exemption list is empty — every
/// kind this vocabulary declares reaches the compared projection — so a kind that stops moving it
/// fails here rather than reporting a green identical to an unchanged movie's.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx)?;
    let before = project_mp4_mutation(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_mp4_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted rather than assumed: `mp4` applies the row's kind, then its own
/// independently computed inverse on top of that result, and the re-muxed movie must project back
/// onto the pristine original. Returning the untouched original (what this used to do) asserted
/// nothing — the scenario passed whenever `mp4` merely parsed the fixture.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_mp4_mutation(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation_inverse(&input, &spec, &mutated)?;
    let projection = project_mp4_mutation(&restored)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The no-byte-pass-through law on the ORACLE side: `Mp4Reader` parses the real movie into
/// tracks and samples and `Mp4Writer` re-muxes a fresh file from that model alone, so the bytes must
/// move (a second muxer's box order and `mdat` layout are not this fixture's) while the semantic
/// projection must not.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_identity_round_trip(&input)?;
    law::reparsed_not_copied(&output, &input)?;
    let before = project_mp4_mutation(&input)?;
    let after = project_mp4_mutation(&output)?;
    law::round_trip_preserves(&after, &before)?;
    Ok(Outcome::with_raw(output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::io::{decode_mp4, encode_mp4};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_mp4::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::mutations::{apply_mp4_mutation, Mp4Mutation};
    use semio_s_artifact_stdio_mp4_test_oracle::standards::v_isobmff::subsets::any::project_mp4_mutation;
    use semio_repo_test_host::law;

    //#region 🔖️SpecReading
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<Mp4Mutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️Scenarios
    /// 🎯️ Decode → apply the declared mutation → re-encode, projected through the SAME independent
    /// reader the oracle used.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode_mp4(&input).map_err(|error| format!("decode_mp4 failed: {error}"))?;
        apply_mp4_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let bytes = encode_mp4(&snapshot);
        let projection = project_mp4_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Decode → apply the declared mutation → apply every mutation the vocabulary's own inverse returns
    /// against the pre-mutation snapshot → re-encode. The result must project back onto the pristine
    /// original — the inverse oracle's own reference claim.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let original = decode_mp4(&input).map_err(|error| format!("decode_mp4 failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = original.clone();
        apply_mp4_mutation(&mut snapshot, &mutation);
        for undo in mutation_inverse(&mutation, &original).expect("valid retained mutation inverse fixture") {
            apply_mp4_mutation(&mut snapshot, &undo);
        }
        let bytes = encode_mp4(&snapshot);
        let projection = project_mp4_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Full parse into the typed snapshot, then re-serialize from the model ALONE — asserted
    /// through `law::carrier_is_exact`, the DOCUMENTED MIRROR of the no-byte-pass-through tripwire,
    /// because for THIS codec reproducing the input exactly is the correct answer and anything else
    /// is the defect. The reason is the third of the law's three admissible ones: `Mp4Snapshot`
    /// carries no raw-byte escape hatch of any kind — every `mvhd`/`tkhd`/`mdhd` field, every edit
    /// list entry, the visual sample entry, `colr`/`pasp`/`btrt`, the `avcC` extension and the
    /// `stsc`/`stco` chunk grouping are typed fields — and `encode_mp4` rebuilds the whole `moov`
    /// from them into one deterministic normal form (`ftyp`, `moov`, canonical empty `free`,
    /// `mdat`) that this ffmpeg `-c copy -movflags +faststart` fixture's own layout already is.
    /// That is not an excuse for a weaker claim, it is a STRONGER one, and the artifact holds
    /// itself to it independently of this case:
    /// `../../🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`'s own
    /// `exact_bauen_mit_bestand_fixture_round_trips_byte_for_byte` asserts the same equality on the
    /// full real recording. `law::reparsed_not_copied` would be exactly backwards here: it would
    /// demand that a lossless container codec LOSE something. The evidence that a parse really
    /// happened is the ten `mutate-*` rows above, which drive the same decode/encode pipeline and
    /// every one of which moves both the bytes and the compared projection.
    /// The ORACLE half of this same scenario keeps `law::reparsed_not_copied`, because `mp4` 0.14's
    /// `Mp4Writer` is a different writer with its own box order and `mdat` layout.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_mp4(&input).map_err(|error| format!("decode_mp4 failed: {error}"))?;
        let output = encode_mp4(&snapshot);
        law::carrier_is_exact(&output, &input)?;
        let projection = project_mp4_mutation(&output)?;
        law::round_trip_preserves(&projection, &project_mp4_mutation(&input)?)?;
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
