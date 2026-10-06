//! 🦀️ LAS 1.0 mutation case — Rust adapter.
//!
//! Every scenario copies the real, derived-once 8,448-point fixture into the case work directory
//! first; the committed fixture is never written to. `oracle` drives the registered `las` 0.11
//! reference implementation (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/🦀️oracle.rs`),
//! `subject` drives this repository's own decode/apply/encode round trip, and both results are read
//! back by the SAME independent `project_las` (built on `las::raw::{Header, Vlr, Point}`) before the
//! `semantic-las-v1` profile compares them. The subject half is gated behind the generated host's
//! `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_las_test_oracle::standards::v1_0::subsets::header::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_las};
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within, round_trip_preserves_within};


//#region 🔖️Profile
/// 📏️ `semantic-las-v1`'s own declared tolerance (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/
/// 🔣️oracle.json`) — the scale/offset quantization LAS 1.0 stores coordinates through.
/// Mirrored here so an in-handler law check is exactly as strict as the profile the case is
/// measured by, never stricter.
const LAS_TOLERANCE: f64 = 1e-3;
//#endregion 🔖️Profile

//#region 🔖️Input
const INPUT: &str = "shared://🧪️pattern-sphere/🧊️.las";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("input.las"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_las(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_las(&input)?, &[], &[], LAS_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies `<id>` forward, then its independently computed inverse — both against the SAME
/// untouched `input`, matching `LasMutation::inverse()`'s own base-relative semantics — and ASSERTS
/// the law here rather than deferring it to the parity phase: the restored cloud's independent
/// `las::raw` projection must equal the REAL original's own, within `semantic-las-v1`'s own declared
/// tolerance (the scale/offset quantization) and no stricter. Without the check the scenario would
/// pass for any inverse the `las` crate merely tolerated.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = match oracle_inverse_spec(&input, &spec)? {
        Some(inverse_spec) => oracle_apply_mutation(&mutated, &inverse_spec)?,
        None => mutated,
    };
    let projection = project_las(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_las(&input)?, &[], LAS_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: the header, VLRs and every point must survive
/// the decode/re-encode unchanged, and the output must not be the input bytes back — the reference
/// re-derives the whole header block and point record layout, so bit-identical output would mean
/// nothing was parsed.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: oracle output is bit-identical to the input".to_string());
    }
    let projection = project_las(&bytes)?;
    round_trip_preserves_within(&projection, &project_las(&input)?, &[], LAS_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_las::standards::v1_0::subsets::any::io::{decode_las, encode_las};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_las::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_las::standards::v1_0::subsets::any::schema::mutations::{apply_las_mutation, LasMutation};
    use semio_s_artifact_stdio_las::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;
    use semio_s_artifact_stdio_las_test_oracle::standards::v1_0::subsets::header::project_las;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.
    fn mutation_of(spec: &Json) -> Result<LasMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(ctx: &Context) -> Result<LasSnapshot, String> {
        decode_las(&mutable_input(ctx)?).map_err(|error| format!("decode_las failed: {error}"))
    }

    fn outcome(snapshot: &LasSnapshot) -> Result<Outcome, String> {
        let output = encode_las(snapshot).map_err(|error| format!("encode_las failed: {error}"))?;
        let projection = project_las(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        apply_las_mutation(&mut snapshot, &mutation_of(&ctx.doc_json()?)?);
        outcome(&snapshot)
    }

    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        let forward = mutation_of(&ctx.doc_json()?)?;
        let backward = mutation_inverse(&forward, &snapshot).expect("valid retained mutation inverse fixture");
        apply_las_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_las_mutation(&mut snapshot, mutation);
        }
        outcome(&snapshot)
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let output = encode_las(&decode(ctx)?).map_err(|error| format!("encode_las failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: subject output is bit-identical to the input".to_string());
        }
        let projection = project_las(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
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
