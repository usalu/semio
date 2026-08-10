//! ⚙️ PlyEngine — real ply codec.
//!
//! Decode supports the real §PLY header grammar: arbitrary `element <name> <count>` /
//! `property <type> <name>` / `property list <count-type> <value-type> <name>` declarations,
//! walked generically (not hardcoded to `x y z` + `vertex_indices`), across all three
//! `format` variants (`ascii`, `binary_little_endian`, `binary_big_endian`). Encode always
//! emits the canonical `vertex{x,y,z}` / `face{list uchar int vertex_indices}` layout — see
//! 🚫️EncodeScopeNote below — in whichever of the three wire formats is requested.

use crate::artifacts::ply::schema::snapshot::{MeshTriangle, MeshVertex};
use crate::artifacts::ply::{PlyArtifact, PlyDiff, PlyMutation, PlySnapshot, STDIO_PLY_DOCUMENT_SCHEMA};

//#region 🔖️WireTypes
/// 🧭 Byte order for `binary_little_endian` / `binary_big_endian`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endianness { Little, Big }

/// 📦 The three `format` lines PLY headers may declare.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlyFormat { Ascii, Binary(Endianness) }

/// 🔢 One `property` declaration inside an `element` block.
#[derive(Clone, Debug)]
enum ElementProperty {
    Scalar { ty: String, name: String },
    List { count_ty: String, value_ty: String, name: String },
}

/// 🧱 One `element <name> <count>` block plus its ordered property declarations.
#[derive(Clone, Debug)]
struct ElementDecl { name: String, count: usize, properties: Vec<ElementProperty> }

/// 📖 A fully parsed PLY header: wire format + every element/property declaration in order.
#[derive(Clone, Debug)]
struct PlyHeader { format: PlyFormat, elements: Vec<ElementDecl> }

/// 📏 Byte width of a PLY scalar type name (both long and short spellings).
fn type_size(ty: &str) -> Result<usize, String> {
    match ty {
        "char" | "int8" => Ok(1),
        "uchar" | "uint8" => Ok(1),
        "short" | "int16" => Ok(2),
        "ushort" | "uint16" => Ok(2),
        "int" | "int32" => Ok(4),
        "uint" | "uint32" => Ok(4),
        "float" | "float32" => Ok(4),
        "double" | "float64" => Ok(8),
        other => Err(format!("ply: unsupported property type '{other}'")),
    }
}
//#endregion 🔖️WireTypes

//#region 🔖️HeaderParse
/// ✂️ Splits raw bytes into `(header_text, body)` at the line following `end_header`. The
/// header itself is always ASCII text per spec, even for binary-format files.
fn split_header(data: &[u8]) -> Result<(String, &[u8]), String> {
    let marker = b"end_header";
    let idx = data.windows(marker.len()).position(|w| w == marker).ok_or("ply: missing end_header")?;
    let after_marker = idx + marker.len();
    let mut nl = after_marker;
    while nl < data.len() && data[nl] != b'\n' { nl += 1; }
    if nl >= data.len() { return Err("ply: truncated header".into()); }
    let body_start = nl + 1;
    let header_text = std::str::from_utf8(&data[0..body_start]).map_err(|e| format!("ply: header not utf8: {e}"))?;
    Ok((header_text.to_string(), &data[body_start..]))
}

