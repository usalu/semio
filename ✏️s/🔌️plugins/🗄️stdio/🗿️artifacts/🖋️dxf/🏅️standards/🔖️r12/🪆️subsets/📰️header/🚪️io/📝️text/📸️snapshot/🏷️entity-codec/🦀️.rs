//! 🏷️ Text codec of the dxf snapshot entities (strings, numbers, lists, group codes, vertices, entities, header variables, tables, blocks), shared by the snapshot and the diff codecs.

#![allow(unused_imports)]
use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfArc, DxfCircle, DxfInsert, DxfLine, DxfOther, DxfPolyline, DxfSolid, DxfText, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue, DxfVertex};
use std::collections::{BTreeMap, BTreeSet, HashSet};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_f64(v: f64) -> String {
    format!("{v}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}

/// 🧺 Self-bracketing plain list — the shared core every `Vec<T>` field's grammar uses (group
/// codes, vertices, entities, header vars, …).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_point3(p: &[f64; 3]) -> String {
    format!("[{},{},{}]", enc_f64(p[0]), enc_f64(p[1]), enc_f64(p[2]))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_point3(s: &str) -> Result<[f64; 3], String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok([dec_f64(x)?, dec_f64(y)?, dec_f64(z)?])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_points4(p: &[[f64; 3]; 4]) -> String {
    enc_list(p.as_slice(), enc_point3)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_points4(s: &str) -> Result<[[f64; 3]; 4], String> {
    let v = dec_list(s, dec_point3)?;
    let arr: [[f64; 3]; 4] = v.clone().try_into().map_err(|_| format!("points4: expected 4 points, got {}", v.len()))?;
    Ok(arr)
}

/// 🧮 `DxfValue` (data-carrying enum — this file's own root cause for the `DslDiff`/`DslOps`
/// rejection): `S[hex]`/`I[digits]`/`D[float]`/`P[x,y,z]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_dxf_value(v: &DxfValue) -> String {
    match v {
        DxfValue::Str { value } => format!("S[{}]", enc_str(value)),
        DxfValue::Int { value } => format!("I[{value}]"),
        DxfValue::Double { value } => format!("D[{}]", enc_f64(*value)),
        DxfValue::Point { value } => format!("P[{},{},{}]", enc_f64(value[0]), enc_f64(value[1]), enc_f64(value[2])),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_dxf_value(s: &str) -> Result<DxfValue, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "S" => Ok(DxfValue::Str { value: dec_str(inner)? }),
        "I" => Ok(DxfValue::Int { value: inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        "D" => Ok(DxfValue::Double { value: dec_f64(inner)? }),
        "P" => {
            let parts = split_top_level(inner, ',');
            let [x, y, z] = parts.as_slice() else { return Err(format!("dxf value point: expected 3 fields, got {}", parts.len())) };
            Ok(DxfValue::Point { value: [dec_f64(x)?, dec_f64(y)?, dec_f64(z)?] })
        }
        other => Err(format!("dxf value: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_group_code(pair: &(i32, DxfValue)) -> String {
    format!("[{},{}]", pair.0, enc_dxf_value(&pair.1))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_group_code(s: &str) -> Result<(i32, DxfValue), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [code, value] = parts.as_slice() else { return Err(format!("group code: expected 2 fields, got {}", parts.len())) };
    Ok((code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, dec_dxf_value(value)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_group_codes(v: &[(i32, DxfValue)]) -> String {
    enc_list(v, enc_group_code)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_group_codes(s: &str) -> Result<Vec<(i32, DxfValue)>, String> {
    dec_list(s, dec_group_code)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_vertex(v: &DxfVertex) -> String {
    format!("[{},{},{},{},{}]", enc_f64(v.x), enc_f64(v.y), enc_f64(v.z), enc_f64(v.bulge), enc_group_codes(&v.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_vertex(s: &str) -> Result<DxfVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z, bulge, unknown] = parts.as_slice() else { return Err(format!("vertex: expected 5 fields, got {}", parts.len())) };
    Ok(DxfVertex { x: dec_f64(x)?, y: dec_f64(y)?, z: dec_f64(z)?, bulge: dec_f64(bulge)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_vertices(vs: &[DxfVertex]) -> String {
    enc_list(vs, enc_vertex)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_vertices(s: &str) -> Result<Vec<DxfVertex>, String> {
    dec_list(s, dec_vertex)
}

/// 📐️ `DxfEntity` (data-carrying enum, the whole entity — not its diff): tag prefix per kind,
/// `L`=Line, `C`=Circle, `A`=Arc, `W`=Polyline, `T`=Text, `S`=Solid, `I`=Insert, `O`=Other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_dxf_entity(e: &DxfEntity) -> String {
    match e {
        DxfEntity::Line(DxfLine { start, end, layer, unknown_group_codes }) => {
            format!("L[{},{},{},{}]", enc_point3(start), enc_point3(end), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Circle(DxfCircle { center, radius, layer, unknown_group_codes }) => {
            format!("C[{},{},{},{}]", enc_point3(center), enc_f64(*radius), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Arc(DxfArc { center, radius, start_angle, end_angle, layer, unknown_group_codes }) => {
            format!("A[{},{},{},{},{},{}]", enc_point3(center), enc_f64(*radius), enc_f64(*start_angle), enc_f64(*end_angle), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Polyline(DxfPolyline { vertices, closed, layer, unknown_group_codes }) => format!("W[{},{},{},{}]", enc_vertices(vertices), if *closed { "1" } else { "0" }, enc_str(layer), enc_group_codes(unknown_group_codes)),
        DxfEntity::Text(DxfText { position, height, value, layer, unknown_group_codes }) => format!("T[{},{},{},{},{}]", enc_point3(position), enc_f64(*height), enc_str(value), enc_str(layer), enc_group_codes(unknown_group_codes)),
        DxfEntity::Solid(DxfSolid { points, layer, unknown_group_codes }) => {
            format!("S[{},{},{}]", enc_points4(points), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Insert(DxfInsert { block_name, position, scale, rotation, layer, unknown_group_codes }) => {
            format!("I[{},{},{},{},{},{}]", enc_str(block_name), enc_point3(position), enc_point3(scale), enc_f64(*rotation), enc_str(layer), enc_group_codes(unknown_group_codes))
        }
        DxfEntity::Other(DxfOther { kind, group_codes }) => format!("O[{},{}]", enc_str(kind), enc_group_codes(group_codes)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_dxf_entity(s: &str) -> Result<DxfEntity, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "L" => {
            let parts = split_top_level(inner, ',');
            let [start, end, layer, unknown] = parts.as_slice() else { return Err(format!("entity line: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Line(DxfLine { start: dec_point3(start)?, end: dec_point3(end)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "C" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, layer, unknown] = parts.as_slice() else { return Err(format!("entity circle: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Circle(DxfCircle { center: dec_point3(center)?, radius: dec_f64(radius)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "A" => {
            let parts = split_top_level(inner, ',');
            let [center, radius, start_angle, end_angle, layer, unknown] = parts.as_slice() else { return Err(format!("entity arc: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntity::Arc(DxfArc { center: dec_point3(center)?, radius: dec_f64(radius)?, start_angle: dec_f64(start_angle)?, end_angle: dec_f64(end_angle)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "W" => {
            let parts = split_top_level(inner, ',');
            let [vertices, closed, layer, unknown] = parts.as_slice() else { return Err(format!("entity polyline: expected 4 fields, got {}", parts.len())) };
            Ok(DxfEntity::Polyline(DxfPolyline { vertices: dec_vertices(vertices)?, closed: *closed == "1", layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [position, height, value, layer, unknown] = parts.as_slice() else { return Err(format!("entity text: expected 5 fields, got {}", parts.len())) };
            Ok(DxfEntity::Text(DxfText { position: dec_point3(position)?, height: dec_f64(height)?, value: dec_str(value)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [points, layer, unknown] = parts.as_slice() else { return Err(format!("entity solid: expected 3 fields, got {}", parts.len())) };
            Ok(DxfEntity::Solid(DxfSolid { points: dec_points4(points)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "I" => {
            let parts = split_top_level(inner, ',');
            let [block_name, position, scale, rotation, layer, unknown] = parts.as_slice() else { return Err(format!("entity insert: expected 6 fields, got {}", parts.len())) };
            Ok(DxfEntity::Insert(DxfInsert { block_name: dec_str(block_name)?, position: dec_point3(position)?, scale: dec_point3(scale)?, rotation: dec_f64(rotation)?, layer: dec_str(layer)?, unknown_group_codes: dec_group_codes(unknown)? }))
        }
        "O" => {
            let parts = split_top_level(inner, ',');
            let [kind, group_codes] = parts.as_slice() else { return Err(format!("entity other: expected 2 fields, got {}", parts.len())) };
            Ok(DxfEntity::Other(DxfOther { kind: dec_str(kind)?, group_codes: dec_group_codes(group_codes)? }))
        }
        other => Err(format!("dxf entity: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_dxf_entities(es: &[DxfEntity]) -> String {
    enc_list(es, enc_dxf_entity)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_dxf_entities(s: &str) -> Result<Vec<DxfEntity>, String> {
    dec_list(s, dec_dxf_entity)
}

/// 🏷️ Full (non-diff) item encoders — self-bracketing positional tuples, used by `added` entries
/// in every collection triple AND by `🧬️mutations`'s `Insert*`/`Set*` argument
/// payloads (hence `pub(crate)`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_header_var(hv: &DxfHeaderVar) -> String {
    format!("[{},{},{},{}]", enc_str(&hv.name), hv.group_code, enc_dxf_value(&hv.value), enc_group_codes(&hv.extra_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_header_var(s: &str) -> Result<DxfHeaderVar, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, group_code, value, extra] = parts.as_slice() else { return Err(format!("header var: expected 4 fields, got {}", parts.len())) };
    Ok(DxfHeaderVar { name: dec_str(name)?, group_code: group_code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, value: dec_dxf_value(value)?, extra_group_codes: dec_group_codes(extra)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_layer(l: &DxfLayer) -> String {
    format!("[{},{},{},{},{}]", enc_str(&l.name), l.color, enc_str(&l.linetype), l.flags, enc_group_codes(&l.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_layer(s: &str) -> Result<DxfLayer, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, color, linetype, flags, unknown] = parts.as_slice() else { return Err(format!("layer: expected 5 fields, got {}", parts.len())) };
    Ok(DxfLayer {
        name: dec_str(name)?,
        color: color.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        linetype: dec_str(linetype)?,
        flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        unknown_group_codes: dec_group_codes(unknown)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_style(s: &DxfStyle) -> String {
    format!("[{},{},{},{}]", enc_str(&s.name), s.flags, enc_str(&s.font_name), enc_group_codes(&s.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_style(s: &str) -> Result<DxfStyle, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, flags, font_name, unknown] = parts.as_slice() else { return Err(format!("style: expected 4 fields, got {}", parts.len())) };
    Ok(DxfStyle { name: dec_str(name)?, flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, font_name: dec_str(font_name)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_linetype(l: &DxfLinetype) -> String {
    format!("[{},{},{},{}]", enc_str(&l.name), l.flags, enc_str(&l.description), enc_group_codes(&l.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_linetype(s: &str) -> Result<DxfLinetype, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, flags, description, unknown] = parts.as_slice() else { return Err(format!("linetype: expected 4 fields, got {}", parts.len())) };
    Ok(DxfLinetype { name: dec_str(name)?, flags: flags.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, description: dec_str(description)?, unknown_group_codes: dec_group_codes(unknown)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_block(b: &DxfBlock) -> String {
    format!("[{},{},{},{}]", enc_str(&b.name), enc_point3(&b.base_point), enc_dxf_entities(&b.entities), enc_group_codes(&b.unknown_group_codes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_block(s: &str) -> Result<DxfBlock, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, base_point, entities, unknown] = parts.as_slice() else { return Err(format!("block: expected 4 fields, got {}", parts.len())) };
    Ok(DxfBlock { name: dec_str(name)?, base_point: dec_point3(base_point)?, entities: dec_dxf_entities(entities)?, unknown_group_codes: dec_group_codes(unknown)? })
}


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_dxf_tag(t: &DxfTag) -> String {
    format!("[{},{}]", t.code, enc_str(&t.value))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_dxf_tag(s: &str) -> Result<DxfTag, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [code, value] = parts.as_slice() else { return Err(format!("tag: expected 2 fields, got {}", parts.len())) };
    Ok(DxfTag { code: code.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, value: dec_str(value)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_other_table(t: &DxfOtherTable) -> String {
    format!("[{},{}]", enc_str(&t.name), enc_list(&t.tags, enc_dxf_tag))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_other_table(s: &str) -> Result<DxfOtherTable, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, tags] = parts.as_slice() else { return Err(format!("other table: expected 2 fields, got {}", parts.len())) };
    Ok(DxfOtherTable { name: dec_str(name)?, tags: dec_list(tags, dec_dxf_tag)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_dxf_tables(t: &DxfTables) -> String {
    format!("[{},{},{}]", enc_list(&t.layers, enc_layer), enc_list(&t.styles, enc_style), enc_list(&t.linetypes, enc_linetype))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dec_dxf_tables(s: &str) -> Result<DxfTables, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [layers, styles, linetypes] = parts.as_slice() else { return Err(format!("tables: expected 3 fields, got {}", parts.len())) };
    Ok(DxfTables { layers: dec_list(layers, dec_layer)?, styles: dec_list(styles, dec_style)?, linetypes: dec_list(linetypes, dec_linetype)? })
}
