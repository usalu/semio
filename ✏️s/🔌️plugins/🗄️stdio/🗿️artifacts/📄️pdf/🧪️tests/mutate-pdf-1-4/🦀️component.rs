//! 🦀️ PDF 1.4 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work
//! directory first; the committed asset is never written to. `oracle` drives the registered
//! `lopdf` reference implementation (both as the independent reader and, for this subset's own
//! thin vocabulary, as the independent writer too — see `../../🏅️standards/🔖️1.4/🪆️subsets/✳️any/
//! 🧪️oracle/🦀️component.rs`'s own doc comment for why `width`/`height` are pinned rather than
//! independently rediscovered); `subject` drives this repository's own `decode_pdf`/`encode_pdf`/
//! `apply_pdf_mutation`. Both results are read back by the SAME independent `project_pdf_1_4`
//! before the `semantic-pdf-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::any::{build_single_page_pdf, independent_first_text, oracle_apply_mutation, project_pdf_1_4};

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bachelor-thesis.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 📄️ `params.snapshot.page.text` out of a `set-snapshot` spec, mirroring the oracle module's own
/// private `target_text` reader (kept local: the SUBJECT side needs the same field pulled into a
/// typed `PdfSnapshot`, not into the oracle's `Json` shape).
fn target_text(spec: &Json) -> String {
    spec.get("params").and_then(|params| params.get("snapshot")).and_then(|snapshot| snapshot.get("page")).map(|page| page.str("text")).unwrap_or_default()
}
//#endregion 🔖️Input

//#region 🔖️Law
/// 🔬️ First structural divergence between two projections — a dotted field path plus both values,
/// so a law that fails names WHICH field moved instead of only "not equal". Kept local to this
/// adapter for the same reason `target_text` is duplicated here: a case adapter is a leaf that
/// links the test host and this subset's own oracle module, nothing else.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    let here = if path.is_empty() { "the projection".to_string() } else { path.to_string() };
    let child = |key: &str| if path.is_empty() { key.to_string() } else { format!("{}.{}", path, key) };
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
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{} has {} entries, the original had {}", here, right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&child(&index.to_string()), value, other))
        }
        (left, right) if left == right => None,
        (left, right) => Some(format!("{} is {} — the original had {}", here, brief(right), brief(left))),
    }
}

