//! 🦀️ SVG 1.1/🔰️basic exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! Every scenario copies the real, committed `🐁️mouse.svg` fixture into the case work directory
//! first; the committed asset is never written to. `oracle` drives the registered `quick-xml`
//! reference implementation through this subset's own oracle module
//! (`../../🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🦀️oracle.rs`); `subject` drives this
//! repository's own `SvgSnapshot::import_utf8`/`export_utf8` and `apply_svg_basic_mutation` over the
//! full 10-kind `SvgBasicMutation` vocabulary. Both results are read back by the SAME independent
//! `project_svg_basic` before the `semantic-svg-basic-1-1-v1` profile compares them. The subject half
//! is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::basic::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_svg_basic};


//#region 🔖️Input
/// 🎨️ The document every mutation row runs on: one SVG composed ONCE, body for body, out of the
/// repository's two real committed drawings — its real animated brand logo and the real onboarding
/// mouse, the only committed SVG that declares a `<clipPath>` — by `🐍️derive-svg-basic-fixture.py`
/// in the ticket folder. 138 219 bytes, 63 root children, 23 real groups, 27 real paths, 138 real
/// animation elements and the real clip path the profile's own rule needs.
const INPUT: &str = "shared://🎨️semio-brand-and-onboarding.svg";
/// 🖱️ The onboarding mouse on its own, kept for `identity-round-trip`: it is the drawing the
/// framework's UI really renders, and reading it where it is committed keeps that tie.
const MOUSE_INPUT: &str = "shared://🐁️mouse.svg";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("brand-and-onboarding.svg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🧫️ The same, for the mouse drawing the round-trip scenario additionally reads.
fn mutable_mouse_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(MOUSE_INPUT, Some("mouse.svg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind whose parameters
/// leave the semantic projection exactly where it was has not been tested by this scenario at all
/// -- it proves only that the reference library declined to error.
/// Every `Examples` row is chosen against the real artifact's actual content for that reason, and
/// this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_svg_basic(&bytes)?;
    if projection_divergence(&projection, &project_svg_basic(&input)?).is_none() {
        return Err(format!("{kind:?} left the semantic projection exactly as it found it -- a mutation whose parameters make it a no-op against the real artifact is not a test of that kind"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ⚖️ First point at which two projections diverge, as a character offset into the canonical
/// rendering plus the window around it on both sides -- an equality check whose failure names WHAT
/// changed rather than only that something did.
fn projection_divergence(restored: &Json, original: &Json) -> Option<String> {
    let (left, right): (Vec<char>, Vec<char>) = (restored.to_string().chars().collect(), original.to_string().chars().collect());
    if left == right {
        return None;
    }
    let at = left.iter().zip(right.iter()).position(|(a, b)| a != b).unwrap_or(left.len().min(right.len()));
    let window = |text: &[char]| text.iter().skip(at.saturating_sub(60)).take(160).collect::<String>();
    Some(format!("first divergence at char {at} of {} vs {} -- got …{}… want …{}…", left.len(), right.len(), window(&left), window(&right)))
}

/// ↩️ The inverse law, checked on the ORACLE side rather than deferred to the parity phase: the
/// reference implementation applies the kind and then its own computed inverse, and the restored
/// document's independent projection must equal the REAL original's own. Without this the scenario
/// would only prove that the inverse ran without erroring, which is not what `@mode-property`
/// claims — and with the subject phase blocked, it is the only place the property can be checked
/// today.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation_inverse(&input, &ctx.doc_json()?)?;
    let projection = project_svg_basic(&bytes)?;
    let original = project_svg_basic(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: the restored drawing's projection differs from the original's -- {divergence}"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `quick-xml`
/// fully parses the real drawing and re-serializes it from its own element tree alone, so BOTH
/// halves of the law are checkable here without a subject -- the re-encoded bytes must differ from
/// the input (SVG 1.1 🔰️basic is XML, not a byte-preserving carrier: the writer re-derives every
/// tag, quote and escape from the tree, so bit-identity would prove the artifact was copied rather
/// than parsed), and the re-encoded drawing's own projection must still equal the input's.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let mouse = round_trip_oracle_once(&mutable_mouse_input(ctx)?, "the onboarding mouse")?;
    let composed = round_trip_oracle_once(&mutable_input(ctx)?, "the composed drawing")?;
    Ok(Outcome::with_raw(composed.0, Json::Object(vec![("mouse".to_string(), mouse.1), ("composed".to_string(), composed.1)])))
}

/// 🔁️ The probe itself, over one drawing.
fn round_trip_oracle_once(input: &[u8], what: &str) -> Result<(Vec<u8>, Json), String> {
    let bytes = oracle_round_trip(input)?;
    if bytes == input {
        return Err(format!("byte pass-through on {what}: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the drawing was parsed rather than copied"));
    }
    let projection = project_svg_basic(&bytes)?;
    let original = project_svg_basic(input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated on {what}: decode then re-encode did not preserve the semantic projection -- {divergence}"));
    }
    Ok((bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::basic::schema::mutations::{apply_svg_basic_mutation, decode_svg_basic_mutation_payload_json, inverse_svg_basic_mutation, SvgBasicMutation};
    use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::basic::project_svg_basic;

    //#region 🔖️SpecCodec
    /// 📄️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<SvgBasicMutation, String> {
        decode_svg_basic_mutation_payload_json(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    fn base_snapshot(ctx: &Context) -> Result<SvgSnapshot, String> {
        SvgSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))
    }

    fn outcome_of(snapshot: &SvgSnapshot) -> Result<Outcome, String> {
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_svg_basic(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base;
        apply_svg_basic_mutation(&mut snapshot, &mutation);
        outcome_of(&snapshot)
    }

    /// ↩️ The subset's OWN `Mutation::inverse`, reached through the typed vocabulary rather than
    /// re-derived here, so the property under test is the implementation's algebra and not a
    /// transcription of it.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_svg_basic_mutation(&mutation, &base);
        let mut snapshot = base;
        apply_svg_basic_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_svg_basic_mutation(&mut snapshot, step);
        }
        outcome_of(&snapshot)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let mouse = round_trip_once(&super::mutable_mouse_input(ctx)?, "the onboarding mouse")?;
        let composed = round_trip_once(&mutable_input(ctx)?, "the composed drawing")?;
        Ok(Outcome::with_raw(composed.0, Json::Object(vec![("mouse".to_string(), mouse.1), ("composed".to_string(), composed.1)])))
    }

    /// 🔁️ The probe itself, over one drawing.
    fn round_trip_once(input: &[u8], what: &str) -> Result<(Vec<u8>, Json), String> {
        let snapshot = SvgSnapshot::import_utf8(input).map_err(|error| format!("import_utf8 of {what} failed: {error}"))?;
        let output = snapshot.export_utf8().map_err(|error| format!("export_utf8 of {what} failed: {error}"))?;
        if output == input {
            return Err(format!("byte pass-through on {what}: output is bit-identical to the input"));
        }
        let projection = project_svg_basic(&output)?;
        Ok((output, projection))
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
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
