//! 🦀️ IFC2X3/✳️base mutation case — Rust adapter. Exhaustive: every declared `Ifc2x3Mutation` kind
//! (`ifc-2x3-base`, 4 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. `ruststep` 0.4 can only READ Part-21 text (confirmed empirically — see the
//! feature file's own description, the same finding the sibling `step/🔖️ap214/✳️base` subset already
//! made), so the oracle dispatcher (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️base/🦀️oracle.rs`)
//! performs every kind with its own from-scratch Part-21 writer against a `ruststep`-parsed
//! document, independent of this subset's own `Ifc2x3Snapshot`/`step::engine::part21` codec; the
//! subject fully parses into `Ifc2x3Snapshot` and re-serializes from it alone (no byte pass-
//! through). Both results are read back by the INDEPENDENT `ruststep` reader
//! (`project_ifc_2x3_any`) before the `semantic-ifc-v1` profile compares them — real third-party
//! evidence about structure, never a byte-level differential claim (fleet brief §6: ruststep is not
//! a second PRODUCER, so nothing here is typed `@mode-differential`).

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::any::{oracle_apply_mutation, project_ifc_2x3_any};

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `Ifc2x3Mutation::KINDS` (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️base/
/// 🧬️schema/🧬️mutations/🦀️.rs`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against the `ifc-2x3-base` catalog) is what keeps the two lists honest against each other.
const KINDS: &[&str] = &["set-snapshot", "upsert-instance", "remove-instance", "set-header"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧪️wellness-center-sama-street-level/🏗️.ifc";

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
fn json_unset() -> Json {
    json_obj(vec![("t", json_str("unset"))])
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️RealEntities
/// 🏗️ The real column/wall/header data this case's forward mutations edit, kept in one place so
/// the forward `Examples` params (feature file), this adapter's `inverse_spec`, and the subject's
/// own re-derivation of the same literals never drift from the real fixture's own real values.
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
fn column_instance(name: &str) -> Json {
    json_obj(vec![("id", json_num(619887.0)), ("entities", Json::Array(vec![json_obj(vec![("name", json_str("IFCCOLUMN")), ("args", column_args(name))])]))])
}

fn wall_args() -> Json {
    Json::Array(vec![
        json_value("string", json_str("29w45MKkv9yu3UjOOOyCma")),
        json_value("reference", json_num(41.0)),
        json_value("string", json_str("Basic Wall:Generic - 300mm:471837")),
        json_unset(),
        json_value("string", json_str("Basic Wall:Generic - 300mm")),
        json_value("reference", json_num(270529.0)),
        json_value("reference", json_num(270547.0)),
        json_value("string", json_str("471837")),
    ])
}
fn wall_instance() -> Json {
    json_obj(vec![("id", json_num(270549.0)), ("entities", Json::Array(vec![json_obj(vec![("name", json_str("IFCWALLSTANDARDCASE")), ("args", wall_args())])]))])
}

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
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// pristine fixture's own known real header/entity values — id-aware, computed independently here
/// since neither the oracle nor this adapter can reach `Ifc2x3Mutation::inverse()` (production's
/// own law degrades every kind to a whole-snapshot `SetSnapshot` restore, which is honest about
/// what the SUBJECT can prove today but would be a vacuous oracle-side inverse).
/// `remove-instance`'s inverse is deliberately cross-kind (`upsert-instance`), the same pattern
/// `step/🔖️ap214/✳️base`'s own `insert-entity`/`remove-entity` pair uses.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("fileSchema", json_str_array(&["IFC2X3"]))])),
        "upsert-instance" => json_spec("upsert-instance", json_obj(vec![("instance", column_instance("UC-Universal Columns-Column:UC305x305x97:552739"))])),
        "remove-instance" => json_spec("upsert-instance", json_obj(vec![("instance", wall_instance())])),
        "set-header" => json_spec("set-header", json_obj(vec![("header", wellness_header("0001"))])),
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
    let baseline = project_ifc_2x3_any(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ifc_2x3_any(&bytes)?;
    if kind != "no-mutation" && projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the IFC2X3 building model unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the reference dispatcher applies the
/// forward mutation and then the independently computed `inverse_spec`, and the restored
/// building model MUST project exactly as the untouched one does. `no-mutation` is NOT short-circuited: it runs the same
/// two cycles as every other kind, so the trivial case is evidence rather than an exemption. The
/// baseline runs two `no-mutation` cycles for the same reason -- both sides then carry identical
/// serializer normalisation and the comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_ifc_2x3_any(&oracle_apply_mutation(&oracle_apply_mutation(&input, &no_mutation())?, &no_mutation())?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind))?;
    let projection = project_ifc_2x3_any(&restored)?;
    assert_same_projection(&format!("inverse law violated for {kind:?} -- undoing it did not restore the building model"), &baseline, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the reference dispatcher fully
/// parses the real building model with `ruststep` and re-serializes it from its own
/// from-scratch Part-21 writer alone, so the re-encoded bytes MUST carry the same semantic projection as the input AND
/// MUST NOT be bit-identical to it. ISO 10303-21 clear text is not a byte-preserving carrier -- the
/// whole exchange structure is regenerated from the parsed model -- so the byte tripwire is real
/// evidence that the document was parsed rather than copied.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_ifc_2x3_any(&input)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
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
    use super::{inverse_spec, json_obj, json_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::io::{decode_ifc2x3, encode_ifc2x3};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, remove_instance, set_header, set_snapshot, upsert_instance, Ifc2x3Mutation};
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::engine::part21::{Part21Header, Part21Instance, Part21Value};
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::any::project_ifc_2x3_any;

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
        value.array(key).iter().filter_map(|entry| match entry { Json::String(text) => Some(text.clone()), _ => None }).collect()
    }
    fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
        num_field(value, key).map(|number| number as u64)
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️ValueGrammar
    /// 🔤️ The same `{"t":..., "v":...}` wire grammar the oracle dispatcher speaks
    /// (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️base/🦀️oracle.rs`'s own `value_from_json`),
    /// independently re-implemented here against `Part21Value` rather than `ruststep::ast::
    /// Parameter`. `Part21Value::Typed` carries a `Vec` (general value list) where `Parameter::
    /// Typed` carries a single boxed value; this grammar's `typed` case only ever needs one
    /// argument, so it wraps into a one-element `Vec` -- a faithful, not lossy, narrowing.
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
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `Ifc2x3Mutation` for this subset's own `apply_ifc2x3_mutation`. `set-snapshot` is pragmatic
    /// (only overrides `FILE_SCHEMA` on `base`, the already-decoded document) — the same precedent
    /// `mutate-pdf-1-7`'s and `mutate-step-ap214`'s own oracles use, needed here since a full
    /// 3464-entity snapshot literal has no place in a readable Gherkin cell.
    fn mutation_from_spec(spec: &Json, base: &Ifc2x3Snapshot) -> Result<Ifc2x3Mutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            // 🧭️ "no-mutation" is no longer a declared `Ifc2x3Mutation` kind (`NoMutation` was
            // dropped, `dsl::Mutations` rejects a wrapper-less variant) but `subject::round_trip`
            // still drives one through this spec grammar as its baseline; a `SetSnapshot` back onto
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
            "remove-instance" => Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: u64_field(params, "id")? }),
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone — the no-byte-pass-
    /// through rule this wave exists to enforce. `decode_ifc2x3`/`encode_ifc2x3` are this subset's
    /// own real codec (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️base/🚪️io/🦀️.rs`): standard-
    /// specific `FILE_SCHEMA` validation plus the shared `step::engine::part21` tokenizer/writer.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let snapshot = decode_ifc2x3(input)?;
        let base = snapshot.clone();
        let mutation = mutation_from_spec(spec, &base)?;
        let mut next = snapshot;
        apply_ifc2x3_mutation(&mut next, &mutation);
        let bytes = encode_ifc2x3(&next)?;
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
        let projection = project_ifc_2x3_any(&bytes)?;
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
        let projection = project_ifc_2x3_any(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = apply_and_encode(&input, &json_spec("no-mutation", json_obj(vec![])))?;
        let projection = project_ifc_2x3_any(&bytes)?;
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
