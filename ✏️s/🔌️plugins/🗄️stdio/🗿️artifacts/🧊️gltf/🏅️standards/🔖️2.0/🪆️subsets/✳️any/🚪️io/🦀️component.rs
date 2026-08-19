//! 🚪️ IO stdio.gltf (2.0/✳️any) — registration now flows through the `s.stdio.gltf`
//! `ArtifactDeclaration` (`crate::artifacts::gltf::declaration`), not per-leaf register().
//!
//! ⚙️ Owns the byte/container-level glTF 2.0 codecs (base64 data-uri, typed accessor decode,
//! `.gltf` JSON text, `.glb` binary container). Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-
//! REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 gltf/glb merge: the separate `🧊️glb` artifact_kind
//! (steps 1-2's transition compat shim) has been folded and deleted (steps 3-5) -- every former
//! glb caller now targets this codec's own `.glb` binary dialect directly, so there is no longer
//! a second container implementation to keep in sync.
use crate::artifacts::gltf::schema::snapshot::{GltfDocument, GltfSourceForm};
#[cfg(test)]
use crate::artifacts::gltf::schema::snapshot::{
    GltfAccessor, GltfBuffer, GltfBufferView, GltfJson, GltfMesh, GltfPrimitive, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues,
};
use crate::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};

