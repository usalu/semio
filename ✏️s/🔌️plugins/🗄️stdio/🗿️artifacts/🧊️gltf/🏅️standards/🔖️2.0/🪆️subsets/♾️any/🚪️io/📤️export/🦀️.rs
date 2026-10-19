//! 📤️ glTF 2.0 byte writer of `stdio.gltf`: the typed snapshot serialized to `.gltf` JSON text (buffers inlined as
//! `data:` URIs) and to the `.glb` binary container. Depends on the model only; its round-trip laws read back through
//! the sibling `import` module.

use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
#[cfg(test)]
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessor, GltfAccessorType, GltfBuffer, GltfBufferView, GltfComponentType, GltfDocument, GltfJson, GltfMesh, GltfPrimitive, GltfSourceForm, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues};
#[cfg(test)]
use crate::standards::v2_0::subsets::any::schema::snapshot_root::STDIO_GLTF_DOCUMENT_SCHEMA;
#[cfg(test)]
use crate::standards::v2_0::subsets::any::io::{b64_decode, decode_accessor, decode_data_uri, normalize_components};
#[cfg(test)]
use crate::standards::v2_0::subsets::any::io::import::{decode_glb, parse_gltf_document};
use crate::standards::v2_0::subsets::any::io::{CHUNK_TYPE_BIN, CHUNK_TYPE_JSON, GLB_MAGIC, GLB_VERSION, b64_encode, encode_data_uri, validate_document};

//#region 🔖️DocumentCodec
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
    out.extend_from_slice(&[0u8; 4]);
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
//#endregion 🔖️GlbContainer
//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
