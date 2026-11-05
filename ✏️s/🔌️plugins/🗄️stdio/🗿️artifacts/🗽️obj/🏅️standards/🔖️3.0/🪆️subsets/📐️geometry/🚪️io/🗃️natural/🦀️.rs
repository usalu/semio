//! 🗽️ Natural OBJ snapshot codecs shared by editor admission and composition.

//#region 🔖️Codec
//#region 🔖️IndexResolution
use crate::schema::snapshot::{ObjFace, ObjFaceVertex, ObjGroup, ObjNormal, ObjObject, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
use crate::schema::snapshot::ObjSnapshot;
use crate::STDIO_OBJ_DOCUMENT_SCHEMA;
use std::collections::HashMap;

/// 🔢 Resolves a raw OBJ index (1-based positive, or negative = relative to the
/// current end of the list at parse time) against `current_len` — the OBJ spec's
/// own negative-index rule.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_index(current_len: usize, raw: i64) -> Result<u32, String> {
    if raw > 0 {
        Ok((raw - 1) as u32)
    } else if raw < 0 {
        let idx = current_len as i64 + raw;
        if idx < 0 {
            return Err(format!("negative index {raw} out of range (list has {current_len} entries)"));
        }
        Ok(idx as u32)
    } else {
        Err("obj index 0 is not valid (indices are 1-based)".into())
    }
}

/// 🧩 Parses one `f` face-vertex token (`v`, `v/vt`, `v//vn`, `v/vt/vn`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_face_vertex(token: &str, vertex_count: usize, texcoord_count: usize, normal_count: usize) -> Result<ObjFaceVertex, String> {
    let mut parts = token.split('/');
    let v_raw: i64 = parts.next().ok_or("empty face token")?.parse().map_err(|e| format!("face vertex index: {e}"))?;
    let vertex = resolve_index(vertex_count, v_raw)?;
    let vt_raw = parts.next().unwrap_or("");
    let texcoord = if vt_raw.is_empty() { None } else { Some(resolve_index(texcoord_count, vt_raw.parse().map_err(|e| format!("face texcoord index: {e}"))?)?) };
    let vn_raw = parts.next().unwrap_or("");
    let normal = if vn_raw.is_empty() { None } else { Some(resolve_index(normal_count, vn_raw.parse().map_err(|e| format!("face normal index: {e}"))?)?) };
    Ok(ObjFaceVertex { vertex, texcoord, normal })
}
//#endregion 🔖️IndexResolution

