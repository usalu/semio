//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `mesh` module rather than by copying it.
//!
//! OBJ has no reference WRITER in the Rust ecosystem (`tobj` is the reference READER, already used
//! by the shared `mesh` family module's `project_obj`/`oracle_create_obj`) — so, mirroring that same
//! precedent, this module performs every declared mutation by direct grammar manipulation of its own
//! independent `Model`, parsed and re-serialized by code that never touches this subset's own
//! `ObjSnapshot`/`decode_obj`/`encode_obj`/`ObjMutation` (`../🧬️schema`). `tobj` remains the
//! independent reader both this oracle's and the subject's results are projected through before
//! comparison — that projection call lives in the test case adapter, not here.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`ObjMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Model
/// 📸️ Independent, minimal OBJ model — every field this subset's `ObjSnapshot` also carries, typed
/// with this module's own structs so nothing here is the subject's type reused under another name.
#[cfg(feature = "oracles")]
#[derive(Clone, Debug, Default)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
    w: Option<f64>,
}

#[cfg(feature = "oracles")]
#[derive(Clone, Debug, Default)]
struct TexCoord {
    u: f64,
    v: f64,
    w: Option<f64>,
}

#[cfg(feature = "oracles")]
#[derive(Clone, Debug, Default)]
struct Normal {
    x: f64,
    y: f64,
    z: f64,
}

#[cfg(feature = "oracles")]
#[derive(Clone, Debug, Default)]
struct FaceVertex {
    vertex: u32,
    texcoord: Option<u32>,
    normal: Option<u32>,
}

#[cfg(feature = "oracles")]
#[derive(Clone, Debug, Default)]
struct Face {
    vertices: Vec<FaceVertex>,
}

/// 🧊️ The whole document: index-keyed geometry, name-keyed group/object face membership
/// (sticky-range, matching this format's own real-file assumption), range-tagged
/// material/smoothing transitions, and retained unknown/comment lines.
#[cfg(feature = "oracles")]
#[derive(Clone, Debug, Default)]
struct Model {
    vertices: Vec<Vertex>,
    texcoords: Vec<TexCoord>,
    normals: Vec<Normal>,
    faces: Vec<Face>,
    groups: Vec<(String, Vec<usize>)>,
    objects: Vec<(String, Vec<usize>)>,
    mtllib: Option<String>,
    usemtl: Vec<(usize, String)>,
    smoothing: Vec<(usize, Option<u32>)>,
    unknown: Vec<String>,
}
//#endregion 🔖️Model

//#region 🔖️Parse
#[cfg(feature = "oracles")]
fn resolve_index(current_len: usize, raw: i64) -> Result<u32, String> {
    if raw > 0 {
        return Ok((raw - 1) as u32);
    }
    if raw < 0 {
        let resolved = current_len as i64 + raw;
        return u32::try_from(resolved).map_err(|_| format!("relative index {raw} out of range (current length {current_len})"));
    }
    Err("index 0 is not valid in a 1-based OBJ reference".to_string())
}

#[cfg(feature = "oracles")]
fn parse_face_vertex_token(token: &str, vertex_count: usize, texcoord_count: usize, normal_count: usize) -> Result<FaceVertex, String> {
    let mut pieces = token.split('/');
    let raw_v: i64 = pieces.next().ok_or("empty face vertex token")?.parse().map_err(|error| format!("{error}"))?;
    let raw_vt = pieces.next().filter(|piece| !piece.is_empty()).map(str::parse::<i64>).transpose().map_err(|error| format!("{error}"))?;
    let raw_vn = pieces.next().filter(|piece| !piece.is_empty()).map(str::parse::<i64>).transpose().map_err(|error| format!("{error}"))?;
    Ok(FaceVertex {
        vertex: resolve_index(vertex_count, raw_v)?,
        texcoord: raw_vt.map(|raw| resolve_index(texcoord_count, raw)).transpose()?,
        normal: raw_vn.map(|raw| resolve_index(normal_count, raw)).transpose()?,
    })
}

