//! 🦀️ AVI 1.0 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `🎬️.avi` fixture (derived once
//! from this repository's only real video — see the feature file's own header) into the case work
//! directory first; the committed fixture is never written to. `oracle` drives the registered
//! independent `riff`-composed codec
//! (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️base/🦀️oracle.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_avi`/`encode_avi`/`apply_avi_mutation` over the full 12-kind `AviMutation` vocabulary.
//! Both results are read back by the SAME independent `project_avi_1_0` before the
//! `semantic-avi-v1` profile compares them. The subject half is gated behind the generated host's
//! `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::avi::standards::v1_0::subsets::hdrl::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_identity_round_trip, project_avi_1_0};
use semio_s_plugin_stdio_test_oracle::law;


//#region 🔖️Input
const INPUT: &str = "shared://🎬️.avi";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bauen-mit-bestand-mjpeg.avi"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, `params` being the leaf's wire payload. It applies the row's
/// kind with the registered reference implementation and ASSERTS the result is distinguishable from
/// the untouched fixture. The exemption list is empty — every kind this vocabulary declares reaches
/// the compared projection — so a kind that stops moving it fails here rather than reporting a green
/// identical to an unchanged container's.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_avi_1_0(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_avi_1_0(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id. `oracle_apply_mutation_inverse`
/// applies the kind and then its OWN independently computed inverse; this handler is what its doc
/// comment always said the caller does -- it ASSERTS the result projects back onto the pristine
/// original. The law needs no subject, so leaving it to the parity phase would make the scenario
/// pass whenever the reference `riff` composition merely did not error.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_avi_1_0(&input)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_avi_1_0(&bytes)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED here and not merely described: the
/// independent `riff` composition fully parses the real video container and re-serializes it from
/// its own model alone, so its output must differ from the input byte-wise -- our writer cannot
/// reproduce another muxer's padding and chunk layout -- while projecting onto exactly the same semantics.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&input)?;
    law::reparsed_not_copied(&bytes, &input)?;
    let before = project_avi_1_0(&input)?;
    let projection = project_avi_1_0(&bytes)?;
    law::round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::io::{decode_avi, encode_avi};
    use semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::schema::mutations::{apply_avi_mutation, decode_avi_mutation_payload, inverse_avi_mutation, AviMutation};
    use semio_s_plugin_stdio_test_oracle::artifacts::avi::standards::v1_0::subsets::hdrl::project_avi_1_0;

    //#region 🔖️SpecCodec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<AviMutation, String> {
        decode_avi_mutation_payload(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_avi(&mutable_input(ctx)?).map_err(|error| format!("decode_avi failed: {error}"))?;
        apply_avi_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let bytes = encode_avi(&snapshot);
        let projection = project_avi_1_0(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Applies the row's mutation, then every mutation the vocabulary's own inverse returns against the
    /// pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_avi(&mutable_input(ctx)?).map_err(|error| format!("decode_avi failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base.clone();
        apply_avi_mutation(&mut snapshot, &mutation);
        for undo in inverse_avi_mutation(&mutation, &base) {
            apply_avi_mutation(&mut snapshot, &undo);
        }
        let bytes = encode_avi(&snapshot);
        let projection = project_avi_1_0(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_avi`/`encode_avi` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_avi(&input).map_err(|error| format!("decode_avi failed: {error}"))?;
        let output = encode_avi(&snapshot);
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_avi_1_0(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