/// 🧩 Parses the `ply` / `format` / `element` / `property` / `comment` header grammar.
fn parse_header_text(text: &str) -> Result<PlyHeader, String> {
    let mut lines = text.lines();
    let first = lines.next().ok_or("ply: empty header")?.trim();
    if first != "ply" { return Err("ply: expected 'ply' magic line".into()); }
    let mut format: Option<PlyFormat> = None;
    let mut elements: Vec<ElementDecl> = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line == "end_header" { continue; }
        if line.starts_with("comment") || line.starts_with("obj_info") { continue; }
        if let Some(rest) = line.strip_prefix("format ") {
            let mut parts = rest.split_whitespace();
            let kind = parts.next().ok_or("ply: missing format kind")?;
            format = Some(match kind {
                "ascii" => PlyFormat::Ascii,
                "binary_little_endian" => PlyFormat::Binary(Endianness::Little),
                "binary_big_endian" => PlyFormat::Binary(Endianness::Big),
                other => return Err(format!("ply: unsupported format '{other}'")),
            });
        } else if let Some(rest) = line.strip_prefix("element ") {
            let mut parts = rest.split_whitespace();
            let name = parts.next().ok_or("ply: element missing name")?.to_string();
            let count: usize = parts.next().ok_or("ply: element missing count")?.parse().map_err(|e| format!("ply: bad element count: {e}"))?;
            elements.push(ElementDecl { name, count, properties: Vec::new() });
        } else if let Some(rest) = line.strip_prefix("property ") {
            let el = elements.last_mut().ok_or("ply: property declared before any element")?;
            let mut parts = rest.split_whitespace();
            let first_tok = parts.next().ok_or("ply: empty property declaration")?;
            if first_tok == "list" {
                let count_ty = parts.next().ok_or("ply: list property missing count type")?.to_string();
                let value_ty = parts.next().ok_or("ply: list property missing value type")?.to_string();
                let name = parts.next().ok_or("ply: list property missing name")?.to_string();
                type_size(&count_ty)?;
                type_size(&value_ty)?;
                el.properties.push(ElementProperty::List { count_ty, value_ty, name });
            } else {
                let ty = first_tok.to_string();
                let name = parts.next().ok_or("ply: property missing name")?.to_string();
                type_size(&ty)?;
                el.properties.push(ElementProperty::Scalar { ty, name });
            }
        }
    }
    let format = format.ok_or("ply: missing format line")?;
    Ok(PlyHeader { format, elements })
}
//#endregion 🔖️HeaderParse

//#region 🔖️BinaryScalarIo
/// 📥 Reads one scalar of `ty` at `data[*pos..]`, advancing `*pos` by its width.
fn read_scalar_bin(ty: &str, data: &[u8], pos: &mut usize, big: bool) -> Result<f64, String> {
    let size = type_size(ty)?;
    if *pos + size > data.len() { return Err("ply: truncated binary body".into()); }
    let b = &data[*pos..*pos + size];
    let v: f64 = match ty {
        "char" | "int8" => b[0] as i8 as f64,
        "uchar" | "uint8" => b[0] as f64,
        "short" | "int16" => (if big { i16::from_be_bytes([b[0], b[1]]) } else { i16::from_le_bytes([b[0], b[1]]) }) as f64,
        "ushort" | "uint16" => (if big { u16::from_be_bytes([b[0], b[1]]) } else { u16::from_le_bytes([b[0], b[1]]) }) as f64,
        "int" | "int32" => (if big { i32::from_be_bytes([b[0], b[1], b[2], b[3]]) } else { i32::from_le_bytes([b[0], b[1], b[2], b[3]]) }) as f64,
        "uint" | "uint32" => (if big { u32::from_be_bytes([b[0], b[1], b[2], b[3]]) } else { u32::from_le_bytes([b[0], b[1], b[2], b[3]]) }) as f64,
        "float" | "float32" => (if big { f32::from_be_bytes([b[0], b[1], b[2], b[3]]) } else { f32::from_le_bytes([b[0], b[1], b[2], b[3]]) }) as f64,
        "double" | "float64" => if big { f64::from_be_bytes(b.try_into().unwrap()) } else { f64::from_le_bytes(b.try_into().unwrap()) },
        _ => unreachable!("validated by type_size"),
    };
    *pos += size;
    Ok(v)
}

/// 📤 Writes one scalar of `ty` (given as `f64`, truncated/rounded to the target width).
fn push_scalar_bin(out: &mut Vec<u8>, ty: &str, v: f64, big: bool) -> Result<(), String> {
    match ty {
        "char" | "int8" => out.push(v as i8 as u8),
        "uchar" | "uint8" => out.push(v as u8),
        "short" | "int16" => out.extend_from_slice(&if big { (v as i16).to_be_bytes() } else { (v as i16).to_le_bytes() }),
        "ushort" | "uint16" => out.extend_from_slice(&if big { (v as u16).to_be_bytes() } else { (v as u16).to_le_bytes() }),
        "int" | "int32" => out.extend_from_slice(&if big { (v as i32).to_be_bytes() } else { (v as i32).to_le_bytes() }),
        "uint" | "uint32" => out.extend_from_slice(&if big { (v as u32).to_be_bytes() } else { (v as u32).to_le_bytes() }),
        "float" | "float32" => out.extend_from_slice(&if big { (v as f32).to_be_bytes() } else { (v as f32).to_le_bytes() }),
        "double" | "float64" => out.extend_from_slice(&if big { v.to_be_bytes() } else { v.to_le_bytes() }),
        other => return Err(format!("ply: unsupported property type '{other}'")),
    }
    Ok(())
}
//#endregion 🔖️BinaryScalarIo

