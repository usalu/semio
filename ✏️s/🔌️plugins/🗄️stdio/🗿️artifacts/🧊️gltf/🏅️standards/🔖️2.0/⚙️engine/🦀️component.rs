//! ⚙️ GltfEngine — owns a real `GltfArtifact` plus the byte/container-level glTF 2.0 codecs
//! (base64 data-uri, typed accessor decode, `.gltf` JSON text, `.glb` binary container). Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 gltf/glb merge steps
//! 1-2: gltf absorbs the real container logic here; `🧊️glb` (a separate artifact_kind kept only
//! for a transition compat window) delegates INTO these functions rather than duplicating them.

use crate::artifacts::gltf::schema::snapshot::GltfSourceForm;
use crate::artifacts::gltf::{GltfArtifact, GltfDiff, GltfMutation, GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
use serde_json::Value;

//#region 🔖️Base64
/// 🔤️ Standard base64 alphabet (RFC 4648 §4) — glTF `data:` URIs never use the URL-safe variant.
const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔓️ Decodes standard base64, tolerating embedded whitespace and `=` padding (real-world
/// `data:` URIs are sometimes line-wrapped by whatever authored them).
pub fn b64_decode(data: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0u32;
    for ch in data.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()) {
        let val = B64_TABLE.iter().position(|&t| t == ch).ok_or("invalid base64 character")? as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

/// 🔒️ Encodes standard base64 with `=` padding.
pub fn b64_encode(data: &[u8]) -> String {
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_TABLE[((n >> 18) & 63) as usize] as char);
        out.push(B64_TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { B64_TABLE[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { B64_TABLE[(n & 63) as usize] as char } else { '=' });
    }
    out
}

/// 🌐️ Decodes a `data:<mediatype>;base64,<payload>` URI (buffer.uri/image.uri). Any media type
/// prefix is accepted (`application/octet-stream`, `image/png`, `image/jpeg`, …) — glTF doesn't
/// constrain buffer media types and images legitimately vary; only the `;base64` encoding marker
/// is required (glTF never emits non-base64 `data:` URIs in practice, and text-percent-encoded
/// data URIs for binary buffers aren't spec-sanctioned, so that shape is a typed error).
pub fn decode_data_uri(uri: &str) -> Result<Vec<u8>, String> {
    let rest = uri.strip_prefix("data:").ok_or("not a data: uri")?;
    let comma = rest.find(',').ok_or("data uri missing ',' separator")?;
    let (meta, payload) = (&rest[..comma], &rest[comma + 1..]);
    if !meta.ends_with(";base64") {
        return Err(format!("unsupported data uri encoding (expected ';base64', got {meta:?})"));
    }
    b64_decode(payload)
}

/// 🌐️ Encodes `bytes` as a `data:<media_type>;base64,<payload>` URI.
pub fn encode_data_uri(media_type: &str, bytes: &[u8]) -> String {
    format!("data:{media_type};base64,{}", b64_encode(bytes))
}
//#endregion 🔖️Base64

//#region 🔖️AccessorModel
/// 🔢️ `accessor.componentType` — the 6 values glTF 2.0 permits (§5.1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float,
}

impl GltfComponentType {
    pub fn from_code(code: u64) -> Result<Self, String> {
        Ok(match code {
            5120 => Self::Byte,
            5121 => Self::UnsignedByte,
            5122 => Self::Short,
            5123 => Self::UnsignedShort,
            5125 => Self::UnsignedInt,
            5126 => Self::Float,
            other => return Err(format!("unsupported accessor.componentType {other}")),
        })
    }

    pub fn code(self) -> u64 {
        match self {
            Self::Byte => 5120,
            Self::UnsignedByte => 5121,
            Self::Short => 5122,
            Self::UnsignedShort => 5123,
            Self::UnsignedInt => 5125,
            Self::Float => 5126,
        }
    }

    pub fn byte_size(self) -> usize {
        match self {
            Self::Byte | Self::UnsignedByte => 1,
            Self::Short | Self::UnsignedShort => 2,
            Self::UnsignedInt | Self::Float => 4,
        }
    }

    fn read_at(self, bytes: &[u8], offset: usize) -> Result<f64, String> {
        let size = self.byte_size();
        if offset + size > bytes.len() {
            return Err("accessor component read out of buffer bounds".into());
        }
        Ok(match self {
            Self::Byte => bytes[offset] as i8 as f64,
            Self::UnsignedByte => bytes[offset] as f64,
            Self::Short => i16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
            Self::UnsignedShort => u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
            Self::UnsignedInt => u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
            Self::Float => f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        })
    }
}

/// 🔢️ `accessor.type` — the 7 shapes glTF 2.0 permits (§5.1.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfAccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl GltfAccessorType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        Ok(match s {
            "SCALAR" => Self::Scalar,
            "VEC2" => Self::Vec2,
            "VEC3" => Self::Vec3,
            "VEC4" => Self::Vec4,
            "MAT2" => Self::Mat2,
            "MAT3" => Self::Mat3,
            "MAT4" => Self::Mat4,
            other => return Err(format!("unsupported accessor.type {other:?}")),
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "SCALAR",
            Self::Vec2 => "VEC2",
            Self::Vec3 => "VEC3",
            Self::Vec4 => "VEC4",
            Self::Mat2 => "MAT2",
            Self::Mat3 => "MAT3",
            Self::Mat4 => "MAT4",
        }
    }

    pub fn components(self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 | Self::Mat2 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}

