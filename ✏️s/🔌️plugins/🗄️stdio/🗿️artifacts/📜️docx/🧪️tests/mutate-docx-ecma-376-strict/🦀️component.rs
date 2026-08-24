//! 🦀️ DOCX ECMA-376 ✳️strict exhaustive conformance-class mutation case — Rust adapter.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. `oracle` handlers drive the registered `quick-xml` + `zip`
//! reference implementation through this subset's own `🧪️oracle/🦀️component.rs`, `subject` handlers
//! drive this repository's own decode/mutate/encode round trip, and both results are read back by
//! the SAME independent reader (`project_package`) before the
//! `semantic-ooxml-docx-strict-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::strict::{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_package};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `docx-ecma-376-strict` catalog. Duplicated, not imported,
/// from `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️strict/🧬️schema/🧬️mutations/🦀️component.rs::KINDS`
/// — that module lives in the SUBJECT crate, and the oracle role must not link the subject crate at
/// all, while this loop registers handlers for both roles from one list. A mismatch here is caught
/// structurally: the contract phase fails with `mutation-kind-uncovered`/`mutation-kind-undeclared`
/// if this list omits or invents a kind, and the runner fails every unregistered scenario id
/// outright.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "insert-vml-part", "remove-vml-part", "insert-alternate-content", "remove-alternate-content"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📜️example-readme.docx";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.docx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's mutation runs on. For the removal kinds this is the real
/// package after the reference implementation has independently inserted their target — no committed
/// ECMA-376 package in this repository carries VML, `mc:AlternateContent` or a `conformance`
/// attribute, which the Feature file records rather than papers over. Every other kind reads the
/// committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let output = oracle_apply_mutation(&base, &spec)?;
    let projection = project_package(&output)?;
    if spec.str("kind") != "no-mutation" && projection == project_package(&base)? {
        return Err(format!("{:?} left the conformance-class projection unchanged — a mutation that is not observable proves nothing", spec.str("kind")));
    }
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ Applies the mutation and then its independently computed inverse, and asserts the metamorphic
/// law on the oracle side too: a run with no subject must still be evidence, not a recorded no-op.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let undo = oracle_inverse_spec(&base, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_package(&restored)?;
    if projection != project_package(&base)? {
        return Err(format!("undoing {:?} did not restore the package's conformance-class projection", spec.str("kind")));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The reference implementation's own container decode/re-encode — proves the independent codec
/// is projection-stable on the real package before the subject's own codec is asked to be.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    if output == input {
        return Err("byte pass-through: output is bit-identical to the input".to_string());
    }
    let projection = project_package(&output)?;
    if projection != project_package(&input)? {
        return Err("the reference container round trip is not projection-stable on the real package".to_string());
    }
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_docx;
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_docx;
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::mutations::{apply_docx_strict_mutation, stamp_conformance_class, vml_markup, DocxStrictMutation};
    use semio_s_plugin_stdio::artifacts::docx::DocxSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::strict::{oracle_inverse_spec, project_package};

    fn decode(bytes: &[u8]) -> Result<DocxSnapshot, String> {
        decode_docx(bytes).map_err(|error| error.to_string())
    }

    fn encode(snapshot: &DocxSnapshot) -> Result<Vec<u8>, String> {
        encode_docx(snapshot).map_err(|error| error.to_string())
    }

    /// 🏅️ The whole-package class stamp `set-snapshot` replaces the document with, built by this
    /// repository's own code from the real decoded snapshot.
    fn stamped(base: &DocxSnapshot, strict: bool) -> Result<DocxSnapshot, String> {
        Ok(stamp_conformance_class(base.clone(), strict))
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `DocxStrictMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(ctx: &Context, spec: &Json) -> Result<DocxStrictMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => DocxStrictMutation::NoMutation,
            "set-snapshot" => DocxStrictMutation::SetSnapshot { snapshot: stamped(&decode(&mutable_input(ctx)?)?, params.str("conformanceClass") == "strict")? },
            "set-main-namespace" => DocxStrictMutation::SetMainNamespace { namespace: params.str("namespace") },
            "set-relationship-base" => DocxStrictMutation::SetRelationshipBase { base: params.str("base") },
            "set-conformance-attribute" => DocxStrictMutation::SetConformanceAttribute { value: params.str("value") },
            "remove-conformance-attribute" => DocxStrictMutation::RemoveConformanceAttribute,
            "insert-vml-part" => DocxStrictMutation::InsertVmlPart { path: params.str("path"), markup: vml_markup() },
            "remove-vml-part" => DocxStrictMutation::RemoveVmlPart { path: params.str("path") },
            "insert-alternate-content" => DocxStrictMutation::InsertAlternateContent { path: params.str("path") },
            "remove-alternate-content" => DocxStrictMutation::RemoveAlternateContent { path: params.str("path") },
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(ctx, &spec)?;
        apply_docx_strict_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = arranged_input(ctx, &spec)?;
        let mut snapshot = decode(&base)?;
        apply_docx_strict_mutation(&mut snapshot, &mutation_from_spec(ctx, &spec)?);
        let undo = oracle_inverse_spec(&base, &spec)?;
        apply_docx_strict_mutation(&mut snapshot, &mutation_from_spec(ctx, &undo)?);
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
