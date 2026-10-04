//! 🦀️ XML 1.0/✳️valid exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! Every scenario copies the real, committed `🧪️macos-uttype-plist/🏷️.xml` fixture into the case work
//! directory first; the committed document is never written to. `oracle` drives the registered
//! `quick-xml` reference implementation through THIS subset's own oracle module
//! (`../../🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🔮️oracles/🦀️.rs`), whose DOCTYPE grammar and
//! §2.8/§2.9 verdicts are written from the W3C text rather than from this repository's
//! `check_valid_conformance`; `subject` drives this repository's own
//! `XmlSnapshot::import_utf8`/`export_utf8` and `apply_xml_valid_mutation` over the full 8-kind
//! `XmlValidMutation` vocabulary. Both results are read back by the SAME independent
//! `project_xml_valid` before the `semantic-xml-valid-1-0-v1` profile compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::valid::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_xml_valid};


//#region 🔖️Input
/// 📰️ The document every mutation row runs on: a real 40 440-byte Apple PropertyList 1.0 document
/// derived ONCE from the real committed 50-row German building-material-reuse survey by
/// `🐍️derive-xml-valid-fixture.py` in the ticket folder, carrying the same real Apple DOCTYPE the
/// committed UTType declaration carries.
const INPUT: &str = "shared://♻️reuse-marketplaces-plist/🏷️.xml";
/// 📰️ The real macOS UTType declaration this case used to rest on — the real production document
/// this repository ships — kept for `identity-round-trip`, which still reads it on its own.
const UTTYPE_INPUT: &str = "shared://🍎️macos-uttype-plist/🏷️.xml";

