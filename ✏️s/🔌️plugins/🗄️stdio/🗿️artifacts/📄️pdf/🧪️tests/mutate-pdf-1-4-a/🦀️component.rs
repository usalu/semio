//! 🦀️ PDF 1.4 ✳️a exhaustive conformance mutation case — Rust adapter.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory
//! first; the committed asset is never written to. `oracle` handlers drive the registered `lopdf`
//! 0.44 reference implementation through this subset's own
//! `../../🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧪️oracle/🦀️component.rs`, `subject` handlers drive this
//! repository's own decode/mutate/encode round trip, and both results are read back by the SAME
//! independent `project_conformance` before the `semantic-pdf-1-4-conformance-a-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never compiles the local implementation.
//!
//! @see ../../🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🦀️component.rs — `check_pdf_a_conformance`, the one
//!      axis list this whole case derives from.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::a::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_conformance};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `pdf-1-4-a` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️1.4/🪆️subsets/✳️a/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that module
/// lives in the SUBJECT crate, and the oracle role must not link the subject crate at all.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-page-text", "clear-page-text"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
/// Nothing in this case arranges a pre-state: the committed document already carries both a positive
/// page box and real page-1 text, so the single axis has something genuine to move in either
/// direction.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bachelor-thesis.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Law
/// 🔬️ First structural divergence between two projections — a dotted field path plus both values,
/// so a law that fails names WHICH axis moved instead of only "not equal".
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    let here = if path.is_empty() { "the projection".to_string() } else { path.to_string() };
    let child = |key: &str| if path.is_empty() { key.to_string() } else { format!("{path}.{key}") };
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&child(key), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{} is gone (the original carried {})", child(key), brief(value))),
                }
            }
            right.iter().find(|(name, _)| !left.iter().any(|(other, _)| other == name)).map(|(name, value)| format!("{} appeared (absent in the original, now {})", child(name), brief(value)))
        }
        (left, right) if left == right => None,
        (left, right) => Some(format!("{here} is {} — the original had {}", brief(right), brief(left))),
    }
}

fn brief(value: &Json) -> String {
    let text = value.to_string();
    match text.char_indices().nth(160) {
        Some((cut, _)) => format!("{}…", &text[..cut]),
        None => text,
    }
}
//#endregion 🔖️Law

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id — and the place the OBSERVABILITY law
/// is asserted in-role: a mutation that leaves this subset's conformance projection untouched proves
/// nothing, whatever the reference implementation returned.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = mutable_input(ctx)?;
    let output = oracle_apply_mutation(&base, &spec)?;
    let projection = project_conformance(&output)?;
    if spec.str("kind") != "no-mutation" && projection == project_conformance(&base)? {
        return Err(format!("mutate-{}: the mutation left the conformance projection unchanged — a mutation that is not observable proves nothing", spec.str("kind")));
    }
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id — and the place the INVERSE law is
/// asserted in-role, without needing the subject.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = mutable_input(ctx)?;
    let original = project_conformance(&base)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let undo = oracle_inverse_spec(&base, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_conformance(&restored)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("inverse-{}: the mutation followed by its own computed inverse ({}) did not restore the document — {}", spec.str("kind"), undo.str("kind"), divergence));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, asserted rather than merely claimed.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    if output == input {
        return Err("byte pass-through: the re-serialized document is bit-identical to the input".to_string());
    }
    let projection = project_conformance(&output)?;
    let original = project_conformance(&input)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("identity round trip: parsing and re-serializing the real document did not preserve its conformance projection — {divergence}"));
    }
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::io::{decode_pdf, encode_pdf};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::a::schema::mutations::{apply_a_conformance_mutation, stamp_conformance, PdfA1Mutation};
    use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::a::{oracle_inverse_spec, project_conformance};

    fn decode(bytes: &[u8]) -> Result<PdfSnapshot, String> {
        decode_pdf(bytes).map_err(|error| error.to_string())
    }

    fn encode(snapshot: &PdfSnapshot) -> Result<Vec<u8>, String> {
        encode_pdf(snapshot).map_err(|error| error.to_string())
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `PdfA1Mutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(base: &PdfSnapshot, spec: &Json) -> Result<PdfA1Mutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => PdfA1Mutation::NoMutation,
            "set-snapshot" => PdfA1Mutation::SetSnapshot { snapshot: stamp_conformance(base.clone(), params.str("conformance") == "stamped") },
            "set-page-text" => PdfA1Mutation::SetPageText { text: params.str("text") },
            "clear-page-text" => PdfA1Mutation::ClearPageText,
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&snapshot, &spec)?;
        apply_a_conformance_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot)?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = mutable_input(ctx)?;
        let mut snapshot = decode(&base)?;
        let forward = mutation_from_spec(&snapshot, &spec)?;
        apply_a_conformance_mutation(&mut snapshot, &forward);
        let undo = oracle_inverse_spec(&base, &spec)?;
        let backward = mutation_from_spec(&snapshot, &undo)?;
        apply_a_conformance_mutation(&mut snapshot, &backward);
        let output = encode(&snapshot)?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode(&snapshot)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