//#region 🔖️Decode
/// 📥 Parses a real Wavefront OBJ text body: `v`/`vt`/`vn` (incl. optional `w`), `f` (v, v/vt,
/// v//vn, v/vt/vn, negative-relative indices, n-gons), `o`/`g` (multi-name)/`usemtl`/`mtllib`/`s`,
/// with every comment and unrecognized statement retained in `unknown_statements`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_obj(text: &str) -> Result<ObjSnapshot, String> {
    let mut vertices = Vec::new();
    let mut texcoords = Vec::new();
    let mut normals = Vec::new();
    let mut faces: Vec<ObjFace> = Vec::new();
    let mut groups: Vec<ObjGroup> = Vec::new();
    let mut group_index: HashMap<String, usize> = HashMap::new();
    let mut objects: Vec<ObjObject> = Vec::new();
    let mut object_index: HashMap<String, usize> = HashMap::new();
    let mut mtllib: Option<String> = None;
    let mut usemtl: Vec<ObjUsemtlRange> = Vec::new();
    let mut smoothing_groups: Vec<ObjSmoothingRange> = Vec::new();
    let mut unknown_statements: Vec<ObjUnknownStatement> = Vec::new();

    let mut cur_active_groups: Vec<String> = Vec::new();
    let mut cur_active_object: Option<String> = None;
    let mut cur_material: Option<String> = None;
    let mut cur_smoothing: Option<u32> = None;
    let mut have_smoothing = false;

    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            unknown_statements.push(ObjUnknownStatement { line_index: line_index as u64, raw: line.to_string() });
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f64 = parts.next().ok_or("v x")?.parse().map_err(|e| format!("{e}"))?;
                let y: f64 = parts.next().ok_or("v y")?.parse().map_err(|e| format!("{e}"))?;
                let z: f64 = parts.next().ok_or("v z")?.parse().map_err(|e| format!("{e}"))?;
                let w = parts.next().map(str::parse::<f64>).transpose().map_err(|e| format!("{e}"))?;
                vertices.push(ObjVertex { x, y, z, w });
            }
            Some("vt") => {
                let u: f64 = parts.next().ok_or("vt u")?.parse().map_err(|e| format!("{e}"))?;
                let v: f64 = parts.next().unwrap_or("0").parse().map_err(|e| format!("{e}"))?;
                let w = parts.next().map(str::parse::<f64>).transpose().map_err(|e| format!("{e}"))?;
                texcoords.push(ObjTexCoord { u, v, w });
            }
            Some("vn") => {
                let x: f64 = parts.next().ok_or("vn x")?.parse().map_err(|e| format!("{e}"))?;
                let y: f64 = parts.next().ok_or("vn y")?.parse().map_err(|e| format!("{e}"))?;
                let z: f64 = parts.next().ok_or("vn z")?.parse().map_err(|e| format!("{e}"))?;
                normals.push(ObjNormal { x, y, z });
            }
            Some("f") => {
                let mut face_vertices = Vec::new();
                for token in parts {
                    face_vertices.push(parse_face_vertex(token, vertices.len(), texcoords.len(), normals.len())?);
                }
                if face_vertices.len() < 3 {
                    return Err(format!("face has fewer than 3 vertices: {line}"));
                }
                let face_index = faces.len() as u64;
                faces.push(ObjFace { vertices: face_vertices });
                for name in &cur_active_groups {
                    let gi = if let Some(&gi) = group_index.get(name) {
                        gi
                    } else {
                        groups.push(ObjGroup { name: name.clone(), faces: Vec::new() });
                        let gi = groups.len() - 1;
                        group_index.insert(name.clone(), gi);
                        gi
                    };
                    groups[gi].faces.push(face_index);
                }
                if let Some(name) = &cur_active_object {
                    let oi = if let Some(&oi) = object_index.get(name) {
                        oi
                    } else {
                        objects.push(ObjObject { name: name.clone(), faces: Vec::new() });
                        let oi = objects.len() - 1;
                        object_index.insert(name.clone(), oi);
                        oi
                    };
                    objects[oi].faces.push(face_index);
                }
            }
            Some("o") => {
                cur_active_object = parts.next().map(|s| s.to_string());
            }
            Some("g") => {
                cur_active_groups = parts.map(|s| s.to_string()).collect();
            }
            Some("usemtl") => {
                let name = parts.next().map(|s| s.to_string());
                if name != cur_material {
                    if let Some(m) = &name {
                        usemtl.push(ObjUsemtlRange { face_index_from: faces.len() as u64, material: m.clone() });
                    }
                    cur_material = name;
                }
            }
            Some("mtllib") => {
                let rest: Vec<&str> = parts.collect();
                if !rest.is_empty() {
                    mtllib = Some(rest.join(" "));
                }
            }
            Some("s") => {
                let next = match parts.next() {
                    Some("off") | None => None,
                    Some(v) => v.parse::<u32>().ok(),
                };
                if !have_smoothing || next != cur_smoothing {
                    smoothing_groups.push(ObjSmoothingRange { face_index_from: faces.len() as u64, group: next });
                    cur_smoothing = next;
                    have_smoothing = true;
                }
            }
            _ => {
                unknown_statements.push(ObjUnknownStatement { line_index: line_index as u64, raw: line.to_string() });
            }
        }
    }

    Ok(ObjSnapshot { schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(), vertices, texcoords, normals, faces, groups, objects, mtllib, usemtl, smoothing_groups, unknown_statements })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_face_vertex(out: &mut String, fv: &ObjFaceVertex) {
    match (fv.texcoord, fv.normal) {
        (Some(vt), Some(vn)) => out.push_str(&format!("{}/{}/{}", fv.vertex + 1, vt + 1, vn + 1)),
        (Some(vt), None) => out.push_str(&format!("{}/{}", fv.vertex + 1, vt + 1)),
        (None, Some(vn)) => out.push_str(&format!("{}//{}", fv.vertex + 1, vn + 1)),
        (None, None) => out.push_str(&format!("{}", fv.vertex + 1)),
    }
}

