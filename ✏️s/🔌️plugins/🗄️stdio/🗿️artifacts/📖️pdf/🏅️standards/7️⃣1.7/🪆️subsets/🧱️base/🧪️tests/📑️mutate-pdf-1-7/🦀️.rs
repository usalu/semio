//! 🦀️ PDF 1.7 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory
//! first; the committed asset is never written to. `oracle` drives the registered `lopdf` reference
//! implementation (`../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_pdf`/`encode_pdf`/`apply_pdf_mutation` over the `PdfMutation` vocabulary, each row's `params` being the
//! leaf wire payload `decode_pdf_mutation_payload` builds the operation from.
//! Both results are read back by the SAME independent `project_pdf_1_7` (`lopdf`, augmented with
//! each page's `/CropBox` and `/Rotate` and with the resolved trailer/catalog object graph) before
//! the `semantic-pdf-v1` profile compares them. The subject half is gated behind the generated
//! host's `sut` feature so the oracle-only run never compiles the local implementation.
//!
//! 🩺 What the differential runs found, each fixed at the cause. First, `encode_pdf` serialized the retained COS graph
//! alone, so every edit in the authored `pages`/`info` lanes applied and then vanished on export. Then its fix
//! regenerated every typed object whenever any lane moved, so a page edit rewrote the catalog and a direct COS edit
//! (`set-object-value` #145, `set-dict-entry`, `remove-object`, the trailer kinds) was overwritten by the stale typed
//! lane, and a custom trailer entry was dropped outright. `encode_pdf` now reconciles with incremental-writer
//! semantics (`../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🦀️.rs`, `reconcile`): a graph edit carries the typed
//! lanes it moves (`carry_graph_edit`), a moved lane re-states only the objects and entries it owns, and every other
//! object, entry and trailer key is written as it stands. Last, the reference's own undo of the three page-content
//! kinds rebuilt a page from its `Tj` text alone; it now captures the page's operators verbatim, so every inverse is
//! held to the whole projection.
//!
//! No measured ratio is recorded here. A parity figure in source is a claim about one moment that
//! silently becomes false when anything moves; the dated ticket record is where measurements live.
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law`
//! module and under `semantic-pdf-v1`'s own tolerance, so a scenario cannot pass merely because
//! `lopdf` declined to error: `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>`
//! must land back on the untouched document's projection, and `identity-round-trip` must both
//! preserve the projection and produce bytes that differ from the input. The one carve-out — one
//! kind exempt from observability — is named by the subset's own oracle module (`UNOBSERVABLE`),
//! argued there in full, and repeated in this case's feature description.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_pdf_1_7, UNOBSERVABLE};
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};

//#region 🔖️Input
const INPUT: &str = "asset://🎓️bachelor-thesis/🎓️bachelor-thesis.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("bachelor-thesis.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

//#endregion 🔖️Input

//#region 🔖️Profile
/// 📏️ `semantic-pdf-v1`'s own declared freedom list and tolerance
/// (`✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔣️.json`), mirrored here so an in-handler law check is exactly as strict as the profile
/// the case is measured by — never stricter, which would invent a failure the comparison itself
/// would forgive, and never looser, which would let a real one through.
const PDF_WRITER_FREEDOM: &[&str] = &["objectNumber", "xrefOffset", "producer", "creationDate", "modificationDate", "documentId", "fileSize", "byteLength", "generation", "streamFilter", "streamLength"];
const PDF_TOLERANCE: f64 = 0.0001;
//#endregion 🔖️Profile

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: the reference applies the kind
/// to the real thesis and the result has to differ from the untouched document under the very
/// profile the case is measured by. Returning the projection uncompared is what made these
/// eighteen scenarios pass whenever `lopdf` merely did not error. The one exemption is this
/// subset's own [`UNOBSERVABLE`], which names `insert-object` and says why in full.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pdf_1_7(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_pdf_1_7(&input)?, UNOBSERVABLE, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The INVERSE law, asserted in role without needing the subject: `apply(inverse(m), apply(m,
/// base))` must land back on the ORIGINAL document's own projection, read through the same
/// independent reader, every axis included.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_pdf_1_7(&bytes)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_pdf_1_7(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity law, both halves asserted: `lopdf` fully parses the real
/// document and re-serializes it from its own object graph alone, the re-serialized bytes must
/// differ from the input — our
/// encoder cannot reproduce another writer's object layout, so bit-identical output would mean the
/// input was smuggled rather than parsed — and the projection must survive intact.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_pdf_1_7(&bytes)?;
    round_trip_preserves_within(&projection, &project_pdf_1_7(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, PDF_TOLERANCE, PDF_WRITER_FREEDOM};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::UNOBSERVABLE;
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_pdf::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;

    //#region 🔖️SpecCodec
    /// 📨️ The scenario's `{kind, params}` row decoded generically: `params` is the leaf wire payload, the only channel
    /// between the feature and the subject's typed `PdfMutation`.
    fn mutation_from_spec(spec: &Json) -> Result<PdfMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    /// 🦠️ The forward half with the OBSERVABILITY law asserted IN THE SUBJECT ROLE, against the
    /// untouched real document and under the same profile the oracle half uses. The one exemption
    /// is the subset's own [`UNOBSERVABLE`], shared verbatim with the oracle half.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_pdf(&input).map_err(|error| format!("decode_pdf failed: {error:?}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let mut snapshot = base;
        apply_pdf_mutation(&mut snapshot, &mutation);
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {error:?}"))?;
        let projection = project_pdf_1_7(&bytes)?;
        mutation_is_observable_within(&spec.str("kind"), &projection, &project_pdf_1_7(&input)?, UNOBSERVABLE, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The INVERSE law asserted IN THE SUBJECT ROLE, against the untouched real document and
    /// under the same profile the oracle half uses: the production inverse restores every axis,
    /// `pages.N.contentOperators` included — a mutation that returns the authored lane to its base
    /// value leaves nothing for the reconciling writer to rewrite.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_pdf(&input).map_err(|error| format!("decode_pdf failed: {error:?}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let undo = mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
        let mut snapshot = base;
        apply_pdf_mutation(&mut snapshot, &mutation);
        for operation in undo {
            apply_pdf_mutation(&mut snapshot, &operation);
        }
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {error:?}"))?;
        let projection = project_pdf_1_7(&bytes)?;
        inverse_restores_within(&spec.str("kind"), &projection, &project_pdf_1_7(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_pdf`/`encode_pdf` are this
    /// subset's ONLY channel from input to output (no separate text-DSL layer over the snapshot).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_pdf(&input).map_err(|error| format!("decode_pdf failed: {error:?}"))?;
        let output = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {error:?}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_pdf_1_7(&output)?;
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
