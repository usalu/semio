//! 🦀️ BCF 2.1 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies its own committed input into the case work directory first — a mutation or inverse row its
//! pair's `⬅️before.bcf`, the identity round trip the real `🏥️wellness-center-coordination-review.bcf` (derived once
//! from a real IFC2X3 model plus a real committed floor plan PNG — see the feature file's own header); committed
//! fixtures are never written to. The judging oracle is the TypeScript reader (`🟦️.ts`, jszip over the committed
//! documents); `oracle` here is the cross-semio SUPPLEMENT, the registered independent `zip`+`quick-xml` composition
//! (`oracle_apply_mutation`/`oracle_apply_mutation_inverse`), asserting its laws in role; `subject` drives this
//! repository's own `decode_bcf`/`encode_bcf`/`apply_bcf_mutation` over the `BcfMutation` vocabulary and hands its
//! archive to the `bcf-2-1-jszip-compare-v1` pipeline as `actual-bcf`. The subject half is gated behind the generated
//! host's `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::bcf::standards::v2_1::subsets::markup::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_bcf_2_1};


//#region 🔖️Input
const INPUT: &str = "shared://🏥️wellness-center-coordination-review.bcf";

/// 🧫️ Copies the scenario's own committed input — its pair's `⬅️before.bcf`, else the real coordination review — into
/// the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let uri = ctx.step_fixture_uris().into_iter().find(|uri| uri.ends_with("/⬅️before.bcf")).unwrap_or_else(|| INPUT.to_string());
    let copy = ctx.copy_fixture(&uri, Some("coordination-review.bcf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Laws
/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law
/// violation must name the field that broke it rather than dump two whole documents at the reader.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&format!("{path}.{key}"), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{path}.{key} is absent from the result")),
                }
            }
            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!("{path}.{key} appeared in the result out of nowhere"))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{path} holds {} member(s), expected {}", right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!("{path}[{index}]"), value, other))
        }
        _ if expected == actual => None,
        _ => Some(format!("{path}: expected {} but read {}", expected.to_string(), actual.to_string())),
    }
}

/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise
/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and
/// its scenario passes whenever the reference library merely declined to error.
fn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {
    match first_divergence("projection", expected, actual) {
        Some(divergence) => Err(format!("{law}: {divergence}")),
        None => Ok(()),
    }
}
//#endregion 🔖️Laws

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in
/// role, before any parity comparison exists: every kind must MOVE the semantic projection. A row
/// whose parameters make the mutation a no-op is not a test -- it passes whenever the reference
/// library declined to error, which is exactly the failure this platform exists to prevent. The
/// baseline runs one unzip/rezip round trip so the comparison isolates the mutation rather than the
/// writer's own normal form.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_bcf_2_1(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_bcf_2_1(&bytes)?;
    if projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the coordination review unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the independent `zip`+`quick-xml`
/// composition applies the forward mutation and then its own base-relative inverse
/// (`oracle_apply_mutation_inverse`), and the restored archive MUST project exactly as the untouched
/// coordination review does. The baseline is taken through one round trip so both sides carry the
/// same re-serialisation and the comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let baseline = project_bcf_2_1(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_bcf_2_1(&bytes)?;
    assert_same_projection(&format!("inverse law violated for {:?} -- undoing it did not restore the coordination review", spec.str("kind")), &baseline, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the independent `zip`+`quick-xml`
/// composition fully parses the real coordination review and re-serializes it from its own model
/// alone, so the re-encoded archive MUST carry the same semantic projection as the input AND MUST NOT
/// be bit-identical to it. A `.bcf` is not a byte-preserving carrier -- every markup part is re-written
/// by the XML writer and every entry re-deflated -- so the byte tripwire is real evidence that the
/// archive was parsed rather than copied.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_bcf_2_1(&input)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the archive was parsed".to_string());
    }
    let projection = project_bcf_2_1(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_bcf::schema::mutations::{apply_bcf_mutation, BcfMutation};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_artifact_stdio_bcf::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_bcf::standards::v2_1::subsets::any::io::{decode_bcf, encode_bcf};
    use semio_s_artifact_stdio_bcf::BcfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::bcf::standards::v2_1::subsets::markup::project_bcf_2_1;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.
    fn mutation_of(spec: &Json) -> Result<BcfMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(ctx: &Context) -> Result<BcfSnapshot, String> {
        decode_bcf(&mutable_input(ctx)?).map_err(|error| format!("decode_bcf failed: {error}"))
    }

    /// 📦️ The produced archive as the `actual-bcf` artifact the `bcf-2-1-jszip-compare-v1` pipeline reads.
    fn actual(ctx: &Context, snapshot: &BcfSnapshot) -> Result<Outcome, String> {
        let bytes = encode_bcf(snapshot).map_err(|error| format!("encode_bcf failed: {error}"))?;
        let projection = project_bcf_2_1(&bytes)?;
        let path = ctx.artifact("actual-bcf", "actual.bcf")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-bcf", &path, "application/octet-stream"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        apply_bcf_mutation(&mut snapshot, &mutation_of(&ctx.doc_json()?)?);
        actual(ctx, &snapshot)
    }

    /// ↩️ The forward op, then the production inverse computed against the pre-mutation snapshot.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(ctx)?;
        let forward = mutation_of(&ctx.doc_json()?)?;
        let backward = mutation_inverse(&forward, &snapshot);
        apply_bcf_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_bcf_mutation(&mut snapshot, mutation);
        }
        actual(ctx, &snapshot)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_bcf`/`encode_bcf` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(ctx)?;
        if encode_bcf(&snapshot).map_err(|error| format!("encode_bcf failed: {error}"))? == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        actual(ctx, &snapshot)
    }
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
