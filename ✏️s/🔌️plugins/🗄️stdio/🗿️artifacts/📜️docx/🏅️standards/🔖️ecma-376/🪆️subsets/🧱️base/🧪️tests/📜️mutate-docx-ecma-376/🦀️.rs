//! 🦀️ DOCX ECMA-376/✳️any exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
//!
//! Every scenario copies the real, committed `📜️example-readme.docx` fixture (derived once from
//! this repository's own real `README.md` — see the feature file's own header for the full
//! provenance) into the case work directory first; the committed fixture is never written to.
//! The judging oracle is the TypeScript reader (`🟦️.ts`, jszip over the committed python-docx afters and the real
//! README); `oracle` here is the cross-semio SUPPLEMENT, the registered `zip`+`quick-xml` composition
//! (`oracle_apply_mutation`/`oracle_apply_mutation_inverse`), asserting its laws in role; `subject` drives this
//! repository's own `decode_docx`/`encode_docx`/`apply_docx_mutation` over the `DocxMutation` vocabulary — every row's `params`
//! is the leaf wire payload `decode_docx_mutation_payload` builds the operation from — and hands its package to the `docx-ecma-376-jszip-compare-v1` pipeline as `actual-docx`. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never links `semio-s-plugin-stdio` -- §5.3's own role separation, NOT a workaround for anything:
//! the Rust subject phase runs (`subject exhaustive --owner 🗄️stdio --case mutate-docx-ecma-376`
//! executes all 25 scenarios), and wave 14 ran the full differential comparison against the oracle.
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law`
//! module, so a scenario cannot pass merely because `zip`+`quick-xml` declined to error:
//! `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>` must land back on the
//! untouched package's projection, and `identity-round-trip` must both preserve the projection and
//! rebuild an archive that differs from the input. There is no carve-out of any kind here: the
//! profile declares no writer freedom, no kind is exempt from observability, and no axis is dropped
//! from the inverse law. The one inverse this vocabulary genuinely cannot express — removing an
//! INTERIOR style, which `InsertStyle`'s append can never put back — is refused by the oracle
//! outright rather than faked, and the feature says so.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_docx_test_oracle::standards::v_ecma_376::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_replace_package, oracle_round_trip, project_docx_ecma_376};
use semio_repo_test_host::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://📜️example-readme.docx";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("example-readme.docx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 📸️ The committed after-document whose decoded snapshot the `set-snapshot` scenarios replace the README with.
fn set_snapshot_document(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture("shared://🧾️readme-afters/📸️set-snapshot/➡️after.docx", Some("set-snapshot-after.docx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: the reference composition
/// applies the kind to the real README document and the result has to differ from the untouched
/// package. Returning the projection uncompared is what made these scenarios pass whenever
/// `zip`+`quick-xml` merely did not error. NOTHING is exempt — `semantic-docx-ecma-376-mutate-v1`
/// declares no writer freedom at all (`ignoreKeys: []`), and the ordered block tree, the ordered
/// style list and the path-keyed digest of every other OPC part between them reach every
/// declared kind.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_docx_ecma_376(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_docx_ecma_376(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The INVERSE law, asserted in role without needing the subject: `apply(inverse(m), apply(m,
/// base))` must land back on the ORIGINAL package's own projection, read through the same
/// independent reader. No axis is dropped and no tolerance is allowed.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_docx_ecma_376(&bytes)?;
    inverse_restores(&kind, &projection, &project_docx_ecma_376(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity law, both halves asserted: the `zip`+`quick-xml` composition
/// fully parses the real package and re-serializes it from its own trees alone (the same
/// `no-mutation` routing every other kind goes through), the rebuilt archive must differ from the
/// input — two independent writers do not agree on compression level, extra fields, entry order or
/// attribute layout, so bit-identical output would mean the input was copied — and the projection
/// must survive intact.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_docx_ecma_376(&bytes)?;
    round_trip_preserves(&projection, &project_docx_ecma_376(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
/// 📸️ The reference side of the whole-document replacement: the committed after-document read and re-written by the
/// `zip`+`quick-xml` composition in place of the README, which has to move the compared projection.
fn set_snapshot_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_replace_package(&input, &set_snapshot_document(ctx)?)?;
    let projection = project_docx_ecma_376(&bytes)?;
    mutation_is_observable("set-snapshot", &projection, &project_docx_ecma_376(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The replacement undone by replacing back, which must land on the untouched README's projection.
fn set_snapshot_inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_replace_package(&oracle_replace_package(&input, &set_snapshot_document(ctx)?)?, &input)?;
    let projection = project_docx_ecma_376(&bytes)?;
    inverse_restores("set-snapshot", &projection, &project_docx_ecma_376(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, set_snapshot_document};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_docx;
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_docx::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_docx;
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::schema::mutations::{apply_docx_mutation, set_snapshot};
    use semio_s_artifact_stdio_docx::{DocxMutation, DocxSnapshot};
    use semio_s_artifact_stdio_docx_test_oracle::standards::v_ecma_376::subsets::base::project_docx_ecma_376;

    fn decode(bytes: &[u8]) -> Result<DocxSnapshot, String> {
        decode_docx(bytes).map_err(|error| format!("decode_docx failed: {error}"))
    }

    fn encode(snapshot: &DocxSnapshot) -> Result<Vec<u8>, String> {
        encode_docx(snapshot).map_err(|error| format!("encode_docx failed: {error}"))
    }

    /// 📨️ The scenario's `{kind, params}` row decoded generically: `params` is the leaf wire payload, the only channel
    /// between the feature and the subject's typed `DocxMutation`.
    fn mutation_from_spec(spec: &Json) -> Result<DocxMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// ↩️ Applies `mutation` to `base` and then the production inverse it plans on `base` — the law every inverse
    /// scenario holds this implementation to.
    fn applied_and_undone(base: &DocxSnapshot, mutation: &DocxMutation) -> DocxSnapshot {
        let mut snapshot = base.clone();
        apply_docx_mutation(&mut snapshot, mutation);
        for undo in mutation_inverse(mutation, base) {
            apply_docx_mutation(&mut snapshot, &undo);
        }
        snapshot
    }

    /// 📸️ The whole-document replacement by the snapshot this repository's own codec decodes from the committed
    /// after-document — a `set-snapshot` payload is an entire package, not a table cell.
    fn replacement(ctx: &Context) -> Result<DocxMutation, String> {
        Ok(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: decode(&set_snapshot_document(ctx)?)? }))
    }

    /// 📦️ The produced package as the `actual-docx` artifact the `docx-ecma-376-jszip-compare-v1` pipeline reads.
    fn actual(ctx: &Context, bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_docx_ecma_376(&bytes)?;
        let path = ctx.artifact("actual-docx", "actual.docx")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-docx", &path, "application/vnd.openxmlformats-officedocument.wordprocessingml.document"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        apply_docx_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        actual(ctx, encode(&snapshot)?)
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode(&mutable_input(ctx)?)?;
        actual(ctx, encode(&applied_and_undone(&base, &mutation_from_spec(&ctx.doc_json()?)?))?)
    }

    pub fn mutate_set_snapshot(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        apply_docx_mutation(&mut snapshot, &replacement(ctx)?);
        actual(ctx, encode(&snapshot)?)
    }

    pub fn inverse_set_snapshot(ctx: &Context) -> Result<Outcome, String> {
        let base = decode(&mutable_input(ctx)?)?;
        actual(ctx, encode(&applied_and_undone(&base, &replacement(ctx)?))?)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_docx`/`encode_docx` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let output = encode(&decode(&input)?)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        actual(ctx, output)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("mutate-set-snapshot", set_snapshot_oracle).oracle("inverse-set-snapshot", set_snapshot_inverse_oracle);
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("mutate-set-snapshot", subject::mutate_set_snapshot).subject("inverse-set-snapshot", subject::inverse_set_snapshot);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