/// 📤 Writes a real Wavefront OBJ 3.0 text body per this module's documented normal form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_obj(snap: &ObjSnapshot) -> String {
    let mut out = String::new();
    if let Some(lib) = &snap.mtllib {
        out.push_str(&format!("mtllib {lib}\n"));
    }
    for v in &snap.vertices {
        match v.w {
            Some(w) => out.push_str(&format!("v {} {} {} {}\n", v.x, v.y, v.z, w)),
            None => out.push_str(&format!("v {} {} {}\n", v.x, v.y, v.z)),
        }
    }
    for vt in &snap.texcoords {
        match vt.w {
            Some(w) => out.push_str(&format!("vt {} {} {}\n", vt.u, vt.v, w)),
            None => out.push_str(&format!("vt {} {}\n", vt.u, vt.v)),
        }
    }
    for vn in &snap.normals {
        out.push_str(&format!("vn {} {} {}\n", vn.x, vn.y, vn.z));
    }

    let object_at = |i: usize| -> Option<&str> { snap.objects.iter().find(|o| o.faces.contains(&(i as u64))).map(|o| o.name.as_str()) };
    let groups_at = |i: usize| -> Vec<&str> { snap.groups.iter().filter(|g| g.faces.contains(&(i as u64))).map(|g| g.name.as_str()).collect() };
    let material_at = |i: usize| -> Option<&str> { snap.usemtl.iter().rev().find(|r| r.face_index_from <= i as u64).map(|r| r.material.as_str()) };
    let smoothing_at = |i: usize| -> Option<Option<u32>> { snap.smoothing_groups.iter().rev().find(|r| r.face_index_from <= i as u64).map(|r| r.group) };

    let mut prev_object: Option<&str> = None;
    let mut prev_groups: Vec<&str> = Vec::new();
    let mut prev_material: Option<&str> = None;
    let mut prev_smoothing_emitted = false;
    let mut prev_smoothing: Option<u32> = None;
    let mut started = false;

    for (i, face) in snap.faces.iter().enumerate() {
        let object = object_at(i);
        if !started || object != prev_object {
            match object {
                Some(name) => out.push_str(&format!("o {name}\n")),
                None => {
                    if prev_object.is_some() {
                        out.push_str("o\n");
                    }
                }
            }
            prev_object = object;
        }
        let groups = groups_at(i);
        if !started || groups != prev_groups {
            if groups.is_empty() {
                if !prev_groups.is_empty() {
                    out.push_str("g\n");
                }
            } else {
                out.push_str(&format!("g {}\n", groups.join(" ")));
            }
            prev_groups = groups;
        }
        let material = material_at(i);
        if !started || material != prev_material {
            if let Some(m) = material {
                out.push_str(&format!("usemtl {m}\n"));
            }
            prev_material = material;
        }
        if let Some(s) = smoothing_at(i) {
            if !prev_smoothing_emitted || s != prev_smoothing {
                match s {
                    Some(n) => out.push_str(&format!("s {n}\n")),
                    None => out.push_str("s off\n"),
                }
                prev_smoothing = s;
                prev_smoothing_emitted = true;
            }
        }
        started = true;

        out.push('f');
        for fv in &face.vertices {
            out.push(' ');
            write_face_vertex(&mut out, fv);
        }
        out.push('\n');
    }

    for u in &snap.unknown_statements {
        out.push_str(&u.raw);
        out.push('\n');
    }
    out
}
//#endregion 🔖️Encode
//#endregion 🔖️Codec
