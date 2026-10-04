//! 🦀️ IFC2X3/🧱️base mutation case — Rust adapter. Exhaustive: every declared `Ifc2x3Mutation` kind
//! (`ifc-2x3-base`, 4 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. Every row's `params` IS the leaf wire payload (`payload_value()`), so the
//! subject decodes it through the derive-generated `from_payload_value` and the oracle reads the same
//! wire through its own grammar — nothing maps parameters onto an operation by hand. `ruststep` 0.4 can
//! only READ Part-21 text, so the oracle dispatcher (`../../🔮️oracles/🦀️.rs`) performs every kind with
//! its own from-scratch Part-21 writer against a `ruststep`-parsed document, independent of this
//! subset's own `Ifc2x3Snapshot` codec; the subject fully parses into `Ifc2x3Snapshot` and re-serializes
//! from it alone (no byte pass-through). Both results are read back by the INDEPENDENT `ruststep`
//! reader (`project_ifc_2x3_any`) before the `semantic-ifc-v1` profile compares them — real
//! third-party evidence about structure, never a byte-level differential claim (fleet brief §6:
//! ruststep is not a second PRODUCER, so nothing here is typed `@mode-differential`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_ifc_test_oracle::standards::v2x3::subsets::base::{oracle_apply_mutation, oracle_round_trip, oracle_snapshot_payload, project_ifc_2x3_any};

//#region 🔖️Input
const INPUT: &str = "shared://🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc";

/// 🧫️ Copies the immutable committed fixture into the work directory and returns the mutable
/// copy's bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.ifc"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Inverse
/// 🏗️ The real column `#619887` as the fixture carries it — the inverse of the `upsert-instance` row,
/// which renames it.
const ORIGINAL_COLUMN: &str = r#"{"instance": {"id": 619887, "entities": [{"typeName": "IFCCOLUMN", "arguments": [{"kind": "str", "value": "0PfeWE7Aj7GBHCsLa67379"}, {"kind": "ref", "value": 41}, {"kind": "str", "value": "UC-Universal Columns-Column:UC305x305x97:552739"}, {"kind": "unset"}, {"kind": "str", "value": "UC-Universal Columns-Column:UC305x305x97"}, {"kind": "ref", "value": 619886}, {"kind": "ref", "value": 619879}, {"kind": "str", "value": "552739"}]}]}}"#;

/// 🧱️ The real wall `#270549` as the fixture carries it — the cross-kind inverse of `remove-instance`,
/// the same pattern `step/🔖️ap214/🧱️base`'s `insert-entity`/`remove-entity` pair uses.
const ORIGINAL_WALL: &str = r#"{"instance": {"id": 270549, "entities": [{"typeName": "IFCWALLSTANDARDCASE", "arguments": [{"kind": "str", "value": "29w45MKkv9yu3UjOOOyCma"}, {"kind": "ref", "value": 41}, {"kind": "str", "value": "Basic Wall:Generic - 300mm:471837"}, {"kind": "unset"}, {"kind": "str", "value": "Basic Wall:Generic - 300mm"}, {"kind": "ref", "value": 270529}, {"kind": "ref", "value": 270547}, {"kind": "str", "value": "471837"}]}]}}"#;

/// 📇️ The fixture's own committed header — the inverse of the `set-header` row.
const ORIGINAL_HEADER: &str = r#"{"header": {"fileDescription": [{"kind": "list", "values": [{"kind": "str", "value": "ViewDefinition [CoordinationView_V2.0]"}]}, {"kind": "str", "value": "2;1"}], "fileName": [{"kind": "str", "value": "0001"}, {"kind": "str", "value": "2021-11-21T06:45:25"}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "str", "value": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"kind": "str", "value": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"kind": "str", "value": ""}], "fileSchema": [{"kind": "list", "values": [{"kind": "str", "value": "IFC2X3"}]}]}}"#;

fn wire_spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}