/// 📦️ One decoded accessor: flat row-major `count * accessor_type.components()` values, every
/// component already widened to `f64` regardless of source `componentType` -- real enough for a
/// downstream mesh-view/analyzer to consume actual vertex data instead of opaque bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct GltfDecodedAccessor {
    pub component_type: GltfComponentType,
    pub accessor_type: GltfAccessorType,
    pub count: usize,
    pub normalized: bool,
    pub components: Vec<f64>,
}

/// 📖️ Reads `count` `accessor_type` elements starting at `base_offset` in `bytes`, honoring an
/// explicit `byte_stride` (bufferView.byteStride, element pitch for interleaved data) when given,
/// defaulting to tightly packed (`component_size * num_components`) otherwise.
fn read_elements(bytes: &[u8], base_offset: usize, component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize, byte_stride: Option<usize>) -> Result<Vec<f64>, String> {
    let nc = accessor_type.components();
    let tight = component_type.byte_size() * nc;
    let stride = byte_stride.unwrap_or(tight);
    let mut out = Vec::with_capacity(count * nc);
    for i in 0..count {
        let elem_off = base_offset + i * stride;
        for c in 0..nc {
            out.push(component_type.read_at(bytes, elem_off + c * component_type.byte_size())?);
        }
    }
    Ok(out)
}

/// 🧩️ Decodes `document.accessors[accessor_index]` against `buffers` (index-aligned with
/// `document.buffers`, see [`resolve_document_buffers`]) — dense `bufferView` read, then
/// `accessor.sparse` substitution (base is zero-filled when there's no `bufferView`, per spec).
pub fn decode_accessor(document: &Value, buffers: &[Vec<u8>], accessor_index: usize) -> Result<GltfDecodedAccessor, String> {
    let accessors = document.get("accessors").and_then(Value::as_array).ok_or("document has no accessors")?;
    let acc = accessors.get(accessor_index).ok_or_else(|| format!("accessor index {accessor_index} out of range"))?;
    let component_type = GltfComponentType::from_code(acc.get("componentType").and_then(Value::as_u64).ok_or("accessor.componentType missing")?)?;
    let accessor_type = GltfAccessorType::from_str(acc.get("type").and_then(Value::as_str).ok_or("accessor.type missing")?)?;
    let count = acc.get("count").and_then(Value::as_u64).ok_or("accessor.count missing")? as usize;
    let normalized = acc.get("normalized").and_then(Value::as_bool).unwrap_or(false);
    let nc = accessor_type.components();

    let mut components = vec![0.0f64; count * nc];
    if let Some(bv_idx) = acc.get("bufferView").and_then(Value::as_u64) {
        let extra_offset = acc.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize;
        components = read_bufferview_elements(document, buffers, bv_idx as usize, extra_offset, component_type, accessor_type, count)?;
    }

    if let Some(sparse) = acc.get("sparse") {
        let sparse_count = sparse.get("count").and_then(Value::as_u64).ok_or("accessor.sparse.count missing")? as usize;
        let indices_obj = sparse.get("indices").ok_or("accessor.sparse.indices missing")?;
        let indices_bv = indices_obj.get("bufferView").and_then(Value::as_u64).ok_or("sparse.indices.bufferView missing")? as usize;
        let indices_offset = indices_obj.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize;
        let indices_component = GltfComponentType::from_code(indices_obj.get("componentType").and_then(Value::as_u64).ok_or("sparse.indices.componentType missing")?)?;
        let indices = read_bufferview_elements(document, buffers, indices_bv, indices_offset, indices_component, GltfAccessorType::Scalar, sparse_count)?;

        let values_obj = sparse.get("values").ok_or("accessor.sparse.values missing")?;
        let values_bv = values_obj.get("bufferView").and_then(Value::as_u64).ok_or("sparse.values.bufferView missing")? as usize;
        let values_offset = values_obj.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize;
        let values = read_bufferview_elements(document, buffers, values_bv, values_offset, component_type, accessor_type, sparse_count)?;

        for i in 0..sparse_count {
            let idx = indices[i] as usize;
            let dst = idx * nc;
            if dst + nc > components.len() {
                return Err(format!("sparse accessor index {idx} out of range for count {count}"));
            }
            components[dst..dst + nc].copy_from_slice(&values[i * nc..i * nc + nc]);
        }
    }

    Ok(GltfDecodedAccessor { component_type, accessor_type, count, normalized, components })
}

