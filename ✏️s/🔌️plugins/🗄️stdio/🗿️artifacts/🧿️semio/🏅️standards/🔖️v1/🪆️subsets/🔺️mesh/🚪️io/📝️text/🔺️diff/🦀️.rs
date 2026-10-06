//! 📝️ Text representation codec surface for `s.stdio.semio.mesh` (diff) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::mesh::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_mesh_diff(d: &SemioMeshDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.meshes {
        tokens.push(format!("meshes={}", enc_meshes_diff(v)));
    }
    if let Some(v) = &d.materials {
        tokens.push(format!("materials={}", enc_materials_diff(v)));
    }
    if let Some(v) = &d.textures {
        tokens.push(format!("textures={}", enc_textures_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_mesh_diff(line: &str) -> Result<SemioMeshDiff, String> {
    let mut d = SemioMeshDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("meshes=") {
            d.meshes = Some(dec_meshes_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("materials=") {
            d.materials = Some(dec_materials_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("textures=") {
            d.textures = Some(dec_textures_diff(rest)?);
        } else {
            return Err(format!("semio mesh diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioMeshDiff {
fn print_diff(&self) -> String {
    print_mesh_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_mesh_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f32(s: &str) -> Result<f32, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_uv(v: &SemioUv) -> String {
    format!("[{},{}]", v.u, v.v)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_uv(s: &str) -> Result<SemioUv, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [u, v] = parts.as_slice() else { return Err(format!("uv: expected 2 fields, got {}", parts.len())) };
    Ok(SemioUv { u: parse_f64(u)?, v: parse_f64(v)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_rgba(c: &SemioRgba) -> String {
    format!("[{},{},{},{}]", c.r, c.g, c.b, c.a)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_rgba(s: &str) -> Result<SemioRgba, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [r, g, b, a] = parts.as_slice() else { return Err(format!("rgba: expected 4 fields, got {}", parts.len())) };
    Ok(SemioRgba { r: parse_f32(r)?, g: parse_f32(g)?, b: parse_f32(b)?, a: parse_f32(a)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topology(t: &SemioTopology) -> String {
    match t {
        SemioTopology::Points => "P".to_string(),
        SemioTopology::Lines => "L".to_string(),
        SemioTopology::LineStrip => "S".to_string(),
        SemioTopology::Triangles => "T".to_string(),
        SemioTopology::TriangleStrip => "X".to_string(),
        SemioTopology::TriangleFan => "F".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topology(s: &str) -> Result<SemioTopology, String> {
    match s {
        "P" => Ok(SemioTopology::Points),
        "L" => Ok(SemioTopology::Lines),
        "S" => Ok(SemioTopology::LineStrip),
        "T" => Ok(SemioTopology::Triangles),
        "X" => Ok(SemioTopology::TriangleStrip),
        "F" => Ok(SemioTopology::TriangleFan),
        other => Err(format!("topology: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive(p: &SemioPrimitive) -> String {
    format!(
        "[{},{},{},{},{},{},{},{}]",
        enc_str(&p.id),
        enc_topology(&p.topology),
        enc_list(&p.positions, enc_point3),
        enc_list(&p.normals, enc_point3),
        enc_list(&p.uvs, enc_uv),
        enc_list(&p.colors, enc_rgba),
        enc_list(&p.indices, |v: &u32| v.to_string()),
        encode_option(&p.material_id, |v: &String| enc_str(v)),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive(s: &str) -> Result<SemioPrimitive, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, topology, positions, normals, uvs, colors, indices, material_id] = parts.as_slice() else {
        return Err(format!("primitive: expected 8 fields, got {}", parts.len()));
    };
    Ok(SemioPrimitive {
        id: dec_str(id)?,
        topology: dec_topology(topology)?,
        positions: dec_list(positions, dec_point3)?,
        normals: dec_list(normals, dec_point3)?,
        uvs: dec_list(uvs, dec_uv)?,
        colors: dec_list(colors, dec_rgba)?,
        indices: dec_list(indices, parse_u32)?,
        material_id: decode_option(material_id, dec_str)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh(m: &SemioMesh) -> String {
    format!("[{},{}]", enc_str(&m.id), enc_list(&m.primitives, enc_primitive))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh(s: &str) -> Result<SemioMesh, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, primitives] = parts.as_slice() else { return Err(format!("mesh: expected 2 fields, got {}", parts.len())) };
    Ok(SemioMesh { id: dec_str(id)?, primitives: dec_list(primitives, dec_primitive)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material(m: &SemioMaterial) -> String {
    format!("[{},{},{},{},{},{},{},{},{}]", enc_str(&m.id), enc_rgba(&m.base_color), m.metallic, m.roughness, encode_option(&m.base_color_texture, |v: &String| enc_str(v)), encode_option(&m.metallic_roughness_texture, |v: &String| enc_str(v)), encode_option(&m.normal_texture, |v: &String| enc_str(v)), encode_option(&m.occlusion_texture, |v: &String| enc_str(v)), encode_option(&m.emissive_texture, |v: &String| enc_str(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material(s: &str) -> Result<SemioMaterial, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, base_color, metallic, roughness, base_color_texture, metallic_roughness_texture, normal_texture, occlusion_texture, emissive_texture] = parts.as_slice() else { return Err(format!("material: expected 9 fields, got {}", parts.len())) };
    Ok(SemioMaterial { id: dec_str(id)?, base_color: dec_rgba(base_color)?, metallic: parse_f32(metallic)?, roughness: parse_f32(roughness)?, base_color_texture: decode_option(base_color_texture, dec_str)?, metallic_roughness_texture: decode_option(metallic_roughness_texture, dec_str)?, normal_texture: decode_option(normal_texture, dec_str)?, occlusion_texture: decode_option(occlusion_texture, dec_str)?, emissive_texture: decode_option(emissive_texture, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture(t: &SemioTexture) -> String {
    format!("[{},{},{}]", enc_str(&t.id), enc_str(&t.mime), enc_bytes(&t.bytes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture(s: &str) -> Result<SemioTexture, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, mime, bytes] = parts.as_slice() else { return Err(format!("texture: expected 3 fields, got {}", parts.len())) };
    Ok(SemioTexture { id: dec_str(id)?, mime: dec_str(mime)?, bytes: dec_bytes(bytes)? })
}

/// 🧷️ `NamedAdded<T>`-wrapping encoders/decoders — `index:item` prefix, same convention
/// `engine::triples::enc_indexed_triple`'s own `IndexAdded<T>` handling uses — used ONLY for a
/// diff's own `added` list (see [`NamedAdded`]'s doc comment); the plain (unwrapped) `enc_*`/
/// `dec_*` above remain the snapshot-level codec for the real entity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_mesh(a: &NamedAdded<SemioMesh>) -> String {
    format!("{}:{}", a.index, enc_mesh(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_mesh(s: &str) -> Result<NamedAdded<SemioMesh>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added mesh: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_mesh(rest)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_primitive(a: &NamedAdded<SemioPrimitive>) -> String {
    format!("{}:{}", a.index, enc_primitive(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_primitive(s: &str) -> Result<NamedAdded<SemioPrimitive>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added primitive: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_primitive(rest)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_material(a: &NamedAdded<SemioMaterial>) -> String {
    format!("{}:{}", a.index, enc_material(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_material(s: &str) -> Result<NamedAdded<SemioMaterial>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added material: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_material(rest)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_added_texture(a: &NamedAdded<SemioTexture>) -> String {
    format!("{}:{}", a.index, enc_texture(&a.item))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_added_texture(s: &str) -> Result<NamedAdded<SemioTexture>, String> {
    let (idx, rest) = s.split_once(':').ok_or_else(|| format!("named added texture: bad entry {s:?}"))?;
    Ok(NamedAdded { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, item: dec_texture(rest)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_primitive_diff(d: &SemioPrimitiveDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        encode_option(&d.topology, |v: &SemioTopology| enc_topology(v)),
        encode_option(&d.positions, |v: &Vec<SemioPoint3>| enc_list(v, enc_point3)),
        encode_option(&d.normals, |v: &Vec<SemioPoint3>| enc_list(v, enc_point3)),
        encode_option(&d.uvs, |v: &Vec<SemioUv>| enc_list(v, enc_uv)),
        encode_option(&d.colors, |v: &Vec<SemioRgba>| enc_list(v, enc_rgba)),
        encode_option(&d.indices, |v: &Vec<u32>| enc_list(v, |x: &u32| x.to_string())),
        encode_option(&d.material_id, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_primitive_diff(s: &str) -> Result<SemioPrimitiveDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [topology, positions, normals, uvs, colors, indices, material_id] = parts.as_slice() else {
        return Err(format!("primitive diff: expected 7 fields, got {}", parts.len()));
    };
    Ok(SemioPrimitiveDiff {
        topology: decode_option(topology, dec_topology)?,
        positions: decode_option(positions, |s| dec_list(s, dec_point3))?,
        normals: decode_option(normals, |s| dec_list(s, dec_point3))?,
        uvs: decode_option(uvs, |s| dec_list(s, dec_uv))?,
        colors: decode_option(colors, |s| dec_list(s, dec_rgba))?,
        indices: decode_option(indices, |s| dec_list(s, parse_u32))?,
        material_id: decode_option(material_id, |s| decode_option(s, dec_str))?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_mesh_item_diff(d: &SemioMeshItemDiff) -> String {
    format!("[{}]", encode_option(&d.primitives, |v: &SemioPrimitivesDiff| enc_named_triple(v, |k: &String| enc_str(k), enc_primitive_diff, enc_named_added_primitive)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_mesh_item_diff(s: &str) -> Result<SemioMeshItemDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(SemioMeshItemDiff { primitives: decode_option(inner, |s| dec_named_triple(s, dec_str, dec_primitive_diff, dec_named_added_primitive))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_material_diff(d: &SemioMaterialDiff) -> String {
    format!("[{},{},{},{},{},{},{},{}]", encode_option(&d.base_color, enc_rgba), encode_option(&d.metallic, |v: &f32| v.to_string()), encode_option(&d.roughness, |v: &f32| v.to_string()), encode_option(&d.base_color_texture, |v| encode_option(v, |id| enc_str(id))), encode_option(&d.metallic_roughness_texture, |v| encode_option(v, |id| enc_str(id))), encode_option(&d.normal_texture, |v| encode_option(v, |id| enc_str(id))), encode_option(&d.occlusion_texture, |v| encode_option(v, |id| enc_str(id))), encode_option(&d.emissive_texture, |v| encode_option(v, |id| enc_str(id))))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_material_diff(s: &str) -> Result<SemioMaterialDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [base_color, metallic, roughness, base_color_texture, metallic_roughness_texture, normal_texture, occlusion_texture, emissive_texture] = parts.as_slice() else { return Err(format!("material diff: expected 8 fields, got {}", parts.len())) };
    Ok(SemioMaterialDiff { base_color: decode_option(base_color, dec_rgba)?, metallic: decode_option(metallic, parse_f32)?, roughness: decode_option(roughness, parse_f32)?, base_color_texture: decode_option(base_color_texture, |value| decode_option(value, dec_str))?, metallic_roughness_texture: decode_option(metallic_roughness_texture, |value| decode_option(value, dec_str))?, normal_texture: decode_option(normal_texture, |value| decode_option(value, dec_str))?, occlusion_texture: decode_option(occlusion_texture, |value| decode_option(value, dec_str))?, emissive_texture: decode_option(emissive_texture, |value| decode_option(value, dec_str))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_texture_diff(d: &SemioTextureDiff) -> String {
    format!("[{},{}]", encode_option(&d.mime, |v: &String| enc_str(v)), encode_option(&d.bytes, |v: &Vec<u8>| enc_bytes(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_texture_diff(s: &str) -> Result<SemioTextureDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [mime, bytes] = parts.as_slice() else { return Err(format!("texture diff: expected 2 fields, got {}", parts.len())) };
    Ok(SemioTextureDiff { mime: decode_option(mime, dec_str)?, bytes: decode_option(bytes, dec_bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_meshes_diff(d: &SemioMeshesDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_mesh_item_diff, enc_named_added_mesh)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_meshes_diff(s: &str) -> Result<SemioMeshesDiff, String> {
    dec_named_triple(s, dec_str, dec_mesh_item_diff, dec_named_added_mesh)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_materials_diff(d: &SemioMaterialsDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_material_diff, enc_named_added_material)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_materials_diff(s: &str) -> Result<SemioMaterialsDiff, String> {
    dec_named_triple(s, dec_str, dec_material_diff, dec_named_added_material)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_textures_diff(d: &SemioTexturesDiff) -> String {
    enc_named_triple(d, |k: &String| enc_str(k), enc_texture_diff, enc_named_added_texture)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_textures_diff(s: &str) -> Result<SemioTexturesDiff, String> {
    dec_named_triple(s, dec_str, dec_texture_diff, dec_named_added_texture)
}
}
pub use diff_codec::*;
