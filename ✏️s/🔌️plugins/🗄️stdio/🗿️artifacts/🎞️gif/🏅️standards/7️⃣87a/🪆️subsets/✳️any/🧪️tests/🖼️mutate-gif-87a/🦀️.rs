//! 🦀️ GIF87a mutation-oracle case — Rust adapter.
//!
//! `oracle` drives the registered `gif` reference crate (`../../🏅️standards/7️⃣87a/🪆️subsets/✳️any/
//! 🦀️oracle.rs`), `subject` drives this repository's own `decode_gif`/`apply_gif_mutation`/
//! `encode_gif` round trip, and both results are read back by that same module's independent
//! `project_gif_87a` reader before the `semantic-raster-v1` profile compares them. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.
//!
//! The projection is this subset's own, not the shared `raster::project_gif`: that one reports
//! screen geometry, per-frame rectangles and an opaque-sample count only, so the Global Color
//! Table, the background-colour index, the pixel-aspect-ratio byte, the interlace flag and the raw
//! index buffers all fell outside the compared surface — five of the twelve declared kinds could
//! not move it at all.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_gif_test_oracle::standards::v87a::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_identity_round_trip, project_gif_87a};
use semio_repo_test_host::law;

//#region 🔖️Input
/// 🖼️ The document every mutation row runs on: a genuine GIF87a of 117 704 bytes, derived ONCE from
/// the real animated `💃️dancing` fixture — three real frames of it (0, 20 and 40), cropped to
/// 400×400, 400×400 and 32×32 rectangles of real already-decoded palette indices, with frame 0's real
/// 256-colour local table promoted to the file's own Global Color Table.
const INPUT: &str = "shared://🐘️dancing-87a-large/🖼️.gif";
/// 🖼️ The 2 936-byte 16×16 derivation this case used to rest on, kept for `identity-round-trip`: it
/// is the smallest genuine GIF87a committed here and the one whose whole index buffer a scenario can
/// still name literally, so nothing it proved is given up.
const SMALL_INPUT: &str = "shared://💃️dancing-87a/🖼️.gif";