/// 📖️ Resolves `document.bufferViews[bv_idx]` against `buffers` and decodes `count` elements
/// starting at the bufferView's own `byteOffset` plus `extra_offset` (the accessor's own
/// `byteOffset`, or a sparse indices/values sub-offset).
fn read_bufferview_elements(document: &Value, buffers: &[Vec<u8>], bv_idx: usize, extra_offset: usize, component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Result<Vec<f64>, String> {
    let buffer_views = document.get("bufferViews").and_then(Value::as_array).ok_or("document has no bufferViews")?;
    let bv = buffer_views.get(bv_idx).ok_or_else(|| format!("bufferView index {bv_idx} out of range"))?;
    let buf_idx = bv.get("buffer").and_then(Value::as_u64).ok_or("bufferView.buffer missing")? as usize;
    let bytes = buffers.get(buf_idx).ok_or_else(|| format!("buffer index {buf_idx} out of range"))?;
    if bytes.is_empty() {
        return Err(format!("buffer {buf_idx} bytes unavailable (external uri not resolvable, or empty embedded buffer)"));
    }
    let bv_offset = bv.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let byte_stride = bv.get("byteStride").and_then(Value::as_u64).map(|v| v as usize);
    read_elements(bytes, bv_offset + extra_offset, component_type, accessor_type, count, byte_stride)
}
//#endregion 🔖️AccessorModel

//#region 🔖️DocumentCodec
/// ✅️ Structural well-formedness only -- glTF 2.0 §3.9: `asset.version` is the one universally
/// mandatory field. No mesh/accessor/POSITION precondition: a document with zero meshes (a
/// scene-only or skin-only document) is legitimately valid glTF and must parse.
fn validate_document(document: &Value) -> Result<(), String> {
    let obj = document.as_object().ok_or("gltf document must be a JSON object")?;
    let asset = obj.get("asset").and_then(Value::as_object).ok_or("gltf document missing required 'asset' object")?;
    if asset.get("version").and_then(Value::as_str).is_none() {
        return Err("gltf document 'asset.version' missing or not a string".into());
    }
    Ok(())
}

/// 📦️ Resolves every `document.buffers[i]` to raw bytes, index-aligned with the JSON array.
/// `embedded_bin` is the `.glb` BIN chunk (if any) -- per spec, ONLY `buffers[0]` may omit `uri`
/// and be sourced from it. Buffers with a `data:` uri are decoded; buffers with an external
/// (file-path) uri are left as an empty `Vec` -- this artifact has no filesystem/network access,
/// so those bytes are simply unresolved (not fabricated); the uri string itself stays verbatim in
/// `document`, so nothing is lost, and any attempt to `decode_accessor` through them surfaces a
/// typed error rather than silently returning garbage.
fn resolve_document_buffers(document: &Value, embedded_bin: Option<&[u8]>) -> Vec<Vec<u8>> {
    let Some(buffers) = document.get("buffers").and_then(Value::as_array) else { return Vec::new() };
    buffers
        .iter()
        .enumerate()
        .map(|(i, buf)| match buf.get("uri").and_then(Value::as_str) {
            Some(uri) => decode_data_uri(uri).unwrap_or_default(),
            None if i == 0 => embedded_bin.map(|b| b.to_vec()).unwrap_or_default(),
            None => Vec::new(),
        })
        .collect()
}

/// 📥️ Parses `.gltf` JSON text bytes into a typed snapshot (lenient: no POSITION/mesh
/// precondition, only `asset.version`). `data:` buffer/image uris are decoded eagerly into
/// `buffers`; unmodeled/unknown top-level fields and extensions stay verbatim in `document`
/// (plain `serde_json::Value`, never dropped).
pub fn parse_gltf_document(bytes: &[u8]) -> Result<GltfSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("gltf json is not valid utf-8: {e}"))?;
    let document: Value = serde_json::from_str(text).map_err(|e| format!("gltf json parse error: {e}"))?;
    validate_document(&document)?;
    let buffers = resolve_document_buffers(&document, None);
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Json })
}

