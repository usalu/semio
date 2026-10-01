//! 🦀️ PPTX ECMA-376/🧱️base exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
//!
//! Every scenario copies the real, committed `📽️.pptx` fixture (a real 7-slide subset
//! derived once from a real 62-slide, 16 MB 2020 conference deck — see the feature file's own
//! header for the full provenance) into the case work directory first; the committed fixture is
//! never written to. `oracle` drives the registered `zip`+`quick-xml` composition
//! (`../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🦀️oracle.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_pptx`/`encode_pptx`/`apply_pptx_mutation` over the full 9-kind `PptxMutation`
//! vocabulary. Both results are read back by the SAME independent `project_pptx_mutation` (the
//! `zip`+`quick-xml` composition) before the `semantic-pptx-mutate-v1` profile compares them. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! links `semio-s-plugin-stdio` -- §5.3's own role separation, NOT a workaround for anything: the
//! Rust subject phase runs, and wave 14 ran the full differential comparison against the oracle.
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law`
//! module, so a scenario cannot pass merely because `zip`+`quick-xml` declined to error:
//! `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>` must land back on the
//! untouched deck's projection, and `identity-round-trip` must both preserve the projection and
//! rebuild an archive that differs from the input. There is no carve-out of any kind: the profile
//! declares no writer freedom, no kind is exempt from observability, and no axis — slide order
//! included — is dropped from the inverse law.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pptx::standards::v_ecma_376::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_pptx_mutation};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://📽️.pptx";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("semio-talk.pptx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: the reference composition
/// applies the kind to the real seven-slide deck and the result has to differ from the untouched
/// presentation. Returning the projection uncompared is what made these nine scenarios pass whenever
/// `zip`+`quick-xml` merely did not error. NOTHING is exempt — every declared kind is defined on the
/// ordered slide list or on a shape inside it, which is exactly what `semantic-pptx-mutate-v1`
/// reports, and that profile declares no writer freedom at all (`ignoreKeys: []`).
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pptx_mutation(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_pptx_mutation(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The INVERSE law, asserted in role without needing the subject: `apply(inverse(m), apply(m,
/// base))` must land back on the ORIGINAL presentation's own projection, read through the same
/// independent reader. No axis is dropped and no tolerance is allowed — slide ORDER included, which
/// is the whole point of a vocabulary that declares `move-slide`.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_pptx_mutation(&bytes)?;
    inverse_restores(&kind, &projection, &project_pptx_mutation(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity law, both halves asserted: the `zip`+`quick-xml` composition
/// unzips the real deck, parses every slide, regenerates every slide-related OPC part from its own
/// typed slide/shape list and rezips — the same rebuild every kind goes through. The result must
/// differ from the input, because two independent writers agree on neither compression nor part
/// layout, and the projection must survive intact.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_pptx_mutation(&bytes)?;
    round_trip_preserves(&projection, &project_pptx_mutation(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx;
    use semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx;
    use semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::schema::mutations::apply_pptx_mutation;
    use semio_s_artifact_stdio_pptx::{mutation_from_payload_json, mutation_inverse, mutation_payload_json, PptxMutation, PptxSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::pptx::standards::v_ecma_376::subsets::base::project_pptx_mutation;
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;

    /// 🦠️ The scenario's `{kind, params}` witness decoded generically: `params` IS the leaf's wire
    /// payload, so the derive-generated `from_payload_value` is the only decoder, and re-emitting the
    /// decoded payload must give back exactly `params`.
    fn mutation_from_spec(spec: &Json) -> Result<PptxMutation, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        wire_operation(&kind, &params, mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(input: &[u8]) -> Result<PptxSnapshot, String> {
        decode_pptx(input).map_err(|error| format!("decode_pptx failed: {error}"))
    }

    fn encode(snapshot: &PptxSnapshot) -> Result<Vec<u8>, String> {
        encode_pptx(snapshot).map_err(|error| format!("encode_pptx failed: {error}"))
    }

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode(&input)?;
        apply_pptx_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let bytes = encode(&snapshot)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".into());
        }
        let projection = project_pptx_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The forward witness is undone by `PptxMutation::inverse` itself — the vocabulary's own
    /// algebra is the law under test, never a transcription of it.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = mutation_inverse(&mutation, &base);
        let mut snapshot = base;
        apply_pptx_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_pptx_mutation(&mut snapshot, step);
        }
        let bytes = encode(&snapshot)?;
        let projection = project_pptx_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let output = encode(&decode(&input)?)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_pptx_mutation(&output)?;
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
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