/// 📥️ Independent parse of the same grammar `decode_obj` documents: sticky `o`/`g`/`usemtl`/`s`
/// ranges, `#` and unrecognised keywords retained verbatim. Its own loop, own structs, own
/// index resolution — not a call into this subset's `decode_obj`.
#[cfg(feature = "oracles")]
fn parse(text: &str) -> Result<Model, String> {
    let mut model = Model::default();
    let mut group_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut object_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut active_groups: Vec<String> = Vec::new();
    let mut active_object: Option<String> = None;
    let mut active_material: Option<String> = None;
    let mut active_smoothing: Option<u32> = None;
    let mut have_smoothing = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            model.unknown.push(line.to_string());
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f64 = parts.next().ok_or("v x")?.parse().map_err(|error| format!("{error}"))?;
                let y: f64 = parts.next().ok_or("v y")?.parse().map_err(|error| format!("{error}"))?;
                let z: f64 = parts.next().ok_or("v z")?.parse().map_err(|error| format!("{error}"))?;
                let w = parts.next().map(str::parse::<f64>).transpose().map_err(|error| format!("{error}"))?;
                model.vertices.push(Vertex { x, y, z, w });
            }
            Some("vt") => {
                let u: f64 = parts.next().ok_or("vt u")?.parse().map_err(|error| format!("{error}"))?;
                let v: f64 = parts.next().unwrap_or("0").parse().map_err(|error| format!("{error}"))?;
                let w = parts.next().map(str::parse::<f64>).transpose().map_err(|error| format!("{error}"))?;
                model.texcoords.push(TexCoord { u, v, w });
            }
            Some("vn") => {
                let x: f64 = parts.next().ok_or("vn x")?.parse().map_err(|error| format!("{error}"))?;
                let y: f64 = parts.next().ok_or("vn y")?.parse().map_err(|error| format!("{error}"))?;
                let z: f64 = parts.next().ok_or("vn z")?.parse().map_err(|error| format!("{error}"))?;
                model.normals.push(Normal { x, y, z });
            }
            Some("f") => {
                let mut vertices = Vec::new();
                for token in parts {
                    vertices.push(parse_face_vertex_token(token, model.vertices.len(), model.texcoords.len(), model.normals.len())?);
                }
                if vertices.len() < 3 {
                    return Err(format!("face has fewer than 3 vertices: {line}"));
                }
                let face_index = model.faces.len();
                model.faces.push(Face { vertices });
                for name in &active_groups {
                    let gi = *group_index.entry(name.clone()).or_insert_with(|| {
                        model.groups.push((name.clone(), Vec::new()));
                        model.groups.len() - 1
                    });
                    model.groups[gi].1.push(face_index);
                }
                if let Some(name) = &active_object {
                    let oi = *object_index.entry(name.clone()).or_insert_with(|| {
                        model.objects.push((name.clone(), Vec::new()));
                        model.objects.len() - 1
                    });
                    model.objects[oi].1.push(face_index);
                }
            }
            Some("o") => active_object = parts.next().map(str::to_string),
            Some("g") => active_groups = parts.map(str::to_string).collect(),
            Some("usemtl") => {
                let name = parts.next().map(str::to_string);
                if name != active_material {
                    if let Some(material) = &name {
                        model.usemtl.push((model.faces.len(), material.clone()));
                    }
                    active_material = name;
                }
            }
            Some("mtllib") => {
                let rest: Vec<&str> = parts.collect();
                if !rest.is_empty() {
                    model.mtllib = Some(rest.join(" "));
                }
            }
            Some("s") => {
                let next = match parts.next() {
                    Some("off") | None => None,
                    Some(value) => value.parse::<u32>().ok(),
                };
                if !have_smoothing || next != active_smoothing {
                    model.smoothing.push((model.faces.len(), next));
                    active_smoothing = next;
                    have_smoothing = true;
                }
            }
            _ => model.unknown.push(line.to_string()),
        }
    }
    Ok(model)
}
//#endregion 🔖️Parse