/// ↩️ The inverse spec of one forward row against the pristine fixture, computed independently of
/// `Ifc2x3Mutation::inverse()` (whose law degrades every kind to a whole-snapshot restore) and spoken in
/// the same leaf wire the rows use: a real per-instance inverse, and for `set-snapshot` the untouched
/// model itself, read by `ruststep`.
fn inverse_spec(kind: &str, input: &[u8]) -> Result<Json, String> {
    Ok(match kind {
        "set-snapshot" | "patch-snapshot" => wire_spec("set-snapshot", oracle_snapshot_payload(input)?),
        "upsert-instance" => wire_spec("upsert-instance", parse_json(ORIGINAL_COLUMN)?),
        "remove-instance" => wire_spec("upsert-instance", parse_json(ORIGINAL_WALL)?),
        "set-header" => wire_spec("set-header", parse_json(ORIGINAL_HEADER)?),
        other => return Err(format!("{other:?} is no declared ifc-2x3-base kind")),
    })
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
    let baseline = project_ifc_2x3_any(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ifc_2x3_any(&bytes)?;
    if projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the IFC2X3 building model unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the reference dispatcher applies the
/// forward mutation and then the independently computed `inverse_spec`, and the restored building
/// model MUST project exactly as the untouched one does. The baseline runs two identity rewrites so
/// both sides carry identical serializer normalisation and the comparison isolates the mutation pair.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_ifc_2x3_any(&oracle_round_trip(&oracle_round_trip(&input)?)?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input)?)?;
    let projection = project_ifc_2x3_any(&restored)?;
    assert_same_projection(&format!("inverse law violated for {kind:?} -- undoing it did not restore the building model"), &baseline, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the reference dispatcher fully
/// parses the real building model with `ruststep` and re-serializes it from its own from-scratch
/// Part-21 writer alone, so the re-encoded bytes MUST carry the same semantic projection as the input
/// AND MUST NOT be bit-identical to it. ISO 10303-21 clear text is not a byte-preserving carrier -- the
/// whole exchange structure is regenerated from the parsed model -- so the byte tripwire is real
/// evidence that the document was parsed rather than copied.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_ifc_2x3_any(&input)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed".to_string());
    }
    let projection = project_ifc_2x3_any(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::io::{decode_ifc2x3, encode_ifc2x3};
    use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation;
    use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use semio_s_artifact_stdio_ifc::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_ifc_test_oracle::standards::v2x3::subsets::base::project_ifc_2x3_any;

    /// 🦠️ The row's `params` IS the leaf wire payload, decoded by the derive-generated constructor.
    fn operation_of(spec: &Json) -> Result<Ifc2x3Mutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// ▶️ Applies `operations` in order through the production diff, refusing the first rejection.
    fn applied(mut snapshot: Ifc2x3Snapshot, operations: &[Ifc2x3Mutation]) -> Result<Ifc2x3Snapshot, String> {
        for operation in operations {
            apply_mutation_checked(&mut snapshot, operation)?;
        }
        Ok(snapshot)
    }

    /// 📐️ Re-serializes from the model alone — `encode_ifc2x3` is this subset's own real codec — and
    /// refuses a byte pass-through of the committed input.
    fn encoded(input: &[u8], snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
        let bytes = encode_ifc2x3(snapshot)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }

    fn outcome(bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_ifc_2x3_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        outcome(encoded(&input, &applied(decode_ifc2x3(&input)?, &[operation])?)?)
    }

    /// ↩️ The mutation's OWN inverse (`Mutation::inverse` against the untouched model), applied to the
    /// re-decoded result of the forward cycle.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        let base = decode_ifc2x3(&input)?;
        let mutated = encoded(&input, &applied(base.clone(), std::slice::from_ref(&operation))?)?;
        outcome(encode_ifc2x3(&applied(decode_ifc2x3(&mutated)?, &mutation_inverse(&operation, &base).expect("valid retained mutation inverse fixture"))?)?)
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        outcome(encoded(&input, &decode_ifc2x3(&input)?)?)
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