/// 📤️ Serializes a snapshot to `.gltf` JSON text bytes. Any buffer that has no `uri` in
/// `document` (i.e. sourced from a `.glb` BIN chunk, `source_form == Glb`) is embedded as a
/// `data:application/octet-stream;base64,…` uri in the emitted copy -- plain `.gltf` JSON has no
/// binary chunk to hold it, so this is the only lossless way to carry those bytes through. Buffers
/// that already declare a `uri` (data or external) are left untouched.
pub fn serialize_gltf_document(snapshot: &GltfSnapshot) -> Vec<u8> {
    let mut document = snapshot.document.clone();
    if let Some(buffers) = document.get_mut("buffers").and_then(Value::as_array_mut) {
        for (i, buf) in buffers.iter_mut().enumerate() {
            let has_uri = buf.get("uri").and_then(Value::as_str).is_some();
            if !has_uri {
                if let Some(bytes) = snapshot.buffers.get(i) {
                    if let Some(obj) = buf.as_object_mut() {
                        obj.insert("uri".into(), Value::String(encode_data_uri("application/octet-stream", bytes)));
                    }
                }
            }
        }
    }
    serde_json::to_vec_pretty(&document).unwrap_or_else(|_| b"{}".to_vec())
}
//#endregion 🔖️DocumentCodec

//#region 🔖️GlbContainer
const GLB_MAGIC: &[u8; 4] = b"glTF";
const GLB_VERSION: u32 = 2;
const CHUNK_TYPE_JSON: &[u8; 4] = b"JSON";
const CHUNK_TYPE_BIN: &[u8; 4] = b"BIN\0";

fn align4(len: usize) -> usize {
    (len + 3) & !3
}

