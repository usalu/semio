//! 🎒️ Neutral preview mesh record codec and bounded transport envelope.
use base64::Engine;
use neural_engine::EvalError;
// #region 🎒️MeshPackCodec

/// 🎒️ Field ids of the mesh record body — stable wire identity shared by the Rust encoder, the
/// TypeScript decoder (`decodeMeshPackBody` in `🧰️framework/🛍️products/💻️os/🟦️.ts`) and every
/// fixture test. Every array rides as one `Shape::Bytes64` little-endian blob rather than a JSON
/// number array, which is what makes the payload a typed array on arrival instead of a parse.
pub const MESH_PACK_FIELD_POSITIONS: u16 = 1;
pub const MESH_PACK_FIELD_NORMALS: u16 = 2;
pub const MESH_PACK_FIELD_INDICES: u16 = 3;
pub const MESH_PACK_FIELD_COLORS: u16 = 4;
pub const MESH_PACK_FIELD_UVS: u16 = 5;
pub const MESH_PACK_FIELD_FACE_IDS: u16 = 6;
pub const MESH_PACK_FIELD_VERTEX_IDS: u16 = 7;
pub const MESH_PACK_FIELD_EDGE_POSITIONS: u16 = 8;
pub const MESH_PACK_FIELD_EDGE_IDS: u16 = 9;
pub const MESH_PACK_FIELD_EDGE_UVS: u16 = 10;
pub const MESH_PACK_FIELD_EDGE_IS_SEAM: u16 = 11;
pub const MESH_PACK_FIELD_PAINT_TEXTURE: u16 = 12;

