//! 🦀️ HTML 5 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, derived `🏚️zukunft-bau-entwerfen-mit-bestand/🌐️.html` fixture (see
//! the feature file's own header for the derivation note) into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `html5ever`/
//! `markup5ever_rcdom` reference implementation
//! (`../../🏅️standards/🔖️5/🪆️subsets/✳️any/🦀️oracle.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `parse_html_document`/`write_html_document`/`apply_html_mutation` over the full 10-kind
//! `HtmlMutation` vocabulary. Both results are read back by the SAME independent `project_html_5`
//! (`html5ever`) before the `semantic-html-v1` profile compares them. The subject half is gated
//! behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::html::standards::v5::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_html_5};


//#region 🔖️Input
const INPUT: &str = "shared://🏚️zukunft-bau-entwerfen-mit-bestand/🌐️.html";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.html"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
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
    let projection = project_html_5(&bytes)?;
    if projection_divergence(&projection, &project_html_5(&input)?).is_none() {
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
/// `html5ever` applies the kind and then its own computed inverse, and the restored document's
/// independent projection must equal the REAL original's own. The original is projected through the
/// same tree-shaped `project_html_5`, so the HTML parser's own normalization (implied
/// `html`/`head`/`body`, tag-name case folding, attribute ordering) is applied to BOTH sides and is
/// not what this compares. Without this the scenario would only prove that the reference library did
/// not error, which is not what `@mode-property` claims.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_html_5(&bytes)?;
    let original = project_html_5(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: {:?} followed by its own inverse did not restore the original document's projection -- {divergence}", spec.str("kind")));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `html5ever`
/// fully parses the real document and re-serializes it from its own tree alone
/// (`oracle_round_trip`), so BOTH halves of
/// the law are checkable here without a subject -- the re-encoded bytes must differ from the input
/// (HTML 5 is not a byte-preserving carrier: the tree builder inserts implied elements and the
/// serializer re-derives every tag and character reference from the tree, so bit-identity would
/// prove the artifact was copied rather than parsed), and the re-encoded document's own projection
/// must still equal the input's -- parse is idempotent on already-serialized HTML, so a divergence
/// here is real loss in `html5ever`'s own write/read cycle, not writer freedom.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the document was parsed rather than copied".to_string());
    }
    let projection = project_html_5(&bytes)?;
    let original = project_html_5(&input)?;
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
    use semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::mutations::{apply_html_mutation, HtmlMutation};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_artifact_stdio_html::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::snapshot::{parse_html_document, write_html_document};
    use semio_s_plugin_stdio_test_oracle::artifacts::html::standards::v5::subsets::any::project_html_5;

    //#region 🔖️SpecCodec
    /// 📄️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<HtmlMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    /// 👁️ The forward mutation, with the OBSERVABILITY law asserted IN ROLE -- the same law
    /// `super::mutate_oracle` asserts on its side, and the feature's own second `Then` step
    /// ("the semantic projection moved"). Without it a mutation the
    /// subset REFUSES (`apply_html_mutation` returns an error `MutationOutcome` and leaves the
    /// snapshot untouched) is indistinguishable here from one it performed, and the handler reports a
    /// green scenario carrying the UNMUTATED document -- which is exactly what this case did until
    /// the parity phase first ran.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let base = parse_html_document(&text).map_err(|error| format!("parse_html_document failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutation = mutation_from_spec(&spec)?;
        let before = project_html_5(&write_html_document(&base).into_bytes())?;
        let mut snapshot = base;
        let outcome = apply_html_mutation(&mut snapshot, &mutation);
        let bytes = write_html_document(&snapshot).into_bytes();
        let projection = project_html_5(&bytes)?;
        if super::projection_divergence(&projection, &before).is_none() {
            return Err(format!("{kind:?} left the semantic projection exactly as it found it -- the subset either refused the mutation or addressed nothing; its own outcome messages were {:?}", outcome.messages()));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The inverse law, ASSERTED on the SUBJECT side too rather than deferred to the parity
    /// phase: apply-then-undo must restore this side's OWN reading of the original document's
    /// projection. Mirrors `super::inverse_oracle` exactly, through the same independent
    /// `project_html_5`.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let base = parse_html_document(&text).map_err(|error| format!("parse_html_document failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let undo = mutation_inverse(&mutation, &base);
        let original = project_html_5(&write_html_document(&base).into_bytes())?;
        let mut snapshot = base;
        let forward = apply_html_mutation(&mut snapshot, &mutation);
        let backward: Vec<_> = undo.iter().map(|step| apply_html_mutation(&mut snapshot, step).messages().to_vec()).collect();
        let bytes = write_html_document(&snapshot).into_bytes();
        let projection = project_html_5(&bytes)?;
        if let Some(divergence) = super::projection_divergence(&projection, &original) {
            return Err(format!(
                "inverse law violated: {:?} followed by its own inverse did not restore the original document's projection -- {divergence}; forward outcome messages {:?}, undo outcome messages {:?}",
                spec.str("kind"),
                forward.messages(),
                backward
            ));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `parse_html_document`/
    /// `write_html_document` are this subset's ONLY channel from input to output (HTML is text-native;
    /// there is no separate binary layer over the same model). The round-trip half is asserted here
    /// too, the same way `super::identity_round_trip_oracle` asserts it: re-encoding must preserve
    /// this side's own semantic projection.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let snapshot = parse_html_document(&text).map_err(|error| format!("parse_html_document failed: {error}"))?;
        let output = write_html_document(&snapshot).into_bytes();
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_html_5(&output)?;
        if let Some(divergence) = super::projection_divergence(&projection, &project_html_5(&input)?) {
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
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
