//! 🦀️ PPTX ECMA-376 🌉️transitional exhaustive conformance-class mutation case — Rust adapter.
//!
//! The input is the real committed `📽️.pptx`, a 55-part OPC package that ALREADY
//! satisfies this class on all three of its axes, with BOTH of its Transitional namespace families
//! declared — PresentationML on the presentation and slide parts, DrawingML inside their shape
//! trees. This case is the mirror of `🔒️mutate-pptx-ecma-376-strict` over the same bytes: seven kinds
//! move the deck OUT of the ISO/IEC 29500-4 class and back in, two of them along namespace families
//! that are addressable independently and that no DOCX or XLSX conformance subset has. `oracle`
//! handlers drive the registered `quick-xml` 0.42 + `zip` 6 pair through this subset's own
//! `../../🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🦀️oracle.rs`; `subject`
//! handlers drive `decode_pptx`/`apply_pptx_transitional_mutation`/`encode_pptx`; both results are
//! read back by the SAME independent `project_package` before
//! `semantic-ooxml-pptx-transitional-v1` compares them. The subject half is `sut`-gated so the
//! oracle-only run never compiles the local implementation.
//!
//! ⚖️ All three laws are asserted IN ROLE through the shared `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law`
//! module, under a profile that declares no writer freedom at all, and no kind is exempt from any
//! of them. Only ONE kind — `remove-conformance-attribute` — needs an arranged pre-state, two fewer
//! than the 🔒️strict sibling. The evidence stops where ISO/IEC 29500-4 does: VML and
//! `mc:AlternateContent` are legal Transitional markup, so this case polices neither.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_pptx_test_oracle::standards::v_ecma_376::subsets::transitional::{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, oracle_stamp, project_package};
use semio_repo_test_host::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://📽️.pptx";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.pptx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's mutation runs on. Only `remove-conformance-attribute` needs
/// one: no ECMA-376 package in this repository carries a conformance attribute, verified by
/// unzipping all three committed OOXML fixtures, so that kind runs on the real 55-part deck after
/// the reference implementation has independently stamped one onto `ppt/presentation.xml`. The
/// other six kinds read the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: each kind has to move the
/// three-axis Transitional projection of the real deck, or its scenario would pass whether or not
/// the mutation ran. Nothing is exempt — including `set-drawing-namespace`, which moves the second
/// namespace family on its own and would otherwise be the easiest kind here to leave unobserved.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let output = oracle_apply_mutation(&base, &spec)?;
    let projection = project_package(&output)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_package(&base)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The INVERSE law, asserted in role and against the ARRANGED pre-state rather than the committed
/// bytes: for `remove-conformance-attribute` the deck to be restored is the stamped one, which is
/// the only baseline that removal can honestly be undone onto.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let undo = oracle_inverse_spec(&base, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_package(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_package(&base)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: `zip` re-reads all 55 entries — the 3 media
/// binaries included — and rebuilds the container from those entries alone, so the rebuilt package
/// must differ from the input while its conformance-class projection must not move at all.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    reparsed_not_copied(&output, &input)?;
    let projection = project_package(&output)?;
    round_trip_preserves(&projection, &project_package(&input)?)?;
    Ok(Outcome::with_raw(output, projection))
}
/// 🏅️ The reference half of the whole-package class stamp (`set-snapshot` of the package's own strict stamp): the
/// independent engine stamps the real package strict, and the conformance-class projection has to move.
fn stamp_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_stamp(&input, true)?;
    let projection = project_package(&output)?;
    mutation_is_observable("set-snapshot", &projection, &project_package(&input)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The class stamp undone by the reference's own stamp back, which must restore the real package's projection exactly.
fn stamp_inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let restored = oracle_stamp(&oracle_stamp(&input, true)?, false)?;
    let projection = project_package(&restored)?;
    inverse_restores("set-snapshot", &projection, &project_package(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx;
    use semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx;
    use semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::transitional::schema::mutations::{apply_pptx_transitional_mutation, stamp_conformance_class_mutation, PptxTransitionalMutation};
    use semio_s_artifact_stdio_pptx::{mutation_from_payload_json, mutation_inverse, mutation_payload_json, PptxSnapshot};
    use semio_s_artifact_stdio_pptx_test_oracle::standards::v_ecma_376::subsets::transitional::project_package;
    use semio_repo_test_host::law::wire_operation;

    fn decode(bytes: &[u8]) -> Result<PptxSnapshot, String> {
        decode_pptx(bytes).map_err(|error| error.to_string())
    }

    fn encode(snapshot: &PptxSnapshot) -> Result<Vec<u8>, String> {
        encode_pptx(snapshot).map_err(|error| error.to_string())
    }

    fn outcome_of(snapshot: &PptxSnapshot) -> Result<Outcome, String> {
        let output = encode(snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🦠️ The scenario's `{kind, params}` witness decoded generically: `params` IS the leaf's wire payload, so the
    /// derive-generated `from_payload_value` is the only decoder, and re-emitting the decoded payload must give back
    /// exactly `params`.
    fn mutation_from_spec(spec: &Json) -> Result<PptxTransitionalMutation, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        wire_operation(&kind, &params, mutation_from_payload_json, mutation_payload_json)
    }

    /// ↩️ Applies `mutation` to `base` and then `PptxTransitionalMutation::inverse` of it — the vocabulary's own algebra is the law
    /// under test, never a transcription of it.
    fn applied_and_undone(base: PptxSnapshot, mutation: &PptxTransitionalMutation) -> PptxSnapshot {
        let undo = mutation_inverse(mutation, &base).expect("valid retained mutation inverse fixture");
        let mut snapshot = base;
        apply_pptx_transitional_mutation(&mut snapshot, mutation);
        for step in &undo {
            apply_pptx_transitional_mutation(&mut snapshot, step);
        }
        snapshot
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        apply_pptx_transitional_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        outcome_of(&snapshot)
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = decode(&arranged_input(ctx, &spec)?)?;
        outcome_of(&applied_and_undone(base, &mutation_from_spec(&spec)?))
    }

    /// 🏅️ The whole-package class stamp recorded as one `set-snapshot` of this repository's own stamp.
    pub fn stamp(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let stamp = stamp_conformance_class_mutation(&snapshot, true);
        apply_pptx_transitional_mutation(&mut snapshot, &stamp);
        outcome_of(&snapshot)
    }

    /// ↩️ The class stamp undone by `set-snapshot`'s own inverse.
    pub fn stamp_inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode(&mutable_input(ctx)?)?;
        let stamp = stamp_conformance_class_mutation(&base, true);
        outcome_of(&applied_and_undone(base, &stamp))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let output = encode(&decode(&input)?)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. The class stamp is the `set-snapshot` kind's own plain scenario
/// pair, registered under its exact ids next to the outline bases.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("mutate-set-snapshot", stamp_oracle).oracle("inverse-set-snapshot", stamp_inverse_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built
            .subject("mutate", subject::mutate)
            .subject("inverse", subject::inverse)
            .subject("mutate-set-snapshot", subject::stamp)
            .subject("inverse-set-snapshot", subject::stamp_inverse)
            .subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
