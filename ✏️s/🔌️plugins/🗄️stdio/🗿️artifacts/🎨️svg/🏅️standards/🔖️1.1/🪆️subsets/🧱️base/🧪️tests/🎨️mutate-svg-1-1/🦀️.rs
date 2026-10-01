//! 🦀️ SVG 1.1 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `🔳️qr-code.svg` fixture into the case work directory
//! first; the committed asset is never written to. `oracle` drives the registered `quick-xml`
//! reference implementation (`../../🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🦀️oracle.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `SvgSnapshot::import_utf8`/`export_utf8`/`apply_svg_mutation` over the full 11-kind `SvgMutation`
//! vocabulary. Both results are read back by the SAME independent `project_svg_1_1` (`quick-xml`)
//! before the `semantic-svg-1-1-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_svg_1_1};


//#region 🔖️Input
const INPUT: &str = "shared://🔳️qr-code.svg";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("qr-code.svg"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
/// 👁️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind whose parameters
/// leave the semantic projection exactly where it was has not been tested by this scenario at all
/// -- it proves only that the reference library declined to error. Every `Examples` row is chosen against the real artifact's actual content for that reason, and
/// this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_svg_1_1(&bytes)?;
    if projection_divergence(&projection, &project_svg_1_1(&input)?).is_none() {
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

/// ↩️ The inverse law, ASSERTED on the ORACLE side rather than deferred to the parity phase:
/// `quick-xml` applies the kind and then its own computed inverse, and the restored drawing's
/// independent projection must equal the REAL original's own. Without this the scenario would only
/// prove that the reference library did not error, which is not what `@mode-property` claims -- and
/// the subject handler asserts the same law on its own side, so neither side can be vacuous.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_svg_1_1(&bytes)?;
    let original = project_svg_1_1(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: {:?} followed by its own inverse did not restore the original drawing's projection -- {divergence}", spec.str("kind")));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `quick-xml`
/// fully parses the real drawing and re-serializes it from its own element tree alone, so BOTH
/// halves of the law are checkable here without a subject -- the re-encoded bytes must differ from
/// the input (SVG 1.1 is XML, not a byte-preserving carrier: the writer re-derives every tag, quote
/// and escape from the tree, so bit-identity would prove the artifact was copied rather than
/// parsed), and the re-encoded drawing's own projection must still equal the input's.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the drawing was parsed rather than copied".to_string());
    }
    let projection = project_svg_1_1(&bytes)?;
    let original = project_svg_1_1(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated: decode then re-encode did not preserve the semantic projection -- {divergence}"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::mutations::{apply_svg_mutation, SvgMutation};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_artifact_stdio_svg::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::svg::standards::v1_1::subsets::base::project_svg_1_1;

    //#region 🔖️SpecCodec
    /// 📄️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<SvgMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    /// 👁️ The forward mutation, with the OBSERVABILITY law asserted IN ROLE — the same law
    /// `super::mutate_oracle` asserts on its side. `apply_svg_mutation` REJECTS a mutation whose
    /// path addresses nothing and leaves the snapshot untouched, so without this a refused mutation
    /// is reported as a green scenario carrying the unmutated drawing.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = SvgSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutation = mutation_from_spec(&spec)?;
        let before = project_svg_1_1(&base.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?)?;
        let mut snapshot = base;
        let outcome = apply_svg_mutation(&mut snapshot, &mutation);
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_svg_1_1(&bytes)?;
        if super::projection_divergence(&projection, &before).is_none() {
            return Err(format!("{kind:?} left the semantic projection exactly as it found it -- the subset either refused the mutation or addressed nothing; its own outcome messages were {:?}", outcome.messages()));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The inverse law, asserted on the SUBJECT side too rather than deferred to the parity
    /// phase: apply-then-undo must restore this side's OWN reading of the original drawing's
    /// projection. Mirrors `super::inverse_oracle` through the same independent `project_svg_1_1`.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = SvgSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let undo = mutation_inverse(&mutation, &base);
        let original = project_svg_1_1(&base.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?)?;
        let mut snapshot = base;
        let forward = apply_svg_mutation(&mut snapshot, &mutation);
        let backward: Vec<_> = undo.iter().map(|step| apply_svg_mutation(&mut snapshot, step).messages().to_vec()).collect();
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_svg_1_1(&bytes)?;
        if let Some(divergence) = super::projection_divergence(&projection, &original) {
            return Err(format!(
                "inverse law violated: {:?} followed by its own inverse did not restore the original drawing's projection -- {divergence}; forward outcome messages {:?}, undo outcome messages {:?}",
                spec.str("kind"),
                forward.messages(),
                backward
            ));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `import_utf8`/`export_utf8` are this
    /// subset's ONLY channel from input to output (no separate text-DSL layer over the snapshot).
    /// The round-trip half is asserted here too, the way `super::identity_round_trip_oracle` asserts
    /// it: re-encoding must preserve this side's own semantic projection.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = SvgSnapshot::import_utf8(&input).map_err(|error| format!("import_utf8 failed: {error}"))?;
        let output = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_svg_1_1(&output)?;
        if let Some(divergence) = super::projection_divergence(&projection, &project_svg_1_1(&input)?) {
            return Err(format!("round-trip law violated: decode then re-encode did not preserve the semantic projection -- {divergence}"));
        }
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