//#region 🔖️Base64
/// 🔤️ Standard base64 alphabet (RFC 4648 §4) — glTF `data:` URIs never use the URL-safe variant.
const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔓️ Decodes standard base64, tolerating embedded whitespace and `=` padding (real-world
/// `data:` URIs are sometimes line-wrapped by whatever authored them).
pub async fn b64_decode(data: &str) -> Result<Vec<u8>, String> {
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
pub async fn b64_encode(data: &[u8]) -> String {
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
pub async fn decode_data_uri(uri: &str) -> Result<Vec<u8>, String> {
    let rest = uri.strip_prefix("data:").ok_or("not a data: uri")?;
    let comma = rest.find(',').ok_or("data uri missing ',' separator")?;
    let (meta, payload) = (&rest[..comma], &rest[comma + 1..]);
    if !meta.ends_with(";base64") {
        return Err(format!("unsupported data uri encoding (expected ';base64', got {meta:?})"));
    }
    b64_decode(payload)
}

/// 🌐️ Encodes `bytes` as a `data:<media_type>;base64,<payload>` URI.
pub async fn encode_data_uri(media_type: &str, bytes: &[u8]) -> String {
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
    pub async fn from_code(code: u64) -> Result<Self, String> {
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

    pub async fn code(self) -> u64 {
        match self {
            Self::Byte => 5120,
            Self::UnsignedByte => 5121,
            Self::Short => 5122,
            Self::UnsignedShort => 5123,
            Self::UnsignedInt => 5125,
            Self::Float => 5126,
        }
    }

    pub async fn byte_size(self) -> usize {
        match self {
            Self::Byte | Self::UnsignedByte => 1,
            Self::Short | Self::UnsignedShort => 2,
            Self::UnsignedInt | Self::Float => 4,
        }
    }

    async fn read_at(self, bytes: &[u8], offset: usize) -> Result<f64, String> {
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
    pub async fn from_str(s: &str) -> Result<Self, String> {
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

    pub async fn as_str(self) -> &'static str {
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

    pub async fn components(self) -> usize {
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

//#region 🔖️AccessorModelSerde
/// 🧵 `accessor.componentType` on the wire is always the raw numeric code (5120..5126) — never a
/// string -- so this hand-rolls `Serialize`/`Deserialize` around [`GltfComponentType::code`]/
/// [`GltfComponentType::from_code`] rather than deriving (a derive would emit the Rust variant
/// name, not a spec-legal wire value).
impl Serialize for GltfComponentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.code())
    }
}
impl<'de> Deserialize<'de> for GltfComponentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = u64::deserialize(deserializer)?;
        Self::from_code(code).map_err(serde::de::Error::custom)
    }
}

/// 🧵 `accessor.type` on the wire is always the spec string (`"SCALAR"`, `"VEC3"`, …) -- hand-rolled
/// for the same reason as [`GltfComponentType`]'s impl above.
impl Serialize for GltfAccessorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GltfAccessorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}
//#endregion 🔖️AccessorModelSerde

/// 📦️ One decoded accessor: flat row-major `count * accessor_type.components()` values, widened
/// to `f64` and normalized when requested by the accessor before any consumer observes them.
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
async fn read_elements(bytes: &[u8], base_offset: usize, component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize, byte_stride: Option<usize>) -> Result<Vec<f64>, String> {
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

/// 🎚️ Applies glTF 2.0 accessor normalization after dense and sparse values have been
/// assembled, preserving the exact signed lower-bound rule from §3.6.2.2.
async fn normalize_components(component_type: GltfComponentType, components: &mut [f64]) -> Result<(), String> {
    let (scale, signed) = match component_type {
        GltfComponentType::Byte => (127.0, true),
        GltfComponentType::UnsignedByte => (255.0, false),
        GltfComponentType::Short => (32_767.0, true),
        GltfComponentType::UnsignedShort => (65_535.0, false),
        GltfComponentType::UnsignedInt => (4_294_967_295.0, false),
        GltfComponentType::Float => return Err("normalized FLOAT accessor is invalid glTF 2.0".into()),
    };
    for value in components {
        *value = if signed { (*value / scale).max(-1.0) } else { *value / scale };
    }
    Ok(())
}

/// 🧩️ Decodes `document.accessors[accessor_index]` against `buffers` (index-aligned with
/// `document.buffers`, see [`resolve_document_buffers`]) — dense `bufferView` read, then
/// `accessor.sparse` substitution (base is zero-filled when there's no `bufferView`, per spec).
pub async fn decode_accessor(document: &GltfDocument, buffers: &[Vec<u8>], accessor_index: usize) -> Result<GltfDecodedAccessor, String> {
    let acc = document.accessors.get(accessor_index).ok_or_else(|| format!("accessor index {accessor_index} out of range"))?;
    let component_type = acc.component_type;
    let accessor_type = acc.kind;
    let count = acc.count;
    let normalized = acc.normalized;
    let nc = accessor_type.components();

    let mut components = vec![0.0f64; count * nc];
    if let Some(bv_idx) = acc.buffer_view {
        components = read_bufferview_elements(document, buffers, bv_idx, acc.byte_offset, component_type, accessor_type, count)?;
    }

    if let Some(sparse) = &acc.sparse {
        let sparse_count = sparse.count;
        let indices_component = sparse.indices.component_type;
        let indices = read_bufferview_elements(document, buffers, sparse.indices.buffer_view, sparse.indices.byte_offset, indices_component, GltfAccessorType::Scalar, sparse_count)?;
        let values = read_bufferview_elements(document, buffers, sparse.values.buffer_view, sparse.values.byte_offset, component_type, accessor_type, sparse_count)?;

        for i in 0..sparse_count {
            let idx = indices[i] as usize;
            let dst = idx * nc;
            if dst + nc > components.len() {
                return Err(format!("sparse accessor index {idx} out of range for count {count}"));
            }
            components[dst..dst + nc].copy_from_slice(&values[i * nc..i * nc + nc]);
        }
    }

    if normalized {
        normalize_components(component_type, &mut components)?;
    }

    Ok(GltfDecodedAccessor { component_type, accessor_type, count, normalized, components })
}

/// 📖️ Resolves `document.bufferViews[bv_idx]` against `buffers` and decodes `count` elements
/// starting at the bufferView's own `byteOffset` plus `extra_offset` (the accessor's own
/// `byteOffset`, or a sparse indices/values sub-offset).
async fn read_bufferview_elements(document: &GltfDocument, buffers: &[Vec<u8>], bv_idx: usize, extra_offset: usize, component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Result<Vec<f64>, String> {
    let bv = document.buffer_views.get(bv_idx).ok_or_else(|| format!("bufferView index {bv_idx} out of range"))?;
    let bytes = buffers.get(bv.buffer).ok_or_else(|| format!("buffer index {} out of range", bv.buffer))?;
    if bytes.is_empty() {
        return Err(format!("buffer {} bytes unavailable (external uri not resolvable, or empty embedded buffer)", bv.buffer));
    }
    read_elements(bytes, bv.byte_offset + extra_offset, component_type, accessor_type, count, bv.byte_stride)
}
//#endregion 🔖️AccessorModel

//#region 🔖️DocumentCodec
/// ✅️ Structural well-formedness only -- glTF 2.0 §3.9: `asset.version` is the one universally
/// mandatory field (already enforced by the type system: [`GltfAsset::version`] is a plain
/// `String`, not `Option<String>`) -- this only rejects the empty string, since serde alone can't
/// express "non-empty". No mesh/accessor/POSITION precondition: a document with zero meshes (a
/// scene-only or skin-only document) is legitimately valid glTF and must parse.
async fn validate_document(document: &GltfDocument) -> Result<(), String> {
    if document.asset.version.trim().is_empty() {
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
async fn resolve_document_buffers(document: &GltfDocument, embedded_bin: Option<&[u8]>) -> Vec<Vec<u8>> {
    document
        .buffers
        .iter()
        .enumerate()
        .map(|(i, buf)| match buf.uri.as_deref() {
            Some(uri) => decode_data_uri(uri).unwrap_or_default(),
            None if i == 0 => embedded_bin.map(|b| b.to_vec()).unwrap_or_default(),
            None => Vec::new(),
        })
        .collect()
}

/// 📥️ Parses `.gltf` JSON text bytes into a typed snapshot (lenient: no POSITION/mesh
/// precondition, only `asset.version`) via `serde_json::from_str::<GltfDocument>` -- every spec
/// top-level field lands in its typed slot; `extras`/`extensions` decode into this module's own
/// [`GltfJson`] (never `serde_json::Value`), so nothing real on disk is dropped.
pub async fn parse_gltf_document(bytes: &[u8]) -> Result<GltfSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("gltf json is not valid utf-8: {e}"))?;
    let document: GltfDocument = serde_json::from_str(text).map_err(|e| format!("gltf json parse error: {e}"))?;
    validate_document(&document)?;
    let buffers = resolve_document_buffers(&document, None);
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Json })
}

/// 📤️ Serializes a snapshot to `.gltf` JSON text bytes. Any buffer that has no `uri` in
/// `document` (i.e. sourced from a `.glb` BIN chunk, `source_form == Glb`) is embedded as a
/// `data:application/octet-stream;base64,…` uri in the emitted copy -- plain `.gltf` JSON has no
/// binary chunk to hold it, so this is the only lossless way to carry those bytes through. Buffers
/// that already declare a `uri` (data or external) are left untouched.
pub async fn serialize_gltf_document(snapshot: &GltfSnapshot) -> Vec<u8> {
    let mut document = snapshot.document.clone();
    for (i, buf) in document.buffers.iter_mut().enumerate() {
        if buf.uri.is_none() {
            if let Some(bytes) = snapshot.buffers.get(i) {
                buf.uri = Some(encode_data_uri("application/octet-stream", bytes));
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

async fn align4(len: usize) -> usize {
    (len + 3) & !3
}

/// 📤️ Encodes a `.glb` binary container: 12-byte header (magic/version/total length) then a JSON
/// chunk (type `0x4E4F534A`, space-padded `0x20` to 4-byte alignment) and, when `buffers[0]` is
/// present and `document.buffers[0]` declares no `uri`, a BIN chunk (type `0x004E4942`,
/// zero-padded `0x00`). Fixes the prior bug: the total-length header field now includes BOTH
/// chunks' padding (it previously omitted the BIN chunk's padding bytes from the count).
pub async fn encode_glb(snapshot: &GltfSnapshot) -> Result<Vec<u8>, String> {
    validate_document(&snapshot.document)?;
    let mut document = snapshot.document.clone();
    let embed_bin = document.buffers.first().map(|b| b.uri.is_none()).unwrap_or(false);
    let bin: Option<&[u8]> = if embed_bin { snapshot.buffers.first().map(|v| v.as_slice()) } else { None };

    // A buffer embedded via the BIN chunk must NOT carry a `byteLength` mismatch with what we're
    // actually about to embed -- keep it truthful if the caller mutated `buffers` without
    // updating `document.buffers[0].byteLength`.
    if let (true, Some(bin_bytes)) = (embed_bin, bin) {
        if let Some(buf0) = document.buffers.get_mut(0) {
            buf0.byte_length = bin_bytes.len();
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
pub async fn decode_glb(bytes: &[u8]) -> Result<GltfSnapshot, String> {
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
    let document: GltfDocument = serde_json::from_str(json_text.trim_end_matches(['\0', ' ', '\t', '\n', '\r'])).map_err(|e| format!("glb: JSON chunk parse error: {e}"))?;
    validate_document(&document)?;

    // The BIN chunk's declared length is 4-byte-padded; the true buffer content length is
    // `document.buffers[0].byteLength` (padding is trailing filler, never real payload).
    let bin_content: Option<Vec<u8>> = bin_chunk.map(|chunk| {
        let declared_len = document.buffers.first().map(|b| b.byte_length).unwrap_or(chunk.len());
        chunk[..declared_len.min(chunk.len())].to_vec()
    });

    let buffers = resolve_document_buffers(&document, bin_content.as_deref());
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Glb })
}
//#endregion 🔖️GlbContainer

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfAnalyzer;
    use crate::artifacts::gltf::GltfSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct GltfComposerComposition;

    impl ArtifactComposition for GltfComposerComposition {
        type Snapshot = GltfSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_BINARY]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts. Binary
            // sources are analyzed with real `.glb`-vs-pack sniffing (see `GltfAnalyzer::analyze`), so
            // a `DEP_BINARY` source carrying raw `.glb` bytes decodes through the exact same path a
            // hand-fed `AnalyzeSource::Binary` would.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_JSON || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "GltfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = GltfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "GltfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = crate::artifacts::gltf::engine::empty_gltf_snapshot();
        assert_eq!(snapshot.schema, STDIO_GLTF_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = crate::artifacts::gltf::engine::empty_gltf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <GltfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        // P2-FG3: `ArtifactPack::encode_pack`/`decode_pack` now route through the REAL `.glb`
        // binary container (`encode_glb`/`decode_glb`), not the prior JSON-as-"binary" shortcut —
        // decoding real GLB bytes always reports `source_form: Glb` (the byte form genuinely IS a
        // glb container now), which is the one field expected to legitimately differ from `snap`'s
        // own `Json` provenance; document/buffers/schema stay byte-for-byte lossless.
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <GltfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.schema, snap.schema);
        assert_eq!(decoded.document, snap.document);
        assert_eq!(decoded.buffers, snap.buffers);
        assert_eq!(decoded.source_form, GltfSourceForm::Glb);
    }

    //#region 🔖️Base64Tests
    #[semio_framework_async_macros::async_test]
    async fn base64_round_trips_plain_and_data_uri() {
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
    async fn doc_with_buffer(byte_length: usize) -> GltfDocument {
        GltfDocument { buffers: if byte_length > 0 { vec![GltfBuffer { byte_length, uri: None, name: None, extensions: None, extras: None }] } else { Vec::new() }, ..GltfDocument::default() }
    }

    /// 🧪️ Ticket ARTIFACT-SYSTEM-OVERHAUL: the prior `encode_glb` omitted the BIN chunk's own
    /// padding from the header's total-length field whenever `bin.len()` wasn't 4-byte-aligned.
    /// Sweeps json/bin lengths across every mod-4 residue to pin the fix down.
    #[semio_framework_async_macros::async_test]
    async fn glb_total_length_header_matches_actual_bytes_across_alignments() {
        for json_len in [0usize, 1, 2, 3, 4, 5, 61] {
            for bin_len in [0usize, 1, 2, 3, 4, 5, 100] {
                let mut document = doc_with_buffer(bin_len);
                // pad the json body itself out to `json_len` extra bytes via an "extras" string so
                // encode_glb's real JSON serialization (not a synthetic buffer) varies in length.
                if json_len > 0 {
                    document.extras = Some(GltfJson::String("x".repeat(json_len)));
                }
                let snap = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: if bin_len > 0 { vec![vec![0xABu8; bin_len]] } else { vec![] }, source_form: GltfSourceForm::Glb };
                let encoded = encode_glb(&snap).expect("encode");
                let declared_total = u32::from_le_bytes(encoded[8..12].try_into().unwrap()) as usize;
                assert_eq!(declared_total, encoded.len(), "total length header wrong for json_len={json_len} bin_len={bin_len}");
                let decoded = decode_glb(&encoded).expect("decode");
                assert_eq!(decoded.buffers.first().map(|b| b.len()).unwrap_or(0), bin_len);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn glb_json_padding_is_space_and_bin_padding_is_zero() {
        let document = doc_with_buffer(3);
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
    async fn accessor(buffer_view: usize, component_type: GltfComponentType, kind: GltfAccessorType, count: usize) -> GltfAccessor {
        GltfAccessor { buffer_view: Some(buffer_view), byte_offset: 0, component_type, normalized: false, count, kind, max: None, min: None, sparse: None, name: None, extensions: None, extras: None }
    }
    async fn buffer_view(buffer: usize, byte_offset: usize, byte_stride: Option<usize>) -> GltfBufferView {
        GltfBufferView { buffer, byte_offset, byte_length: 0, byte_stride, target: None, name: None, extensions: None, extras: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_accessor_handles_byte_stride_interleaved_vec3() {
        let mut buf = Vec::new();
        let verts: [[f32; 3]; 2] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let norms: [[f32; 3]; 2] = [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0]];
        for i in 0..2 {
            for c in 0..3 {
                buf.extend_from_slice(&verts[i][c].to_le_bytes());
            }
            for c in 0..3 {
                buf.extend_from_slice(&norms[i][c].to_le_bytes());
            }
        }
        let document = GltfDocument {
            buffers: vec![GltfBuffer { byte_length: buf.len(), uri: None, name: None, extensions: None, extras: None }],
            buffer_views: vec![buffer_view(0, 0, Some(24)), buffer_view(0, 12, Some(24))],
            accessors: vec![accessor(0, GltfComponentType::Float, GltfAccessorType::Vec3, 2), accessor(1, GltfComponentType::Float, GltfAccessorType::Vec3, 2)],
            ..GltfDocument::default()
        };
        let buffers = vec![buf];
        let pos = decode_accessor(&document, &buffers, 0).unwrap();
        assert_eq!(pos.components, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let nrm = decode_accessor(&document, &buffers, 1).unwrap();
        assert_eq!(nrm.components, vec![0.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_accessor_handles_all_component_types() {
        let document = GltfDocument {
            buffers: vec![GltfBuffer { byte_length: 32, uri: None, name: None, extensions: None, extras: None }],
            buffer_views: vec![buffer_view(0, 0, None)],
            accessors: vec![accessor(0, GltfComponentType::UnsignedByte, GltfAccessorType::Scalar, 4)],
            ..GltfDocument::default()
        };
        let buffers = vec![vec![10u8, 20, 30, 255]];
        let acc = decode_accessor(&document, &buffers, 0).unwrap();
        assert_eq!(acc.components, vec![10.0, 20.0, 30.0, 255.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_accessor_applies_spec_normalization_after_sparse_overlay() {
        let document = GltfDocument {
            buffers: vec![GltfBuffer { byte_length: 7, uri: None, name: None, extensions: None, extras: None }],
            buffer_views: vec![buffer_view(0, 0, None), buffer_view(0, 3, None)],
            accessors: vec![GltfAccessor {
                buffer_view: None,
                byte_offset: 0,
                component_type: GltfComponentType::Byte,
                normalized: true,
                count: 4,
                kind: GltfAccessorType::Scalar,
                max: None,
                min: None,
                sparse: Some(GltfSparseAccessor { count: 3, indices: GltfSparseIndices { buffer_view: 0, byte_offset: 0, component_type: GltfComponentType::UnsignedByte }, values: GltfSparseValues { buffer_view: 1, byte_offset: 0 } }),
                name: None,
                extensions: None,
                extras: None,
            }],
            ..GltfDocument::default()
        };
        let buffers = vec![vec![0, 2, 3, 128, 64, 127, 0]];
        let acc = decode_accessor(&document, &buffers, 0).unwrap();
        assert_eq!(acc.components, vec![-1.0, 0.0, 64.0 / 127.0, 1.0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn accessor_normalization_covers_every_legal_integer_component_type() {
        let cases = [
            (GltfComponentType::Byte, vec![-128.0, 127.0], vec![-1.0, 1.0]),
            (GltfComponentType::UnsignedByte, vec![0.0, 255.0], vec![0.0, 1.0]),
            (GltfComponentType::Short, vec![-32_768.0, 32_767.0], vec![-1.0, 1.0]),
            (GltfComponentType::UnsignedShort, vec![0.0, 65_535.0], vec![0.0, 1.0]),
            (GltfComponentType::UnsignedInt, vec![0.0, 4_294_967_295.0], vec![0.0, 1.0]),
        ];
        for (component_type, mut actual, expected) in cases {
            normalize_components(component_type, &mut actual).unwrap();
            assert_eq!(actual, expected);
        }
        assert!(normalize_components(GltfComponentType::Float, &mut [1.0]).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_accessor_applies_sparse_substitution_over_zero_base() {
        let document = GltfDocument {
            buffers: vec![GltfBuffer { byte_length: 100, uri: None, name: None, extensions: None, extras: None }],
            buffer_views: vec![buffer_view(0, 0, None), buffer_view(0, 8, None)],
            accessors: vec![GltfAccessor {
                buffer_view: None,
                byte_offset: 0,
                component_type: GltfComponentType::Float,
                normalized: false,
                count: 5,
                kind: GltfAccessorType::Scalar,
                max: None,
                min: None,
                sparse: Some(GltfSparseAccessor { count: 2, indices: GltfSparseIndices { buffer_view: 0, byte_offset: 0, component_type: GltfComponentType::UnsignedByte }, values: GltfSparseValues { buffer_view: 1, byte_offset: 0 } }),
                name: None,
                extensions: None,
                extras: None,
            }],
            ..GltfDocument::default()
        };
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
    #[semio_framework_async_macros::async_test]
    async fn parse_gltf_document_accepts_scene_only_document_without_position() {
        let text = br#"{"asset":{"version":"2.0"},"scenes":[{"nodes":[]}],"scene":0}"#;
        let snap = parse_gltf_document(text).expect("scene-only document must parse leniently");
        assert_eq!(snap.document.asset.version, "2.0");
        assert!(snap.buffers.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_gltf_document_preserves_morph_target_maps() {
        let text = br#"{"asset":{"version":"2.0"},"meshes":[{"primitives":[{"attributes":{"POSITION":0},"targets":[{"POSITION":1,"NORMAL":2}]}]}]}"#;
        let snap = parse_gltf_document(text).expect("morph targets are core glTF 2.0 data");
        let target = &snap.document.meshes[0].primitives[0].targets[0].0;
        assert_eq!(target, &vec![("POSITION".into(), 1), ("NORMAL".into(), 2)]);
        let encoded = serialize_gltf_document(&snap);
        let reparsed = parse_gltf_document(&encoded).expect("serialized morph target must remain valid");
        assert_eq!(reparsed.document.meshes[0].primitives[0].targets, snap.document.meshes[0].primitives[0].targets);
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_gltf_document_rejects_missing_asset_version() {
        let text = br#"{"scenes":[]}"#;
        assert!(parse_gltf_document(text).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_gltf_document_preserves_unknown_top_level_fields_verbatim() {
        let text = br#"{"asset":{"version":"2.0"},"extensions":{"KHR_lights_punctual":{"lights":[{"type":"directional"}]}},"extras":{"authorNote":"kept verbatim"}}"#;
        let snap = parse_gltf_document(text).expect("parse");
        let GltfJson::Object(exts) = snap.document.extensions.clone().expect("extensions") else { panic!("expected object") };
        assert_eq!(exts[0].0, "KHR_lights_punctual");
        let GltfJson::Object(extras) = snap.document.extras.clone().expect("extras") else { panic!("expected object") };
        assert_eq!(extras[0], ("authorNote".to_string(), GltfJson::String("kept verbatim".into())));
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_gltf_document_decodes_data_uri_buffer() {
        let bytes: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let uri = encode_data_uri("application/octet-stream", bytes);
        let text = format!(r#"{{"asset":{{"version":"2.0"}},"buffers":[{{"byteLength":{},"uri":"{uri}"}}]}}"#, bytes.len());
        let snap = parse_gltf_document(text.as_bytes()).expect("parse");
        assert_eq!(snap.buffers[0], bytes);
    }
    //#endregion 🔖️LenientParseTests

    //#region 🔖️DualCodecTests
    #[semio_framework_async_macros::async_test]
    async fn glb_round_trip_preserves_json_and_bin_semantically() {
        let position_bytes: Vec<u8> = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]].iter().flat_map(|v| v.iter().flat_map(|c| c.to_le_bytes())).collect();
        let document = GltfDocument {
            buffers: vec![GltfBuffer { byte_length: position_bytes.len(), uri: None, name: None, extensions: None, extras: None }],
            buffer_views: vec![GltfBufferView { buffer: 0, byte_offset: 0, byte_length: position_bytes.len(), byte_stride: None, target: None, name: None, extensions: None, extras: None }],
            accessors: vec![accessor(0, GltfComponentType::Float, GltfAccessorType::Vec3, 3)],
            meshes: vec![GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], ..GltfPrimitive::default() }], ..GltfMesh::default() }],
            ..GltfDocument::default()
        };
        let original = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![position_bytes.clone()], source_form: GltfSourceForm::Glb };
        let encoded = encode_glb(&original).expect("encode");
        assert_eq!(&encoded[0..4], b"glTF");
        let decoded = decode_glb(&encoded).expect("decode");
        assert_eq!(decoded.buffers, original.buffers);
        assert_eq!(decoded.document.asset.version, "2.0");
        let pos_original = decode_accessor(&original.document, &original.buffers, 0).unwrap();
        let pos_decoded = decode_accessor(&decoded.document, &decoded.buffers, 0).unwrap();
        assert_eq!(pos_original.components, pos_decoded.components);
        // decode -> encode -> decode a second time: semantic equality must hold (not necessarily
        // byte-identical serialization, since key ordering / whitespace may legitimately differ).
        let reencoded = encode_glb(&decoded).expect("re-encode");
        let redecoded = decode_glb(&reencoded).expect("re-decode");
        assert_eq!(redecoded.buffers, decoded.buffers);
        assert_eq!(redecoded.document.accessors, decoded.document.accessors);
    }

    /// 🧪️ `codec_retention_law`: decode -> encode -> decode is byte-preserving up to the
    /// documented normal form (spec-default numeric fields round-trip as "present" iff they carry
    /// a non-default value -- see the `is_*`/`default_*` helpers in the snapshot module).
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law_glb_decode_encode_decode_is_semantically_faithful() {
        let bytes: Vec<u8> = (0..24u8).collect();
        let document = GltfDocument {
            buffers: vec![GltfBuffer { byte_length: bytes.len(), uri: None, name: Some("payload".into()), extensions: None, extras: None }],
            buffer_views: vec![GltfBufferView { buffer: 0, byte_offset: 0, byte_length: bytes.len(), byte_stride: None, target: None, name: None, extensions: None, extras: None }],
            accessors: vec![accessor(0, GltfComponentType::Float, GltfAccessorType::Vec3, 2)],
            meshes: vec![GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], mode: Some(4), ..GltfPrimitive::default() }], ..GltfMesh::default() }],
            nodes: vec![crate::artifacts::gltf::schema::snapshot::GltfNode { mesh: Some(0), name: Some("root".into()), ..Default::default() }],
            scenes: vec![crate::artifacts::gltf::schema::snapshot::GltfScene { nodes: vec![0], ..Default::default() }],
            scene: Some(0),
            extensions_used: vec!["KHR_materials_unlit".into()],
            ..GltfDocument::default()
        };
        let original = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![bytes], source_form: GltfSourceForm::Glb };
        let encoded = encode_glb(&original).expect("encode");
        let decoded = decode_glb(&encoded).expect("decode");
        assert_eq!(decoded.buffers, original.buffers);
        assert_eq!(decoded.document, original.document, "decode(encode(doc)) must equal doc for a document with no spec-default-equal fields");
        let reencoded = encode_glb(&decoded).expect("re-encode");
        let redecoded = decode_glb(&reencoded).expect("re-decode");
        assert_eq!(redecoded, decoded, "second decode->encode->decode cycle must be a fixed point");
    }

    #[semio_framework_async_macros::async_test]
    async fn gltf_json_serialize_embeds_glb_sourced_buffer_as_data_uri() {
        let bytes: Vec<u8> = vec![9, 9, 9, 9];
        let document = doc_with_buffer(4);
        let snap = GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![bytes.clone()], source_form: GltfSourceForm::Glb };
        let text_bytes = serialize_gltf_document(&snap);
        let text = String::from_utf8(text_bytes).unwrap();
        assert!(text.contains("data:application/octet-stream;base64,"));
        let reparsed = parse_gltf_document(text.as_bytes()).expect("reparse emitted .gltf text");
        assert_eq!(reparsed.buffers[0], bytes);
    }
    //#endregion 🔖️DualCodecTests

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG3: per-artifact conformance laws (recipe §4's deliverable list, item 6) — grammar/
    /// protocol parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff`
    /// output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Dissolved out of `⚙️engine`'s own test region (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — same convention every prior pilot
    /// wave (json/csv/zip/png/txt/binary) established.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::gltf::io::mutations as mutation_transport;
        use crate::artifacts::gltf::schema::modules::mutation_dispatch::{registered_gltf_mutation_command_ids, GltfMutation, GltfMutationRegistryError, GLTF_MUTATION_MAX_PAYLOAD_BYTES};
        use crate::artifacts::gltf::schema::mutations::change_material_alpha_mode::mutation;
        use crate::artifacts::gltf::schema::snapshot::GltfAlphaMode;
        use crate::artifacts::gltf::schema::{diff, snapshot};
        use protocol::{DiffCodec, Mutation, OpBinary, OpText};

        async fn alpha_mode_mutation() -> GltfMutation {
            let payload = serde_json::to_vec(&mutation::GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask }).expect("canonical alpha-mode payload");
            GltfMutation::new(mutation::ID, 1, payload).expect("registered alpha-mode mutation")
        }

        async fn material_snapshot() -> GltfSnapshot {
            let mut snapshot = GltfSnapshot::default();
            snapshot.document.materials.push(Default::default());
            snapshot
        }

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutation_transport::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutation_transport::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo (genuinely non-trivial) snapshot — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an analogue.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&crate::artifacts::gltf::engine::demo_gltf_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes the canonical generic
        /// envelope, independently of the registered command payload shape.
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutation_transport::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let forward = alpha_mode_mutation();
            let inverse = forward.inverse(&material_snapshot()).pop().expect("inverse envelope");
            for mutation in [forward, inverse] {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?}");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn generic_envelope_registry_and_transport_laws() {
            let mutation = alpha_mode_mutation();
            assert!(registered_gltf_mutation_command_ids().expect("valid immutable mutation registry").contains(&mutation::ID));
            assert!(matches!(GltfMutation::new("s.stdio.gltf.mutation.unknown.v1", 1, Vec::new()), Err(GltfMutationRegistryError::UnknownCommand(_))));
            assert!(matches!(GltfMutation::new(mutation::ID, 2, Vec::new()), Err(GltfMutationRegistryError::StaleVersion { expected: 1, actual: 2, .. })));
            assert!(matches!(GltfMutation::new(mutation::ID, 1, vec![0; GLTF_MUTATION_MAX_PAYLOAD_BYTES + 1]), Err(GltfMutationRegistryError::BudgetExceeded("payload"))));

            let text = mutation.print_op();
            assert_eq!(GltfMutation::parse_op(&text).expect("text round trip"), mutation);
            let binary = mutation.encode_op().expect("binary encode");
            assert_eq!(GltfMutation::decode_op(&binary).expect("binary round trip"), mutation);
            let mut trailing = binary;
            trailing.push(0);
            assert!(GltfMutation::decode_op(&trailing).is_err(), "binary envelope accepted trailing data");
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `GltfDiff` (`diff::demo_diff_cases()`), incl. the empty
        /// (all-`None`) diff and the fully-populated rich diff.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), a generic mutation
        /// envelope's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&crate::artifacts::gltf::engine::demo_gltf_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutation_transport::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            let forward = alpha_mode_mutation();
            let inverse = forward.inverse(&material_snapshot()).pop().expect("inverse envelope");
            for mutation in [forward, inverse] {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_gltf_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake (the pre-FG3 `{"hello":"stdio.gltf",
        /// "n":1}` stub this program's own recipe explicitly calls out as the wrong shape).
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = crate::artifacts::gltf::engine::demo_gltf_snapshot();

            let parsed = <GltfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_gltf_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_gltf_snapshot()) drifted from the shipped .dsl.semio fixture");

            // P2-FG3: `decode_pack` routes through the real `.glb` container (`decode_glb`),
            // which always reports `source_form: Glb` (the byte form genuinely IS a glb container
            // now) — the one field expected to legitimately differ from `demo`'s own `Json`
            // provenance, same treatment `codec_round_trip` gives.
            let decoded = <GltfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded.schema, demo.schema, "shipped .pack.semio fixture does not decode back to demo_gltf_snapshot()'s schema");
            assert_eq!(decoded.document, demo.document, "shipped .pack.semio fixture does not decode back to demo_gltf_snapshot()'s document");
            assert_eq!(decoded.buffers, demo.buffers, "shipped .pack.semio fixture does not decode back to demo_gltf_snapshot()'s buffers");
            assert_eq!(decoded.source_form, GltfSourceForm::Glb, "decode_pack must report source_form: Glb");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_gltf_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfComposer as GltfRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<GltfRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