/// 🧫️ Copies the immutable real document into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("reuse-marketplaces.plist.xml"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🧫️ The same, for the UTType declaration the round-trip scenario additionally reads.
fn mutable_uttype_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(UTTYPE_INPUT, Some("uttype.plist.xml"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// ⚖️ First point at which two projections diverge, as a character offset into the canonical
/// rendering plus the window around it on both sides -- an equality check whose failure names WHAT
/// changed rather than only that something did.
fn projection_divergence(actual: &Json, expected: &Json) -> Option<String> {
    let (left, right): (Vec<char>, Vec<char>) = (actual.to_string().chars().collect(), expected.to_string().chars().collect());
    if left == right {
        return None;
    }
    let at = left.iter().zip(right.iter()).position(|(one, other)| one != other).unwrap_or(left.len().min(right.len()));
    let window = |text: &[char]| text.iter().skip(at.saturating_sub(60)).take(160).collect::<String>();
    Some(format!("first divergence at char {at} of {} vs {} -- got …{}… want …{}…", left.len(), right.len(), window(&left), window(&right)))
}

/// 🔮️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind whose parameters
/// leave the semantic projection exactly where it was has not been tested by this scenario at all
/// -- it proves only that the reference library declined to error.
/// The `Examples` rows are chosen against the real document's actual content for exactly this
/// reason, and this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_xml_valid(&bytes)?;
    if projection_divergence(&projection, &project_xml_valid(&input)?).is_none() {
        return Err(format!("{kind:?} left the semantic projection exactly as it found it -- a mutation whose parameters make it a no-op against the real document is not a test of that kind"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, ASSERTED on the ORACLE side rather than deferred to the parity phase: the
/// reference implementation applies the kind and then its own computed inverse, and the restored
/// document's independent projection must equal the REAL original's own. With the subject phase
/// blocked this is the only place the property can be checked today.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_xml_valid(&bytes)?;
    let original = project_xml_valid(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("inverse law violated: {:?} followed by its own inverse did not restore the original document's projection -- {divergence}", spec.str("kind")));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, ASSERTED rather than narrated: `quick-xml`
/// fully parses the real document and re-serializes it from its own tree alone, so both halves of
/// the law are checkable here without a subject -- the re-encoded bytes must differ from the input
/// (XML 1.0 is no byte-preserving carrier: this fixture's prolog is line-broken between the
/// declaration, the DOCTYPE and the document element, and a canonical writer re-derives the prolog
/// without that insignificant whitespace), and the re-encoded document's own projection must still
/// equal the input's.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let uttype = round_trip_oracle_once(&mutable_uttype_input(ctx)?, "the UTType declaration")?;
    let survey = round_trip_oracle_once(&mutable_input(ctx)?, "the survey property list")?;
    Ok(Outcome::with_raw(survey.0, Json::Object(vec![("uttype".to_string(), uttype.1), ("survey".to_string(), survey.1)])))
}

/// 🔁️ The probe itself, over one document.
fn round_trip_oracle_once(input: &[u8], what: &str) -> Result<(Vec<u8>, Json), String> {
    let bytes = oracle_round_trip(input)?;
    if bytes == input {
        return Err(format!("byte pass-through on {what}: the oracle's re-encoded bytes are bit-identical to the input, so nothing here proves the document was parsed rather than copied"));
    }
    let projection = project_xml_valid(&bytes)?;
    let original = project_xml_valid(input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated on {what}: decode then re-encode did not preserve the semantic projection -- {divergence}"));
    }
    Ok((bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, projection_divergence};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_xml::standards::v1_0::subsets::valid::schema::{apply_xml_valid_mutation, XmlValidMutation};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_xml::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_xml::XmlSnapshot;
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::valid::project_xml_valid;

    //#region 🔖️SpecCodec
    /// 📄️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<XmlValidMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    fn base_snapshot(ctx: &Context) -> Result<XmlSnapshot, String> {
        XmlSnapshot::import_utf8(&mutable_input(ctx)?).map_err(|error| format!("import_utf8 failed: {error}"))
    }

    fn rendered(snapshot: &XmlSnapshot) -> Result<(Vec<u8>, Json), String> {
        let bytes = snapshot.export_utf8().map_err(|error| format!("export_utf8 failed: {error}"))?;
        let projection = project_xml_valid(&bytes)?;
        Ok((bytes, projection))
    }

    /// 🔮️ The forward mutation, with the same observability law the oracle side asserts: this
    /// subset's vocabulary REJECTS rather than silently ignores, so a kind that left the projection
    /// untouched here means either a refused mutation or parameters that address nothing.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutation = mutation_from_spec(&spec)?;
        let mut snapshot = base.clone();
        apply_xml_valid_mutation(&mut snapshot, &mutation);
        let (bytes, projection) = rendered(&snapshot)?;
        if projection_divergence(&projection, &rendered(&base)?.1).is_none() {
            return Err(format!("{kind:?} left the semantic projection exactly as it found it -- either the vocabulary refused it or the parameters address nothing in the real document"));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ `inverse_xml_valid_mutation` is called for real rather than transcribed: the property
    /// under test is the implementation's own algebra, not a copy of it in the adapter.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let undo = mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
        let mut snapshot = base.clone();
        apply_xml_valid_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_xml_valid_mutation(&mut snapshot, step);
        }
        let (bytes, projection) = rendered(&snapshot)?;
        if let Some(divergence) = projection_divergence(&projection, &rendered(&base)?.1) {
            return Err(format!("inverse law violated: apply-then-undo did not restore the original document's projection -- {divergence}"));
        }
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `XmlSnapshot::import_utf8`/
    /// `export_utf8` are this subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let uttype = round_trip_once(&super::mutable_uttype_input(ctx)?, "the UTType declaration")?;
        let survey = round_trip_once(&mutable_input(ctx)?, "the survey property list")?;
        Ok(Outcome::with_raw(survey.0, Json::Object(vec![("uttype".to_string(), uttype.1), ("survey".to_string(), survey.1)])))
    }

    /// 🔁️ The probe itself, over one document.
    fn round_trip_once(input: &[u8], what: &str) -> Result<(Vec<u8>, Json), String> {
        let snapshot = XmlSnapshot::import_utf8(input).map_err(|error| format!("import_utf8 of {what} failed: {error}"))?;
        let output = snapshot.export_utf8().map_err(|error| format!("export_utf8 of {what} failed: {error}"))?;
        if output == input {
            return Err(format!("byte pass-through on {what}: output is bit-identical to the input"));
        }
        let projection = project_xml_valid(&output)?;
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
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
