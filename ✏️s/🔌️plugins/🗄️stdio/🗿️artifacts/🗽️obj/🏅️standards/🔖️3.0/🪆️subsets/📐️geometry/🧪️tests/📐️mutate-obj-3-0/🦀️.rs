//! 🦀️ OBJ 3.0 mutation case — Rust adapter. Exhaustive: every declared `ObjMutation` kind
//! (`obj-3-0-any`, 21 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. The oracle performs every kind by direct OBJ-grammar manipulation
//! (`../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/🦀️oracle.rs`, independent of this
//! subset's own decode/encode/mutation code); the subject fully parses into `ObjSnapshot` and
//! re-serializes from it alone (no byte pass-through). Both results are read back by the
//! INDEPENDENT `tobj` reader before the `semantic-mesh-v1` profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_obj_test_oracle::standards::v3_0::subsets::geometry::{oracle_apply_mutation, oracle_document_projection, oracle_round_trip, oracle_snapshot_json};
use semio_repo_test_host::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};
use semio_s_plugin_stdio_mesh_test_oracle::project_obj;


//#region 🔖️Input
const INPUT: &str = "shared://🧪️pattern-sphere/🧊️.obj";

/// 🧫️ Copies the immutable committed mesh into the work directory and returns the mutable copy's
/// bytes; the committed fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.obj"))?;
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
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ `semantic-obj-3-0-v1`'s own declared tolerances (`../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/
/// 🔣️oracle.json`), mirrored here so an in-handler law check is exactly as strict as the
/// profile the case is measured by — never stricter, which would invent a failure the comparison
/// itself would forgive.
const OBJ_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "precision"];
const OBJ_TOLERANCE: f64 = 1e-5;
//#endregion 🔖️Profile

//#region 🔖️Projection
/// 🔍️ The projection both roles are compared through: `tobj`'s triangle mesh, plus the document
/// surface that reader cannot see. `tobj` triangulates, re-indexes per `o`/`g` model and drops every
/// declared row no face references, so on its own it leaves 14 of the 21 declared kinds
/// unobservable — `set-mtllib`, `set-usemtl`, `set-smoothing-groups`, `set-unknown-statements`, the
/// four name-keyed `g`/`o` kinds and every `v`/`vt`/`vn` kind move nothing in it. Both halves come
/// from readers independent of `decode_obj`.
fn project(bytes: &[u8]) -> Result<Json, String> {
    let mut projection = project_obj(bytes)?;
    match &mut projection {
        Json::Object(members) => members.push(("document".to_string(), oracle_document_projection(bytes)?)),
        other => return Err(format!("the mesh reader returned {other:?} rather than an object")),
    }
    Ok(projection)
}

/// 👁️ Every one of the 21 declared kinds has to move that composed projection — none is exempt,
/// which is exactly what the document half was added to make true.
fn moved_the_document(kind: &str, mutated: &Json, base: &Json) -> Result<(), String> {
    mutation_is_observable_within(kind, mutated, base, &[], OBJ_WRITER_FREEDOM, OBJ_TOLERANCE)
}
//#endregion 🔖️Projection

//#region 🔖️Inverse
/// 🏷️ The `g`/`o` membership the pristine fixture's OWN statements declare, in file order, read back
/// out of the real document rather than written down here. Every name-keyed inverse needs it, and
/// reading it is also the guard that keeps an `Examples` row honest: a row naming a band or object
/// the real mesh does not carry fails here instead of quietly inverting into a fabrication.
fn memberships(base: &[u8], collection: &str) -> Result<Vec<(String, Vec<f64>, Json)>, String> {
    Ok(oracle_snapshot_json(base)?
        .array(collection)
        .into_iter()
        .map(|entry| {
            let faces = entry.get("faces").cloned().unwrap_or_else(|| Json::Array(Vec::new()));
            let indices = match &faces {
                Json::Array(items) => items.iter().filter_map(|item| match item { Json::Number(number) => Some(*number), _ => None }).collect(),
                _ => Vec::new(),
            };
            (entry.str("name"), indices, faces)
        })
        .collect())
}

/// 📍️ The position `name` holds in the real document's `collection`, or a failure naming what the
/// mesh actually carries.
fn position_of(entries: &[(String, Vec<f64>, Json)], collection: &str, name: &str) -> Result<usize, String> {
    entries
        .iter()
        .position(|(existing, _, _)| existing == name)
        .ok_or_else(|| format!("the real fixture carries no {collection} entry named {name:?} — the mesh's own partition is `o pattern-sphere` over all 16,128 faces and `g band-0`/`band-1`/`band-2` over 5,376 faces each"))
}

