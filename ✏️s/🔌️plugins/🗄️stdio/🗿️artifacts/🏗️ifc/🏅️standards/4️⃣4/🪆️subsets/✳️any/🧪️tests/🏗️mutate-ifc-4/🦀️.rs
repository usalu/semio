//! 🦀️ IFC4/✳️any mutation case — Rust adapter. Exhaustive: every declared `IfcMutation` kind
//! (`ifc-4-any`, 10 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one identity
//! round trip. Every row's `params` IS the leaf wire payload (`payload_value()`), so the subject decodes
//! it through the derive-generated `from_payload_value` and the oracle reads the same `IfcValue` wire
//! through its own grammar — nothing maps parameters onto an operation by hand. `ruststep` 0.4 can only
//! READ Part-21 text, so the oracle dispatcher (`../../🔮️oracles/🦀️.rs`) performs every kind with its own
//! from-scratch Part-21 writer against a `ruststep`-parsed document, independent of this subset's own
//! `IfcSnapshot` codec; the subject fully parses into `IfcSnapshot` and re-serializes from it alone (no
//! byte pass-through). Both results are read back by the INDEPENDENT `ruststep` reader
//! (`project_ifc_4_any`) before the `semantic-ifc-v1` profile compares them — real third-party evidence
//! about structure, never a byte-level differential claim (fleet brief §6: ruststep is not a second
//! PRODUCER, so nothing here is typed `@mode-differential`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_ifc_test_oracle::standards::v4::subsets::any::{oracle_apply_mutation, oracle_round_trip, oracle_snapshot_payload, project_ifc_4_any};

//#region 🔖️Input
const INPUT: &str = "shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc";

/// 🧫️ Copies the immutable committed fixture into the work directory and returns the mutable
/// copy's bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("input.ifc"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Inverse
/// ↩️ The inverse spec of one forward row against the pristine Nakagin Capsule Tower fixture's own
/// real header and entity values, id/index-aware and spoken in the same `IfcValue` leaf wire the rows
/// use — computed independently of `IfcMutation::inverse()`; for `set-snapshot` the untouched model
/// itself, read by `ruststep`.
fn inverse_spec(kind: &str, input: &[u8]) -> Result<Json, String> {
    let params = match kind {
        "set-snapshot" | "patch-snapshot" => return Ok(Json::Object(vec![("kind".to_string(), Json::String("set-snapshot".to_string())), ("params".to_string(), oracle_snapshot_payload(input)?)])),
        "set-file-description" => r#"{"kind": "set-file-description", "params": {"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "ViewDefinition[DesignTransferView]"}]}, {"kind": "string", "value": "2;1"}]}}"#,
        "set-file-name" => r#"{"kind": "set-file-name", "params": {"values": [{"kind": "string", "value": "/dev/null"}, {"kind": "string", "value": "2026-03-20T21:51:27+00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "Nobody"}]}}"#,
        "set-file-schema" => r#"{"kind": "set-file-schema", "params": {"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4"}]}]}}"#,
        "insert-entity" => r#"{"kind": "remove-entity", "params": {"id": 90001}}"#,
        "remove-entity" => r#"{"kind": "insert-entity", "params": {"index": 16975, "entity": {"id": 16976, "name": "IFCBUILDINGELEMENTPROXY", "args": [{"kind": "string", "value": "0POPlhUSnC1REPvcqnensi"}, {"kind": "unset"}, {"kind": "string", "value": "b"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "reference", "value": 16996}, {"kind": "reference", "value": 16985}, {"kind": "unset"}, {"kind": "unset"}]}}}"#,
        "set-entity-name" => r#"{"kind": "set-entity-name", "params": {"id": 16976, "name": "IFCBUILDINGELEMENTPROXY"}}"#,
        "set-entity-arg" => r#"{"kind": "set-entity-arg", "params": {"id": 16976, "index": 2, "value": {"kind": "string", "value": "b"}}}"#,
        "insert-entity-arg" => r#"{"kind": "remove-entity-arg", "params": {"id": 16976, "index": 9}}"#,
        "remove-entity-arg" => r#"{"kind": "insert-entity-arg", "params": {"id": 16976, "index": 8, "value": {"kind": "unset"}}}"#,
        other => return Err(format!("{other:?} is no declared ifc-4-any kind")),
    };
    parse_json(params)
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
    let baseline = project_ifc_4_any(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ifc_4_any(&bytes)?;
    if projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the IFC4 exchange structure unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
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
    let baseline = project_ifc_4_any(&oracle_round_trip(&oracle_round_trip(&input)?)?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input)?)?;
    let projection = project_ifc_4_any(&restored)?;
    assert_same_projection(&format!("inverse law violated for {kind:?} -- undoing it did not restore the exchange structure"), &baseline, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the reference dispatcher fully
/// parses the real exchange structure with `ruststep` and re-serializes it from its own from-scratch
/// Part-21 writer alone, so the re-encoded bytes MUST carry the same semantic projection as the input
/// AND MUST NOT be bit-identical to it. ISO 10303-21 clear text is not a byte-preserving carrier -- the
/// whole exchange structure is regenerated from the parsed model -- so the byte tripwire is real
/// evidence that the document was parsed rather than copied.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_ifc_4_any(&input)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed".to_string());
    }
    let projection = project_ifc_4_any(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_ifc::part21::{parse_part21, write_part21};
    use semio_s_artifact_stdio_ifc::standards::v4::subsets::any::schema::mutations::IfcMutation;
    use semio_s_artifact_stdio_ifc::standards::v4::subsets::any::schema::snapshot::{from_part21_document, to_part21_document, IfcSnapshot};
    use semio_s_artifact_stdio_ifc::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, STDIO_IFC_DOCUMENT_SCHEMA};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_ifc_test_oracle::standards::v4::subsets::any::project_ifc_4_any;

    /// 🦠️ The row's `params` IS the leaf wire payload, decoded by the derive-generated constructor.
    fn operation_of(spec: &Json) -> Result<IfcMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// 📥️ Genuine ISO 10303-21 text decoded into the subset's own snapshot through the shared Part-21
    /// tokenizer — no semio pack/DSL envelope, the shape `ruststep` and the committed fixture expect.
    fn decoded(bytes: &[u8]) -> Result<IfcSnapshot, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?))
    }

    /// ▶️ Applies `operations` in order through the production diff, refusing the first rejection.
    fn applied(mut snapshot: IfcSnapshot, operations: &[IfcMutation]) -> Result<IfcSnapshot, String> {
        for operation in operations {
            apply_mutation_checked(&mut snapshot, operation)?;
        }
        Ok(snapshot)
    }

    /// 📐️ Re-serializes from the model alone and refuses a byte pass-through of the committed input.
    fn encoded(input: &[u8], snapshot: &IfcSnapshot) -> Result<Vec<u8>, String> {
        let bytes = write_part21(&to_part21_document(snapshot)).into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }

    fn outcome(bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_ifc_4_any(&bytes)?;
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
        outcome(write_part21(&to_part21_document(&applied(decoded(&mutated)?, &mutation_inverse(&operation, &base).expect("valid retained mutation inverse fixture"))?)).into_bytes())
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