/// 🎒️ The `RecordSpec` both `pack::encode_record_body` and `pack::decode_record_body_exact` drive
/// the mesh payload through — container-less, no chunk table, no manifest.
pub fn mesh_pack_spec() -> crate::os_dsl::schema::RecordSpec {
    use crate::os_dsl::schema::{FieldSpec, RecordLayout, RecordSpec, Shape};
    let bytes = |id: u16, key: &str| FieldSpec::new(id, key, Shape::Bytes64).optional();
    RecordSpec::new(
        Some("mesh"),
        RecordLayout::Lines,
        vec![
            bytes(MESH_PACK_FIELD_POSITIONS, "positions"),
            bytes(MESH_PACK_FIELD_NORMALS, "normals"),
            bytes(MESH_PACK_FIELD_INDICES, "indices"),
            bytes(MESH_PACK_FIELD_COLORS, "colors"),
            bytes(MESH_PACK_FIELD_UVS, "uvs"),
            bytes(MESH_PACK_FIELD_FACE_IDS, "faceIds"),
            bytes(MESH_PACK_FIELD_VERTEX_IDS, "vertexIds"),
            bytes(MESH_PACK_FIELD_EDGE_POSITIONS, "edgePositions"),
            bytes(MESH_PACK_FIELD_EDGE_IDS, "edgeIds"),
            bytes(MESH_PACK_FIELD_EDGE_UVS, "edgeUvs"),
            bytes(MESH_PACK_FIELD_EDGE_IS_SEAM, "edgeIsSeam"),
            FieldSpec::new(MESH_PACK_FIELD_PAINT_TEXTURE, "paintTextureBase64", Shape::Text).optional(),
        ],
    )
}

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync encode/decode call sites — see R9
fn f32_blob(values: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 4);
    for value in values {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync encode/decode call sites — see R9
fn u32_blob(values: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 4);
    for value in values {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync encode/decode call sites — see R9
fn f32_from_blob(bytes: &[u8]) -> Result<Vec<f32>, String> {
    if bytes.len() % 4 != 0 {
        return Err("mesh pack f32 blob length is not a multiple of 4".to_string());
    }
    Ok(bytes.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
}

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync encode/decode call sites — see R9
fn u32_from_blob(bytes: &[u8]) -> Result<Vec<u32>, String> {
    if bytes.len() % 4 != 0 {
        return Err("mesh pack u32 blob length is not a multiple of 4".to_string());
    }
    Ok(bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
}

/// 🎒️ Encodes `mesh` as one `pack` record body — the compact binary form the preview mesh crosses
/// the host↔extension boundary in, replacing the JSON number-array string.
pub fn encode_mesh_pack(mesh: &semio_framework::MeshData) -> Result<Vec<u8>, String> {
    use crate::os_dsl::schema::{FieldValue, RecordValue};
    let mut record = RecordValue::default();
    let mut put = |id: u16, bytes: Vec<u8>| {
        if !bytes.is_empty() {
            record.fields.insert(id, FieldValue::Bytes64(bytes));
        }
    };
    put(MESH_PACK_FIELD_POSITIONS, f32_blob(&mesh.positions));
    put(MESH_PACK_FIELD_NORMALS, f32_blob(&mesh.normals));
    put(MESH_PACK_FIELD_INDICES, u32_blob(&mesh.indices));
    put(MESH_PACK_FIELD_COLORS, f32_blob(&mesh.colors));
    put(MESH_PACK_FIELD_UVS, f32_blob(&mesh.uvs));
    put(MESH_PACK_FIELD_FACE_IDS, u32_blob(&mesh.face_ids));
    put(MESH_PACK_FIELD_VERTEX_IDS, u32_blob(&mesh.vertex_ids));
    put(MESH_PACK_FIELD_EDGE_POSITIONS, f32_blob(&mesh.edge_positions));
    put(MESH_PACK_FIELD_EDGE_IDS, u32_blob(&mesh.edge_ids));
    put(MESH_PACK_FIELD_EDGE_UVS, f32_blob(&mesh.edge_uvs));
    put(MESH_PACK_FIELD_EDGE_IS_SEAM, mesh.edge_is_seam.clone());
    if let Some(texture) = mesh.paint_texture_base64.as_ref() {
        record.fields.insert(MESH_PACK_FIELD_PAINT_TEXTURE, FieldValue::Text(texture.clone()));
    }
    crate::os_pack::encode_record_body(&mesh_pack_spec(), &record, &crate::os_pack::EncodeOptions::default()).map_err(|error| error.to_string())
}

/// 🎒️ Decodes an [`encode_mesh_pack`] body back into `MeshData` — the exact inverse, byte-for-byte.
pub fn decode_mesh_pack(bytes: &[u8]) -> Result<semio_framework::MeshData, String> {
    use crate::os_dsl::schema::FieldValue;
    let record = crate::os_pack::decode_record_body_exact(bytes, &mesh_pack_spec(), &crate::os_pack::DecodeOptions::default()).map_err(|error| error.to_string())?;
    let blob = |id: u16| match record.get(id) {
        Some(FieldValue::Bytes64(bytes)) => bytes.as_slice(),
        _ => &[][..],
    };
    Ok(semio_framework::MeshData {
        positions: f32_from_blob(blob(MESH_PACK_FIELD_POSITIONS))?,
        normals: f32_from_blob(blob(MESH_PACK_FIELD_NORMALS))?,
        colors: f32_from_blob(blob(MESH_PACK_FIELD_COLORS))?,
        indices: u32_from_blob(blob(MESH_PACK_FIELD_INDICES))?,
        uvs: f32_from_blob(blob(MESH_PACK_FIELD_UVS))?,
        face_ids: u32_from_blob(blob(MESH_PACK_FIELD_FACE_IDS))?,
        vertex_ids: u32_from_blob(blob(MESH_PACK_FIELD_VERTEX_IDS))?,
        edge_positions: f32_from_blob(blob(MESH_PACK_FIELD_EDGE_POSITIONS))?,
        edge_ids: u32_from_blob(blob(MESH_PACK_FIELD_EDGE_IDS))?,
        edge_uvs: f32_from_blob(blob(MESH_PACK_FIELD_EDGE_UVS))?,
        edge_is_seam: blob(MESH_PACK_FIELD_EDGE_IS_SEAM).to_vec(),
        paint_texture_base64: match record.get(MESH_PACK_FIELD_PAINT_TEXTURE) {
            Some(FieldValue::Text(text)) => Some(text.clone()),
            _ => None,
        },
    })
}

/// 🧱️ Base64 characters per continuation chunk. One `flowTessellateResolve` dispatch carries at most
/// this much of the mesh body, so a dense mesh streams across turns instead of blowing one turn's
/// intake budget on a single oversized continuation.
///
/// 📏️ Sized as one guest-contiguous page minus the envelope header: the answer is handed to the
/// consuming guest through pages of `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` (64 KiB) and the
/// PRODUCING guest must itself hold the envelope as one contiguous `String`, which dlmalloc never
/// returns to the OS — so the whole envelope stays inside one page.
///
/// ⚖️ It is NOT 4 KiB. That value was derived from the 8 KiB gesture quota the response action used
/// to share with 28 unrelated interactive routes, and it made the transfer unit twelve times
/// smaller than the wire can carry: a 38 584-character `sphere-cut-with-torus` body took TEN
/// `flowEvalTick` round trips — seconds apiece in a served build — so the preview never painted
/// inside any patience window (`📓️preview-mesh-delivery-2026-09-12.md`). The chain now owns its own
/// route (`GENERATION3D_FLOW_EVAL_RAW_BYTES` / `GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES`), whose bound
/// is pinned to [`tessellate_envelope_maximum_bytes`] by both surfaces' own tests, so the transfer
/// unit and the bound can never drift apart (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const MESH_PACK_CHUNK_BASE64_CHARS: usize = 48 * 1024;

/// ⏱️ Wall-clock microseconds one `tessellate` round trip spends stepping before it answers with
/// whatever progress it reached. The unit budget alone is NOT a time bound — a single face of a
/// boolean-cut sphere costs seconds while a box edge costs 29 µs — so a pure unit budget both
/// over-runs (one 5.9 s step) and under-runs (five consecutive µs-long steps, each paying a whole
/// interactive round trip). The unit budget stays the CANCELLATION granularity; this is the
/// interactivity bound (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const TESSELLATE_STEP_WALL_MICROS: u64 = 6_000;

/// 📏️ Largest `tessellate` step envelope [`tessellate_step_envelope_json`] can emit: one full
/// [`MESH_PACK_CHUNK_BASE64_CHARS`] chunk plus the widest progress/accounting header. Every consumer
/// that declares a wire bound for its `flowTessellateResolve`-shaped response action asserts against
/// this, so the transfer unit and the bound can never drift apart.
pub fn tessellate_envelope_maximum_bytes() -> usize {
    use crate::os_pack::json::{object, Value};
    crate::os_pack::json::to_string(&object([
        ("done".to_string(), Value::Bool(true)),
        ("cancellable".to_string(), Value::Bool(false)),
        ("phase".to_string(), Value::String("complete".to_string())),
        ("unitsDone".to_string(), Value::from(u64::MAX)),
        ("unitsTotal".to_string(), Value::from(u64::MAX)),
        ("facesDone".to_string(), Value::from(u64::MAX)),
        ("facesTotal".to_string(), Value::from(u64::MAX)),
        ("chunk".to_string(), Value::from(u64::MAX)),
        ("chunks".to_string(), Value::from(u64::MAX)),
        ("packBytes".to_string(), Value::from(u64::MAX)),
        ("meshPack".to_string(), Value::String("A".repeat(MESH_PACK_CHUNK_BASE64_CHARS))),
    ]))
    .len()
}

/// 🧱️ Splits a base64 mesh body into intake-sized chunks (never an empty vector — an empty mesh
/// still transfers as exactly one empty chunk so the receiver's chunk accounting is uniform).
pub fn chunk_mesh_base64(base64: &str) -> Vec<String> {
    if base64.is_empty() {
        return vec![String::new()];
    }
    base64.as_bytes().chunks(MESH_PACK_CHUNK_BASE64_CHARS).map(|chunk| String::from_utf8_lossy(chunk).into_owned()).collect()
}

// #endregion 🎒️MeshPackCodec
pub fn decode_base64(text: &str) -> Result<Vec<u8>, EvalError> {
    base64::engine::general_purpose::STANDARD.decode(text.trim()).map_err(|error| EvalError::InvalidInput(format!("invalid base64: {error}")))
}
pub fn encode_base64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}
