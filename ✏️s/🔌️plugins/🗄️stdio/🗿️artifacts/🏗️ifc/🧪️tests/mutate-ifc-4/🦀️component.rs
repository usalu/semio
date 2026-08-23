//! 🦀️ IFC4/✳️any mutation case — Rust adapter. Exhaustive: every declared `IfcMutation` kind
//! (`ifc-4-any`, 11 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one identity
//! round trip. `ruststep` 0.4 can only READ Part-21 text (confirmed empirically — see the feature
//! file's own description), so the oracle dispatcher (`../../🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/
//! 🦀️component.rs`) performs every kind with its own from-scratch Part-21 writer against a
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
/// 🧬️schema/🧬️mutations/🦀️component.rs`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against the `ifc-4-any` catalog) is what keeps the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-description", "set-file-name", "set-file-schema", "insert-entity", "remove-entity", "set-entity-name", "set-entity-arg", "insert-entity-arg", "remove-entity-arg"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🏗️nakagin-capsule-tower.ifc";

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
/// ✳️any/🧬️schema/🧬️mutations/🦀️component.rs` documents, computed independently here since neither
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

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ifc_4_any(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let restored = if kind == "no-mutation" {
        input.clone()
    } else {
        let mutated = oracle_apply_mutation(&input, &spec)?;
        oracle_apply_mutation(&mutated, &inverse_spec(&kind))?
    };
    let projection = project_ifc_4_any(&restored)?;
    Ok(Outcome::with_raw(restored, projection))
}

fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &json_spec("no-mutation", json_obj(vec![])))?;
    let projection = project_ifc_4_any(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, json_obj, json_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v4::subsets::any::schema::mutations::{apply_ifc_mutation, IfcMutation};
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
    /// (`../../🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own `value_from_json`),
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
            // 🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`), which speaks a single nested `v` value for
            // `typed` (matching `ruststep::ast::Parameter::Typed`'s own `Box<Parameter>` shape) —
            // wrapped into `IfcValue::TypedValue`'s `Vec<IfcValue>` field, its production shape.
            "typed" => Ok(IfcValue::TypedValue(str_field(value, "name")?, vec![value_from_json(value.get("v").ok_or("typed value requires a v field")?)?])),
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
            "no-mutation" => IfcMutation::NoMutation,
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
                IfcMutation::SetSnapshot { snapshot }
            }
            "set-file-description" => IfcMutation::SetFileDescription { values: params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()? },
            "set-file-name" => IfcMutation::SetFileName { values: params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()? },
            "set-file-schema" => IfcMutation::SetFileSchema { values: params.array("values").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()? },
            "insert-entity" => {
                let entity_json = params.get("entity").ok_or("insert-entity requires an entity field")?;
                let args = entity_json.array("args").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?;
                IfcMutation::InsertEntity { index: usize_field(params, "index")?, entity: IfcEntity { id: u64_field(entity_json, "id")?, name: str_field(entity_json, "name")?, args, complex: Vec::new() } }
            }
            "remove-entity" => IfcMutation::RemoveEntity { id: u64_field(params, "id")? },
            "set-entity-name" => IfcMutation::SetEntityName { id: u64_field(params, "id")?, name: str_field(params, "name")? },
            "set-entity-arg" => IfcMutation::SetEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")?, value: value_from_json(params.get("value").ok_or("set-entity-arg requires a value field")?)? },
            "insert-entity-arg" => IfcMutation::InsertEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")?, value: value_from_json(params.get("value").ok_or("insert-entity-arg requires a value field")?)? },
            "remove-entity-arg" => IfcMutation::RemoveEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")? },
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
