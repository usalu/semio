//! 🦀️ RFC1950 mutation case — Rust adapter.
//!
//! Every scenario copies one of the two real, committed zlib fixtures into the case work directory
//! first; the committed fixtures are never written to. Every `{kind, params}` witness carries the
//! leaf's own wire payload. `oracle` drives the registered `flate2` reference implementation through
//! this subset's own oracle module, which reads that wire by field name; `subject` decodes it
//! generically through `Mutation::from_payload_value` and drives this repository's own
//! `apply_deflate_mutation`/`decode_deflate_snapshot`/`encode_deflate_snapshot`, undoing it with the
//! vocabulary's own `Mutation::inverse`. Both results are read back by the INDEPENDENT `flate2`
//! projection before the `ordered-json-v1` profile compares them. The subject half is gated behind
//! the generated host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_deflate_test_oracle::standards::v_rfc1950::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_deflate};
use semio_repo_test_host::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const MUTATE_INPUT: &str = "shared://🗜️readme-level9.zz";
const IDENTITY_INPUT: &str = "shared://🪶️readme-level1.zz";

/// 🧫️ Copies the immutable fixture into the work directory and returns its bytes.
fn mutable_input(ctx: &Context, uri: &str, name: &str) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(uri, Some(name))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_deflate(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_deflate(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies the row's forward mutation, then the reference's own algebraic undo of it (computed
/// from the ORIGINAL header/payload by the independent reader alone), and asserts the restoration
/// against the ORIGINAL document's own projection before the oracle-vs-subject comparison.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&input, &spec)?)?;
    let projection = project_deflate(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_deflate(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role. The semantic half: inflating and
/// re-deflating must leave the typed header fields and the payload digest exactly where they were.
/// The no-byte-pass-through half: this scenario deliberately reads the LEVEL-1 fixture while the
/// reference re-compresses at `flate2`'s own default level, so an output equal to the input would
/// mean the DEFLATE stream was copied rather than genuinely inflated and re-coded.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx, IDENTITY_INPUT, "input.zz")?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_deflate(&bytes)?;
    round_trip_preserves(&projection, &project_deflate(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, IDENTITY_INPUT, MUTATE_INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::io::{decode_deflate_snapshot, encode_deflate_snapshot};
    use semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::schema::mutations::apply_deflate_mutation;
    use semio_s_artifact_stdio_deflate::{mutation_from_payload_json, mutation_inverse, mutation_payload_json, DeflateMutation, DeflateSnapshot};
    use semio_s_artifact_stdio_deflate_test_oracle::standards::v_rfc1950::subsets::any::project_deflate;
    use semio_repo_test_host::law::wire_operation;

    /// 🦠️ The scenario's `{kind, params}` witness decoded generically: `params` IS the leaf's wire
    /// payload, so the derive-generated `from_payload_value` is the only decoder, and re-emitting
    /// the decoded payload must give back exactly `params`.
    fn mutation_from_spec(spec: &Json) -> Result<DeflateMutation, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        wire_operation(&kind, &params, mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(input: &[u8]) -> Result<DeflateSnapshot, String> {
        decode_deflate_snapshot(input).map_err(|error| format!("decode_deflate_snapshot failed: {error}"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
        let mut snapshot = decode(&input)?;
        apply_deflate_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let bytes = encode_deflate_snapshot(&snapshot);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_deflate(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The forward witness is undone by `DeflateMutation::inverse` itself — the law under test —
    /// and the restored stream must project exactly onto the original's projection.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
        let original = decode(&input)?;
        let mutation = mutation_from_spec(&spec)?;
        let mut restored = original.clone();
        apply_deflate_mutation(&mut restored, &mutation);
        if encode_deflate_snapshot(&restored) == input {
            return Err("byte pass-through: mutated output is bit-identical to the input".to_string());
        }
        for step in mutation_inverse(&mutation, &original).expect("valid retained mutation inverse fixture") {
            apply_deflate_mutation(&mut restored, &step);
        }
        let restored_bytes = encode_deflate_snapshot(&restored);
        let projection = project_deflate(&restored_bytes)?;
        if projection != project_deflate(&input)? {
            return Err(format!("inverse of {} did not restore the subject's original semantic projection", spec.str("kind")));
        }
        Ok(Outcome::with_raw(restored_bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx, IDENTITY_INPUT, "input.zz")?;
        let bytes = encode_deflate_snapshot(&decode(&input)?);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_deflate(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
