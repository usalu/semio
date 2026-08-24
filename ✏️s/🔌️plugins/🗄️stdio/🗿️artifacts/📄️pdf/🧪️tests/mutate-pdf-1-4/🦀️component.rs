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
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module and under `semantic-pdf-v1`'s own tolerance, so a scenario cannot pass merely because
//! `lopdf` declined to error: `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>`
//! must land back on the un-mutated document's projection, and `identity-round-trip` must recover
//! the real input's own page-1 text from bytes that differ from the input. Nothing is exempt from
//! observability here; what the laws are measured AGAINST is the reference's own rebuild rather
//! than the committed bytes, and [`rebuilt_base`] argues why in full.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_4::subsets::any::{build_single_page_pdf, independent_first_text, oracle_apply_mutation, project_pdf_1_4, KINDS};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied};

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

/// ↩️ The spec that undoes `kind` against a document whose page-1 text was `base_text`. Both
/// declared kinds invert in closed form: `NoMutation`'s inverse is itself, and `SetSnapshot`'s
/// restores the original base — the same `impl Mutation<PdfSnapshot>` this subset's own
/// `../../🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` declares, written out
/// here so the ORACLE-only build needs no production dependency to compute it.
fn inverse_spec(kind: &str, base_text: &str) -> Json {
    Json::Object(vec![
        ("kind".to_string(), Json::String(kind.to_string())),
        ("params".to_string(), Json::Object(vec![("snapshot".to_string(), Json::Object(vec![("page".to_string(), Json::Object(vec![("text".to_string(), Json::String(base_text.to_string()))]))]))])),
    ])
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
/// 🎬️ The pre-state every law here is measured against: the reference's OWN rebuild of the real
/// document with nothing mutated.
///
/// ⚖️ This, and not `project_pdf_1_4(&input)`, is the honest baseline, and the reason is a documented
/// property of the subset rather than a softened law. The oracle is a rebuild-from-text writer that
/// pins `MediaBox [0 0 612 792]` for every document, mirroring `decode_pdf`, which hardcodes the
/// same constant and never reads a real page's geometry (this fixture's true box is
/// `[0 0 595.276 841.89]`). Measured against the committed input, `set-snapshot` would be credited
/// with a `595.276 → 612` move the REBUILD made and the mutation did not — a green for something
/// never observed. Measured against the rebuild, the only field that can move is `text`, the one
/// this subset genuinely reads out of a document, and it has to.
fn rebuilt_base(input: &[u8]) -> Result<Json, String> {
    project_pdf_1_4(&oracle_apply_mutation(input, &Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]))?)
}

/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: `set-snapshot` has to move the
/// projection this case is compared through, or its scenario would pass whether or not the
/// mutation ran. No kind is exempt — `no-mutation` is the law's own base case, not a carve-out.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pdf_1_4(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &rebuilt_base(&input)?, &[], PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies the forward mutation, then its algebraic inverse, and ASSERTS the law: the restored
/// document must project onto exactly what the un-mutated rebuild projects onto. `SetSnapshot`'s
/// inverse restores the ORIGINAL base, so the base's `text` is read independently out of the real
/// input before the forward mutation ever runs; `NoMutation`'s inverse is itself, so that row
/// naturally degenerates to applying `no-mutation` twice.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let base_text = independent_first_text(&input)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &base_text))?;
    let projection = project_pdf_1_4(&restored)?;
    inverse_restores_within(&kind, &projection, &rebuilt_base(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted rather than merely produced: the
/// reference writer emits the real document's own page-1 text through `lopdf`'s object writer, the
/// INDEPENDENT reader reads that back, and the two must agree — plus the emitted bytes must differ
/// from the input, so nothing can pass by copying.
///
/// 🚫️ `width`/`height` are deliberately NOT part of this law, for the reason [`rebuilt_base`] gives:
/// the subset pins `612×792` on both sides by documented design, so the real fixture's
/// `595.276×841.89` is unreachable by construction and a check demanding it would be contrived
/// rather than true. What the law does cover is the whole of what this subset reads out of a
/// document: `text`.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let base_text = independent_first_text(&input)?;
    let bytes = build_single_page_pdf(&base_text)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_pdf_1_4(&bytes)?;
    let recovered = projection.str("text");
    if recovered != base_text {
        return Err(format!("identity law violated: the independent reader recovered {recovered:?} from the re-encoded document, but the real input carries {base_text:?}"));
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
