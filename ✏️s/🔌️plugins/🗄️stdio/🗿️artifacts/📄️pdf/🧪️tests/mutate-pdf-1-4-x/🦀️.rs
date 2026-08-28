//! 🦀️ PDF 1.4 ✳️x exhaustive conformance mutation case — Rust adapter.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory
//! first; the committed asset is never written to. `oracle` handlers drive the registered `lopdf`
//! 0.44 reference implementation through this subset's own
//! `../../🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧪️oracle/🦀️component.rs`, `subject` handlers drive this
//! repository's own decode/mutate/encode round trip, and both results are read back by the SAME
//! independent `project_conformance` before the `semantic-pdf-1-4-conformance-x-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never compiles the local implementation.
//!
//! @see ../../🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🦀️component.rs — `check_pdf_x_conformance`, the one
//!      axis list this whole case derives from.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::x::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_conformance};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `pdf-1-4-x` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that module
/// lives in the SUBJECT crate, and the oracle role must not link the subject crate at all.
const KINDS: &[&str] = &["set-page-size", "collapse-page-size"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";

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
    if projection == project_conformance(&base)? {
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
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::x::schema::mutations::{apply_x_conformance_mutation, inverse_x_conformance_mutation, CollapsePageSize, PdfX1Mutation, SetPageSize};
    use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::x::project_conformance;

    fn mutation_from_spec(spec: &Json) -> Result<PdfX1Mutation, String> {
        let params = spec.get("params").ok_or("Missing mutation parameters")?;
        let number = |key| match params.get(key) {
            Some(Json::Number(value)) if value.is_finite() => Ok(*value),
            _ => Err(format!("{key} must be finite")),
        };
        Ok(match spec.str("kind").as_str() {
            "set-page-size" => PdfX1Mutation::SetPageSize(SetPageSize { width: number("width")?, height: number("height")? }),
            "collapse-page-size" => PdfX1Mutation::CollapsePageSize(CollapsePageSize {}),
            other => return Err(format!("Unknown subject mutation {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_pdf(&mutable_input(ctx)?).map_err(|error| error.to_string())?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let outcome = apply_x_conformance_mutation(&mut snapshot, &mutation);
        if !outcome.messages().is_empty() {
            return Err(format!("Mutation refused: {:?}", outcome.messages()));
        }
        let output = encode_pdf(&snapshot).map_err(|error| error.to_string())?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_pdf(&mutable_input(ctx)?).map_err(|error| error.to_string())?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let inverse = inverse_x_conformance_mutation(&mutation, &base);
        let mut snapshot = base.clone();
        if !apply_x_conformance_mutation(&mut snapshot, &mutation).messages().is_empty() {
            return Err("Forward mutation refused".into());
        }
        for step in inverse {
            if !apply_x_conformance_mutation(&mut snapshot, &step).messages().is_empty() {
                return Err("Inverse mutation refused".into());
            }
        }
        if snapshot != base {
            return Err("Concrete inverse did not restore the complete snapshot".into());
        }
        let output = encode_pdf(&snapshot).map_err(|error| error.to_string())?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_pdf(&input).map_err(|error| error.to_string())?;
        let output = encode_pdf(&snapshot).map_err(|error| error.to_string())?;
        if output == input {
            return Err("Byte pass-through instead of reconstruction".into());
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