/// ✂️ A projection value, truncated: a divergence message must stay readable, and these projections
/// carry whole documents.
fn brief(value: &Json) -> String {
    let text = value.to_string();
    match text.char_indices().nth(160) {
        Some((cut, _)) => format!("{}…", &text[..cut]),
        None => text,
    }
}
//#endregion 🔖️Law

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pdf_1_4(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔁️ Applies the forward mutation, then its algebraic inverse, and ASSERTS the law: the restored
/// document must project onto exactly what the un-mutated document projects onto through this
/// oracle's own writer. `SetSnapshot`'s inverse restores the ORIGINAL base snapshot, so the base's
/// `text` is read independently from the un-mutated input before the forward mutation ever runs;
/// `NoMutation`'s inverse is itself, so this naturally degenerates to applying `no-mutation` twice.
///
/// ⚖️ The baseline is `build_single_page_pdf(base_text)`, NOT `project_pdf_1_4(&input)`, and that is
/// a documented property of this subset rather than a softened law: the oracle here is a
/// rebuild-from-text writer that pins `MediaBox [0 0 612 792]` for every document, mirroring
/// `decode_pdf`, which hardcodes the same constant and never reads a real page's geometry (the
/// oracle module's own header, confirmed against this fixture's true `[0 0 595.276 841.89]`).
/// Geometry therefore carries no round-trip information on EITHER side; `text` — the one field this
/// subset genuinely reads out of a document — is read from the REAL input before any mutation and
/// must come back unchanged.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let base_text = independent_first_text(&input)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let inverse_spec = Json::Object(vec![("kind".to_string(), spec.get("kind").cloned().unwrap_or(Json::Null)), ("params".to_string(), Json::Object(vec![("snapshot".to_string(), Json::Object(vec![("page".to_string(), Json::Object(vec![("text".to_string(), Json::String(base_text.clone()))]))]))]))]);
    let restored = oracle_apply_mutation(&mutated, &inverse_spec)?;
    let projection = project_pdf_1_4(&restored)?;
    let baseline = project_pdf_1_4(&build_single_page_pdf(&base_text)?)?;
    if let Some(divergence) = first_divergence("", &baseline, &projection) {
        return Err(format!("inverse-{}: applying the mutation and then its algebraic inverse did not restore the un-mutated document — {}", spec.str("kind"), divergence));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted rather than merely produced: the
/// reference writer emits the real document's own text through `lopdf`'s object writer, the
/// INDEPENDENT reader reads that back, and the two must agree — plus the emitted bytes must differ
/// from the input, so nothing can pass by copying.
///
/// 🚫️ `width`/`height` are deliberately NOT part of this law. This subset pins `612×792` on both
/// sides by documented design (oracle module header; `decode_pdf` hardcodes the same), so the real
/// fixture's `595.276×841.89` is unreachable by construction and a check demanding it would be
/// contrived rather than true. What the law does cover is the whole of what this subset reads out
/// of a document: `text`.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let base_text = independent_first_text(&input)?;
    let bytes = build_single_page_pdf(&base_text)?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded document is bit-identical to the input".to_string());
    }
    let projection = project_pdf_1_4(&bytes)?;
    let recovered = projection.str("text");
    if recovered != base_text {
        return Err(format!("identity round trip: the independent reader recovered {:?} from the re-encoded document, but the real input carries {:?}", recovered, base_text));
    }
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, target_text};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::{PageDoc, PdfSnapshot};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::io::{decode_pdf, encode_pdf};
    use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
    use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::any::project_pdf_1_4;

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed mutation this subset
    /// declares for it. `no-mutation` ignores `params`; `set-snapshot` reads the target page.
    fn mutation_from_spec(spec: &semio_repo_test_host::Json) -> Result<PdfMutation, String> {
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(PdfMutation::NoMutation),
            "set-snapshot" => {
                let page = spec.get("params").and_then(|params| params.get("snapshot")).and_then(|snapshot| snapshot.get("page"));
                let number = |key: &str| -> f64 {
                    match page.and_then(|value| value.get(key)) {
                        Some(semio_repo_test_host::Json::Number(value)) => *value,
                        _ => 0.0,
                    }
                };
                Ok(PdfMutation::SetSnapshot { snapshot: PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: number("width"), height: number("height"), text: target_text(spec) } } })
            }
            other => Err(format!("mutation kind {:?} has no subject implementation", other)),
        }
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_pdf(&mutable_input(ctx)?).map_err(|error| format!("decode_pdf failed: {}", error))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base;
        apply_pdf_mutation(&mut snapshot, &mutation);
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {}", error))?;
        let projection = project_pdf_1_4(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔁️ `PdfMutation::inverse` in closed form (`../../../../🏅️standards/🔖️1.4/🪆️subsets/✳️any/
    /// 🧬️schema/🧬️mutations/🦀️component.rs`'s own `impl Mutation<PdfSnapshot>`): `NoMutation`'s
    /// inverse is itself; `SetSnapshot`'s inverse restores the ORIGINAL base. Written out here
    /// rather than calling the trait method so this adapter crate needs no `protocol` dependency.
    fn inverse_of(mutation: &PdfMutation, base: &PdfSnapshot) -> PdfMutation {
        match mutation {
            PdfMutation::NoMutation => PdfMutation::NoMutation,
            PdfMutation::SetSnapshot { .. } => PdfMutation::SetSnapshot { snapshot: base.clone() },
        }
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_pdf(&mutable_input(ctx)?).map_err(|error| format!("decode_pdf failed: {}", error))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base.clone();
        apply_pdf_mutation(&mut snapshot, &mutation);
        apply_pdf_mutation(&mut snapshot, &inverse_of(&mutation, &base));
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {}", error))?;
        let projection = project_pdf_1_4(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone — decode_pdf/encode_pdf are this
    /// subset's ONLY channel from input to output (it has no separate text-DSL layer beyond
    /// wrapping the same codec, see `PdfSnapshot`'s `ArtifactDsl` impl).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_pdf(&input).map_err(|error| format!("decode_pdf failed: {}", error))?;
        let output = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {}", error))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_pdf_1_4(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust")
        .oracle("mutate-no-mutation", mutate_oracle)
        .oracle("mutate-set-snapshot", mutate_oracle)
        .oracle("inverse-no-mutation", inverse_oracle)
        .oracle("inverse-set-snapshot", inverse_oracle)
        .oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        return built.subject("mutate-no-mutation", subject::mutate).subject("mutate-set-snapshot", subject::mutate).subject("inverse-no-mutation", subject::inverse).subject("inverse-set-snapshot", subject::inverse).subject("identity-round-trip", subject::identity_round_trip);
    }
    #[cfg(not(feature = "sut"))]
    built
}
//#endregion 🔖️Registration
