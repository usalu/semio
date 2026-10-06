//! 🚪️ IO stdio.gltf (2.0/♾️any) — registration now flows through the `s.stdio.gltf`
//! `ArtifactDeclaration` (`crate::declaration`), not per-leaf register().
//!
//! ⚙️ Owns the byte/container-level glTF 2.0 codecs (base64 data-uri, typed accessor decode,
//! `.gltf` JSON text, `.glb` binary container). Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-
//! REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 gltf/glb merge: the separate `🧊️glb` artifact_kind
//! (steps 1-2's transition compat shim) has been folded and deleted (steps 3-5) -- every former
//! glb caller now targets this codec's own `.glb` binary dialect directly, so there is no longer
//! a second container implementation to keep in sync.
//!
//! 🔤️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`:
//! [`parse_gltf_document`]/[`serialize_gltf_document`]/[`encode_glb`]/[`decode_glb`] now round-trip
//! real `.gltf`/`.glb` bytes through the first-party [`pack::json`] codec and
//! [`dsl::ToValue`]/[`dsl::FromValue`] (`GltfDocument` and everything it recursively contains),
//! never `serde_json`. The hand-rolled `Serialize`/`Deserialize` pairs still in this file (and in
//! the sibling `📸️snapshot` module) stay UNCONDITIONAL, additive alongside `ToValue`/`FromValue`
//! (not `#[cfg(test)]`): a production call site outside this module (the `wasm32-wasip2` component
//! build) still serializes a `GltfSnapshot` through them, so gating them broke that build — see
//! `GltfSnapshot`'s own doc comment in the sibling `📸️snapshot` module.
#[cfg(test)]
use crate::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView, GltfJson, GltfMesh, GltfPrimitive, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues};
use crate::schema::snapshot::{GltfDocument, GltfSourceForm};
use crate::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};

//#region 🔖️Base64
/// 🔤️ Standard base64 alphabet (RFC 4648 §4) — glTF `data:` URIs never use the URL-safe variant.
const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔓️ Decodes standard base64, tolerating embedded whitespace and `=` padding (real-world
/// `data:` URIs are sometimes line-wrapped by whatever authored them).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_data_uri(media_type: &str, bytes: &[u8]) -> String {
    format!("data:{media_type};base64,{}", b64_encode(bytes))
}
//#endregion 🔖️Base64

//#region 🔖️AccessorModel
/// 🔢️ `accessor.componentType` — the 6 values glTF 2.0 permits (§5.1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub enum GltfComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float,
}

