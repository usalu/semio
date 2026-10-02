//! 🦀️ STEP AP214/🧱️base mutation case — Rust adapter. Exhaustive: every declared `StepMutation` kind
//! (`step-ap214-base`, 10 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. Every row's `params` IS the leaf wire payload (`payload_value()`), so the subject
//! decodes it through the derive-generated `from_payload_value` and the oracle reads the same `StepValue`
//! wire through the standard's own grammar — nothing maps parameters onto an operation by hand.
//! `ruststep` 0.4 can only READ Part-21 text, so the oracle dispatcher (`../../🔮️oracles/🦀️.rs`) performs
//! every kind with the standard's own from-scratch Part-21 writer against a `ruststep`-parsed document,
//! independent of this subset's own `StepSnapshot` codec; the subject fully parses into `StepSnapshot`
//! and re-serializes from it alone (no byte pass-through). Both results are read back by the INDEPENDENT
//! `ruststep` reader (`project_step_ap214_any`) before the `semantic-step-v1` profile compares them —
//! real third-party evidence about structure, never a byte-level differential claim (fleet brief §6:
//! ruststep is not a second PRODUCER, so nothing here is typed `@mode-differential`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::{oracle_apply_mutation, oracle_round_trip, oracle_snapshot_payload, project_step_ap214_any};

//#region 🔖️Input
const INPUT: &str = "shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp";

/// 🧫️ Copies the immutable committed fixture into the work directory and returns the mutable
/// copy's bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.stp"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Inverse
/// ↩️ The inverse spec of one forward row against the pristine fixture's own real header and entity
/// values, id/index-aware and spoken in the same `StepValue` leaf wire the rows use — computed
/// independently of `StepMutation::inverse()`; for `set-snapshot` the untouched model itself, read by
/// `ruststep`.
fn inverse_spec(kind: &str, input: &[u8]) -> Result<Json, String> {
    let spec = match kind {
        "set-snapshot" => return Ok(Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), oracle_snapshot_payload(input)?)])),
        "set-file-description" => r#"{"kind": "set-file-description", "params": {"fileDescription": {"description": [""], "implementationLevel": "2;1"}}}"#,
        "set-file-name" => r#"{"kind": "set-file-name", "params": {"fileName": {"name": "hexagonal-cut-concrete-forest-left", "timestamp": "2026-06-06T18:37:11+02:00", "author": [""], "organization": [""], "preprocessorVersion": "ST-DEVELOPER v19.2", "originatingSystem": "Rhino 8.31", "authorization": ""}}}"#,
        "set-file-schema" => r#"{"kind": "set-file-schema", "params": {"fileSchema": {"schemas": ["AUTOMOTIVE_DESIGN"]}}}"#,
        "insert-entity" => r#"{"kind": "remove-entity", "params": {"id": 9001}}"#,
        "remove-entity" => r#"{"kind": "insert-entity", "params": {"index": 1395, "entity": {"id": 1405, "name": "CARTESIAN_POINT", "args": [{"string": ""}, {"aggregate": [{"real": 0.0}, {"real": 0.0}, {"real": 0.0}]}]}}}"#,
        "set-entity-name" => r#"{"kind": "set-entity-name", "params": {"id": 1394, "name": "CARTESIAN_POINT"}}"#,
        "set-entity-arg" => r#"{"kind": "set-entity-arg", "params": {"id": 1394, "argIndex": 0, "value": {"string": ""}}}"#,
        "insert-entity-arg" => r#"{"kind": "remove-entity-arg", "params": {"id": 1394, "argIndex": 2}}"#,
        "remove-entity-arg" => r#"{"kind": "insert-entity-arg", "params": {"id": 1394, "argIndex": 1, "value": {"aggregate": [{"real": 2.7}, {"real": 4.67653718043597}, {"real": 2.735}]}}}"#,
        other => return Err(format!("{other:?} is no declared step-ap214-base kind")),
    };
    parse_json(spec)
}
//#endregion 🔖️Inverse

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
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id. It asserts ONE thing in role, before
/// any parity comparison exists: every kind must MOVE the semantic projection. A row whose parameters
/// make the mutation a no-op is not a test -- it passes whenever the reference library declined to
/// error, which is exactly the failure this platform exists to prevent. The baseline runs one identity
/// rewrite so the comparison isolates the mutation rather than the writer's own normal form.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_step_ap214_any(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_step_ap214_any(&bytes)?;
    if projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the STEP AP214 exchange structure unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the reference dispatcher applies the
/// forward mutation and then the independently computed `inverse_spec`, and the restored exchange
/// structure MUST project exactly as the untouched one does. The baseline runs two identity rewrites so
/// both sides carry identical serializer normalisation and the comparison isolates the mutation pair.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_step_ap214_any(&oracle_round_trip(&oracle_round_trip(&input)?)?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input)?)?;
    let projection = project_step_ap214_any(&restored)?;
    assert_same_projection(&format!("inverse law violated for {kind:?} -- undoing it did not restore the exchange structure"), &baseline, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the reference dispatcher fully
/// parses the real exchange structure with `ruststep` and re-serializes it from the standard's own
/// from-scratch Part-21 writer alone, so the re-encoded bytes MUST carry the same semantic projection
/// as the input AND MUST NOT be bit-identical to it. ISO 10303-21 clear text is not a byte-preserving
/// carrier -- the whole exchange structure is regenerated from the parsed model -- so the byte tripwire
/// is real evidence that the document was parsed rather than copied.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_step_ap214_any(&input)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed".to_string());
    }
    let projection = project_step_ap214_any(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_step::part21::{parse_part21, write_part21};
    use semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::mutations::StepMutation;
    use semio_s_artifact_stdio_step::StepSnapshot;
    use semio_s_artifact_stdio_step::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::project_step_ap214_any;

    /// 🦠️ The row's `params` IS the leaf wire payload, decoded by the derive-generated constructor.
    fn operation_of(spec: &Json) -> Result<StepMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// 📥️ Genuine ISO 10303-21 text decoded into the subset's own snapshot through the shared Part-21
    /// tokenizer — no semio pack/DSL envelope, the shape `ruststep` and the committed fixture expect.
    fn decoded(bytes: &[u8]) -> Result<StepSnapshot, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(StepSnapshot::from_part21_document(&parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?))
    }

    /// ▶️ Applies `operations` in order through the production diff, refusing the first rejection.
    fn applied(mut snapshot: StepSnapshot, operations: &[StepMutation]) -> Result<StepSnapshot, String> {
        for operation in operations {
            apply_mutation_checked(&mut snapshot, operation)?;
        }
        Ok(snapshot)
    }

    /// 📐️ Re-serializes from the model alone and refuses a byte pass-through of the committed input.
    fn encoded(input: &[u8], snapshot: &StepSnapshot) -> Result<Vec<u8>, String> {
        let bytes = write_part21(&snapshot.to_part21_document()).into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }

    fn outcome(bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_step_ap214_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        outcome(encoded(&input, &applied(decoded(&input)?, &[operation])?)?)
    }

    /// ↩️ The mutation's OWN inverse (`Mutation::inverse` against the untouched model), applied to the
    /// re-decoded result of the forward cycle.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        let base = decoded(&input)?;
        let mutated = encoded(&input, &applied(base.clone(), std::slice::from_ref(&operation))?)?;
        outcome(write_part21(&applied(decoded(&mutated)?, &mutation_inverse(&operation, &base))?.to_part21_document()).into_bytes())
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        outcome(encoded(&input, &decoded(&input)?)?)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
