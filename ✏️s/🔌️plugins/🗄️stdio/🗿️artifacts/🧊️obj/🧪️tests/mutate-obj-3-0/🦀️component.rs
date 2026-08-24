//! 🦀️ OBJ 3.0 mutation case — Rust adapter. Exhaustive: every declared `ObjMutation` kind
//! (`obj-3-0-any`, 22 kinds) gets a `mutate-<kind>` and an `inverse-<kind>` scenario, plus one
//! identity round trip. The oracle performs every kind by direct OBJ-grammar manipulation
//! (`../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`, independent of this
//! subset's own decode/encode/mutation code); the subject fully parses into `ObjSnapshot` and
//! re-serializes from it alone (no byte pass-through). Both results are read back by the
//! INDEPENDENT `tobj` reader before the `semantic-mesh-v1` profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::obj::standards::v3_0::subsets::any::{oracle_apply_mutation, oracle_snapshot_json};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, reparsed_not_copied, round_trip_preserves_within};
use semio_s_plugin_stdio_test_oracle::mesh::project_obj;

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `ObjMutation::KINDS` (`../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/
/// 🧬️schema/🧬️mutations/🦀️component.rs`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against the `obj-3-0-any` catalog) is what keeps the two lists honest against each other.
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-vertex",
    "remove-vertex",
    "set-vertex",
    "insert-texcoord",
    "remove-texcoord",
    "set-texcoord",
    "insert-normal",
    "remove-normal",
    "set-normal",
    "insert-face",
    "remove-face",
    "set-face",
    "set-group",
    "remove-group",
    "set-object",
    "remove-object",
    "set-mtllib",
    "set-usemtl",
    "set-smoothing-groups",
    "set-unknown-statements",
];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧊️pattern-sphere.obj";

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
fn json_index_list(indices: &[usize]) -> Json {
    Json::Array(indices.iter().map(|index| json_num(*index as f64)).collect())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Profile
/// 📏️ `semantic-mesh-v1`'s own declared tolerances (`../../../../🧪️oracle/🔣️component.json`),
/// mirrored here so an in-handler law check is exactly as strict as the profile the case is
/// measured by — never stricter, which would invent a failure the comparison itself would forgive.
const MESH_WRITER_FREEDOM: &[&str] = &["generator", "comment", "byteLength", "fileSize", "precision", "solidName"];
const MESH_TOLERANCE: f64 = 1e-5;
//#endregion 🔖️Profile

//#region 🔖️Inverse
/// ↩️ The original real fixture's 7 retained comment lines, in file order (`🧫️fixtures/
/// 🧊️pattern-sphere.obj`'s own header plus its trailing orphan-vertex note) — `set-unknown-
/// statements`'s inverse restores exactly this list.
const ORIGINAL_UNKNOWN_STATEMENTS: [&str; 7] = [
    "# stdio.obj 3.0 real-world fixture, derived once from real committed geometry.",
    "# source: shared-glb 🧰️framework/🔨️modules/🖼️assets/🖼️images/🧊️pattern-sphere.glb",
    "# derivation: hand-parsed GLB container (12-byte header, JSON chunk, BIN chunk); POSITION/NORMAL/TEXCOORD_0",
    "# accessors and the index accessor read directly with plain Rust-equivalent struct decoding (this script), no",
    "# gltf crate. Vertex/normal/texcoord/face data below is the real mesh; o/g/usemtl band names are an editorial",
    "# partition of that same real face range, not fabricated geometry. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.",
    "# trailing orphan v/vt/vn above: a duplicate of index 0, unreferenced by any face on purpose",
];

/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// pristine fixture's own known real values — index/name-aware, mirroring the same per-variant
/// `ObjMutation::inverse()` semantics `../../🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
/// 🦀️component.rs` documents, computed independently here since neither the oracle nor this adapter
/// can reach that subject-side method. `set-snapshot`'s inverse is a REAL `set-snapshot` carrying
/// the original document's own independently emitted payload (`oracle_snapshot_json`) — never a
/// hand-back of the pristine input bytes, which would let the scenario pass without the reference
/// re-serializing anything at all, which is why it needs `base` rather than the kind alone.
fn inverse_spec(kind: &str, base: &[u8]) -> Result<Json, String> {
    Ok(match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("snapshot", oracle_snapshot_json(base)?)])),
        "no-mutation" => json_spec("no-mutation", json_obj(vec![])),
        "insert-vertex" => json_spec("remove-vertex", json_obj(vec![("index", json_num(8449.0))])),
        "remove-vertex" => json_spec("insert-vertex", json_obj(vec![("index", json_num(8448.0)), ("vertex", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))])),
        "set-vertex" => json_spec("set-vertex", json_obj(vec![("index", json_num(0.0)), ("vertex", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))])),
        "insert-texcoord" => json_spec("remove-texcoord", json_obj(vec![("index", json_num(8449.0))])),
        "remove-texcoord" => json_spec("insert-texcoord", json_obj(vec![("index", json_num(8448.0)), ("texcoord", json_obj(vec![("u", json_num(0.00390625)), ("v", json_num(0.0))]))])),
        "set-texcoord" => json_spec("set-texcoord", json_obj(vec![("index", json_num(0.0)), ("texcoord", json_obj(vec![("u", json_num(0.00390625)), ("v", json_num(0.0))]))])),
        "insert-normal" => json_spec("remove-normal", json_obj(vec![("index", json_num(8449.0))])),
        "remove-normal" => json_spec("insert-normal", json_obj(vec![("index", json_num(8448.0)), ("normal", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))])),
        "set-normal" => json_spec("set-normal", json_obj(vec![("index", json_num(0.0)), ("normal", json_obj(vec![("x", json_num(0.0)), ("y", json_num(-1.0)), ("z", json_num(0.0))]))])),
        "insert-face" => json_spec("remove-face", json_obj(vec![("index", json_num(16128.0))])),
        "remove-face" | "set-face" => {
            let face = json_obj(vec![(
                "vertices",
                Json::Array(vec![
                    json_obj(vec![("vertex", json_num(8384.0)), ("texcoord", json_num(8384.0)), ("normal", json_num(8384.0))]),
                    json_obj(vec![("vertex", json_num(8318.0)), ("texcoord", json_num(8318.0)), ("normal", json_num(8318.0))]),
                    json_obj(vec![("vertex", json_num(8383.0)), ("texcoord", json_num(8383.0)), ("normal", json_num(8383.0))]),
                ]),
            )]);
            let restore_kind = if kind == "remove-face" { "insert-face" } else { "set-face" };
            json_spec(restore_kind, json_obj(vec![("index", json_num(16127.0)), ("face", face)]))
        }
        "set-group" => json_spec("remove-group", json_obj(vec![("name", json_str("equator"))])),
        "remove-group" => json_spec("set-group", json_obj(vec![("name", json_str("apex-band")), ("faces", json_index_list(&[0, 1, 2]))])),
        "set-object" => json_spec("remove-object", json_obj(vec![("name", json_str("north-cap"))])),
        "remove-object" => json_spec("set-object", json_obj(vec![("name", json_str("apex")), ("faces", json_index_list(&[0, 1, 2]))])),
        "set-mtllib" => json_spec("set-mtllib", json_obj(vec![])),
        "set-usemtl" => json_spec("set-usemtl", json_obj(vec![("usemtl", Json::Array(vec![json_obj(vec![("faceIndexFrom", json_num(0.0)), ("material", json_str("pattern"))])]))])),
        "set-smoothing-groups" => json_spec("set-smoothing-groups", json_obj(vec![("smoothingGroups", Json::Array(vec![]))])),
        "set-unknown-statements" => {
            let lines = ORIGINAL_UNKNOWN_STATEMENTS.iter().map(|raw| json_obj(vec![("raw", json_str(raw))])).collect();
            json_spec("set-unknown-statements", json_obj(vec![("unknownStatements", Json::Array(lines))]))
        }
        other => json_spec(other, json_obj(vec![])),
    })
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_obj(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase: every kind — INCLUDING
/// `set-snapshot`, which now inverts through a real `set-snapshot` of the original document instead
/// of returning the pristine bytes — is applied forward and then undone, and the restored mesh's
/// independent `tobj` projection must equal the REAL original's own. `semantic-mesh-v1`'s own
/// tolerance (1e-5, `generator`/`comment`/`precision` writer freedom) is what the comparison uses,
/// never a stricter one.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input)?)?;
    let projection = project_obj(&restored)?;
    inverse_restores_within(&kind, &projection, &project_obj(&input)?, MESH_WRITER_FREEDOM, MESH_TOLERANCE)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, both halves asserted in role: parsing the real fixture and re-rendering the
/// whole OBJ grammar from the parsed model alone must preserve the mesh projection, and must NOT
/// hand back the input bytes — `render` re-derives every statement and emits the retained comment
/// lines after the geometry rather than where the file carried them, so bit-identical output would
/// mean nothing was parsed.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &json_spec("no-mutation", json_obj(vec![])))?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_obj(&bytes)?;
    round_trip_preserves_within(&projection, &project_obj(&input)?, MESH_WRITER_FREEDOM, MESH_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::subsets::any::io::{decode_obj, encode_obj};
    use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::subsets::any::schema::mutations::{apply_obj_mutation, ObjMutation};
    use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::subsets::any::schema::snapshot::{ObjFace, ObjFaceVertex, ObjGroup, ObjNormal, ObjObject, ObjSmoothingRange, ObjSnapshot, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
    use semio_s_plugin_stdio_test_oracle::mesh::project_obj;

    //#region 🔖️SpecReading
    fn json_num(value: &Json, key: &str) -> Option<f64> {
        match value.get(key) {
            Some(Json::Number(number)) => Some(*number),
            _ => None,
        }
    }
    fn json_str(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }
    fn json_usize(value: &Json, key: &str) -> Result<usize, String> {
        json_num(value, key).map(|number| number as usize).ok_or_else(|| format!("expected numeric field {key:?}"))
    }
    fn usize_array(value: &Json, key: &str) -> Result<Vec<usize>, String> {
        value.array(key).iter().map(|entry| match entry { Json::Number(number) => Ok(*number as usize), other => Err(format!("expected a numeric array for {key:?}, found {other:?}")) }).collect()
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️ItemParsing
    fn parse_vertex(value: &Json) -> Result<ObjVertex, String> {
        Ok(ObjVertex { x: json_num(value, "x").ok_or("vertex.x")?, y: json_num(value, "y").ok_or("vertex.y")?, z: json_num(value, "z").ok_or("vertex.z")?, w: json_num(value, "w") })
    }
    fn parse_texcoord(value: &Json) -> Result<ObjTexCoord, String> {
        Ok(ObjTexCoord { u: json_num(value, "u").ok_or("texcoord.u")?, v: json_num(value, "v").unwrap_or(0.0), w: json_num(value, "w") })
    }
    fn parse_normal(value: &Json) -> Result<ObjNormal, String> {
        Ok(ObjNormal { x: json_num(value, "x").ok_or("normal.x")?, y: json_num(value, "y").ok_or("normal.y")?, z: json_num(value, "z").ok_or("normal.z")? })
    }
    fn parse_face(value: &Json) -> Result<ObjFace, String> {
        let vertices = value
            .array("vertices")
            .iter()
            .map(|entry| Ok(ObjFaceVertex { vertex: json_usize(entry, "vertex")? as u32, texcoord: json_num(entry, "texcoord").map(|number| number as u32), normal: json_num(entry, "normal").map(|number| number as u32) }))
            .collect::<Result<Vec<_>, String>>()?;
        Ok(ObjFace { vertices })
    }
    fn snapshot_from_json(value: &Json) -> Result<ObjSnapshot, String> {
        let mut snapshot = ObjSnapshot::default();
        for entry in value.array("vertices") {
            snapshot.vertices.push(parse_vertex(&entry)?);
        }
        for entry in value.array("texcoords") {
            snapshot.texcoords.push(parse_texcoord(&entry)?);
        }
        for entry in value.array("normals") {
            snapshot.normals.push(parse_normal(&entry)?);
        }
        for entry in value.array("faces") {
            snapshot.faces.push(parse_face(&entry)?);
        }
        for entry in value.array("groups") {
            snapshot.groups.push(ObjGroup { name: json_str(&entry, "name").ok_or("group.name")?, faces: usize_array(&entry, "faces")? });
        }
        for entry in value.array("objects") {
            snapshot.objects.push(ObjObject { name: json_str(&entry, "name").ok_or("object.name")?, faces: usize_array(&entry, "faces")? });
        }
        snapshot.mtllib = json_str(value, "mtllib");
        for entry in value.array("usemtlRanges") {
            snapshot.usemtl.push(ObjUsemtlRange { face_index_from: json_usize(&entry, "faceIndexFrom")?, material: json_str(&entry, "material").ok_or("usemtl.material")? });
        }
        for entry in value.array("smoothingGroups") {
            snapshot.smoothing_groups.push(ObjSmoothingRange { face_index_from: json_usize(&entry, "faceIndexFrom")?, group: json_num(&entry, "group").map(|number| number as u32) });
        }
        for entry in value.array("unknownStatements") {
            snapshot.unknown_statements.push(ObjUnknownStatement { line_index: 0, raw: json_str(&entry, "raw").ok_or("unknown.raw")? });
        }
        Ok(snapshot)
    }
    //#endregion 🔖️ItemParsing

    //#region 🔖️MutationFromSpec
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `ObjMutation` value for this subset's own `apply_obj_mutation`.
    fn mutation_from_spec(spec: &Json) -> Result<ObjMutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            "no-mutation" => ObjMutation::NoMutation,
            "set-snapshot" => ObjMutation::SetSnapshot { snapshot: snapshot_from_json(params.get("snapshot").ok_or("set-snapshot requires a snapshot field")?)? },
            "insert-vertex" => ObjMutation::InsertVertex { index: json_usize(params, "index")?, vertex: parse_vertex(params.get("vertex").ok_or("insert-vertex requires a vertex field")?)? },
            "remove-vertex" => ObjMutation::RemoveVertex { index: json_usize(params, "index")? },
            "set-vertex" => ObjMutation::SetVertex { index: json_usize(params, "index")?, vertex: parse_vertex(params.get("vertex").ok_or("set-vertex requires a vertex field")?)? },
            "insert-texcoord" => ObjMutation::InsertTexCoord { index: json_usize(params, "index")?, texcoord: parse_texcoord(params.get("texcoord").ok_or("insert-texcoord requires a texcoord field")?)? },
            "remove-texcoord" => ObjMutation::RemoveTexCoord { index: json_usize(params, "index")? },
            "set-texcoord" => ObjMutation::SetTexCoord { index: json_usize(params, "index")?, texcoord: parse_texcoord(params.get("texcoord").ok_or("set-texcoord requires a texcoord field")?)? },
            "insert-normal" => ObjMutation::InsertNormal { index: json_usize(params, "index")?, normal: parse_normal(params.get("normal").ok_or("insert-normal requires a normal field")?)? },
            "remove-normal" => ObjMutation::RemoveNormal { index: json_usize(params, "index")? },
            "set-normal" => ObjMutation::SetNormal { index: json_usize(params, "index")?, normal: parse_normal(params.get("normal").ok_or("set-normal requires a normal field")?)? },
            "insert-face" => ObjMutation::InsertFace { index: json_usize(params, "index")?, face: parse_face(params.get("face").ok_or("insert-face requires a face field")?)? },
            "remove-face" => ObjMutation::RemoveFace { index: json_usize(params, "index")? },
            "set-face" => ObjMutation::SetFace { index: json_usize(params, "index")?, face: parse_face(params.get("face").ok_or("set-face requires a face field")?)? },
            "set-group" => ObjMutation::SetGroup { name: json_str(params, "name").ok_or("set-group requires a name field")?, faces: usize_array(params, "faces")? },
            "remove-group" => ObjMutation::RemoveGroup { name: json_str(params, "name").ok_or("remove-group requires a name field")? },
            "set-object" => ObjMutation::SetObject { name: json_str(params, "name").ok_or("set-object requires a name field")?, faces: usize_array(params, "faces")? },
            "remove-object" => ObjMutation::RemoveObject { name: json_str(params, "name").ok_or("remove-object requires a name field")? },
            "set-mtllib" => ObjMutation::SetMtllib { mtllib: json_str(params, "mtllib") },
            "set-usemtl" => ObjMutation::SetUsemtl {
                usemtl: params.array("usemtl").iter().map(|entry| Ok(ObjUsemtlRange { face_index_from: json_usize(entry, "faceIndexFrom")?, material: json_str(entry, "material").ok_or("usemtl.material")? })).collect::<Result<Vec<_>, String>>()?,
            },
            "set-smoothing-groups" => ObjMutation::SetSmoothingGroups {
                smoothing_groups: params.array("smoothingGroups").iter().map(|entry| Ok(ObjSmoothingRange { face_index_from: json_usize(entry, "faceIndexFrom")?, group: json_num(entry, "group").map(|number| number as u32) })).collect::<Result<Vec<_>, String>>()?,
            },
            "set-unknown-statements" => ObjMutation::SetUnknownStatements {
                unknown_statements: params.array("unknownStatements").iter().enumerate().map(|(index, entry)| Ok(ObjUnknownStatement { line_index: index, raw: json_str(entry, "raw").ok_or("unknown.raw")? })).collect::<Result<Vec<_>, String>>()?,
            },
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ Full parse → typed mutation → re-serialize from the model alone — the no-byte-pass-
    /// through rule this wave exists to enforce.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let mut snapshot = decode_obj(text).map_err(|error| format!("decode_obj failed: {error}"))?;
        let mutation = mutation_from_spec(spec)?;
        apply_obj_mutation(&mut snapshot, &mutation);
        let bytes = encode_obj(&snapshot).into_bytes();
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
        let projection = project_obj(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Every kind, INCLUDING `set-snapshot`, is genuinely applied forward and then undone through
    /// this repository's own `ObjMutation` pipeline — `set-snapshot` inverts through a real
    /// `set-snapshot` carrying the original document's independently emitted payload, never through
    /// a hand-back of the pristine input bytes.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutated = apply_and_encode(&input, &spec)?;
        let restored = apply_and_encode(&mutated, &inverse_spec(&kind, &input)?)?;
        let projection = project_obj(&restored)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = std::str::from_utf8(&input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let snapshot = decode_obj(text).map_err(|error| format!("decode_obj failed: {error}"))?;
        let bytes = encode_obj(&snapshot).into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_obj(&bytes)?;
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