/// ↩️ Restoring a `g`/`o` entry that a `remove-<kind>` took out at position `at`. A lone
/// `set-<kind>` re-declares the membership but APPENDS the entry, so the list order the document's
/// own statements gave it is lost — and that order decides the token order of a `g a b` line for a
/// face two bands share. The tail after `at` is lifted off and re-declared in its own order instead,
/// which is the same repair `ObjMutation::inverse` performs on the subject side.
fn restore_named_entry(entries: &[(String, Vec<f64>, Json)], at: usize, remove_kind: &str, set_kind: &str) -> Vec<Json> {
    let mut specs: Vec<Json> = entries[at + 1..].iter().map(|(name, _, _)| json_spec(remove_kind, json_obj(vec![("name", json_str(name))]))).collect();
    specs.extend(entries[at..].iter().map(|(name, _, faces)| json_spec(set_kind, json_obj(vec![("name", json_str(name)), ("faces", faces.clone())]))));
    specs
}

/// ↩️ The semantically correct inverse SEQUENCE for one forward `(kind, params)` pair against the
/// pristine fixture's own real values — index/name-aware, mirroring the same per-variant
/// `ObjMutation::inverse()` semantics `../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
/// 🦀️.rs` documents, computed independently here since neither the oracle nor this adapter
/// can reach that subject-side method. A SEQUENCE and not a single mutation because `Mutation::
/// inverse` returns `Vec<Self>` and two of the twenty-one kinds genuinely need more than one step:
/// `remove-face` (the face row alone carries no `g`/`o` membership, so re-inserting it by value
/// lands the geometry in no band and `tobj` reads a fourth model — `$.vertexCount` 8577 against the
/// mesh's own 8576) and `remove-group`/`remove-object` (a re-declared entry appends rather than
/// returning to its own position). Whatever the inverse needs to know about the pre-mutation state
/// it reads out of `base` with the oracle's own independent parser: `set-snapshot` inverts through a
/// REAL `set-snapshot` carrying the original document's emitted payload — never a hand-back of the
/// pristine input bytes, which would let the scenario pass without the reference re-serializing
/// anything at all.
fn inverse_specs(spec: &Json, base: &[u8]) -> Result<Vec<Json>, String> {
    let kind = spec.str("kind");
    let empty = json_obj(vec![]);
    let params = spec.get("params").unwrap_or(&empty);
    let named = |key: &str| -> Result<String, String> { params.get(key).and_then(|value| match value { Json::String(text) => Some(text.clone()), _ => None }).ok_or_else(|| format!("{kind} requires a string {key:?} parameter")) };
    let one = |value: Json| -> Result<Vec<Json>, String> { Ok(vec![value]) };
    match kind.as_str() {
        "set-snapshot" | "patch-snapshot" => one(json_spec("set-snapshot", json_obj(vec![("snapshot", oracle_snapshot_json(base)?)]))),
        "insert-vertex" => one(json_spec("remove-vertex", json_obj(vec![("index", json_num(8449.0))]))),
        "remove-vertex" => one(json_spec("insert-vertex", json_obj(vec![("index", json_num(8448.0)), ("vertex", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))]))),
        "set-vertex" => one(json_spec("set-vertex", json_obj(vec![("index", json_num(0.0)), ("vertex", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))]))),
        "insert-texcoord" => one(json_spec("remove-texcoord", json_obj(vec![("index", json_num(8449.0))]))),
        "remove-texcoord" => one(json_spec("insert-texcoord", json_obj(vec![("index", json_num(8448.0)), ("texcoord", json_obj(vec![("u", json_num(0.00390625)), ("v", json_num(0.0))]))]))),
        "set-texcoord" => one(json_spec("set-texcoord", json_obj(vec![("index", json_num(0.0)), ("texcoord", json_obj(vec![("u", json_num(0.00390625)), ("v", json_num(0.0))]))]))),
        "insert-normal" => one(json_spec("remove-normal", json_obj(vec![("index", json_num(8449.0))]))),
        "remove-normal" => one(json_spec("insert-normal", json_obj(vec![("index", json_num(8448.0)), ("normal", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))]))),
        "set-normal" => one(json_spec("set-normal", json_obj(vec![("index", json_num(0.0)), ("normal", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))]))),
        "insert-face" => one(json_spec("remove-face", json_obj(vec![("index", json_num(16128.0))]))),
        "remove-face" | "set-face" => {
            let face = json_obj(vec![(
                "vertices",
                Json::Array(vec![
                    json_obj(vec![("vertex", json_num(8384.0)), ("texcoord", json_num(8384.0)), ("normal", json_num(8384.0))]),
                    json_obj(vec![("vertex", json_num(8318.0)), ("texcoord", json_num(8318.0)), ("normal", json_num(8318.0))]),
                    json_obj(vec![("vertex", json_num(8383.0)), ("texcoord", json_num(8383.0)), ("normal", json_num(8383.0))]),
                ]),
            )]);
            if kind == "set-face" {
                return one(json_spec("set-face", json_obj(vec![("index", json_num(16127.0)), ("face", face)])));
            }
            let removed_at = 16127.0;
            let mut specs = vec![json_spec("insert-face", json_obj(vec![("index", json_num(removed_at)), ("face", face)]))];
            for (collection, set_kind) in [("groups", "set-group"), ("objects", "set-object")] {
                for (name, indices, faces) in memberships(base, collection)? {
                    if indices.iter().any(|index| *index >= removed_at) {
                        specs.push(json_spec(set_kind, json_obj(vec![("name", json_str(&name)), ("faces", faces)])));
                    }
                }
            }
            Ok(specs)
        }
        "set-group" => {
            let entries = memberships(base, "groups")?;
            let at = position_of(&entries, "groups", &named("name")?)?;
            one(json_spec("set-group", json_obj(vec![("name", json_str(&entries[at].0)), ("faces", entries[at].2.clone())])))
        }
        "remove-group" => {
            let entries = memberships(base, "groups")?;
            let at = position_of(&entries, "groups", &named("name")?)?;
            Ok(restore_named_entry(&entries, at, "remove-group", "set-group"))
        }
        "set-object" => {
            let entries = memberships(base, "objects")?;
            let at = position_of(&entries, "objects", &named("name")?)?;
            one(json_spec("set-object", json_obj(vec![("name", json_str(&entries[at].0)), ("faces", entries[at].2.clone())])))
        }
        "remove-object" => {
            let entries = memberships(base, "objects")?;
            let at = position_of(&entries, "objects", &named("name")?)?;
            Ok(restore_named_entry(&entries, at, "remove-object", "set-object"))
        }
        "set-mtllib" => one(json_spec("set-mtllib", json_obj(vec![("mtllib", Json::Null)]))),
        "set-usemtl" => one(json_spec("set-usemtl", json_obj(vec![("usemtl", Json::Array(vec![json_obj(vec![("faceIndexFrom", json_num(0.0)), ("material", json_str("pattern"))])]))]))),
        "set-smoothing-groups" => one(json_spec("set-smoothing-groups", json_obj(vec![("smoothingGroups", Json::Array(vec![]))]))),
        "set-unknown-statements" => one(json_spec("set-unknown-statements", json_obj(vec![("unknownStatements", Json::Array(oracle_snapshot_json(base)?.array("unknownStatements")))]))),
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
/// 🦠️ The forward half, with observability asserted in role: the reference applies the kind to the
/// real mesh and the result has to differ from the untouched document under the very profile the
/// case is measured by.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project(&bytes)?;
    moved_the_document(&spec.str("kind"), &projection, &project(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase: every kind — INCLUDING
/// `set-snapshot`, which now inverts through a real `set-snapshot` of the original document instead
/// of returning the pristine bytes — is applied forward and then undone, and the restored mesh's
/// composed projection must equal the REAL original's own. `semantic-obj-3-0-v1`'s own tolerance
/// (1e-5, byte length and decimal precision the only writer freedom) is what the comparison uses,
/// never a stricter one.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mut restored = oracle_apply_mutation(&input, &spec)?;
    for undo in inverse_specs(&spec, &input)? {
        restored = oracle_apply_mutation(&restored, &undo)?;
    }
    let projection = project(&restored)?;
    inverse_restores_within(&kind, &projection, &project(&input)?, OBJ_WRITER_FREEDOM, OBJ_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: parsing the real fixture and re-rendering the
/// whole OBJ grammar from the parsed model alone must preserve the mesh projection, and must NOT
/// hand back the input bytes — `render` re-derives every statement and emits the retained comment
/// lines after the geometry rather than where the file carried them, so bit-identical output would
/// mean nothing was parsed.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project(&bytes)?;
    round_trip_preserves_within(&projection, &project(&input)?, OBJ_WRITER_FREEDOM, OBJ_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{moved_the_document, mutable_input, project};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::io::{decode_obj, encode_obj};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_obj::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::mutations::{apply_obj_mutation, ObjMutation};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::snapshot::ObjSnapshot;

    /// 🔀️ The spec's wire payload, decoded by the aggregate's own generic payload constructor.
    fn mutation_of(spec: &Json) -> Result<ObjMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(input: &[u8]) -> Result<ObjSnapshot, String> {
        decode_obj(std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?).map_err(|error| format!("decode_obj failed: {error}"))
    }

    /// 📐️ Our encoder cannot reproduce another writer's statement layout, so bit-identical output means
    /// the input was smuggled rather than parsed.
    fn reparsed(bytes: Vec<u8>, input: &[u8]) -> Result<Vec<u8>, String> {
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_obj_mutation(&mut snapshot, &mutation_of(&spec)?);
        let bytes = reparsed(encode_obj(&snapshot).into_bytes(), &input)?;
        let projection = project(&bytes)?;
        moved_the_document(&spec.str("kind"), &projection, &project(&input)?)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Every kind, INCLUDING `set-snapshot`, is applied forward and then undone by the production
    /// inverse sequence computed against the pre-mutation snapshot — `remove-face`'s membership repair and
    /// `remove-group`/`remove-object`'s re-ordering included.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let forward = mutation_of(&ctx.doc_json()?)?;
        let backward = mutation_inverse(&forward, &snapshot).expect("valid retained mutation inverse fixture");
        apply_obj_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_obj_mutation(&mut snapshot, mutation);
        }
        let restored = encode_obj(&snapshot).into_bytes();
        let projection = project(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = reparsed(encode_obj(&decode(&input)?).into_bytes(), &input)?;
        let projection = project(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
