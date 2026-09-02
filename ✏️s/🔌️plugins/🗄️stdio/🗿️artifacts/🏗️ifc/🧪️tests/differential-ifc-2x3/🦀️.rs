//! 🦀️ IFC2X3/✳️any differential case — Rust adapter, SUBJECT half only.
//!
//! The oracle half of this case is `🐍️component.py`: IfcOpenShell 0.8.4.post1 applies each mutation
//! through its own API and re-serializes the whole exchange structure with its own C++ Part-21
//! writer. That is the second PRODUCER `ruststep` cannot be, which is why every scenario here is
//! `@mode-differential` while the sibling `../mutate-ifc-2x3` — same vocabulary, same fixture, all
//! five kinds, `ruststep` as an independent READER — stays `@mode-property` and is left untouched.
//!
//! This file therefore registers NOTHING in the oracle role. The subject does exactly what the
//! sibling case's subject does: `decode_ifc2x3` into this subset's own `Ifc2x3Snapshot`, the typed
//! `Ifc2x3Mutation` applied to it, `encode_ifc2x3` from the snapshot alone — no byte pass-through —
//! followed by an independent `ruststep` read-back (`project_ifc_2x3_any`) before `semantic-ifc-v1`
//! compares it with what IfcOpenShell produced from the same input.
//!
//! Four of the five kinds appear here. `remove-instance` does not: `ifcopenshell.file.remove`
//! repairs the references that point at the removed instance, `Ifc2x3Mutation::RemoveInstance` is a
//! bare `retain` that deliberately leaves them dangling, and `#270549` — the instance the sibling
//! case removes precisely because it is referenced — has 8 inverse references in this fixture.
//! Comparing two different verbs is not a differential; it keeps its `ruststep`-backed scenarios
//! next door.
//!
//! @see component.feature — the differential claim and the measurement that bounds it.
//! @see ../mutate-ifc-2x3/🦀️.rs — the exhaustive five-kind case this one does not replace.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ The four kinds of this subset's `Ifc2x3Mutation::KINDS` that both implementations can produce.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "upsert-instance", "set-header"];
/// ↩️ The kinds whose INVERSE both implementations can also produce. `set-snapshot` is absent
/// because IfcOpenShell cannot read back its own two-identifier `FILE_SCHEMA` output — the defect
/// `🐍️component.py`'s `open_model` guard names — so there is no second producer for the second half
/// of that chain. `inverse-set-snapshot` keeps its ruststep-backed scenario in `../mutate-ifc-2x3`.
const INVERSE_KINDS: &[&str] = &["no-mutation", "upsert-instance", "set-header"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧪️wellness-center-sama-street-level/🏗️.ifc";
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
fn json_unset() -> Json {
    json_obj(vec![("t", json_str("unset"))])
}
#[cfg(feature = "sut")]
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️RealEntities
/// 🏗️ The real `#619887` column and the real header this fixture carries, kept in one place so the
/// forward `Examples` params, this adapter's `inverse_spec` and the Python oracle's own independent
/// copy of the same literals can be checked against the real file rather than against each other.
#[cfg(feature = "sut")]
fn column_args(name: &str) -> Json {
    Json::Array(vec![
        json_value("string", json_str("0PfeWE7Aj7GBHCsLa67379")),
        json_value("reference", json_num(41.0)),
        json_value("string", json_str(name)),
        json_unset(),
        json_value("string", json_str("UC-Universal Columns-Column:UC305x305x97")),
        json_value("reference", json_num(619886.0)),
        json_value("reference", json_num(619879.0)),
        json_value("string", json_str("552739")),
    ])
}
#[cfg(feature = "sut")]
fn column_instance(name: &str) -> Json {
    json_obj(vec![("id", json_num(619887.0)), ("entities", Json::Array(vec![json_obj(vec![("name", json_str("IFCCOLUMN")), ("args", column_args(name))])]))])
}
#[cfg(feature = "sut")]
fn wellness_header(name0: &str) -> Json {
    json_obj(vec![
        ("fileDescription", Json::Array(vec![json_value("aggregate", Json::Array(vec![json_value("string", json_str("ViewDefinition [CoordinationView_V2.0]"))])), json_value("string", json_str("2;1"))])),
        (
            "fileName",
            Json::Array(vec![
                json_value("string", json_str(name0)),
                json_value("string", json_str("2021-11-21T06:45:25")),
                json_value("aggregate", Json::Array(vec![json_value("string", json_str(""))])),
                json_value("aggregate", Json::Array(vec![json_value("string", json_str(""))])),
                json_value("string", json_str("The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013")),
                json_value("string", json_str("21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383")),
                json_value("string", json_str("")),
            ]),
        ),
        ("fileSchema", Json::Array(vec![json_value("aggregate", Json::Array(vec![json_value("string", json_str("IFC2X3"))]))])),
    ])
}
//#endregion 🔖️RealEntities

//#region 🔖️Inverse
/// ↩️ The inverse spec for one forward `(kind, params)` pair against the pristine fixture's own real
/// values — the same inverses the Python oracle computes independently on its own side.
#[cfg(feature = "sut")]
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("fileSchema", json_str_array(&["IFC2X3"]))])),
        "upsert-instance" => json_spec("upsert-instance", json_obj(vec![("instance", column_instance("UC-Universal Columns-Column:UC305x305x97:552739"))])),
        "set-header" => json_spec("set-header", json_obj(vec![("header", wellness_header("0001"))])),
        other => json_spec(other, json_obj(vec![])),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, json_obj, json_spec, INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::io::{decode_ifc2x3, encode_ifc2x3};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, set_header, set_snapshot, upsert_instance, Ifc2x3Mutation};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::engine::part21::{Part21Header, Part21Instance, Part21Value};
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::any::project_ifc_2x3_any;

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
    fn str_array(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .filter_map(|entry| match entry {
                Json::String(text) => Some(text.clone()),
                _ => None,
            })
            .collect()
    }
    fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
        num_field(value, key).map(|number| number as u64)
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️ValueGrammar
    /// 🔤️ The same `{"t":…, "v":…}` wire grammar the Python oracle speaks, re-implemented here
    /// against `Part21Value` rather than against IfcOpenShell's own typed attribute API.
    fn value_from_json(value: &Json) -> Result<Part21Value, String> {
        match str_field(value, "t")?.as_str() {
            "unset" => Ok(Part21Value::Unset),
            "derived" => Ok(Part21Value::Derived),
            "integer" => Ok(Part21Value::Int(num_field(value, "v")? as i64)),
            "real" => Ok(Part21Value::Real(num_field(value, "v")?.into())),
            "string" => Ok(Part21Value::Str(str_field(value, "v")?)),
            "enum" => Ok(Part21Value::Enum(str_field(value, "v")?)),
            "reference" => Ok(Part21Value::Ref(u64_field(value, "v")?)),
            "aggregate" => Ok(Part21Value::List(value.array("v").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?)),
            "typed" => Ok(Part21Value::Typed { name: str_field(value, "name")?, items: vec![value_from_json(value.get("v").ok_or("typed value requires a v field")?)?] }),
            other => Err(format!("unknown value type {other:?}")),
        }
    }
    //#endregion 🔖️ValueGrammar

    //#region 🔖️MutationFromSpec
    /// 🦠️ The wire `(kind, params)` pair translated into a real `Ifc2x3Mutation`. `set-snapshot`
    /// only overrides `FILE_SCHEMA` on the already-decoded document, the same pragmatic reading the
    /// sibling case uses — a full 3464-entity snapshot literal has no place in a Gherkin cell.
    fn mutation_from_spec(spec: &Json, base: &Ifc2x3Snapshot) -> Result<Ifc2x3Mutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            // 🧭️ "no-mutation" is no longer a declared `Ifc2x3Mutation` kind (`NoMutation` was
            // dropped, `dsl::Mutations` rejects a wrapper-less variant); a `SetSnapshot` back onto
            // the identical base is a real no-op mutation, not a fabricated sentinel.
            "no-mutation" => Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            "set-snapshot" => {
                let schemas = str_array(params, "fileSchema");
                if schemas.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                let mut snapshot = base.clone();
                snapshot.document.header.file_schema = vec![Part21Value::List(schemas.into_iter().map(Part21Value::Str).collect())];
                Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })
            }
            "set-header" => {
                let field = params.get("header").ok_or("set-header requires a header field")?;
                let value_list = |key: &str| -> Result<Vec<Part21Value>, String> { field.array(key).iter().map(value_from_json).collect() };
                Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: Part21Header { file_description: value_list("fileDescription")?, file_name: value_list("fileName")?, file_schema: value_list("fileSchema")? } })
            }
            "upsert-instance" => {
                let instance_json = params.get("instance").ok_or("upsert-instance requires an instance field")?;
                let id = u64_field(instance_json, "id")?;
                let entities = instance_json
                    .array("entities")
                    .iter()
                    .map(|entry| -> Result<(String, Vec<Part21Value>), String> { Ok((str_field(entry, "name")?, entry.array("args").iter().map(value_from_json).collect::<Result<Vec<_>, String>>()?)) })
                    .collect::<Result<Vec<_>, String>>()?;
                if entities.is_empty() {
                    return Err("upsert-instance requires a non-empty entities array".to_string());
                }
                Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: Part21Instance { id, entities } })
            }
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
        let snapshot = decode_ifc2x3(input)?;
        let base = snapshot.clone();
        let mutation = mutation_from_spec(spec, &base)?;
        let mut next = snapshot;
        apply_ifc2x3_mutation(&mut next, &mutation);
        let bytes = encode_ifc2x3(&next)?;
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
        let projection = project_ifc_2x3_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutated = apply_and_encode(&input, &spec, true)?;
        let restored = apply_and_encode(&mutated, &inverse_spec(&kind), false)?;
        let projection = project_ifc_2x3_any(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = apply_and_encode(&input, &json_spec("no-mutation", json_obj(vec![])), true)?;
        let projection = project_ifc_2x3_any(&bytes)?;
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
            built = built.subject(&format!("differential-{kind}"), subject::mutate);
        }
        for kind in INVERSE_KINDS {
            built = built.subject(&format!("differential-inverse-{kind}"), subject::inverse);
        }
        built = built.subject("differential-identity-round-trip", subject::round_trip);
    }
    let _ = (KINDS, INVERSE_KINDS);
    built
}
//#endregion 🔖️Registration
