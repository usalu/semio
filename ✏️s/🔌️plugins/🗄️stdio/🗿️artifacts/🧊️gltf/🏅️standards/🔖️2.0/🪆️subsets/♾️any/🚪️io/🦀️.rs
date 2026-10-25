//! 🚪️ IO stdio.gltf (2.0/♾️any): the native text, binary and SQLite representations plus the format vocabulary the
//! byte reader (`import`) and writer (`export`) share — base64 `data:` URIs, typed accessor decode, the GLB chunk
//! constants and the accessor model's value/serde mapping. Registration flows through the `s.stdio.gltf`
//! `ArtifactDeclaration`.
//!
//! 🔤️ Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: `.gltf`/`.glb` bytes round-trip
//! through the first-party [`pack::json`] codec and [`dsl::ToValue`]/[`dsl::FromValue`], never `serde_json`. The
//! hand-rolled `Serialize`/`Deserialize` pairs in this file (and in the sibling `📸️snapshot` module) stay
//! unconditional: the `wasm32-wasip2` component build serializes a `GltfSnapshot` through them.
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType, GltfDecodedAccessor, GltfDocument};
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
/// 🌐️ Decodes a `data:<mediatype>;base64,<payload>` URI (buffer.uri/image.uri). Any media pub type
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
/// 🌐️ Encodes `bytes` as a `data:<media_pub type>;base64,<payload>` URI.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_data_uri(media_type: &str, bytes: &[u8]) -> String {
    format!("data:{media_type};base64,{}", b64_encode(bytes))
}
//#endregion 🔖️Base64
//#region 🔖️AccessorModel
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
/// [`GltfSnapshot`](crate::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot)'s doc comment).
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
pub fn normalize_components(component_type: GltfComponentType, components: &mut [f64]) -> Result<(), String> {
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
pub fn validate_document(document: &GltfDocument) -> Result<(), String> {
    if document.asset.version.trim().is_empty() {
        return Err("gltf document 'asset.version' missing or not a string".into());
    }
    Ok(())
}
//#endregion 🔖️DocumentCodec
//#region 🔖️GlbContainer
pub const GLB_MAGIC: &[u8; 4] = b"glTF";
pub const GLB_VERSION: u32 = 2;
pub const CHUNK_TYPE_JSON: &[u8; 4] = b"JSON";
pub const CHUNK_TYPE_BIN: &[u8; 4] = b"BIN\0";
//#endregion 🔖️GlbContainer
//#region 🪢️TaxonomyMounts




//#endregion 🪢️TaxonomyMounts
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
pub mod text {
    pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::diff;
}

