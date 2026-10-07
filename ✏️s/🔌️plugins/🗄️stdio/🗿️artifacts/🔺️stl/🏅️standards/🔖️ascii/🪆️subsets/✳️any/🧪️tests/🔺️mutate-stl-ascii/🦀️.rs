//! 🦀️ STL ascii mutation case — Rust adapter.
//!
//! Every scenario copies the real, derived-once fixture into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `stl_io` reference
//! implementation (`../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🦀️oracle.rs`), `subject`
//! drives this repository's own decode/apply/encode round trip, and both results are read back by
//! the independent `stl_io` reader before the `semantic-mesh-v1` profile compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_stl_test_oracle::standards::v_ascii::subsets::any::{oracle_apply_mutation, oracle_document_projection, oracle_inverse_spec, oracle_round_trip};
use semio_s_plugin_stdio_mesh_test_oracle::project_stl;
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within, round_trip_preserves_within};


//#region 🔖️Profile
/// 📏️ `semantic-stl-ascii-v1`'s own declared tolerances (`../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/
/// 🔮️oracles/🔣️.json`), mirrored here so an in-handler law check is exactly as strict as the
/// profile the case is measured by — never stricter, which would invent a failure the comparison
/// itself would forgive.
const STL_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "precision"];
const STL_TOLERANCE: f64 = 1e-5;
//#endregion 🔖️Profile

//#region 🔖️Projection
/// 🔍️ The projection both roles are compared through: `stl_io`'s resolved triangle soup, plus the
/// `solid <name>` header and the EXPLICIT per-facet normals it cannot carry. Without that second
/// half `set-solid-name` and `set-triangle-normal` — 2 of the 6 declared kinds — leave the
/// projection exactly as they found it, and their scenarios measure nothing. Both halves are read
/// independently of `decode_stl_ascii`.
fn project(bytes: &[u8]) -> Result<Json, String> {
    let mut projection = project_stl(bytes)?;
    match &mut projection {
        Json::Object(members) => members.push(("document".to_string(), oracle_document_projection(bytes)?)),
        other => return Err(format!("the mesh reader returned {other:?} rather than an object")),
    }
    Ok(projection)
}

/// 👁️ All 6 declared kinds have to move that composed projection — none is exempt, which is exactly
/// what the solid name and the facet normals were added to it to make true.
fn moved_the_document(kind: &str, mutated: &Json, base: &Json) -> Result<(), String> {
    mutation_is_observable_within(kind, mutated, base, &[], STL_WRITER_FREEDOM, STL_TOLERANCE)
}
//#endregion 🔖️Projection

//#region 🔖️Input
const INPUT: &str = "shared://🏛️hexagonal-cut-concrete-forest-left/🧊️.stl";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("📥️input.stl"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with observability asserted in role: the reference applies the kind to the
/// real solid and the result has to differ from the untouched document under the very profile the
/// case is measured by.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project(&bytes)?;
    moved_the_document(&spec.str("kind"), &projection, &project(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies `<id>` forward, then its independently computed inverse — both against the SAME
/// untouched `input`, matching `StlMutation::inverse()`'s own base-relative semantics — and ASSERTS
/// the law here rather than deferring it to the parity phase: the restored solid's independent
/// `stl_io` projection must equal the REAL original's own, within `semantic-mesh-v1`'s own declared
/// tolerance and no stricter. Without the check the scenario would pass for any inverse `stl_io`
/// merely tolerated.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = match oracle_inverse_spec(&input, &spec)? {
        Some(inverse_spec) => oracle_apply_mutation(&mutated, &inverse_spec)?,
        None => mutated,
    };
    let projection = project(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project(&input)?, STL_WRITER_FREEDOM, STL_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: the solid must survive the decode/re-encode
/// unchanged — name, facet normals and corners alike — and the output must not be the input back
/// again. `stl_io` resolves every coordinate through `f32` while the committed fixture carries the
/// `f64` decimals its GLB derivation produced, so a bit-identical result could only be a copy.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: oracle output is bit-identical to the input".to_string());
    }
    let projection = project(&bytes)?;
    round_trip_preserves_within(&projection, &project(&input)?, STL_WRITER_FREEDOM, STL_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{moved_the_document, mutable_input, project};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::io::{decode_stl_ascii, encode_stl_ascii};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_stl::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::schema::mutations::{apply_stl_mutation,StlMutation};

    use semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::schema::snapshot::StlSnapshot;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.
    fn mutation_of(spec: &Json) -> Result<StlMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(ctx: &Context) -> Result<StlSnapshot, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| error.to_string())?;
        decode_stl_ascii(&text).map_err(|error| format!("decode_stl_ascii failed: {error}"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        let spec = ctx.doc_json()?;
        apply_stl_mutation(&mut snapshot, &mutation_of(&spec)?);
        let bytes = encode_stl_ascii(&snapshot).into_bytes();
        let projection = project(&bytes)?;
        moved_the_document(&spec.str("kind"), &projection, &project(&mutable_input(ctx)?)?)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        let forward = mutation_of(&ctx.doc_json()?)?;
        let backward = mutation_inverse(&forward, &snapshot).expect("valid retained mutation inverse fixture");
        apply_stl_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_stl_mutation(&mut snapshot, mutation);
        }
        let bytes = encode_stl_ascii(&snapshot).into_bytes();
        let projection = project(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(ctx)?;
        let bytes = encode_stl_ascii(&snapshot).into_bytes();
        if bytes == input {
            return Err("byte pass-through: subject output is bit-identical to the input".to_string());
        }
        let projection = project(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
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