impl GltfComponentType {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn byte_size(self) -> usize {
        match self {
            Self::Byte | Self::UnsignedByte => 1,
            Self::Short | Self::UnsignedShort => 2,
            Self::UnsignedInt | Self::Float => 4,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub enum GltfAccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl std::str::FromStr for GltfAccessorType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
}

impl GltfAccessorType {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

//#region 🔖️AccessorModelSerde
/// 🧵 `accessor.componentType` on the wire is always the raw numeric code (5120..5126) — never a
/// string -- so this hand-rolls `Serialize`/`Deserialize` around [`GltfComponentType::code`]/
/// [`GltfComponentType::from_code`] rather than deriving (a derive would emit the Rust variant
/// name, not a spec-legal wire value). Additive alongside the `ToValue`/`FromValue` pair below (the
/// REAL runtime mapping [`parse_gltf_document`]/[`serialize_gltf_document`]/[`encode_glb`]/
/// [`decode_glb`] use, via `pack::json`) — kept UNCONDITIONAL, not `#[cfg(test)]`, because the
/// sibling `📸️snapshot` module's own `GltfAccessor`/`GltfSparseIndices` etc. still derive real
/// `Serialize`/`Deserialize` for a production call site outside this file (see
/// [`GltfSnapshot`](crate::schema::snapshot::GltfSnapshot)'s doc comment).
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
/// for the same reason as [`GltfComponentType`]'s impl above -- additive, kept unconditional, same reason.
impl Serialize for GltfAccessorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GltfAccessorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(serde::de::Error::custom)
    }
}

/// 🧵 `ToValue`/`FromValue` mirrors of the two hand-rolled `serde` impls just above — this is now
/// the REAL runtime mapping: [`parse_gltf_document`]/[`serialize_gltf_document`]/
/// [`encode_glb`]/[`decode_glb`] below parse/serialize `GltfDocument` through `pack::json` +
/// `ToValue`/`FromValue`, never `serde_json`, and the sibling `📸️snapshot` module's own
/// `GltfSparseIndices`/`GltfAccessor` etc. need these two leaf types to implement `ToValue`/
/// `FromValue` too, for the SAME numeric-code / spec-string wire shape (never the bare Rust
/// variant name).
impl semio_framework_value::ToValue for GltfComponentType {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { semio_framework_value::ToValue::to_value_controlled(&self.code(), control) }
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.code())
    }
}
impl semio_framework_value::FromValue for GltfComponentType {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let code = <u64 as semio_framework_value::FromValue>::from_value(value)?;
        Self::from_code(code).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))
    }
}
impl semio_framework_value::ToValue for GltfAccessorType {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> { control.copy_text(self.as_str()).map(semio_framework_value::DslValue::String) }
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.as_str().to_string())
    }
}
impl semio_framework_value::FromValue for GltfAccessorType {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let s = <String as semio_framework_value::FromValue>::from_value(value)?;
        s.parse::<Self>().map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error))
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🎚️ Applies glTF 2.0 accessor normalization after dense and sparse values have been
/// assembled, preserving the exact signed lower-bound rule from §3.6.2.2.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize_components(component_type: GltfComponentType, components: &mut [f64]) -> Result<(), String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_accessor(document: &GltfDocument, buffers: &[Vec<u8>], accessor_index: usize) -> Result<GltfDecodedAccessor, String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bufferview_elements(document: &GltfDocument, buffers: &[Vec<u8>], bv_idx: usize, extra_offset: usize, component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Result<Vec<f64>, String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_document(document: &GltfDocument) -> Result<(), String> {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_document_buffers(document: &GltfDocument, embedded_bin: Option<&[u8]>) -> Vec<Vec<u8>> {
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

/// 🔤️ `serde_json::to_vec_pretty` analog over [`dsl::ToValue`] instead of `Serialize` -- layers
/// `pack`'s generic `DslValue`/`JsonValue` bridge with `pack::json_to_string_pretty`'s 2-space
/// indented layout, matching `serde_json::to_string_pretty`'s own (verified byte-identical for
/// floats, see the ticket's float-format-parity research note). `pack::to_json_string` (compact)
/// is used directly at other call sites in this file; this is the one spot needing the pretty
/// variant, so it stays local rather than growing `pack`'s own public surface.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn to_json_string_pretty<T: semio_framework_value::ToValue>(value: &T) -> String {
    semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value.to_value()))
}

/// 📥️ Parses `.gltf` JSON text bytes into a typed snapshot (lenient: no POSITION/mesh
/// precondition, only `asset.version`) via `pack::from_json_str::<GltfDocument>` -- every spec
/// top-level field lands in its typed slot; `extras`/`extensions` decode into this module's own
/// [`GltfJson`] (never `serde_json::Value`/`pack::JsonValue`), so nothing real on disk is dropped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_gltf_document(bytes: &[u8]) -> Result<GltfSnapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("gltf json is not valid utf-8: {e}"))?;
    let document: GltfDocument = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| format!("gltf json parse error: {e}"))?;
    validate_document(&document)?;
    let buffers = resolve_document_buffers(&document, None);
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Json })
}

/// 📤️ Serializes a snapshot to `.gltf` JSON text bytes via `pack::json`/[`to_json_string_pretty`]
/// (never `serde_json`). Any buffer that has no `uri` in `document` (i.e. sourced from a `.glb`
/// BIN chunk, `source_form == Glb`) is embedded as a `data:application/octet-stream;base64,…` uri
/// in the emitted copy -- plain `.gltf` JSON has no binary chunk to hold it, so this is the only
/// lossless way to carry those bytes through. Buffers that already declare a `uri` (data or
/// external) are left untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_gltf_document(snapshot: &GltfSnapshot) -> Vec<u8> {
    let mut document = snapshot.document.clone();
    for (i, buf) in document.buffers.iter_mut().enumerate() {
        if buf.uri.is_none() {
            if let Some(bytes) = snapshot.buffers.get(i) {
                buf.uri = Some(encode_data_uri("application/octet-stream", bytes));
            }
        }
    }
    to_json_string_pretty(&document).into_bytes()
}

