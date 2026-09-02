//! 🦀️ XLSX ECMA-376 ✳️transitional exhaustive conformance-class mutation case — Rust adapter.
//!
//! The input is the real committed `📕️reuse-marketplaces.xlsx`, an 11-part OPC package that ALREADY
//! satisfies this class on all four of its axes: Transitional SpreadsheetML `xmlns`, Transitional
//! `xmlns:r`, no `conformance` attribute claiming `strict`, and the ordinary worksheet content type
//! on both sheet parts. This case is the mirror of `mutate-xlsx-ecma-376-strict` over the same
//! bytes: seven kinds move the workbook OUT of the ISO/IEC 29500-4 class and back in, including
//! along the per-worksheet content-type axis no DOCX or PPTX conformance subset has. `oracle`
//! handlers drive the registered `quick-xml` 0.42 + `zip` 6 pair through this subset's own
//! `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️transitional/🦀️oracle.rs`; `subject`
//! handlers drive `decode_xlsx`/`apply_xlsx_transitional_mutation`/`encode_xlsx`; both results are
//! read back by the SAME independent `project_package` before
//! `semantic-ooxml-xlsx-transitional-v1` compares them. The subject half is `sut`-gated so the
//! oracle-only run never compiles the local implementation.
//!
//! ⚖️ All three laws are asserted IN ROLE through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module, under a profile that declares no writer freedom at all, and no kind is exempt from any
//! of them. Only ONE kind — `remove-conformance-attribute` — needs an arranged pre-state. This
//! catalog has NO VML rule at all: ISO/IEC 29500-4 Transitional deliberately retains VML, so
//! policing it would be inventing a rule the specification does not have, and that is the whole
//! reason this subset declares two kinds fewer than its ✳️strict sibling.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_package, KINDS};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://📕️reuse-marketplaces.xlsx";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.xlsx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's mutation runs on. Only `remove-conformance-attribute` needs
/// one: no ECMA-376 package in this repository carries a conformance attribute, verified by
/// unzipping all three committed OOXML fixtures, so that kind runs on the real 11-part workbook
/// after the reference implementation has independently stamped one onto `xl/workbook.xml`. The
/// other six kinds read the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: each kind has to move the
/// four-axis Transitional projection of the real workbook, or its scenario would pass whether or
/// not the mutation ran. Nothing is exempt — `set-worksheet-content-type` included, which moves a
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
/// bytes: for `remove-conformance-attribute` the workbook to be restored is the stamped one, which
/// is the only baseline that removal can honestly be undone onto.
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
    use semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::{apply_xlsx_transitional_mutation, stamp_conformance_class, XlsxTransitionalMutation};
    use semio_s_plugin_stdio::artifacts::xlsx::XlsxSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::{oracle_inverse_spec, project_package};

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
    /// `XlsxTransitionalMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(ctx: &Context, spec: &Json) -> Result<XlsxTransitionalMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "set-snapshot" => XlsxTransitionalMutation::SetSnapshot(semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::set_snapshot::SetSnapshot { snapshot: stamped(&decode(&mutable_input(ctx)?)?, params.str("conformanceClass") == "strict")? }),
            "set-main-namespace" => XlsxTransitionalMutation::SetMainNamespace(semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::set_main_namespace::SetMainNamespace { namespace: params.str("namespace") }),
            "set-relationships-namespace" => XlsxTransitionalMutation::SetRelationshipsNamespace(semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::set_relationships_namespace::SetRelationshipsNamespace { namespace: params.str("namespace") }),
            "set-conformance-attribute" => XlsxTransitionalMutation::SetConformanceAttribute(semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::set_conformance_attribute::SetConformanceAttribute { value: params.str("value") }),
            "remove-conformance-attribute" => XlsxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
            "set-worksheet-content-type" => XlsxTransitionalMutation::SetWorksheetContentType(semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::set_worksheet_content_type::SetWorksheetContentType { path: params.str("path"), content_type: params.str("contentType") }),
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(ctx, &spec)?;
        apply_xlsx_transitional_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = arranged_input(ctx, &spec)?;
        let mut snapshot = decode(&base)?;
        apply_xlsx_transitional_mutation(&mut snapshot, &mutation_from_spec(ctx, &spec)?);
        let undo = oracle_inverse_spec(&base, &spec)?;
        apply_xlsx_transitional_mutation(&mut snapshot, &mutation_from_spec(ctx, &undo)?);
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
