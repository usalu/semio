//! 🦀️ IFC2X3 / ✳️sav mutation case — Rust adapter. Exhaustive: every declared `Ifc2x3SavMutation`
//! kind (`ifc-2x3-sav`, 6 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. `ruststep` 0.4 can only READ Part-21 text, so the oracle dispatcher
//! (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🦀️oracle.rs`) performs every kind against a
//! `ruststep`-parsed document and re-serializes through the standard-level from-scratch writer,
//! independent of this repository's own `step::engine::part21` codec; the subject fully parses into
//! `Ifc2x3Snapshot` and re-serializes from it alone (no byte pass-through). Both results are read
//! back by the INDEPENDENT `ruststep` reader (`project_ifc_2x3_sav`) before `semantic-ifc-v1`
//! compares them — real third-party evidence about structure, never a byte-level differential claim.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::sav::{oracle_apply_mutation, project_ifc_2x3_sav};

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `Ifc2x3SavMutation::KINDS`
/// (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧬️schema/🧬️mutations/🦀️.rs`). The contract gate
/// (mutation coverage against the `ifc-2x3-sav` catalog) keeps the two lists honest.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-view-definition", "set-analysis-model", "set-load-group", "set-group-assignment"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧪️wellness-center-sama-structural-seed/🏗️.ifc";

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
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️SeededRows
/// 🏗️ The three structural entities the committed input carries, exactly as it holds them — see the
/// feature file for why they are seeded rather than real. Kept in one place so the inverses never
/// drift from the fixture.
fn seed_model() -> Json {
    json_obj(vec![("globalId", json_str("2SavAnalysisModelSeed001")), ("ownerHistory", json_num(41.0)), ("name", json_str("Street level analysis model"))])
}
fn seed_load_group() -> Json {
    json_obj(vec![("globalId", json_str("2SavLoadGroupSeed00000001")), ("ownerHistory", json_num(41.0)), ("name", json_str("Self weight"))])
}
fn seed_assignment() -> Json {
    json_obj(vec![
        ("globalId", json_str("2SavGroupAssignmentSeed01")),
        ("ownerHistory", json_num(41.0)),
        ("relatedObjects", Json::Array(vec![json_num(270549.0), json_num(523123.0)])),
        ("relatingGroup", json_num(9_200_001.0)),
    ])
}
//#endregion 🔖️SeededRows

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair. Every structural
/// forward clears its concept, so every structural inverse restores the exact row the committed
/// input holds; `set-analysis-model`'s inverse is also what heals the dangling `#9200001` reference
/// the forward direction deliberately leaves in `#9200003`.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("fileSchema", json_str_array(&["IFC2X3"]))])),
        "set-view-definition" => json_spec("set-view-definition", json_obj(vec![("view", json_str("StructuralAnalysisView"))])),
        "set-analysis-model" => json_spec("set-analysis-model", json_obj(vec![("id", json_num(9_200_001.0)), ("model", seed_model())])),
        "set-load-group" => json_spec("set-load-group", json_obj(vec![("id", json_num(9_200_002.0)), ("group", seed_load_group())])),
        "set-group-assignment" => json_spec("set-group-assignment", json_obj(vec![("id", json_num(9_200_003.0)), ("assignment", seed_assignment())])),
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
    let baseline = project_ifc_2x3_sav(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_ifc_2x3_sav(&bytes)?;
    if kind != "no-mutation" && projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the Structural Analysis View model view unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the reference dispatcher applies the
/// forward mutation and then the independently computed `inverse_spec`, and the restored model
/// MUST project exactly as the untouched one does. `no-mutation` is NOT short-circuited: it runs the same
/// two cycles as every other kind, so the trivial case is evidence rather than an exemption. The
/// baseline runs two `no-mutation` cycles for the same reason -- both sides then carry identical
/// serializer normalisation and the comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_ifc_2x3_sav(&oracle_apply_mutation(&oracle_apply_mutation(&input, &no_mutation())?, &no_mutation())?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind))?;
    let projection = project_ifc_2x3_sav(&restored)?;
    assert_same_projection(&format!("inverse law violated for {kind:?} -- undoing it did not restore the model"), &baseline, &projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the reference dispatcher fully
/// parses the real model with `ruststep` and re-serializes it from its own from-scratch
/// Part-21 writer alone, so the re-encoded bytes MUST carry the same semantic projection as the input AND
/// MUST NOT be bit-identical to it. ISO 10303-21 clear text is not a byte-preserving carrier -- the
/// whole exchange structure is regenerated from the parsed model -- so the byte tripwire is real
/// evidence that the document was parsed rather than copied.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_ifc_2x3_sav(&input)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the document was parsed".to_string());
    }
    let projection = project_ifc_2x3_sav(&bytes)?;
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
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
    use semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::sav::schema::mutations::{apply_ifc2x3_sav_mutation, Ifc2x3SavMutation, SavAnalysisModel, SavGroupAssignment, SavLoadGroup};
    use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::engine::part21::Part21Value;
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::sav::project_ifc_2x3_sav;

    //#region 🔖️SpecReading
    fn num_field(value: &Json, key: &str) -> Result<f64, String> {
        match value.get(key) {
            Some(Json::Number(number)) => Ok(*number),
            _ => Err(format!("expected numeric field {key:?}")),
        }
    }
    fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
        num_field(value, key).map(|number| number as u64)
    }
    fn str_field(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(text)) => Ok(text.clone()),
            _ => Err(format!("expected string field {key:?}")),
        }
    }
    fn opt_u64_field(value: &Json, key: &str) -> Option<u64> {
        match value.get(key) {
            Some(Json::Number(number)) => Some(*number as u64),
            _ => None,
        }
    }
    fn opt_str_field(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }
    fn u64_array(value: &Json, key: &str) -> Vec<u64> {
        value
            .array(key)
            .iter()
            .filter_map(|entry| match entry {
                Json::Number(number) => Some(*number as u64),
                _ => None,
            })
            .collect()
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
    //#endregion 🔖️SpecReading

    //#region 🔖️MutationFromSpec
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `Ifc2x3SavMutation`. `set-snapshot` only overrides `FILE_SCHEMA` on the already-decoded
    /// document — the same precedent `mutate-ifc-2x3` uses.
    fn mutation_from_spec(spec: &Json, base: &Ifc2x3Snapshot) -> Result<Ifc2x3SavMutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            "set-snapshot" => {
                let schemas = str_array(params, "fileSchema");
                if schemas.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                let mut snapshot = base.clone();
                snapshot.document.header.file_schema = vec![Part21Value::List(schemas.into_iter().map(Part21Value::Str).collect())];
                Ifc2x3SavMutation::SetSnapshot(semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::sav::schema::mutations::set_snapshot::SetSnapshot { snapshot })
            }
            "set-view-definition" => Ifc2x3SavMutation::SetViewDefinition(semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::sav::schema::mutations::set_view_definition::SetViewDefinition { view: str_field(params, "view")? }),
            "set-analysis-model" => {
                let model = match params.get("model") {
                    Some(value @ Json::Object(_)) => Some(SavAnalysisModel {
                        global_id: str_field(value, "globalId")?,
                        owner_history: opt_u64_field(value, "ownerHistory"),
                        name: str_field(value, "name")?,
                        predefined_type: opt_str_field(value, "predefinedType"),
                    }),
                    _ => None,
                };
                Ifc2x3SavMutation::SetAnalysisModel(semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::sav::schema::mutations::set_analysis_model::SetAnalysisModel { id: u64_field(params, "id")?, model })
            }
            "set-load-group" => {
                let group = match params.get("group") {
                    Some(value @ Json::Object(_)) => Some(SavLoadGroup {
                        global_id: str_field(value, "globalId")?,
                        owner_history: opt_u64_field(value, "ownerHistory"),
                        name: str_field(value, "name")?,
                        predefined_type: opt_str_field(value, "predefinedType"),
                        action_type: opt_str_field(value, "actionType"),
                        action_source: opt_str_field(value, "actionSource"),
                    }),
                    _ => None,
                };
                Ifc2x3SavMutation::SetLoadGroup(semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::sav::schema::mutations::set_load_group::SetLoadGroup { id: u64_field(params, "id")?, group })
            }
            "set-group-assignment" => {
                let assignment = match params.get("assignment") {
                    Some(value @ Json::Object(_)) => Some(SavGroupAssignment {
                        global_id: str_field(value, "globalId")?,
                        owner_history: opt_u64_field(value, "ownerHistory"),
                        related_objects: u64_array(value, "relatedObjects"),
                        relating_group: u64_field(value, "relatingGroup")?,
                    }),
                    _ => None,
                };
                Ifc2x3SavMutation::SetGroupAssignment(semio_s_plugin_stdio::artifacts::ifc::standards::v2x3::subsets::sav::schema::mutations::set_group_assignment::SetGroupAssignment { id: u64_field(params, "id")?, assignment })
            }
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone — the no-byte-pass-
    /// through rule this wave exists to enforce.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let snapshot = decode_ifc2x3(input)?;
        let mutation = mutation_from_spec(spec, &snapshot)?;
        let mut next = snapshot;
        apply_ifc2x3_sav_mutation(&mut next, &mutation);
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
        let projection = project_ifc_2x3_sav(&bytes)?;
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
        let projection = project_ifc_2x3_sav(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = apply_and_encode(&input, &json_spec("no-mutation", json_obj(vec![])))?;
        let projection = project_ifc_2x3_sav(&bytes)?;
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