//#region 🔖️Render
#[cfg(feature = "oracles")]
fn write_face_vertex(fv: &FaceVertex) -> String {
    match (fv.texcoord, fv.normal) {
        (Some(vt), Some(vn)) => format!("{}/{}/{}", fv.vertex + 1, vt + 1, vn + 1),
        (Some(vt), None) => format!("{}/{}", fv.vertex + 1, vt + 1),
        (None, Some(vn)) => format!("{}//{}", fv.vertex + 1, vn + 1),
        (None, None) => format!("{}", fv.vertex + 1),
    }
}

#[cfg(feature = "oracles")]
fn faces_by_group(groups: &[(String, Vec<usize>)], face_count: usize) -> Vec<Vec<String>> {
    let mut out = vec![Vec::new(); face_count];
    for (name, faces) in groups {
        for &index in faces {
            if let Some(slot) = out.get_mut(index) {
                slot.push(name.clone());
            }
        }
    }
    out
}

#[cfg(feature = "oracles")]
fn faces_by_object(objects: &[(String, Vec<usize>)], face_count: usize) -> Vec<Option<String>> {
    let mut out = vec![None; face_count];
    for (name, faces) in objects {
        for &index in faces {
            if let Some(slot) = out.get_mut(index) {
                *slot = Some(name.clone());
            }
        }
    }
    out
}

#[cfg(feature = "oracles")]
fn faces_by_material_range(ranges: &[(usize, String)], face_count: usize) -> Vec<Option<String>> {
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|(from, _)| *from);
    let mut out = vec![None; face_count];
    for (from, material) in &sorted {
        for slot in out.iter_mut().skip(*from) {
            *slot = Some(material.clone());
        }
    }
    out
}

#[cfg(feature = "oracles")]
fn faces_by_smoothing_range(ranges: &[(usize, Option<u32>)], face_count: usize) -> Vec<Option<u32>> {
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|(from, _)| *from);
    let mut out = vec![None; face_count];
    for (from, group) in &sorted {
        for slot in out.iter_mut().skip(*from) {
            *slot = *group;
        }
    }
    out
}