//#region 🔖️Triangulate
/// 🔺 Fans an arbitrary-length polygon index list into triangles (PLY faces are n-gons).
fn triangulate_face(indices: &[u32]) -> Vec<MeshTriangle> {
    if indices.len() < 3 { return Vec::new(); }
    let mut tris = Vec::with_capacity(indices.len() - 2);
    for i in 1..indices.len() - 1 {
        tris.push(MeshTriangle { i0: indices[0], i1: indices[i], i2: indices[i + 1] });
    }
    tris
}
//#endregion 🔖️Triangulate

//#region 🔖️BodyDecode
/// 📚 Decodes an `ascii`-format body against the parsed header, extracting `vertex.{x,y,z}`
/// and `face.vertex_indices`/`vertex_index` by name while correctly consuming (and
/// discarding) every other declared property so element boundaries stay aligned.
fn decode_body_ascii(body: &str, header: &PlyHeader) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut tokens = body.split_whitespace();
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for el in &header.elements {
        for _ in 0..el.count {
            let (mut x, mut y, mut z) = (None, None, None);
            let mut list_vals: Option<Vec<u32>> = None;
            for prop in &el.properties {
                match prop {
                    ElementProperty::Scalar { name, .. } => {
                        let tok = tokens.next().ok_or("ply: unexpected eof in ascii body")?;
                        let v: f64 = tok.parse().map_err(|e| format!("ply: bad scalar value: {e}"))?;
                        if el.name == "vertex" {
                            match name.as_str() {
                                "x" => x = Some(v as f32),
                                "y" => y = Some(v as f32),
                                "z" => z = Some(v as f32),
                                _ => {}
                            }
                        }
                    }
                    ElementProperty::List { name, .. } => {
                        let n_tok = tokens.next().ok_or("ply: unexpected eof reading list count")?;
                        let n: usize = n_tok.parse().map_err(|e| format!("ply: bad list count: {e}"))?;
                        let mut vals = Vec::with_capacity(n);
                        for _ in 0..n {
                            let vt = tokens.next().ok_or("ply: unexpected eof reading list value")?;
                            let v: i64 = vt.parse().map_err(|e| format!("ply: bad list value: {e}"))?;
                            vals.push(v as u32);
                        }
                        if el.name == "face" && (name == "vertex_indices" || name == "vertex_index") {
                            list_vals = Some(vals);
                        }
                    }
                }
            }
            if el.name == "vertex" {
                let (x, y, z) = (x.ok_or("ply: vertex missing x")?, y.ok_or("ply: vertex missing y")?, z.ok_or("ply: vertex missing z")?);
                vertices.push(MeshVertex { x, y, z });
            }
            if el.name == "face" {
                if let Some(vals) = list_vals { faces.extend(triangulate_face(&vals)); }
            }
        }
    }
    Ok((vertices, faces))
}

/// 📚 Binary counterpart of `decode_body_ascii` — same element/property walk, reading each
/// declared scalar/list at its real byte width (endianness-aware) instead of tokenizing text.
fn decode_body_binary(body: &[u8], header: &PlyHeader, big: bool) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut pos = 0usize;
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for el in &header.elements {
        for _ in 0..el.count {
            let (mut x, mut y, mut z) = (None, None, None);
            let mut list_vals: Option<Vec<u32>> = None;
            for prop in &el.properties {
                match prop {
                    ElementProperty::Scalar { ty, name } => {
                        let v = read_scalar_bin(ty, body, &mut pos, big)?;
                        if el.name == "vertex" {
                            match name.as_str() {
                                "x" => x = Some(v as f32),
                                "y" => y = Some(v as f32),
                                "z" => z = Some(v as f32),
                                _ => {}
                            }
                        }
                    }
                    ElementProperty::List { count_ty, value_ty, name } => {
                        let n = read_scalar_bin(count_ty, body, &mut pos, big)? as usize;
                        let mut vals = Vec::with_capacity(n);
                        for _ in 0..n {
                            let v = read_scalar_bin(value_ty, body, &mut pos, big)?;
                            vals.push(v as i64 as u32);
                        }
                        if el.name == "face" && (name == "vertex_indices" || name == "vertex_index") {
                            list_vals = Some(vals);
                        }
                    }
                }
            }
            if el.name == "vertex" {
                let (x, y, z) = (x.ok_or("ply: vertex missing x")?, y.ok_or("ply: vertex missing y")?, z.ok_or("ply: vertex missing z")?);
                vertices.push(MeshVertex { x, y, z });
            }
            if el.name == "face" {
                if let Some(vals) = list_vals { faces.extend(triangulate_face(&vals)); }
            }
        }
    }
    Ok((vertices, faces))
}
//#endregion 🔖️BodyDecode