/// 🚦️ Moves prepared document ownership through bounded data URIs and the existing controlled JSON writer.
pub fn serialize_gltf_document_owned_controlled(mut snapshot: GltfSnapshot, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Vec<u8>, semio_framework_value::ValueError> {
    for (index, buffer) in snapshot.document.buffers.iter_mut().enumerate() {
        control.checkpoint()?;
        if buffer.uri.is_none() {
            if let Some(bytes) = snapshot.buffers.get(index) { buffer.uri = Some(encode_data_uri_controlled("application/octet-stream", bytes, control)?); }
        }
    }
    let output = semio_framework_pack_json::to_json_string_controlled(&snapshot.document, control)?;
    control.checkpoint()?;
    Ok(output.into_bytes())
}

/// 🔤️ Admits one data URI and encodes at most4095 input bytes between checkpoints.
fn encode_data_uri_controlled(mime: &str, bytes: &[u8], control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, semio_framework_value::ValueError> {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    control.scoped_stage(|control| {
        let length = bytes.len().div_ceil(3).checked_mul(4).and_then(|length| length.checked_add(mime.len() + 13)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "glTF data URI size overflow"))?;
        control.begin_stage(bytes.len())?;
        control.charge(length)?;
        control.charge(bytes.len().min(4095).div_ceil(3) * 4)?;
        let mut output = String::new();
        output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "glTF data URI allocation failed"))?;
        output.push_str("data:"); output.push_str(mime); output.push_str(";base64,");
        for chunk in bytes.chunks(4095) { output.push_str(&b64_encode(chunk)); control.advance(chunk.len())?; }
        Ok(output)
    })
}
//#endregion 🔖️DocumentCodec

//#region 🔖️GlbContainer
const GLB_MAGIC: &[u8; 4] = b"glTF";
const GLB_VERSION: u32 = 2;
const CHUNK_TYPE_JSON: &[u8; 4] = b"JSON";
const CHUNK_TYPE_BIN: &[u8; 4] = b"BIN\0";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn align4(len: usize) -> usize {
    (len + 3) & !3
}

/// 📤️ Encodes a `.glb` binary container: 12-byte header (magic/version/total length) then a JSON
/// chunk (type `0x4E4F534A`, space-padded `0x20` to 4-byte alignment) and, when `buffers[0]` is
/// present and `document.buffers[0]` declares no `uri`, a BIN chunk (type `0x004E4942`,
/// zero-padded `0x00`). Fixes the prior bug: the total-length header field now includes BOTH
/// chunks' padding (it previously omitted the BIN chunk's padding bytes from the count).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_glb(snapshot: &GltfSnapshot) -> Result<Vec<u8>, String> {
    validate_document(&snapshot.document)?;
    let mut document = snapshot.document.clone();
    let embed_bin = document.buffers.first().is_some_and(|b| b.uri.is_none());
    let bin: Option<&[u8]> = if embed_bin { snapshot.buffers.first().map(|v| v.as_slice()) } else { None };

    // A buffer embedded via the BIN chunk must NOT carry a `byteLength` mismatch with what we're
    // actually about to embed -- keep it truthful if the caller mutated `buffers` without
    // updating `document.buffers[0].byteLength`.
    if let (true, Some(bin_bytes)) = (embed_bin, bin) {
        if let Some(buf0) = document.buffers.get_mut(0) {
            buf0.byte_length = bin_bytes.len();
        }
    }

    let json = semio_framework_pack_json::to_json_string(&document).into_bytes();
    let json_padded_len = align4(json.len());

    let mut out = Vec::new();
    out.extend_from_slice(GLB_MAGIC);
    out.extend_from_slice(&GLB_VERSION.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]); // total length patched below
    out.extend_from_slice(&(json_padded_len as u32).to_le_bytes());
    out.extend_from_slice(CHUNK_TYPE_JSON);
    out.extend_from_slice(&json);
    out.extend(std::iter::repeat_n(0x20u8, json_padded_len - json.len()));

    if let Some(bin_bytes) = bin {
        let bin_padded_len = align4(bin_bytes.len());
        out.extend_from_slice(&(bin_padded_len as u32).to_le_bytes());
        out.extend_from_slice(CHUNK_TYPE_BIN);
        out.extend_from_slice(bin_bytes);
        out.extend(std::iter::repeat_n(0x00u8, bin_padded_len - bin_bytes.len()));
    }

    let total = out.len() as u32;
    out[8..12].copy_from_slice(&total.to_le_bytes());
    Ok(out)
}

