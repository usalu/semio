//! 📥️ glTF 2.0 byte reader of `stdio.gltf`: `.gltf` JSON text and the `.glb` binary container parsed into the typed
//! snapshot through the first-party [`pack::json`] codec, with every `data:` buffer resolved. Depends on the model only.

use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfDocument, GltfSourceForm};
use crate::standards::v2_0::subsets::any::schema::snapshot_root::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::snapshot_root::STDIO_GLTF_DOCUMENT_SCHEMA;
use crate::standards::v2_0::subsets::any::io::{CHUNK_TYPE_BIN, CHUNK_TYPE_JSON, GLB_MAGIC, GLB_VERSION, decode_data_uri, validate_document};

//#region 🔖️DocumentCodec
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
//#endregion 🔖️DocumentCodec
//#region 🔖️GlbContainer
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
