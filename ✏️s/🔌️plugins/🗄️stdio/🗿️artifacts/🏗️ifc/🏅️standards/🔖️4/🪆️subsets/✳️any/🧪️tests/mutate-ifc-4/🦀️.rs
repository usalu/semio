//! 🦀️ IFC4/✳️any mutation case — Rust adapter. Exhaustive: every declared `IfcMutation` kind
//! (`ifc-4-any`, 11 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one identity
//! round trip. `ruststep` 0.4 can only READ Part-21 text (confirmed empirically — see the feature
//! file's own description), so the oracle dispatcher (`../../🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/
//! 🦀️.rs`) performs every kind with its own from-scratch Part-21 writer against a
//! `ruststep`-parsed document, independent of this subset's own `IfcSnapshot`/`step::engine::part21`
//! codec; the subject fully parses into `IfcSnapshot` and re-serializes from it alone (no byte
//! pass-through). Both results are read back by the INDEPENDENT `ruststep` reader
//! (`project_ifc_4_any`) before the `semantic-ifc-v1` profile compares them — real third-party
//! evidence about structure, never a byte-level differential claim (fleet brief §6: ruststep is not
//! a second PRODUCER, so nothing here is typed `@mode-differential`).

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v4::subsets::any::{oracle_apply_mutation, project_ifc_4_any};

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `IfcMutation::KINDS` (`../../🏅️standards/🔖️4/🪆️subsets/✳️any/
/// 🧬️schema/🧬️mutations/🦀️.rs`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against the `ifc-4-any` catalog) is what keeps the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-description", "set-file-name", "set-file-schema", "insert-entity", "remove-entity", "set-entity-name", "set-entity-arg", "insert-entity-arg", "remove-entity-arg"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧪️nakagin-capsule-tower/🏗️.ifc";

/// 🧫️ Copies the immutable committed fixture into the work directory and returns the mutable
/// copy's bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.ifc"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️JsonBuild
fn json_obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn json_num(value: f64) -> Json {
    Json::Number(value)
}
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
fn json_str_array(values: &[&str]) -> Json {
    Json::Array(values.iter().map(|value| json_str(value)).collect())
}
fn json_value(t: &str, v: Json) -> Json {
    json_obj(vec![("t", json_str(t)), ("v", v)])
}
fn json_agg(values: &[&str]) -> Json {
    json_value("aggregate", Json::Array(values.iter().map(|value| json_value("string", json_str(value))).collect()))
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// pristine real Nakagin Capsule Tower fixture's own known real header/entity values — id/index-
/// aware, mirroring the same `IfcMutation::inverse()` semantics `../../🏅️standards/🔖️4/🪆️subsets/
/// ✳️any/🧬️schema/🧬️mutations/🦀️.rs` documents, computed independently here since neither
/// the oracle nor this adapter can reach that subject-side method.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("fileSchema", json_str_array(&["IFC4"]))])),
        "set-file-description" => json_spec("set-file-description", json_obj(vec![("values", Json::Array(vec![json_agg(&["ViewDefinition[DesignTransferView]"]), json_value("string", json_str("2;1"))]))])),
        "set-file-name" => json_spec(
            "set-file-name",
            json_obj(vec![(
                "values",
                Json::Array(vec![
                    json_value("string", json_str("/dev/null")),
                    json_value("string", json_str("2026-03-20T21:51:27+00:00")),
                    json_agg(&[""]),
                    json_agg(&[""]),
                    json_value("string", json_str("IfcOpenShell 0.8.4.post1")),
                    json_value("string", json_str("IfcOpenShell 0.8.4.post1")),
                    json_value("string", json_str("Nobody")),
                ]),
            )]),
        ),
        "set-file-schema" => json_spec("set-file-schema", json_obj(vec![("values", Json::Array(vec![json_agg(&["IFC4"])]))])),
        "insert-entity" => json_spec("remove-entity", json_obj(vec![("id", json_num(90001.0))])),
        "remove-entity" => {
            let args = Json::Array(vec![
                json_value("string", json_str("0POPlhUSnC1REPvcqnensi")),
                json_value("unset", json_obj(vec![])),
                json_value("string", json_str("b")),
                json_value("unset", json_obj(vec![])),
                json_value("unset", json_obj(vec![])),
                json_value("reference", json_num(16996.0)),
                json_value("reference", json_num(16985.0)),
                json_value("unset", json_obj(vec![])),
                json_value("unset", json_obj(vec![])),
            ]);
            let entity = json_obj(vec![("id", json_num(16976.0)), ("name", json_str("IFCBUILDINGELEMENTPROXY")), ("args", args)]);
            json_spec("insert-entity", json_obj(vec![("index", json_num(16975.0)), ("entity", entity)]))
        }
        "set-entity-name" => json_spec("set-entity-name", json_obj(vec![("id", json_num(16976.0)), ("name", json_str("IFCBUILDINGELEMENTPROXY"))])),
        "set-entity-arg" => json_spec("set-entity-arg", json_obj(vec![("id", json_num(16976.0)), ("index", json_num(2.0)), ("value", json_value("string", json_str("b")))])),
        "insert-entity-arg" => json_spec("remove-entity-arg", json_obj(vec![("id", json_num(16976.0)), ("index", json_num(9.0))])),
        "remove-entity-arg" => json_spec("insert-entity-arg", json_obj(vec![("id", json_num(16976.0)), ("index", json_num(8.0)), ("value", json_value("unset", json_obj(vec![])))])),
        other => json_spec(other, json_obj(vec![])),
    }
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
/// 🧾️ The `no-mutation` spec, spelled once. Every kind this dispatcher performs is one full
/// `ruststep` parse plus one from-scratch Part-21 write, so a law's baseline must go through exactly
/// as many of those cycles as the document it judges -- otherwise a divergence would name the
/// writer's own normal form instead of the mutation pair.
fn no_mutation() -> Json {
    json_spec("no-mutation", json_obj(vec![]))
}