/// 📤️ Independent re-serialization: `mtllib`, the `v`/`vt`/`vn` blocks, then one `f` line per face
/// preceded by whichever of `o`/`g`/`usemtl`/`s` actually changed since the previous face — the same
/// transition-only shape `encode_obj` documents, computed here from this module's own per-face
/// lookup tables rather than shared with it.
#[cfg(feature = "oracles")]
fn render(model: &Model) -> String {
    let mut out = String::new();
    if let Some(lib) = &model.mtllib {
        out.push_str(&format!("mtllib {lib}\n"));
    }
    for vertex in &model.vertices {
        match vertex.w {
            Some(w) => out.push_str(&format!("v {} {} {} {}\n", vertex.x, vertex.y, vertex.z, w)),
            None => out.push_str(&format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z)),
        }
    }
    for texcoord in &model.texcoords {
        match texcoord.w {
            Some(w) => out.push_str(&format!("vt {} {} {}\n", texcoord.u, texcoord.v, w)),
            None => out.push_str(&format!("vt {} {}\n", texcoord.u, texcoord.v)),
        }
    }
    for normal in &model.normals {
        out.push_str(&format!("vn {} {} {}\n", normal.x, normal.y, normal.z));
    }

    let face_count = model.faces.len();
    let group_of_face = faces_by_group(&model.groups, face_count);
    let object_of_face = faces_by_object(&model.objects, face_count);
    let material_of_face = faces_by_material_range(&model.usemtl, face_count);
    let smoothing_of_face = faces_by_smoothing_range(&model.smoothing, face_count);

    let mut cur_groups: Vec<String> = Vec::new();
    let mut cur_object: Option<String> = None;
    let mut cur_material: Option<String> = None;
    let mut cur_smoothing: Option<u32> = None;
    let mut have_smoothing = false;

    for (index, face) in model.faces.iter().enumerate() {
        if group_of_face[index] != cur_groups {
            out.push_str(&format!("g {}\n", group_of_face[index].join(" ")));
            cur_groups = group_of_face[index].clone();
        }
        if object_of_face[index] != cur_object {
            match &object_of_face[index] {
                Some(name) => out.push_str(&format!("o {name}\n")),
                None => out.push_str("o\n"),
            }
            cur_object = object_of_face[index].clone();
        }
        if material_of_face[index] != cur_material {
            match &material_of_face[index] {
                Some(name) => out.push_str(&format!("usemtl {name}\n")),
                None => out.push_str("usemtl\n"),
            }
            cur_material = material_of_face[index].clone();
        }
        if !have_smoothing || smoothing_of_face[index] != cur_smoothing {
            match smoothing_of_face[index] {
                Some(group) => out.push_str(&format!("s {group}\n")),
                None => out.push_str("s off\n"),
            }
            cur_smoothing = smoothing_of_face[index];
            have_smoothing = true;
        }
        out.push_str("f ");
        out.push_str(&face.vertices.iter().map(write_face_vertex).collect::<Vec<_>>().join(" "));
        out.push('\n');
    }
    for line in &model.unknown {
        out.push_str(line);
        out.push('\n');
    }
    out
}
//#endregion 🔖️Render

//#region 🔖️JsonHelpers
#[cfg(feature = "oracles")]
fn num(value: &Json, key: &str) -> Result<f64, String> {
    match value.get(key) {
        Some(Json::Number(number)) => Ok(*number),
        _ => Err(format!("expected numeric field {key:?}")),
    }
}

#[cfg(feature = "oracles")]
fn num_opt(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}

#[cfg(feature = "oracles")]
fn str_opt(value: &Json, key: &str) -> Option<String> {
    match value.get(key) {
        Some(Json::String(text)) => Some(text.clone()),
        _ => None,
    }
}

#[cfg(feature = "oracles")]
fn str_field(value: &Json, key: &str) -> Result<String, String> {
    str_opt(value, key).ok_or_else(|| format!("expected string field {key:?}"))
}

#[cfg(feature = "oracles")]
fn usize_field(value: &Json, key: &str) -> Result<usize, String> {
    num(value, key).map(|number| number as usize)
}

#[cfg(feature = "oracles")]
fn usize_array(value: &Json, key: &str) -> Result<Vec<usize>, String> {
    value.array(key).iter().map(|entry| match entry { Json::Number(number) => Ok(*number as usize), other => Err(format!("expected a numeric array for {key:?}, found {other:?}")) }).collect()
}
//#endregion 🔖️JsonHelpers

//#region 🔖️ItemParsing
#[cfg(feature = "oracles")]
fn parse_vertex(value: &Json) -> Result<Vertex, String> {
    Ok(Vertex { x: num(value, "x")?, y: num(value, "y")?, z: num(value, "z")?, w: num_opt(value, "w") })
}

#[cfg(feature = "oracles")]
fn parse_texcoord(value: &Json) -> Result<TexCoord, String> {
    Ok(TexCoord { u: num(value, "u")?, v: num_opt(value, "v").unwrap_or(0.0), w: num_opt(value, "w") })
}

#[cfg(feature = "oracles")]
fn parse_normal(value: &Json) -> Result<Normal, String> {
    Ok(Normal { x: num(value, "x")?, y: num(value, "y")?, z: num(value, "z")? })
}