/// 📤️ Encodes a `.glb` binary container: 12-byte header (magic/version/total length) then a JSON
/// chunk (type `0x4E4F534A`, space-padded `0x20` to 4-byte alignment) and, when `buffers[0]` is
/// present and `document.buffers[0]` declares no `uri`, a BIN chunk (type `0x004E4942`,
/// zero-padded `0x00`). Fixes the prior bug: the total-length header field now includes BOTH
/// chunks' padding (it previously omitted the BIN chunk's padding bytes from the count).
pub fn encode_glb(snapshot: &GltfSnapshot) -> Result<Vec<u8>, String> {
    validate_document(&snapshot.document)?;
    let mut document = snapshot.document.clone();
    let embed_bin = document
        .get("buffers")
        .and_then(Value::as_array)
        .and_then(|bs| bs.first())
        .map(|b| b.get("uri").and_then(Value::as_str).is_none())
        .unwrap_or(false);
    let bin: Option<&[u8]> = if embed_bin { snapshot.buffers.first().map(|v| v.as_slice()) } else { None };

    // A buffer embedded via the BIN chunk must NOT carry a `byteLength` mismatch with what we're
    // actually about to embed -- keep it truthful if the caller mutated `buffers` without
    // updating `document.buffers[0].byteLength`.
    if let (true, Some(bin_bytes)) = (embed_bin, bin) {
        if let Some(buf0) = document.get_mut("buffers").and_then(Value::as_array_mut).and_then(|bs| bs.get_mut(0)) {
            if let Some(obj) = buf0.as_object_mut() {
                obj.insert("byteLength".into(), Value::from(bin_bytes.len()));
            }
        }
    }

    let json = serde_json::to_vec(&document).map_err(|e| format!("gltf json encode error: {e}"))?;
    let json_padded_len = align4(json.len());

    let mut out = Vec::new();
    out.extend_from_slice(GLB_MAGIC);
    out.extend_from_slice(&GLB_VERSION.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]); // total length patched below
    out.extend_from_slice(&(json_padded_len as u32).to_le_bytes());
    out.extend_from_slice(CHUNK_TYPE_JSON);
    out.extend_from_slice(&json);
    out.extend(std::iter::repeat(0x20u8).take(json_padded_len - json.len()));

    if let Some(bin_bytes) = bin {
        let bin_padded_len = align4(bin_bytes.len());
        out.extend_from_slice(&(bin_padded_len as u32).to_le_bytes());
        out.extend_from_slice(CHUNK_TYPE_BIN);
        out.extend_from_slice(bin_bytes);
        out.extend(std::iter::repeat(0x00u8).take(bin_padded_len - bin_bytes.len()));
    }

    let total = out.len() as u32;
    out[8..12].copy_from_slice(&total.to_le_bytes());
    Ok(out)
}

