//! 🦀️ XLSX ECMA-376 ✳️strict exhaustive conformance-class mutation case — Rust adapter.
//!
//! The input is the real committed `📕️reuse-marketplaces.xlsx`, an 11-part OPC package with two
//! worksheets and a 229-entry shared-string table, which fails this class on its first two axes —
//! Transitional SpreadsheetML `xmlns` and Transitional `xmlns:r` on `xl/workbook.xml`. A workbook is
//! the only OOXML package with a PER-WORKSHEET axis, and `set-worksheet-content-type` moves exactly
//! the `[Content_Types].xml` Override that carries it, which is a kind neither the DOCX nor the PPTX
//! conformance subsets have. `oracle` handlers drive the registered `quick-xml` 0.42 + `zip` 6 pair
//! through this subset's own `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧪️oracle/🦀️component.rs`;
//! `subject` handlers drive `decode_xlsx`/`apply_xlsx_strict_mutation`/`encode_xlsx`; both results
//! are read back by the SAME independent `project_package` before `semantic-ooxml-xlsx-strict-v1`
//! compares them. The subject half is `sut`-gated so the oracle-only run never compiles the local
//! implementation.
//!
//! ⚖️ All three laws are asserted IN ROLE through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module, under a profile that declares no writer freedom at all, and no kind is exempt from any
//! of them. Two of the nine — `remove-conformance-attribute` and `remove-vml-part` — run against a
//! pre-state [`arranged_input`] builds with the SAME reference implementation. Worth stating
//! plainly: this case IS differential, unlike `mutate-xlsx-ecma-376`, where no single crate both
//! reads and writes a workbook and `calamine`/`rust_xlsxwriter` have to be composed — at the
//! CONTAINER level `zip` + `quick-xml` read and write on their own. What it cannot witness is cell
//! content: the two worksheets and the shared-string table are carried faithfully and read by no
//! class axis.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::strict::{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_package, KINDS};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://📕️reuse-marketplaces.xlsx";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.xlsx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's mutation runs on. `remove-conformance-attribute` and
/// `remove-vml-part` need a target this workbook does not have — no ECMA-376 package in this
/// repository carries either, verified by unzipping all three committed OOXML fixtures — so for
/// those two this is the real 11-part package after the reference implementation has independently
/// put their target into it. The other seven kinds read the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: each kind has to move the
/// five-axis Strict projection of the real workbook, or its scenario would pass whether or not the
/// mutation ran. Nothing is exempt — `set-worksheet-content-type` included, which moves a
/// `[Content_Types].xml` Override rather than any part's own markup.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let output = oracle_apply_mutation(&base, &spec)?;
    let projection = project_package(&output)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_package(&base)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The INVERSE law, asserted in role and against the ARRANGED pre-state rather than the committed
/// bytes: for the two arranged kinds the workbook to be restored is the one with the target already
/// inserted, which is the only baseline a removal can honestly be undone onto.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let undo = oracle_inverse_spec(&base, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_package(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_package(&base)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: `zip` re-reads all 11 entries and rebuilds the
/// container from those entries alone, so the rebuilt workbook must differ from the input while its
/// conformance-class projection must not move at all.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    reparsed_not_copied(&output, &input)?;
    let projection = project_package(&output)?;
    round_trip_preserves(&projection, &project_package(&input)?)?;
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_xlsx;
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_xlsx;
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::mutations::{apply_xlsx_strict_mutation, stamp_conformance_class, vml_markup, XlsxStrictMutation};
    use semio_s_plugin_stdio::artifacts::xlsx::XlsxSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::strict::{oracle_inverse_spec, project_package};

    fn decode(bytes: &[u8]) -> Result<XlsxSnapshot, String> {
        decode_xlsx(bytes).map_err(|error| error.to_string())
    }

    fn encode(snapshot: &XlsxSnapshot) -> Result<Vec<u8>, String> {
        encode_xlsx(snapshot).map_err(|error| error.to_string())
    }

    /// 🏅️ The whole-package class stamp `set-snapshot` replaces the document with, built by this
    /// repository's own code from the real decoded snapshot.
    fn stamped(base: &XlsxSnapshot, strict: bool) -> Result<XlsxSnapshot, String> {
        Ok(stamp_conformance_class(base.clone(), strict))
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `XlsxStrictMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(ctx: &Context, spec: &Json) -> Result<XlsxStrictMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => XlsxStrictMutation::NoMutation,
            "set-snapshot" => XlsxStrictMutation::SetSnapshot { snapshot: stamped(&decode(&mutable_input(ctx)?)?, params.str("conformanceClass") == "strict")? },
            "set-main-namespace" => XlsxStrictMutation::SetMainNamespace { namespace: params.str("namespace") },
            "set-relationships-namespace" => XlsxStrictMutation::SetRelationshipsNamespace { namespace: params.str("namespace") },
            "set-conformance-attribute" => XlsxStrictMutation::SetConformanceAttribute { value: params.str("value") },
            "remove-conformance-attribute" => XlsxStrictMutation::RemoveConformanceAttribute,
            "insert-vml-part" => XlsxStrictMutation::InsertVmlPart { path: params.str("path"), markup: vml_markup() },
            "remove-vml-part" => XlsxStrictMutation::RemoveVmlPart { path: params.str("path") },
            "set-worksheet-content-type" => XlsxStrictMutation::SetWorksheetContentType { path: params.str("path"), content_type: params.str("contentType") },
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(ctx, &spec)?;
        apply_xlsx_strict_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = arranged_input(ctx, &spec)?;
        let mut snapshot = decode(&base)?;
        apply_xlsx_strict_mutation(&mut snapshot, &mutation_from_spec(ctx, &spec)?);
        let undo = oracle_inverse_spec(&base, &spec)?;
        apply_xlsx_strict_mutation(&mut snapshot, &mutation_from_spec(ctx, &undo)?);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode(&snapshot)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_package(&output)?;
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