#[cfg(feature = "oracles")]
fn parse_face(value: &Json) -> Result<Face, String> {
    let mut vertices = Vec::new();
    for entry in value.array("vertices") {
        vertices.push(FaceVertex { vertex: usize_field(&entry, "vertex")? as u32, texcoord: num_opt(&entry, "texcoord").map(|number| number as u32), normal: num_opt(&entry, "normal").map(|number| number as u32) });
    }
    if vertices.len() < 3 {
        return Err("face needs at least 3 vertices".to_string());
    }
    Ok(Face { vertices })
}

/// 📦️ The whole-document payload `set-snapshot` carries — parsed independently of this subset's
/// own `ObjSnapshot` deserialization.
#[cfg(feature = "oracles")]
fn model_from_json(value: &Json) -> Result<Model, String> {
    let mut model = Model::default();
    for entry in value.array("vertices") {
        model.vertices.push(parse_vertex(&entry)?);
    }
    for entry in value.array("texcoords") {
        model.texcoords.push(parse_texcoord(&entry)?);
    }
    for entry in value.array("normals") {
        model.normals.push(parse_normal(&entry)?);
    }
    for entry in value.array("faces") {
        model.faces.push(parse_face(&entry)?);
    }
    for entry in value.array("groups") {
        model.groups.push((str_field(&entry, "name")?, usize_array(&entry, "faces")?));
    }
    for entry in value.array("objects") {
        model.objects.push((str_field(&entry, "name")?, usize_array(&entry, "faces")?));
    }
    model.mtllib = str_opt(value, "mtllib");
    for entry in value.array("usemtlRanges") {
        model.usemtl.push((usize_field(&entry, "faceIndexFrom")?, str_field(&entry, "material")?));
    }
    for entry in value.array("smoothingGroups") {
        model.smoothing.push((usize_field(&entry, "faceIndexFrom")?, num_opt(&entry, "group").map(|number| number as u32)));
    }
    for entry in value.array("unknownStatements") {
        model.unknown.push(str_field(&entry, "raw")?);
    }
    Ok(model)
}
//#endregion 🔖️ItemParsing