//#region 🔖️Codec
/// 🚫 EncodeScopeNote: `encode_ply`/`encode_ply_with_format` always emit the canonical
/// `element vertex {x,y,z: float}` / `element face {list uchar int vertex_indices}` layout —
/// `PlySnapshot` is a canonical triangulated-mesh model, so re-encoding a decoded source that
/// used other property names/types/extra elements will not byte-for-byte round-trip the
/// original file, only its vertex/face content. Decode (above) fully supports the input
/// diversity (arbitrary elements/properties, all three formats); only encode canonicalizes.
fn header_text(format: PlyFormat, n_vertices: usize, n_faces: usize) -> String {
    let fmt_line = match format {
        PlyFormat::Ascii => "format ascii 1.0\n",
        PlyFormat::Binary(Endianness::Little) => "format binary_little_endian 1.0\n",
        PlyFormat::Binary(Endianness::Big) => "format binary_big_endian 1.0\n",
    };
    let mut out = String::new();
    out.push_str("ply\n");
    out.push_str(fmt_line);
    out.push_str(&format!("element vertex {n_vertices}\n"));
    out.push_str("property float x\nproperty float y\nproperty float z\n");
    out.push_str(&format!("element face {n_faces}\n"));
    out.push_str("property list uchar int vertex_indices\n");
    out.push_str("end_header\n");
    out
}

/// 🏗️ Encodes `snap` in the given wire `format` (ascii / binary LE / binary BE).
pub fn encode_ply_with_format(snap: &PlySnapshot, format: PlyFormat) -> Result<Vec<u8>, String> {
    let mut out = header_text(format, snap.vertices.len(), snap.faces.len()).into_bytes();
    match format {
        PlyFormat::Ascii => {
            for v in &snap.vertices {
                out.extend_from_slice(format!("{} {} {}\n", v.x, v.y, v.z).as_bytes());
            }
            for f in &snap.faces {
                out.extend_from_slice(format!("3 {} {} {}\n", f.i0, f.i1, f.i2).as_bytes());
            }
        }
        PlyFormat::Binary(endian) => {
            let big = endian == Endianness::Big;
            for v in &snap.vertices {
                push_scalar_bin(&mut out, "float", v.x as f64, big)?;
                push_scalar_bin(&mut out, "float", v.y as f64, big)?;
                push_scalar_bin(&mut out, "float", v.z as f64, big)?;
            }
            for f in &snap.faces {
                push_scalar_bin(&mut out, "uchar", 3.0, big)?;
                push_scalar_bin(&mut out, "int", f.i0 as f64, big)?;
                push_scalar_bin(&mut out, "int", f.i1 as f64, big)?;
                push_scalar_bin(&mut out, "int", f.i2 as f64, big)?;
            }
        }
    }
    Ok(out)
}

/// 🏗️ Canonical encode — ascii wire format, matches the DSL/pack default.
pub fn encode_ply(snap: &PlySnapshot) -> Result<Vec<u8>, String> {
    encode_ply_with_format(snap, PlyFormat::Ascii)
}

/// 🔍 Decodes any of the three wire formats, dispatching on the header's own `format` line.
pub fn decode_ply(data: &[u8]) -> Result<PlySnapshot, String> {
    let (header_str, body) = split_header(data)?;
    let header = parse_header_text(&header_str)?;
    let (vertices, faces) = match header.format {
        PlyFormat::Ascii => {
            let body_text = std::str::from_utf8(body).map_err(|e| format!("ply: ascii body not utf8: {e}"))?;
            decode_body_ascii(body_text, &header)?
        }
        PlyFormat::Binary(endian) => decode_body_binary(body, &header, endian == Endianness::Big)?,
    };
    Ok(PlySnapshot { schema: STDIO_PLY_DOCUMENT_SCHEMA.into(), vertices, faces })
}