/// 🔮️ One handler shared by every `mutate-<kind>` scenario id. It asserts ONE thing in role, before
/// any parity comparison exists: every kind other than `no-mutation` must MOVE the semantic
/// projection. A row whose parameters make the mutation a no-op is not a test -- it passes whenever
/// the reference library declined to error, which is exactly the failure this platform exists to
/// prevent. The baseline runs one `no-mutation` cycle so the comparison isolates the mutation
/// rather than the writer's own normal form.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_ifc_4_any(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ifc_4_any(&bytes)?;
    if kind != "no-mutation" && projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the IFC4 exchange structure unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the reference dispatcher applies the
/// forward mutation and then the independently computed `inverse_spec`, and the restored
/// exchange structure MUST project exactly as the untouched one does. `no-mutation` is NOT short-circuited: it runs the same
/// two cycles as every other kind, so the trivial case is evidence rather than an exemption. The
/// baseline runs two `no-mutation` cycles for the same reason -- both sides then carry identical
/// serializer normalisation and the comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_ifc_4_any(&oracle_apply_mutation(&oracle_apply_mutation(&input, &no_mutation())?, &no_mutation())?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind))?;
    let projection = project_ifc_4_any(&restored)?;
    assert_same_projection(&format!("inverse law violated for {kind:?} -- undoing it did not restore the exchange structure"), &baseline, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the reference dispatcher fully
/// parses the real exchange structure with `ruststep` and re-serializes it from its own
/// from-scratch Part-21 writer alone, so the re-encoded bytes MUST carry the same semantic projection as the input AND
/// MUST NOT be bit-identical to it. ISO 10303-21 clear text is not a byte-preserving carrier -- the
/// whole exchange structure is regenerated from the parsed model -- so the byte tripwire is real
/// evidence that the document was parsed rather than copied.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_ifc_4_any(&input)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
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
    use super::{inverse_spec, json_obj, json_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v4::subsets::any::schema::mutations::{
        apply_ifc_mutation, insert_entity, insert_entity_arg, remove_entity, remove_entity_arg, set_entity_arg, set_entity_name, set_file_description, set_file_name, set_file_schema, set_snapshot, IfcMutation,
    };
    use semio_s_plugin_stdio::artifacts::ifc::standards::v4::subsets::any::schema::snapshot::{from_part21_document, to_part21_document, IfcEntity, IfcSnapshot, IfcValue};
    use semio_s_plugin_stdio::artifacts::step::engine::part21::{parse_part21, write_part21};
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v4::subsets::any::project_ifc_4_any;

    //#region 🔖️SpecReading
    fn num_field(value: &Json, key: &str) -> Result<f64, String> {
        match value.get(key) {
            Some(Json::Number(number)) => Ok(*number),
            _ => Err(format!("expected numeric field {key:?}")),
        }
    }
    fn str_field(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(text)) => Ok(text.clone()),
            _ => Err(format!("expected string field {key:?}")),
        }
    }
    fn usize_field(value: &Json, key: &str) -> Result<usize, String> {
        num_field(value, key).map(|number| number as usize)
    }
    fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
        num_field(value, key).map(|number| number as u64)
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️ValueGrammar
    /// 🔤️ The same `{"t":..., "v":...}` wire grammar the oracle dispatcher speaks
    /// (`../../🏅️standards/🔖️4/🪆️subsets/✳️any/🦀️oracle.rs`'s own `value_from_json`),
    /// independently re-implemented here against `IfcValue` rather than `ruststep::ast::Parameter`.
    fn value_from_json(value: &Json) -> Result<IfcValue, String> {
        match str_field(value, "t")?.as_str() {
            "unset" => Ok(IfcValue::Unset),
            "derived" => Ok(IfcValue::Derived),
            "integer" => Ok(IfcValue::Integer(num_field(value, "v")? as i64)),
            "real" => Ok(IfcValue::Real(num_field(value, "v")?)),
            "string" => Ok(IfcValue::String(str_field(value, "v")?)),
            "enum" => Ok(IfcValue::Enum(str_field(value, "v")?)),
            "reference" => Ok(IfcValue::Reference(u64_field(value, "v")?)),
            "aggregate" => Ok(IfcValue::Aggregate(value.array("v").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?)),
            // 📎️ Mirrors the oracle dispatcher's own `value_from_json` (`../../🏅️standards/🔖️4/
            // 🪆️subsets/✳️any/🦀️oracle.rs`), which speaks a single nested `v` value for
            // `typed` (matching `ruststep::ast::Parameter::Typed`'s own `Box<Parameter>` shape) —
            // wrapped into `IfcValue::TypedValue`'s `Vec<IfcValue>` field, its production shape.
            "typed" => Ok(IfcValue::TypedValue { name: str_field(value, "name")?, items: vec![value_from_json(value.get("v").ok_or("typed value requires a v field")?)?] }),
            other => Err(format!("unknown value type {other:?}")),
        }
    }
    //#endregion 🔖️ValueGrammar

    //#region 🔖️MutationFromSpec
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `IfcMutation` for this subset's own `apply_ifc_mutation`. `set-snapshot` is pragmatic (only
    /// overrides `FILE_SCHEMA` on `base`, the already-decoded document) — the same precedent
    /// `mutate-pdf-1-7`/`mutate-step-ap214`'s own oracles use, needed here since a full
    /// 24792-entity snapshot literal has no place in a readable Gherkin cell.
    fn mutation_from_spec(spec: &Json, base: &IfcSnapshot) -> Result<IfcMutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            // 🧭️ `NoMutation` was dropped from the enum (a wrapped variant is required by
            // `#[derive(dsl::Mutations)]`), but `no-mutation` stays a real, deliberately-tested
            // scenario id at this test-harness/oracle level (see this subset's own `../../🏅️
            // standards/🔖️4/🪆️subsets/✳️any/🔣️oracle.json` catalog, which still declares it).
            // A `SetSnapshot` carrying `base` back to itself is the same true no-op: it goes
            // through the full apply/diff/re-serialize pipeline and changes nothing.
            "no-mutation" => IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            "set-snapshot" => {
                let mut schema_names = Vec::new();
                for name in params.array("fileSchema") {
                    if let Json::String(s) = name {
                        schema_names.push(s.clone());
                    }
                }
                if schema_names.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                let mut snapshot = base.clone();
                snapshot.header.file_schema = vec![IfcValue::Aggregate(schema_names.into_iter().map(IfcValue::String).collect())];
                IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })
            }
            "set-file-description" => IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()? }),
            "set-file-name" => IfcMutation::SetFileName(set_file_name::SetFileName { values: params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()? }),
            "set-file-schema" => IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()? }),
            "insert-entity" => {
                let entity_json = params.get("entity").ok_or("insert-entity requires an entity field")?;
                let args = entity_json.array("args").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                IfcMutation::InsertEntity(insert_entity::InsertEntity { index: usize_field(params, "index")?, entity: IfcEntity { id: u64_field(entity_json, "id")?, name: str_field(entity_json, "name")?, args, complex: Vec::new() } })
            }
            "remove-entity" => IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: u64_field(params, "id")? }),
            "set-entity-name" => IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: u64_field(params, "id")?, name: str_field(params, "name")? }),
            "set-entity-arg" => IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")?, value: value_from_json(params.get("value").ok_or("set-entity-arg requires a value field")?)? }),
            "insert-entity-arg" => IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")?, value: value_from_json(params.get("value").ok_or("insert-entity-arg requires a value field")?)? }),
            "remove-entity-arg" => IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")? }),
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone — the no-byte-pass-
    /// through rule this wave exists to enforce. `parse_part21`/`write_part21` are the shared
    /// low-level Part-21 tokenizer this subset's own `IfcSnapshot::to_part21_document`/
    /// `from_part21_document` already convert through at the codec boundary, used directly rather
    /// than through `store::ArtifactDsl` so the output is genuine ISO 10303-21 text (no semio
    /// pack/DSL envelope wrapper) — the same shape `ruststep`'s independent reader and the real
    /// committed fixture both expect.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let document = parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?;
        let mut snapshot = from_part21_document("stdio.ifc", &document);
        let base = snapshot.clone();
        let mutation = mutation_from_spec(spec, &base)?;
        apply_ifc_mutation(&mut snapshot, &mutation);
        let bytes = write_part21(&to_part21_document(&snapshot)).into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let bytes = apply_and_encode(&input, &spec)?;
        let projection = project_ifc_4_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let restored = if kind == "no-mutation" {
            input.clone()
        } else {
            let mutated = apply_and_encode(&input, &spec)?;
            apply_and_encode(&mutated, &inverse_spec(&kind))?
        };
        let projection = project_ifc_4_any(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = apply_and_encode(&input, &json_spec("no-mutation", json_obj(vec![])))?;
        let projection = project_ifc_4_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
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
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
