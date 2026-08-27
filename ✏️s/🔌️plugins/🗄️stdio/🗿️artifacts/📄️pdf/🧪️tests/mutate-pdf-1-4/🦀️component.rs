//! 🦀️ PDF 1.4 exhaustive mutation case — Rust adapter.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work
//! directory first; the committed asset is never written to. `oracle` drives the registered
//! `lopdf` reference implementation — as the independent READER (`lopdf`'s own page-tree walk,
//! `/MediaBox` inheritance chain and content-stream decoder) and as the independent WRITER (a
//! fresh `lopdf::Document` assembled object by object) — through
//! `../../🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`; `subject` drives this
//! repository's own `decode_pdf`/`encode_pdf`/`apply_pdf_mutation`. Both results are read back by
//! the SAME independent `project_pdf_1_4` before the `semantic-pdf-v1` profile compares them. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation.
//!
//! 📚️ **What the comparison covers.** The whole page tree: the page count, and per page the
//! `/MediaBox` extent and the shown text. Until this wave the subset's snapshot held ONE page and
//! both halves of this case were written to mirror that — the oracle rebuilt every document as a
//! single synthetic page pinned to `612×792`, and the baseline every law was measured against was
//! that rebuild rather than the committed bytes, because the subject's decoder could not read a
//! real page's geometry. Both are gone, and with them the carve-out: the laws are now measured
//! against the REAL DOCUMENT's own projection, all 65 pages of it.
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module and under `semantic-pdf-v1`'s own tolerance, so a scenario cannot pass merely because
//! `lopdf` declined to error: `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>`
//! must land back on the un-mutated document's projection, and `identity-round-trip` must recover
//! the real input's own page tree from bytes that differ from the input. Nothing is exempt.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_pdf_1_4, KINDS};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bachelor-thesis.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Profile
/// 📏️ `semantic-pdf-v1`'s own declared freedom list and tolerance (`../../../../🧪️oracle/
/// 🔣️component.json`), mirrored here so an in-handler law check is exactly as strict as the profile
/// the case is measured by — never stricter, never looser.
const PDF_WRITER_FREEDOM: &[&str] = &["objectNumber", "xrefOffset", "producer", "creationDate", "modificationDate", "documentId", "fileSize", "byteLength", "generation", "streamFilter", "streamLength"];
const PDF_TOLERANCE: f64 = 0.0001;
//#endregion 🔖️Profile

//#region 🔖️Oracle
/// 🦠️ Forward observability against the real document projection.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pdf_1_4(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_pdf_1_4(&input)?, &[], PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Independently restores the real document using the concrete inverse spec.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&input, &spec)?)?;
    let projection = project_pdf_1_4(&restored)?;
    inverse_restores_within(&kind, &projection, &project_pdf_1_4(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted rather than merely produced: the
/// reference re-serializes the real document from its own object graph, the INDEPENDENT reader
/// reads that back, and the whole page tree must survive — plus the emitted bytes must differ from
/// the input, so nothing can pass by copying.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_pdf_1_4(&bytes)?;
    round_trip_preserves_within(&projection, &project_pdf_1_4(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::io::{decode_pdf, encode_pdf};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::{apply_pdf_mutation, inverse_pdf_mutation, InsertPage, MovePage, PdfMutation, RemovePage, ReplacePageText, ResizePage};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;
    use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::any::project_pdf_1_4;

    fn mutation_from_spec(spec: &Json) -> Result<PdfMutation, String> {
        let params = spec.get("params").ok_or("Missing mutation parameters")?;
        let number = |key| match params.get(key) {
            Some(Json::Number(value)) if value.is_finite() => Ok(*value),
            _ => Err(format!("{key} must be finite")),
        };
        let index = |key| {
            let value = number(key)?;
            if value < 0.0 || value.fract() != 0.0 || value >= usize::MAX as f64 {
                Err(format!("{key} must be an index"))
            } else {
                Ok(value as usize)
            }
        };
        Ok(match spec.str("kind").as_str() {
            "insert-page" => {
                let page = params.get("page").ok_or("Missing inserted page")?;
                let dimension = |key| match page.get(key) {
                    Some(Json::Number(value)) if value.is_finite() => Ok(*value),
                    _ => Err(format!("{key} must be finite")),
                };
                PdfMutation::InsertPage(InsertPage { index: index("index")?, page: PageDoc { width: dimension("width")?, height: dimension("height")?, text: page.str("text") } })
            }
            "remove-page" => PdfMutation::RemovePage(RemovePage { index: index("index")? }),
            "move-page" => PdfMutation::MovePage(MovePage { from: index("from")?, to: index("to")? }),
            "resize-page" => PdfMutation::ResizePage(ResizePage { index: index("index")?, width: number("width")?, height: number("height")? }),
            "replace-page-text" => PdfMutation::ReplacePageText(ReplacePageText { index: index("index")?, text: params.str("text") }),
            other => return Err(format!("Unknown subject mutation {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_pdf(&mutable_input(ctx)?).map_err(|error| error.to_string())?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let outcome = apply_pdf_mutation(&mut snapshot, &mutation);
        if !outcome.messages().is_empty() {
            return Err(format!("Mutation refused: {:?}", outcome.messages()));
        }
        let output = encode_pdf(&snapshot).map_err(|error| error.to_string())?;
        let projection = project_pdf_1_4(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_pdf(&mutable_input(ctx)?).map_err(|error| error.to_string())?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let inverse = inverse_pdf_mutation(&mutation, &base);
        let mut snapshot = base.clone();
        if !apply_pdf_mutation(&mut snapshot, &mutation).messages().is_empty() {
            return Err("Forward mutation refused".into());
        }
        for step in inverse {
            if !apply_pdf_mutation(&mut snapshot, &step).messages().is_empty() {
                return Err("Inverse mutation refused".into());
            }
        }
        if snapshot != base {
            return Err("Concrete inverse did not restore the complete snapshot".into());
        }
        let output = encode_pdf(&snapshot).map_err(|error| error.to_string())?;
        let projection = project_pdf_1_4(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_pdf(&input).map_err(|error| error.to_string())?;
        let output = encode_pdf(&snapshot).map_err(|error| error.to_string())?;
        if output == input {
            return Err("Byte pass-through instead of reconstruction".into());
        }
        let projection = project_pdf_1_4(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so both roles are registered in one loop over the subset's own `KINDS` — the same list the
/// oracle module's `kinds_matches_the_catalog_and_every_feature_row` pins against the catalog and
/// the feature's `Examples` rows.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
