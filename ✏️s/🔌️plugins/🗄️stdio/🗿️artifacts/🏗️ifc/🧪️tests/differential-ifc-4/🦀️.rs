//! 🦀️ IFC4/✳️any differential case — Rust adapter, SUBJECT half only.
//!
//! The oracle half of this case is `🐍️component.py`: IfcOpenShell 0.8.4.post1 applies each mutation
//! through its own API and re-serializes the whole exchange structure with its own C++ Part-21
//! writer. That is the second PRODUCER `ruststep` cannot be, which is why every scenario here is
//! `@mode-differential` while the sibling `../mutate-ifc-4` — same vocabulary, same fixture, all
//! eleven kinds, `ruststep` as an independent READER — stays `@mode-property` and is left untouched.
//!
//! This file therefore registers NOTHING in the oracle role. The subject does exactly what the
//! sibling case's subject does: a full parse into this subset's own `IfcSnapshot`, the typed
//! `IfcMutation` applied to it, and a re-serialization from the snapshot alone — no byte pass-
//! through — followed by an independent `ruststep` read-back (`project_ifc_4_any`) before
//! `semantic-ifc-v1` compares it with what IfcOpenShell produced from the same input.
//!
//! Seven of the eleven kinds appear here. The four that do not (`set-entity-name`,
//! `insert-entity-arg`, `remove-entity-arg`, `remove-entity`) are the ones IfcOpenShell cannot
//! produce or cannot read back faithfully, each measured against this exact fixture and recorded in
//! the feature file; they keep their `ruststep`-backed scenarios next door.
//!
//! @see component.feature — the differential claim and the four measurements that bound it.
//! @see ../mutate-ifc-4/🦀️component.rs — the exhaustive eleven-kind case this one does not replace.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ The seven kinds of this subset's `IfcMutation::KINDS` that both implementations can produce.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-description", "set-file-name", "set-file-schema", "insert-entity", "set-entity-arg"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🏗️nakagin-capsule-tower.ifc";
//#endregion 🔖️Input

//#region 🔖️JsonBuild
#[cfg(feature = "sut")]
fn json_obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
#[cfg(feature = "sut")]
fn json_num(value: f64) -> Json {
    Json::Number(value)
}
#[cfg(feature = "sut")]
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
#[cfg(feature = "sut")]
fn json_str_array(values: &[&str]) -> Json {
    Json::Array(values.iter().map(|value| json_str(value)).collect())
}
#[cfg(feature = "sut")]
fn json_value(t: &str, v: Json) -> Json {
    json_obj(vec![("t", json_str(t)), ("v", v)])
}
#[cfg(feature = "sut")]
fn json_agg(values: &[&str]) -> Json {
    json_value("aggregate", Json::Array(values.iter().map(|value| json_value("string", json_str(value))).collect()))
}
#[cfg(feature = "sut")]
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Inverse
/// ↩️ The inverse spec for one forward `(kind, params)` pair against the pristine real Nakagin
/// Capsule Tower fixture's own known real header and entity values, computed here from the
/// committed fixture — the same values the Python oracle computes independently on its own side.
#[cfg(feature = "sut")]
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
        "set-entity-arg" => json_spec("set-entity-arg", json_obj(vec![("id", json_num(16976.0)), ("index", json_num(2.0)), ("value", json_value("string", json_str("b")))])),
        other => json_spec(other, json_obj(vec![])),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, json_obj, json_spec, INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v4::subsets::any::schema::mutations::{apply_ifc_mutation, IfcMutation};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v4::subsets::any::schema::snapshot::{from_part21_document, to_part21_document, IfcEntity, IfcSnapshot, IfcValue};
    use semio_s_plugin_stdio::artifacts::step::engine::part21::{parse_part21, write_part21};
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v4::subsets::any::project_ifc_4_any;

    //#region 🔖️Input
    fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
        let copy = ctx.copy_fixture(INPUT, Some("input.ifc"))?;
        std::fs::read(&copy).map_err(|error| error.to_string())
    }
    //#endregion 🔖️Input

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
    /// 🔤️ The same `{"t":…, "v":…}` wire grammar the Python oracle speaks, re-implemented here
    /// against `IfcValue` rather than against IfcOpenShell's own typed attribute API.
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
            "typed" => Ok(IfcValue::TypedValue(str_field(value, "name")?, vec![value_from_json(value.get("v").ok_or("typed value requires a v field")?)?])),
            other => Err(format!("unknown value type {other:?}")),
        }
    }
    //#endregion 🔖️ValueGrammar

    //#region 🔖️MutationFromSpec
    /// 🦠️ The wire `(kind, params)` pair translated into a real `IfcMutation`. `set-snapshot` only
    /// overrides `FILE_SCHEMA` on the already-decoded document, the same pragmatic reading the
    /// sibling case uses — a full 24792-entity snapshot literal has no place in a Gherkin cell.
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
            "set-entity-arg" => IfcMutation::SetEntityArg { id: u64_field(params, "id")?, index: usize_field(params, "index")?, value: value_from_json(params.get("value").ok_or("set-entity-arg requires a value field")?)? },
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone.
    ///
    /// ⚠️ `refuse_identity` is the no-byte-pass-through tripwire, and it is applied ONLY to the
    /// cycle whose input is the committed foreign fixture. That is the only cycle where identical
    /// bytes would mean the document was copied instead of decoded. Measured, not assumed: the FIRST
    /// parity run of this case reported `byte pass-through: output is bit-identical to the input` on
    /// `differential-inverse-no-mutation`, because this repository's Part-21 writer is IDEMPOTENT —
    /// re-encoding a document it already wrote reproduces it exactly, which is what a correct writer
    /// does. Asserting non-identity on the second cycle asserts something false. The property is
    /// still asserted where it is true and where it matters, against the real committed artifact.
    /// The sibling `../mutate-ifc-2x3` never hits this because its inverse handler short-circuits
    /// `no-mutation` to `input.clone()` and so does not run the codec at all for that row; this case
    /// runs both cycles for every kind, including the trivial one.
    fn apply_and_encode(input: &[u8], spec: &Json, refuse_identity: bool) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let document = parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?;
        let mut snapshot = from_part21_document("stdio.ifc", &document);
        let base = snapshot.clone();
        let mutation = mutation_from_spec(spec, &base)?;
        apply_ifc_mutation(&mut snapshot, &mutation);
        let bytes = write_part21(&to_part21_document(&snapshot)).into_bytes();
        if refuse_identity && bytes == input {
            return Err("byte pass-through: the re-encoded output is bit-identical to the committed input, so nothing here proves the document was parsed".to_string());
        }
        Ok(bytes)
    }
    //#endregion 🔖️Codec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let bytes = apply_and_encode(&input, &spec, true)?;
        let projection = project_ifc_4_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutated = apply_and_encode(&input, &spec, true)?;
        let restored = apply_and_encode(&mutated, &inverse_spec(&kind), false)?;
        let projection = project_ifc_4_any(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = apply_and_encode(&input, &json_spec("no-mutation", json_obj(vec![])), true)?;
        let projection = project_ifc_4_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. The oracle role belongs to
/// `🐍️component.py`; without the `sut` feature this adapter registers nothing at all, which is
/// exactly right — it has no reference implementation of its own to offer.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built.subject(&format!("differential-{kind}"), subject::mutate).subject(&format!("differential-inverse-{kind}"), subject::inverse);
        }
        built = built.subject("differential-identity-round-trip", subject::round_trip);
    }
    let _ = KINDS;
    built
}
//#endregion 🔖️Registration