/// 🧾️ `{"kind": <id>, "params": <params>}` from the scenario's own doc string.
fn spec(ctx: &Context) -> Result<Json, String> {
    ctx.doc_json()
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the reference `gif` codec and ASSERTS the result is
/// distinguishable from the untouched fixture. Every one of this vocabulary's twelve kinds reaches
/// the projection — nothing is exempt — so the exemption list is empty and stays empty: a kind that
/// stops moving it is a regression in the oracle or the projection, not a fact about GIF87a.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.input_bytes(INPUT)?;
    let forward = spec(ctx)?;
    let before = project_gif_87a(&input)?;
    let bytes = oracle_apply_mutation(&input, &forward)?;
    let projection = project_gif_87a(&bytes)?;
    law::mutation_is_observable(&forward.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ `@id-inverse`: applies the row's kind with the reference `gif` codec, applies the reference's
/// OWN computed inverse on top, and ASSERTS the semantic projection is back to the pristine
/// original's. The law is checkable without a subject, so it is checked here rather than left for
/// the parity phase: a scenario that only re-serializes and returns would pass whenever `gif` did
/// not error.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = ctx.input_bytes(INPUT)?;
    let before = project_gif_87a(&input)?;
    let forward = spec(ctx)?;
    let mutated = oracle_apply_mutation(&input, &forward)?;
    let restored = oracle_apply_mutation_inverse(&input, &forward, &mutated)?;
    let projection = project_gif_87a(&restored)?;
    law::inverse_restores(&forward.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🚫️ `@id-refuse`: an edit the stream cannot carry must be REFUSED by the reference — rejection is the passing outcome,
/// and a reference that writes the file anyway is the failure. Any other error fails the scenario too.
fn refuse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let attempt = spec(ctx)?;
    match oracle_apply_mutation(&ctx.input_bytes(INPUT)?, &attempt) {
        Err(reason) if reason.starts_with("refused: ") => Ok(Outcome::projection(Json::Object(vec![("rejected".to_string(), Json::Bool(true))]))),
        Err(reason) => Err(format!("the reference failed {} for a reason other than the raster rules: {reason}", attempt.str("kind"))),
        Ok(bytes) => Err(format!("expected {} to be refused, but the reference wrote {} byte(s)", attempt.str("kind"), bytes.len())),
    }
}

/// 🔁️ `@id-identity-round-trip`: the no-byte-pass-through law, asserted on the ORACLE side too.
/// The reference `gif` codec fully parses the real GIF87a and re-serializes it from its own model
/// alone, so the bytes must change (its own LZW writer and block layout are not the fixture's) while
/// the semantic projection must not.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let small = round_trip_oracle_once(&ctx.input_bytes(SMALL_INPUT)?)?;
    let large = round_trip_oracle_once(&ctx.input_bytes(INPUT)?)?;
    Ok(Outcome::with_raw(large.0, Json::Object(vec![("small".to_string(), small.1), ("large".to_string(), large.1)])))
}

/// 🔁️ The probe itself, over one GIF87a document.
fn round_trip_oracle_once(input: &[u8]) -> Result<(Vec<u8>, Json), String> {
    let output = oracle_identity_round_trip(input)?;
    law::reparsed_not_copied(&output, input)?;
    let before = project_gif_87a(input)?;
    let after = project_gif_87a(&output)?;
    law::round_trip_preserves(&after, &before)?;
    Ok((output, after))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{spec, INPUT, SMALL_INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_gif_test_oracle::standards::v87a::subsets::any::project_gif_87a;
    use semio_s_artifact_stdio_gif::standards::v87a::subsets::any::io::{decode_gif, encode_gif};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_gif::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::mutations::{apply_gif_mutation,GifMutation};

    use semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<GifMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️MutationFromSpec

    /// 🧫️ Copies the immutable fixture into the work directory and decodes the mutable copy through
    /// the repository's own, complete GIF87a parser.
    fn original_snapshot(ctx: &Context) -> Result<GifSnapshot, String> {
        let copy = ctx.copy_input(INPUT, Some("input.gif"))?;
        let bytes = std::fs::read(&copy).map_err(|error| error.to_string())?;
        decode_gif(&bytes)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = original_snapshot(ctx)?;
        apply_gif_mutation(&mut snapshot, &mutation_from_spec(&spec(ctx)?)?);
        let bytes = encode_gif(&snapshot)?;
        let projection = project_gif_87a(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let original = original_snapshot(ctx)?;
        let mutation = mutation_from_spec(&spec(ctx)?)?;
        let mut restored = original.clone();
        apply_gif_mutation(&mut restored, &mutation);
        for undo in mutation_inverse(&mutation, &original).expect("valid retained mutation inverse fixture") {
            apply_gif_mutation(&mut restored, &undo);
        }
        let bytes = encode_gif(&restored)?;
        let projection = project_gif_87a(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🚫️ `@id-refuse`: the vocabulary itself refuses the edit with `mutation.target-mismatch` and leaves the snapshot as
    /// it was; an edit that applies, or a refusal by another code, fails the scenario.
    pub fn refuse(ctx: &Context) -> Result<Outcome, String> {
        let original = original_snapshot(ctx)?;
        let mut attempted = original.clone();
        match apply_mutation_checked(&mut attempted, &mutation_from_spec(&spec(ctx)?)?) {
            Err(refusal) if refusal.code == "mutation.target-mismatch" && attempted == original => Ok(Outcome::projection(Json::Object(vec![("rejected".to_string(), Json::Bool(true))]))),
            Err(refusal) => Err(format!("refused as {refusal}, not by the raster rules' mutation.target-mismatch")),
            Ok(()) => Err("expected the edit to be refused, but it applied".to_string()),
        }
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let small = round_trip_once(ctx, SMALL_INPUT, "small-input.gif")?;
        let large = round_trip_once(ctx, INPUT, "input.gif")?;
        Ok(Outcome::with_raw(large.0, Json::Object(vec![("small".to_string(), small.1), ("large".to_string(), large.1)])))
    }

    /// 🔁️ The probe itself, over one GIF87a document.
    fn round_trip_once(ctx: &Context, uri: &str, name: &str) -> Result<(Vec<u8>, Json), String> {
        let copy = ctx.copy_input(uri, Some(name))?;
        let input = std::fs::read(&copy).map_err(|error| error.to_string())?;
        let snapshot = decode_gif(&input)?;
        let output = encode_gif(&snapshot)?;
        if output == input {
            return Err(format!("byte pass-through on {uri}: output is bit-identical to the input"));
        }
        let projection = project_gif_87a(&output)?;
        Ok((output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("refuse", refuse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("refuse", subject::refuse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