//#region 🔖️Apply
/// 🦠️ Applies one declared kind to the independently-parsed model — one arm per `ObjMutation`
/// variant this subset's catalog declares, matched by its kebab-case `KINDS` spelling.
#[cfg(feature = "oracles")]
fn apply(model: &mut Model, kind: &str, params: &Json) -> Result<(), String> {
    match kind {
        "no-mutation" => Ok(()),
        "set-snapshot" => {
            *model = model_from_json(params.get("snapshot").ok_or("set-snapshot requires a snapshot field")?)?;
            Ok(())
        }
        "insert-vertex" => {
            let index = usize_field(params, "index")?.min(model.vertices.len());
            model.vertices.insert(index, parse_vertex(params.get("vertex").ok_or("insert-vertex requires a vertex field")?)?);
            Ok(())
        }
        "remove-vertex" => {
            let index = usize_field(params, "index")?;
            if index >= model.vertices.len() {
                return Err(format!("remove-vertex index {index} out of range"));
            }
            model.vertices.remove(index);
            Ok(())
        }
        "set-vertex" => {
            let index = usize_field(params, "index")?;
            let vertex = parse_vertex(params.get("vertex").ok_or("set-vertex requires a vertex field")?)?;
            *model.vertices.get_mut(index).ok_or_else(|| format!("set-vertex index {index} out of range"))? = vertex;
            Ok(())
        }
        "insert-texcoord" => {
            let index = usize_field(params, "index")?.min(model.texcoords.len());
            model.texcoords.insert(index, parse_texcoord(params.get("texcoord").ok_or("insert-texcoord requires a texcoord field")?)?);
            Ok(())
        }
        "remove-texcoord" => {
            let index = usize_field(params, "index")?;
            if index >= model.texcoords.len() {
                return Err(format!("remove-texcoord index {index} out of range"));
            }
            model.texcoords.remove(index);
            Ok(())
        }
        "set-texcoord" => {
            let index = usize_field(params, "index")?;
            let texcoord = parse_texcoord(params.get("texcoord").ok_or("set-texcoord requires a texcoord field")?)?;
            *model.texcoords.get_mut(index).ok_or_else(|| format!("set-texcoord index {index} out of range"))? = texcoord;
            Ok(())
        }
        "insert-normal" => {
            let index = usize_field(params, "index")?.min(model.normals.len());
            model.normals.insert(index, parse_normal(params.get("normal").ok_or("insert-normal requires a normal field")?)?);
            Ok(())
        }
        "remove-normal" => {
            let index = usize_field(params, "index")?;
            if index >= model.normals.len() {
                return Err(format!("remove-normal index {index} out of range"));
            }
            model.normals.remove(index);
            Ok(())
        }
        "set-normal" => {
            let index = usize_field(params, "index")?;
            let normal = parse_normal(params.get("normal").ok_or("set-normal requires a normal field")?)?;
            *model.normals.get_mut(index).ok_or_else(|| format!("set-normal index {index} out of range"))? = normal;
            Ok(())
        }
        "insert-face" => {
            let index = usize_field(params, "index")?.min(model.faces.len());
            model.faces.insert(index, parse_face(params.get("face").ok_or("insert-face requires a face field")?)?);
            Ok(())
        }
        "remove-face" => {
            let index = usize_field(params, "index")?;
            if index >= model.faces.len() {
                return Err(format!("remove-face index {index} out of range"));
            }
            model.faces.remove(index);
            Ok(())
        }
        "set-face" => {
            let index = usize_field(params, "index")?;
            let face = parse_face(params.get("face").ok_or("set-face requires a face field")?)?;
            *model.faces.get_mut(index).ok_or_else(|| format!("set-face index {index} out of range"))? = face;
            Ok(())
        }
        "set-group" => {
            let name = str_field(params, "name")?;
            let faces = usize_array(params, "faces")?;
            match model.groups.iter_mut().find(|(existing, _)| existing == &name) {
                Some(entry) => entry.1 = faces,
                None => model.groups.push((name, faces)),
            }
            Ok(())
        }
        "remove-group" => {
            let name = str_field(params, "name")?;
            model.groups.retain(|(existing, _)| existing != &name);
            Ok(())
        }
        "set-object" => {
            let name = str_field(params, "name")?;
            let faces = usize_array(params, "faces")?;
            match model.objects.iter_mut().find(|(existing, _)| existing == &name) {
                Some(entry) => entry.1 = faces,
                None => model.objects.push((name, faces)),
            }
            Ok(())
        }
        "remove-object" => {
            let name = str_field(params, "name")?;
            model.objects.retain(|(existing, _)| existing != &name);
            Ok(())
        }
        "set-mtllib" => {
            model.mtllib = str_opt(params, "mtllib");
            Ok(())
        }
        "set-usemtl" => {
            let mut ranges = Vec::new();
            for entry in params.array("usemtl") {
                ranges.push((usize_field(&entry, "faceIndexFrom")?, str_field(&entry, "material")?));
            }
            model.usemtl = ranges;
            Ok(())
        }
        "set-smoothing-groups" => {
            let mut ranges = Vec::new();
            for entry in params.array("smoothingGroups") {
                ranges.push((usize_field(&entry, "faceIndexFrom")?, num_opt(&entry, "group").map(|number| number as u32)));
            }
            model.smoothing = ranges;
            Ok(())
        }
        "set-unknown-statements" => {
            let mut lines = Vec::new();
            for entry in params.array("unknownStatements") {
                lines.push(str_field(&entry, "raw")?);
            }
            model.unknown = lines;
            Ok(())
        }
        other => Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
    let mut model = parse(text)?;
    let empty_params = Json::Object(Vec::new());
    let params = spec.get("params").unwrap_or(&empty_params);
    apply(&mut model, &kind, params)?;
    Ok(render(&model).into_bytes())
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
