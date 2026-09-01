//! 🦀️ XML 1.0/✳️valid exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! Every scenario copies the real, committed `📰️macos-uttype-plist.xml` fixture into the case work
//! directory first; the committed document is never written to. `oracle` drives the registered
//! `quick-xml` reference implementation through THIS subset's own oracle module
//! (`../../🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧪️oracle/🦀️component.rs`), whose DOCTYPE grammar and
//! §2.8/§2.9 verdicts are written from the W3C text rather than from this repository's
//! `check_valid_conformance`; `subject` drives this repository's own
//! `XmlSnapshot::import_utf8`/`export_utf8` and `apply_xml_valid_mutation` over the full 8-kind
//! `XmlValidMutation` vocabulary. Both results are read back by the SAME independent
//! `project_xml_valid` before the `semantic-xml-valid-1-0-v1` profile compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::xml::standards::v1_0::subsets::valid::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_xml_valid};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every scenario row THIS CASE registers, oracle and subject alike --
/// `no-mutation` plus every `XmlValidMutation` variant, mirrored from
/// `../../🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS`.
/// `no-mutation` is NOT one of that production `KINDS`' entries -- it carries no `XmlValidMutation`
/// variant of its own (dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration: `no` is not
/// an approved semantic verb) and is handled directly by `subject::mutate`/`subject::inverse` below
/// as the identity probe the feature file's own `no-mutation` row names it. The eight REAL kinds are
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio`; `kinds_matches_enum_variants_in_declaration_order` on the production side
/// and the framework's own catalog-completeness gate on this side keep the two lists honest.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "declare-doctype", "rename-document-element", "set-external-subset", "set-standalone", "declare-entity", "set-internal-subset", "set-text"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 📰️ The document every mutation row runs on: a real 40 440-byte Apple PropertyList 1.0 document
/// derived ONCE from the real committed 50-row German building-material-reuse survey by
/// `🐍️derive-xml-valid-fixture.py` in the ticket folder, carrying the same real Apple DOCTYPE the
/// committed UTType declaration carries.
const INPUT: &str = "shared://📰️reuse-marketplaces-plist.xml";
/// 📰️ The real macOS UTType declaration this case used to rest on — the real production document
/// this repository ships — kept for `identity-round-trip`, which still reads it on its own.
const UTTYPE_INPUT: &str = "shared://📰️macos-uttype-plist.xml";

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

/// 🔮️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind other than
/// `no-mutation` whose parameters leave the semantic projection exactly where it was has not been
/// tested by this scenario at all -- it proves only that the reference library declined to error.
/// The `Examples` rows are chosen against the real document's actual content for exactly this
/// reason, and this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_xml_valid(&bytes)?;
    if kind != "no-mutation" && projection_divergence(&projection, &project_xml_valid(&input)?).is_none() {
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
    use super::{mutable_input, projection_divergence, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::xml::standards::v1_0::subsets::base::schema::mutations::XmlNodePath;
    use semio_s_plugin_stdio::artifacts::xml::standards::v1_0::subsets::base::schema::snapshot::{XmlDtdDeclaration, XmlExternalId};
    use semio_s_plugin_stdio::artifacts::xml::standards::v1_0::subsets::valid::schema::valid_mutations::{declare_doctype::DeclareDoctype, declare_entity::DeclareEntity, rename_document_element::RenameDocumentElement, set_external_subset::SetExternalSubset, set_internal_subset::SetInternalSubset, set_snapshot::SetSnapshot, set_standalone::SetStandalone, set_text::SetText};
    use semio_s_plugin_stdio::artifacts::xml::standards::v1_0::subsets::valid::schema::{apply_xml_valid_mutation, inverse_xml_valid_mutation, XmlValidMutation};
    use semio_s_plugin_stdio::artifacts::xml::XmlSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::xml::standards::v1_0::subsets::valid::project_xml_valid;

    //#region 🔖️SpecCodec
    fn usize_field(value: &Json, key: &str) -> usize {
        match value.get(key) {
            Some(Json::Number(number)) => number.max(0.0) as usize,
            _ => 0,
        }
    }

    fn bool_field(value: &Json, key: &str) -> bool {
        matches!(value.get(key), Some(Json::Bool(true)))
    }

    fn usize_path(items: Vec<Json>) -> Vec<usize> {
        items
            .iter()
            .map(|item| match item {
                Json::Number(number) => number.max(0.0) as usize,
                _ => 0,
            })
            .collect()
    }

    /// 🔗️ The same `{"kind":"system"|"public", ...}` external-identifier grammar the oracle side
    /// speaks, decoded into the PRODUCTION `XmlExternalId` here instead of the oracle's own type.
    fn json_to_external_id(value: &Json) -> Option<XmlExternalId> {
        match value.get("externalId") {
            Some(entry) if !matches!(entry, Json::Null) => match entry.str("kind").as_str() {
                "system" => Some(XmlExternalId::System { system_id: entry.str("systemId") }),
                "public" => Some(XmlExternalId::Public { public_id: entry.str("publicId"), system_id: entry.str("systemId") }),
                _ => None,
            },
            _ => None,
        }
    }

    fn json_to_declarations(value: &Json) -> Vec<XmlDtdDeclaration> {
        value.array("declarations").iter().map(|entry| XmlDtdDeclaration::Entity { parameter: bool_field(entry, "parameter"), name: entry.str("name"), value: entry.str("value") }).collect()
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `XmlValidMutation` this
    /// subset declares for it. `no-mutation` has no arm here -- it carries no `XmlValidMutation`
    /// variant (dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration) and is handled
    /// directly by `mutate`/`inverse` below before this function is ever called.
    fn mutation_from_spec(spec: &Json) -> Result<XmlValidMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "set-snapshot" => Ok(XmlValidMutation::SetSnapshot(SetSnapshot { snapshot: XmlSnapshot::import_utf8(params.str("xml").as_bytes()).map_err(|error| format!("set-snapshot xml parse failed: {error}"))? })),
            "declare-doctype" => Ok(XmlValidMutation::DeclareDoctype(DeclareDoctype { external_id: json_to_external_id(&params) })),
            "rename-document-element" => Ok(XmlValidMutation::RenameDocumentElement(RenameDocumentElement { name: params.str("name") })),
            "set-external-subset" => Ok(XmlValidMutation::SetExternalSubset(SetExternalSubset { external_id: json_to_external_id(&params) })),
            "set-standalone" => Ok(XmlValidMutation::SetStandalone(SetStandalone {
                standalone: match params.get("standalone") {
                    Some(Json::Bool(value)) => Some(*value),
                    _ => None,
                },
            })),
            "declare-entity" => Ok(XmlValidMutation::DeclareEntity(DeclareEntity { index: usize_field(&params, "index"), parameter: bool_field(&params, "parameter"), name: params.str("name"), value: params.str("value") })),
            "set-internal-subset" => Ok(XmlValidMutation::SetInternalSubset(SetInternalSubset { declarations: json_to_declarations(&params) })),
            "set-text" => Ok(XmlValidMutation::SetText(SetText { path: XmlNodePath(usize_path(params.array("path"))), text: params.str("text") })),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
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
    /// `no-mutation` is handled BEFORE `mutation_from_spec` is even called: it carries no
    /// `XmlValidMutation` variant of its own (dropped by the `26/08/29/S-END-TO-END` mutation-leaf
    /// migration), so its identity is asserted directly here rather than through the vocabulary --
    /// the base, rendered and re-projected but otherwise untouched, exactly what `NoMutation`'s own
    /// `XmlDiff::default()` used to produce.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        if kind == "no-mutation" {
            let (bytes, projection) = rendered(&base)?;
            return Ok(Outcome::with_raw(bytes, projection));
        }
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
    /// under test is the implementation's own algebra, not a copy of it in the adapter. `no-mutation`
    /// is handled the same way `mutate` handles it above: no `XmlValidMutation` to construct, no
    /// inverse to compute, the base restores itself trivially.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let spec = ctx.doc_json()?;
        if spec.str("kind") == "no-mutation" {
            let (bytes, projection) = rendered(&base)?;
            return Ok(Outcome::with_raw(bytes, projection));
        }
        let mutation = mutation_from_spec(&spec)?;
        let undo = inverse_xml_valid_mutation(&mutation, &base);
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

    /// 🧭️ Re-exported so `super::adapter()` can register the same 9-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 9 kinds -- the scenario id only selects which `Examples` row's
/// `<id>`/`<params>` doc string the shared handler reads.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