/// 📥️ Decodes a `.glb` binary container into a typed snapshot. Walks chunks by their declared
/// (already-padded) length rather than assuming exactly two chunks in a fixed order, per spec
/// (only JSON-first is mandated; BIN is optional and, if present, must be second -- any further
/// chunk types are simply skipped, not fabricated into anything).
pub fn decode_glb(bytes: &[u8]) -> Result<GltfSnapshot, String> {
    if bytes.len() < 12 {
        return Err("glb: truncated 12-byte header".into());
    }
    if &bytes[0..4] != GLB_MAGIC {
        return Err(format!("glb: bad magic {:?}, expected 'glTF'", &bytes[0..4]));
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version != GLB_VERSION {
        return Err(format!("glb: unsupported version {version}, only 2 is supported"));
    }
    let mut pos = 12usize;
    let mut json_chunk: Option<&[u8]> = None;
    let mut bin_chunk: Option<&[u8]> = None;
    while pos + 8 <= bytes.len() {
        let clen = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        let ctype: [u8; 4] = bytes[pos + 4..pos + 8].try_into().unwrap();
        pos += 8;
        if pos + clen > bytes.len() {
            return Err("glb: chunk length overruns container".into());
        }
        let chunk = &bytes[pos..pos + clen];
        if &ctype == CHUNK_TYPE_JSON && json_chunk.is_none() {
            json_chunk = Some(chunk);
        } else if &ctype == CHUNK_TYPE_BIN && bin_chunk.is_none() {
            bin_chunk = Some(chunk);
        }
        pos += clen;
    }
    let json_chunk = json_chunk.ok_or("glb: missing JSON chunk")?;
    // Real-world encoders pad the JSON chunk with spaces (per spec) but some historical writers
    // used NUL or trimmed whitespace -- trim both so lenient real-world files still parse.
    let json_text = std::str::from_utf8(json_chunk).map_err(|e| format!("glb: JSON chunk is not valid utf-8: {e}"))?;
    let document: Value = serde_json::from_str(json_text.trim_end_matches(['\0', ' ', '\t', '\n', '\r'])).map_err(|e| format!("glb: JSON chunk parse error: {e}"))?;
    validate_document(&document)?;

    // The BIN chunk's declared length is 4-byte-padded; the true buffer content length is
    // `document.buffers[0].byteLength` (padding is trailing filler, never real payload).
    let bin_content: Option<Vec<u8>> = bin_chunk.map(|chunk| {
        let declared_len = document
            .get("buffers")
            .and_then(Value::as_array)
            .and_then(|bs| bs.first())
            .and_then(|b| b.get("byteLength"))
            .and_then(Value::as_u64)
            .map(|v| v as usize)
            .unwrap_or(chunk.len());
        chunk[..declared_len.min(chunk.len())].to_vec()
    });

    let buffers = resolve_document_buffers(&document, bin_content.as_deref());
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Glb })
}
//#endregion 🔖️GlbContainer

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_gltf_snapshot() -> GltfSnapshot {
    GltfSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::gltf::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<GltfSnapshot, GltfMutation>(STDIO_GLTF_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.gltf",
        extension: Some("gltf"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::gltf::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::gltf::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::gltf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::gltf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.gltf"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.gltf`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::gltf::schema::gltf_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.gltf` artifact engine.
pub struct GltfEngine {
    artifact_state: GltfArtifact,
    snapshot_state: GltfSnapshot,
}

impl GltfEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: GltfSnapshot) -> Self {
        let artifact_state = GltfArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}

impl protocol::ArtifactEngine for GltfEngine {
    type Artifact = GltfArtifact;
    type Snapshot = GltfSnapshot;
    type Mutation = GltfMutation;
    type Diff = GltfDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact_state
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot_state
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_gltf_snapshot();
        assert_eq!(snapshot.schema, STDIO_GLTF_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_gltf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <GltfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <GltfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️Base64Tests
    #[test]
    fn base64_round_trips_plain_and_data_uri() {
        for payload in [&b""[..], b"a", b"ab", b"abc", b"abcd", &[0u8, 255, 128, 1, 2, 3][..]] {
            let enc = b64_encode(payload);
            assert_eq!(b64_decode(&enc).unwrap(), payload);
            let uri = encode_data_uri("application/octet-stream", payload);
            assert_eq!(decode_data_uri(&uri).unwrap(), payload);
            let img_uri = format!("data:image/png;base64,{}", b64_encode(payload));
            assert_eq!(decode_data_uri(&img_uri).unwrap(), payload);
        }
    }
    //#endregion 🔖️Base64Tests

    //#region 🔖️GlbPaddingTests
    /// 🧪️ Ticket ARTIFACT-SYSTEM-OVERHAUL: the prior `encode_glb` omitted the BIN chunk's own
    /// padding from the header's total-length field whenever `bin.len()` wasn't 4-byte-aligned.
    /// Sweeps json/bin lengths across every mod-4 residue to pin the fix down.
    #[test]
    fn glb_total_length_header_matches_actual_bytes_across_alignments() {
        for json_len in [0usize, 1, 2, 3, 4, 5, 61] {
            for bin_len in [0usize, 1, 2, 3, 4, 5, 100] {
                let document = serde_json::json!({ "asset": { "version": "2.0" }, "buffers": if bin_len > 0 { serde_json::json!([{ "byteLength": bin_len }]) } else { serde_json::json!([]) } });
                let mut snap = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: if bin_len > 0 { vec![vec![0xABu8; bin_len]] } else { vec![] }, source_form: GltfSourceForm::Glb };
                // pad the json body itself out to `json_len` extra bytes via an "extras" string so
                // encode_glb's real JSON serialization (not a synthetic buffer) varies in length.
                if json_len > 0 {
                    snap.document["extras"] = Value::String("x".repeat(json_len));
                }
                let encoded = encode_glb(&snap).expect("encode");
                let declared_total = u32::from_le_bytes(encoded[8..12].try_into().unwrap()) as usize;
                assert_eq!(declared_total, encoded.len(), "total length header wrong for json_len={json_len} bin_len={bin_len}");
                let decoded = decode_glb(&encoded).expect("decode");
                assert_eq!(decoded.buffers.first().map(|b| b.len()).unwrap_or(0), bin_len);
            }
        }
    }

    #[test]
    fn glb_json_padding_is_space_and_bin_padding_is_zero() {
        let document = serde_json::json!({ "asset": { "version": "2.0" }, "buffers": [{ "byteLength": 3 }] });
        let snap = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![vec![1u8, 2, 3]], source_form: GltfSourceForm::Glb };
        let encoded = encode_glb(&snap).expect("encode");
        let json_len = u32::from_le_bytes(encoded[12..16].try_into().unwrap()) as usize;
        let json_start = 20usize;
        let json_chunk = &encoded[json_start..json_start + json_len];
        let real_json_len = serde_json::to_vec(&snap.document).unwrap().len();
        for &b in &json_chunk[real_json_len..] {
            assert_eq!(b, 0x20, "json padding byte must be space");
        }
        let bin_header_start = json_start + json_len;
        let bin_len = u32::from_le_bytes(encoded[bin_header_start..bin_header_start + 4].try_into().unwrap()) as usize;
        let bin_start = bin_header_start + 8;
        let bin_chunk = &encoded[bin_start..bin_start + bin_len];
        assert_eq!(&bin_chunk[..3], &[1u8, 2, 3]);
        for &b in &bin_chunk[3..] {
            assert_eq!(b, 0x00, "bin padding byte must be zero");
        }
    }
    //#endregion 🔖️GlbPaddingTests

    //#region 🔖️AccessorTests
    #[test]
    fn decode_accessor_handles_byte_stride_interleaved_vec3() {
        let mut buf = Vec::new();
        let verts: [[f32; 3]; 2] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let norms: [[f32; 3]; 2] = [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0]];
        for i in 0..2 {
            for c in 0..3 { buf.extend_from_slice(&verts[i][c].to_le_bytes()); }
            for c in 0..3 { buf.extend_from_slice(&norms[i][c].to_le_bytes()); }
        }
        let document = serde_json::json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": buf.len() }],
            "bufferViews": [
                { "buffer": 0, "byteOffset": 0, "byteStride": 24 },
                { "buffer": 0, "byteOffset": 12, "byteStride": 24 }
            ],
            "accessors": [
                { "bufferView": 0, "componentType": 5126, "type": "VEC3", "count": 2 },
                { "bufferView": 1, "componentType": 5126, "type": "VEC3", "count": 2 }
            ]
        });
        let buffers = vec![buf];
        let pos = decode_accessor(&document, &buffers, 0).unwrap();
        assert_eq!(pos.components, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let nrm = decode_accessor(&document, &buffers, 1).unwrap();
        assert_eq!(nrm.components, vec![0.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn decode_accessor_handles_all_component_types() {
        let document = serde_json::json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": 32 }],
            "bufferViews": [{ "buffer": 0, "byteOffset": 0 }],
            "accessors": [{ "bufferView": 0, "componentType": 5121, "type": "SCALAR", "count": 4 }]
        });
        let buffers = vec![vec![10u8, 20, 30, 255]];
        let acc = decode_accessor(&document, &buffers, 0).unwrap();
        assert_eq!(acc.components, vec![10.0, 20.0, 30.0, 255.0]);
    }

    #[test]
    fn decode_accessor_applies_sparse_substitution_over_zero_base() {
        let document = serde_json::json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": 100 }],
            "bufferViews": [
                { "buffer": 0, "byteOffset": 0 },
                { "buffer": 0, "byteOffset": 8 }
            ],
            "accessors": [{
                "componentType": 5126, "type": "SCALAR", "count": 5,
                "sparse": {
                    "count": 2,
                    "indices": { "bufferView": 0, "componentType": 5121 },
                    "values": { "bufferView": 1, "componentType": 5126 }
                }
            }]
        });
        let mut buffers_bytes = vec![0u8; 16];
        buffers_bytes[0] = 1; // sparse index 0 -> element 1
        buffers_bytes[1] = 3; // sparse index 1 -> element 3
        buffers_bytes[8..12].copy_from_slice(&9.5f32.to_le_bytes());
        buffers_bytes[12..16].copy_from_slice(&7.5f32.to_le_bytes());
        let buffers = vec![buffers_bytes];
        let acc = decode_accessor(&document, &buffers, 0).unwrap();
        assert_eq!(acc.components, vec![0.0, 9.5, 0.0, 7.5, 0.0]);
    }
    //#endregion 🔖️AccessorTests

    //#region 🔖️LenientParseTests
    /// 🧪️ Ticket ARTIFACT-SYSTEM-OVERHAUL: the prior parser hard-failed any document lacking a
    /// POSITION accessor. A scene-only document (zero meshes) is legitimately valid glTF.
    #[test]
    fn parse_gltf_document_accepts_scene_only_document_without_position() {
        let text = br#"{"asset":{"version":"2.0"},"scenes":[{"nodes":[]}],"scene":0}"#;
        let snap = parse_gltf_document(text).expect("scene-only document must parse leniently");
        assert_eq!(snap.document["asset"]["version"], "2.0");
        assert!(snap.buffers.is_empty());
    }

    #[test]
    fn parse_gltf_document_rejects_missing_asset_version() {
        let text = br#"{"scenes":[]}"#;
        assert!(parse_gltf_document(text).is_err());
    }

    #[test]
    fn parse_gltf_document_preserves_unknown_top_level_fields_verbatim() {
        let text = br#"{"asset":{"version":"2.0"},"extensions":{"KHR_lights_punctual":{"lights":[{"type":"directional"}]}},"extras":{"authorNote":"kept verbatim"}}"#;
        let snap = parse_gltf_document(text).expect("parse");
        assert_eq!(snap.document["extensions"]["KHR_lights_punctual"]["lights"][0]["type"], "directional");
        assert_eq!(snap.document["extras"]["authorNote"], "kept verbatim");
    }

    #[test]
    fn parse_gltf_document_decodes_data_uri_buffer() {
        let bytes: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let uri = encode_data_uri("application/octet-stream", bytes);
        let text = format!(r#"{{"asset":{{"version":"2.0"}},"buffers":[{{"byteLength":{},"uri":"{uri}"}}]}}"#, bytes.len());
        let snap = parse_gltf_document(text.as_bytes()).expect("parse");
        assert_eq!(snap.buffers[0], bytes);
    }
    //#endregion 🔖️LenientParseTests

    //#region 🔖️DualCodecTests
    #[test]
    fn glb_round_trip_preserves_json_and_bin_semantically() {
        let position_bytes: Vec<u8> = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
            .iter()
            .flat_map(|v| v.iter().flat_map(|c| c.to_le_bytes()))
            .collect();
        let document = serde_json::json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": position_bytes.len() }],
            "bufferViews": [{ "buffer": 0, "byteOffset": 0, "byteLength": position_bytes.len() }],
            "accessors": [{ "bufferView": 0, "componentType": 5126, "type": "VEC3", "count": 3 }],
            "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 } }] }]
        });
        let original = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![position_bytes.clone()], source_form: GltfSourceForm::Glb };
        let encoded = encode_glb(&original).expect("encode");
        assert_eq!(&encoded[0..4], b"glTF");
        let decoded = decode_glb(&encoded).expect("decode");
        assert_eq!(decoded.buffers, original.buffers);
        assert_eq!(decoded.document["asset"]["version"], "2.0");
        let pos_original = decode_accessor(&original.document, &original.buffers, 0).unwrap();
        let pos_decoded = decode_accessor(&decoded.document, &decoded.buffers, 0).unwrap();
        assert_eq!(pos_original.components, pos_decoded.components);
        // decode -> encode -> decode a second time: semantic equality must hold (not necessarily
        // byte-identical serialization, since key ordering / whitespace may legitimately differ).
        let reencoded = encode_glb(&decoded).expect("re-encode");
        let redecoded = decode_glb(&reencoded).expect("re-decode");
        assert_eq!(redecoded.buffers, decoded.buffers);
        assert_eq!(redecoded.document["accessors"], decoded.document["accessors"]);
    }

    #[test]
    fn gltf_json_serialize_embeds_glb_sourced_buffer_as_data_uri() {
        let bytes: Vec<u8> = vec![9, 9, 9, 9];
        let document = serde_json::json!({
            "asset": { "version": "2.0" },
            "buffers": [{ "byteLength": 4 }]
        });
        let snap = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![bytes.clone()], source_form: GltfSourceForm::Glb };
        let text_bytes = serialize_gltf_document(&snap);
        let text = String::from_utf8(text_bytes).unwrap();
        assert!(text.contains("data:application/octet-stream;base64,"));
        let reparsed = parse_gltf_document(text.as_bytes()).expect("reparse emitted .gltf text");
        assert_eq!(reparsed.buffers[0], bytes);
    }
    //#endregion 🔖️DualCodecTests
}
//#endregion 🧪️Tests