/// 📥️ Decodes a `.glb` binary container into a typed snapshot. Walks chunks by their declared
/// (already-padded) length rather than assuming exactly two chunks in a fixed order, per spec
/// (only JSON-first is mandated; BIN is optional and, if present, must be second -- any further
/// chunk types are simply skipped, not fabricated into anything).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    let document: GltfDocument = semio_framework_pack_json::from_json_str(json_text.trim_end_matches(['\0', ' ', '\t', '\n', '\r']), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| format!("glb: JSON chunk parse error: {e}"))?;
    validate_document(&document)?;

    // The BIN chunk's declared length is 4-byte-padded; the true buffer content length is
    // `document.buffers[0].byteLength` (padding is trailing filler, never real payload).
    let bin_content: Option<Vec<u8>> = bin_chunk.map(|chunk| {
        let declared_len = document.buffers.first().map_or(chunk.len(), |b| b.byte_length);
        chunk[..declared_len.min(chunk.len())].to_vec()
    });

    let buffers = resolve_document_buffers(&document, bin_content.as_deref());
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers, source_form: GltfSourceForm::Glb })
}
//#endregion 🔖️GlbContainer

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_0::subsets::any::io::GltfAnalyzer;
    use crate::GltfSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct GltfComposerComposition;

    impl ArtifactComposition for GltfComposerComposition {
        type Snapshot = GltfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v2_0::subsets::any::io::GltfComposer as GltfRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<GltfRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🪢️TaxonomyMounts
#[path = "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️.rs"]
pub mod serializers_artifacts_json;
#[path = "📤️export/🧵️serializers/🗿️artifacts/🦀️.rs"]
pub mod export_serializers_artifacts;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️.rs"]
pub mod deserializers_artifacts_json;
#[path = "📥️import/🧩️deserializers/🗿️artifacts/🦀️.rs"]
pub mod import_deserializers_artifacts;
//#endregion 🪢️TaxonomyMounts

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::engine::{GltfAccessorType, GltfComponentType};
    use crate::schema::mutations::GltfMutation;
    use crate::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView, GltfJson, GltfMaterial, GltfMesh, GltfNode, GltfPrimitive, GltfScene};
    use crate::{GltfDiff, GltfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.gltf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct GltfBuilderConstruction {
        snapshot: GltfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for GltfBuilderConstruction {
        type Snapshot = GltfSnapshot;
        type Mutation = GltfMutation;
        type Diff = GltfDiff;
        fn empty() -> Self {
            Self { snapshot: GltfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<GltfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GltfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = protocol::Mutation::diff(&mutation, &self.snapshot);
            if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
                self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.gltf.mutation-rejected", semio_framework_diagnostic::TextSpan::at(1, 1), format!("{:?}", outcome.messages())));
            }
            match <GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => {
                    self.snapshot = snapshot;
                }
                Err(error) => {
                    self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.gltf.mutation-rejected", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string()));
                }
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️AccessorSpec
    /// 📐️ Everything `add_accessor` needs beyond the buffer-view-and-offset plumbing common to every
    /// accessor. Built via `new` + chained `with_*` setters (values, not consuming-`Self` document
    /// mutation -- this is a plain value type, not the builder itself).
    #[derive(Clone, Debug)]
    pub struct GltfAccessorSpec {
        pub buffer_view: Option<usize>,
        pub byte_offset: usize,
        pub component_type: GltfComponentType,
        pub accessor_type: GltfAccessorType,
        pub count: usize,
        pub normalized: bool,
        pub min: Option<Vec<f64>>,
        pub max: Option<Vec<f64>>,
    }

    impl GltfAccessorSpec {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Self {
            Self { buffer_view: None, byte_offset: 0, component_type, accessor_type, count, normalized: false, min: None, max: None }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_buffer_view(mut self, buffer_view: usize, byte_offset: usize) -> Self {
            self.buffer_view = Some(buffer_view);
            self.byte_offset = byte_offset;
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_normalized(mut self, normalized: bool) -> Self {
            self.normalized = normalized;
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_min_max(mut self, min: Vec<f64>, max: Vec<f64>) -> Self {
            self.min = Some(min);
            self.max = Some(max);
            self
        }
    }
    //#endregion 🔖️AccessorSpec

    //#region 🔖️DocumentConstructors
    impl GltfBuilderConstruction {
        /// 🌱 Sets `asset.version` (the one glTF-mandatory field).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_asset_version(&mut self, version: &str) -> &mut Self {
            self.snapshot.document.asset.version = version.to_string();
            self
        }

        /// 📦️ Appends a buffer, storing its real bytes on the snapshot's `buffers` (index-aligned with
        /// `document.buffers`) and recording `byteLength` in the document. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_buffer(&mut self, bytes: Vec<u8>) -> usize {
            let byte_length = bytes.len();
            let idx = self.snapshot.buffers.len();
            self.snapshot.buffers.push(bytes);
            self.snapshot.document.buffers.push(GltfBuffer { byte_length, uri: None, name: None, extensions: None, extras: None });
            idx
        }

        /// 🪟️ Appends a `bufferView` (buffer index, byte offset/length, optional `byteStride` for
        /// interleaved data, optional `target` -- 34962 `ARRAY_BUFFER` / 34963 `ELEMENT_ARRAY_BUFFER`).
        /// Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_buffer_view(&mut self, buffer: usize, byte_offset: usize, byte_length: usize, byte_stride: Option<usize>, target: Option<u64>) -> usize {
            let idx = self.snapshot.document.buffer_views.len();
            self.snapshot.document.buffer_views.push(GltfBufferView { buffer, byte_offset, byte_length, byte_stride, target, name: None, extensions: None, extras: None });
            idx
        }

        /// 🔢️ Appends an `accessor` from a typed [`GltfAccessorSpec`]. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_accessor(&mut self, spec: GltfAccessorSpec) -> usize {
            let idx = self.snapshot.document.accessors.len();
            self.snapshot.document.accessors.push(GltfAccessor {
                buffer_view: spec.buffer_view,
                byte_offset: spec.byte_offset,
                component_type: spec.component_type,
                normalized: spec.normalized,
                count: spec.count,
                kind: spec.accessor_type,
                max: spec.max,
                min: spec.min,
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            });
            idx
        }

        /// 🎨️ Appends a fully typed `material`. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_material(&mut self, material: GltfMaterial) -> usize {
            let idx = self.snapshot.document.materials.len();
            self.snapshot.document.materials.push(material);
            idx
        }

        /// 🕸️ Appends an empty `mesh` (primitives added via [`Self::add_mesh_primitive`]). Returns the
        /// new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_mesh(&mut self) -> usize {
            let idx = self.snapshot.document.meshes.len();
            self.snapshot.document.meshes.push(GltfMesh::default());
            idx
        }

        /// 🔺️ Appends a primitive to `meshes[mesh]` -- `attributes` are `(semantic, accessor index)`
        /// pairs (e.g. `("POSITION", 0)`), `indices`/`material` are optional accessor/material
        /// indices, `mode` is the primitive topology (defaults to `4` TRIANGLES per spec when unset).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_mesh_primitive(&mut self, mesh: usize, attributes: &[(&str, usize)], indices: Option<usize>, material: Option<usize>, mode: Option<u64>) {
            let primitive = GltfPrimitive { attributes: attributes.iter().map(|(name, idx)| ((*name).to_string(), *idx)).collect(), indices, material, mode, targets: Vec::new(), extensions: None, extras: None };
            let mesh_entry = self.snapshot.document.meshes.get_mut(mesh).expect("mesh index out of range -- call add_mesh first");
            mesh_entry.primitives.push(primitive);
        }

        /// 🧍️ Appends a `node`, optionally referencing a mesh. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_node(&mut self, mesh: Option<usize>) -> usize {
            let idx = self.snapshot.document.nodes.len();
            self.snapshot.document.nodes.push(GltfNode { mesh, ..GltfNode::default() });
            idx
        }

        /// 🎬️ Appends a `scene` referencing `nodes` (root node indices), with an optional passthrough
        /// `extensions` object (real documents sometimes carry a declared-but-empty `{}` here).
        /// Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_scene(&mut self, nodes: Vec<usize>, extensions: Option<GltfJson>) -> usize {
            let idx = self.snapshot.document.scenes.len();
            self.snapshot.document.scenes.push(GltfScene { nodes, name: None, extensions, extras: None });
            idx
        }

        /// 🎬️ Sets the document's default `scene` index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_default_scene(&mut self, scene: usize) -> &mut Self {
            self.snapshot.document.scene = Some(scene);
            self
        }

        /// 🧩️ Sets `extensionsUsed` (declared, not necessarily applied -- mirrors real-world documents
        /// that declare an extension namespace without every element using it).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_extensions_used(&mut self, names: Vec<String>) -> &mut Self {
            self.snapshot.document.extensions_used = names;
            self
        }

        /// 📸️ Peeks the in-progress document -- used by tests/callers that need to inspect state
        /// mid-construction without consuming the builder via `build()`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn document(&self) -> &crate::schema::snapshot::GltfDocument {
            &self.snapshot.document
        }

        /// 📦️ Peeks the in-progress resolved buffer bytes.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn buffers(&self) -> &[Vec<u8>] {
            &self.snapshot.buffers
        }
    }
    //#endregion 🔖️DocumentConstructors

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::GltfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.gltf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct GltfParts {
        pub snapshot: Option<GltfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 👃️ `.glb` binary container magic: `glTF` + little-endian version `2`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_glb(bytes: &[u8]) -> bool {
        bytes.len() >= 12 && &bytes[0..4] == b"glTF" && u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) == 2
    }

    /// 👃️ `.gltf` JSON text: a JSON object whose top-level `asset` object carries a `version` string
    /// -- the one field glTF 2.0 §3.9 makes universally mandatory, so this is a real (if cheap) probe
    /// rather than a content-blind guess.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_gltf_json(text: &str) -> bool {
        let trimmed = text.trim_start();
        if !trimmed.starts_with('{') {
            return false;
        }
        match semio_framework_pack_json::parse(trimmed, semio_framework_pack_json::JsonMemberPolicy::Reject) {
            Ok(value) => value.get("asset").and_then(|a| a.get("version")).and_then(|v| v.as_str()).is_some(),
            Err(_) => false,
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.gltf` (2.0/♾️any) sources.
    pub struct GltfAnalyzerAnalysis;

    impl ArtifactAnalysis for GltfAnalyzerAnalysis {
        type Parts = GltfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if looks_like_glb(bytes) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if looks_like_gltf_json(text) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Medium
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GltfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => {
                        // A genuine `.gltf` JSON document parses directly through the real codec; only
                        // fall back to the SemioEnvelope-wrapped `ArtifactDsl` preamble form (used by
                        // this crate's own internal store round-trips) when the text isn't bare JSON.
                        let result = if looks_like_gltf_json(text) { crate::engine::parse_gltf_document(text.trim().as_bytes()) } else { <GltfSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string()) };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                    AnalyzeSource::Binary(bytes) => {
                        // A genuine raw `.glb` container decodes directly through the real codec; only
                        // fall back to the SemioEnvelope-wrapped `ArtifactPack` form (this crate's own
                        // internal store round-trip encoding) when the bytes aren't a `.glb` container.
                        let result = if looks_like_glb(bytes) { crate::engine::decode_glb(bytes) } else { <GltfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string()) };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec GltfBuilderFacets {
        construction: GltfBuilderConstruction,
        analysis: GltfAnalyzerAnalysis,
        composition: crate::standards::v2_0::subsets::any::io::derived_composition::GltfComposerComposition,
    }
    builder: GltfBuilder,
    analyzer: GltfAnalyzer,
    composer: GltfComposer,
);