/// 🌱 Empty persisted snapshot.
pub fn empty_ply_snapshot() -> PlySnapshot {
    PlySnapshot::default()
}
//#endregion 🔖️Codec

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::ply::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<PlySnapshot, PlyMutation>(STDIO_PLY_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.ply",
        extension: Some("ply"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::ply::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::ply::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::ply::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::ply::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.ply"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.ply`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::ply::schema::ply_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.ply` artifact engine.
pub struct PlyEngine {
    artifact_state: PlyArtifact,
    snapshot_state: PlySnapshot,
}

impl PlyEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: PlySnapshot) -> Self {
        let artifact_state = PlyArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_ply_snapshot();
        assert_eq!(snapshot.schema, STDIO_PLY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_ply_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <PlySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <PlySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️MeshFixture
    /// 🔺 A tetrahedron: 4 vertices, 4 triangular faces — small enough to hand-check, non
    /// trivial enough (mixed-sign coordinates, several faces) to catch real layout bugs.
    fn tetrahedron() -> PlySnapshot {
        PlySnapshot {
            schema: STDIO_PLY_DOCUMENT_SCHEMA.into(),
            vertices: vec![
                MeshVertex { x: 0.0, y: 0.0, z: 0.0 },
                MeshVertex { x: 1.0, y: 0.0, z: 0.0 },
                MeshVertex { x: 0.0, y: 1.0, z: 0.0 },
                MeshVertex { x: 0.0, y: 0.0, z: 1.0 },
            ],
            faces: vec![
                MeshTriangle { i0: 0, i1: 1, i2: 2 },
                MeshTriangle { i0: 0, i1: 1, i2: 3 },
                MeshTriangle { i0: 0, i1: 2, i2: 3 },
                MeshTriangle { i0: 1, i1: 2, i2: 3 },
            ],
        }
    }
    //#endregion 🔖️MeshFixture

    #[test]
    fn ascii_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::Ascii).expect("encode ascii");
        let decoded = decode_ply(&bytes).expect("decode ascii");
        assert_eq!(decoded.vertices, snap.vertices);
        assert_eq!(decoded.faces, snap.faces);
    }

    #[test]
    fn binary_little_endian_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::Binary(Endianness::Little)).expect("encode binary LE");
        assert!(bytes.starts_with(b"ply\nformat binary_little_endian 1.0\n"), "header must declare binary_little_endian");
        let decoded = decode_ply(&bytes).expect("decode binary LE");
        assert_eq!(decoded.vertices, snap.vertices, "binary LE vertices must exactly match the original");
        assert_eq!(decoded.faces, snap.faces, "binary LE faces must exactly match the original");
    }

    #[test]
    fn binary_big_endian_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_ply_with_format(&snap, PlyFormat::Binary(Endianness::Big)).expect("encode binary BE");
        let decoded = decode_ply(&bytes).expect("decode binary BE");
        assert_eq!(decoded.vertices, snap.vertices, "binary BE vertices must exactly match the original");
        assert_eq!(decoded.faces, snap.faces, "binary BE faces must exactly match the original");
    }

    #[test]
    fn binary_decode_skips_unmodeled_properties() {
        // Hand-crafted binary_little_endian stream with per-vertex normals (nx/ny/nz) that
        // PlySnapshot does not model — proves decode walks the real header-declared property
        // list (consuming and discarding nx/ny/nz) instead of assuming a fixed 12-byte stride.
        let header = "ply\nformat binary_little_endian 1.0\nelement vertex 1\nproperty float x\nproperty float y\nproperty float z\nproperty float nx\nproperty float ny\nproperty float nz\nelement face 0\nproperty list uchar int vertex_indices\nend_header\n";
        let mut bytes = header.as_bytes().to_vec();
        for f in [1.5f32, 2.5, 3.5, 9.0, 9.0, 9.0] {
            bytes.extend_from_slice(&f.to_le_bytes());
        }
        let decoded = decode_ply(&bytes).expect("decode with skipped normal properties");
        assert_eq!(decoded.vertices, vec![MeshVertex { x: 1.5, y: 2.5, z: 3.5 }]);
        assert!(decoded.faces.is_empty());
    }

    #[test]
    fn ascii_decode_rejects_truncated_body() {
        let header = "ply\nformat ascii 1.0\nelement vertex 2\nproperty float x\nproperty float y\nproperty float z\nelement face 0\nproperty list uchar int vertex_indices\nend_header\n1 2 3\n";
        let err = decode_ply(header.as_bytes()).unwrap_err();
        assert!(err.contains("eof"), "unexpected error: {err}");
    }

    #[test]
    fn missing_end_header_is_rejected() {
        let err = decode_ply(b"ply\nformat ascii 1.0\n").unwrap_err();
        assert!(err.contains("end_header"));
    }
}
//#endregion 🧪️Tests
