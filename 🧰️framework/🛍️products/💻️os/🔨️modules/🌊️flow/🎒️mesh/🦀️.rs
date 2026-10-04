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
pub const MESH_PACK_FIELD_METADATA: u16 = 13;

/// 🎒️ The `RecordSpec` both `pack::encode_record_body` and `pack::decode_record_body_exact` drive
/// the mesh payload through — container-less, no chunk table, no manifest.
pub fn mesh_pack_spec() -> semio_framework_dsl_record::RecordSpec {
    use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::Shape;
    let bytes = |id: u16, key: &str| semio_framework_dsl_record::FieldSpec::new(id, key, semio_framework_dsl_record::Shape::Bytes64).optional();
    semio_framework_dsl_record::RecordSpec::new(
        Some("mesh"),
        semio_framework_dsl_record::RecordLayout::Lines,
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
            bytes(MESH_PACK_FIELD_METADATA, "metadata"),
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
    use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordValue;
    if mesh.paint_texture_base64.is_none() {let mut job=MeshPackEncodingJob::new(mesh.clone(),usize::MAX)?;loop {if let Some(bytes)=job.step(4096)? {return Ok(bytes);}}}
    let mut record = semio_framework_dsl_record::RecordValue::default();
    let mut put = |id: u16, bytes: Vec<u8>| {
        if !bytes.is_empty() {
            record.fields.insert(id, semio_framework_dsl_record::FieldValue::Bytes64(bytes));
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
        record.fields.insert(MESH_PACK_FIELD_PAINT_TEXTURE, semio_framework_dsl_record::FieldValue::Text(texture.clone()));
    }
    if !mesh.attributes.is_empty() || !mesh.materials.is_empty() || !mesh.textures.is_empty() || !mesh.component_references.is_empty() {
        let mut cursor=semio_framework::MeshMetadataCursor::default();let mut text=String::from("{");
        while !cursor.step(&mesh.attributes,&mesh.materials,&mesh.textures,Some(&mesh.component_references),None,None,&mut text)? {}text.push('}');
        record.fields.insert(MESH_PACK_FIELD_METADATA,semio_framework_dsl_record::FieldValue::Bytes64(text.into_bytes()));
    }
    crate::os_pack::encode_record_body(&mesh_pack_spec(), &record, &crate::os_pack::EncodeOptions::default()).map_err(|error| error.to_string())
}

/// ⏳️ Retained numeric preview record encoder using the mesh record field identities.
pub struct MeshPackEncodingJob {
    mesh: semio_framework::MeshData,
    field: u16,
    cursor: usize,
    bytes: Option<Vec<u8>>,
    cancelled: bool,
    metadata_cursor:semio_framework::MeshMetadataCursor,
    metadata_text:String,
    metadata_ready:bool,
    maximum_bytes:usize,
}

impl MeshPackEncodingJob {
    pub fn new(mesh: semio_framework::MeshData, maximum_bytes: usize) -> Result<Self, String> {
        if mesh.paint_texture_base64.is_some() { return Err("retained numeric mesh preview cannot encode a paint texture".into()); }
        let mut job = Self { mesh, field: 1, cursor: 0, bytes: Some(Vec::new()), cancelled: false,metadata_cursor:Default::default(),metadata_text:String::from("{"),metadata_ready:false,maximum_bytes };
        let mut count = 0; let mut size = 2usize;
        for field in 1..=11 {
            let len = job.field_len(field);
            if len > 0 { count += 1; size = size.checked_add(len).and_then(|size| size.checked_add(12)).ok_or("mesh preview capacity overflow")?; }
        }
        if !job.mesh.attributes.is_empty() || !job.mesh.materials.is_empty() || !job.mesh.textures.is_empty() || !job.mesh.component_references.is_empty() {count+=1;size=size.checked_add(12).ok_or("mesh preview capacity overflow")?;}
        if size > maximum_bytes { return Err(format!("mesh preview exceeds {maximum_bytes} bytes")); }
        job.bytes.as_mut().unwrap().extend_from_slice(&[0, count]);
        Ok(job)
    }
    fn field_len(&self, field: u16) -> usize {
        match field {
            1 => self.mesh.positions.len()*4, 2 => self.mesh.normals.len()*4, 3 => self.mesh.indices.len()*4,
            4 => self.mesh.colors.len()*4, 5 => self.mesh.uvs.len()*4, 6 => self.mesh.face_ids.len()*4,
            7 => self.mesh.vertex_ids.len()*4, 8 => self.mesh.edge_positions.len()*4, 9 => self.mesh.edge_ids.len()*4,
            10 => self.mesh.edge_uvs.len()*4, 11 => self.mesh.edge_is_seam.len(), _ => 0,
        }
    }
    pub fn cancel(&mut self) { self.cancelled = true; self.bytes = None; }
    /// 📏️ One unit emits a field header or at most 1024 little-endian payload bytes.
    pub fn step(&mut self, budget: usize) -> Result<Option<Vec<u8>>, String> {
        if self.cancelled { return Err("mesh encoding cancelled".into()); }
        if self.bytes.is_none() { return Err("mesh encoding job is retired".into()); }
        for _ in 0..budget {
            if self.field>11 {
                if self.mesh.attributes.is_empty() && self.mesh.materials.is_empty() && self.mesh.textures.is_empty() && self.mesh.component_references.is_empty() {return Ok(self.bytes.take());}
                if !self.metadata_ready {
                    if self.metadata_cursor.step(&self.mesh.attributes,&self.mesh.materials,&self.mesh.textures,Some(&self.mesh.component_references),None,None,&mut self.metadata_text)? {self.metadata_text.push('}');self.metadata_ready=true;self.cursor=0;}
                    if self.metadata_text.len()>16_000_000 || self.bytes.as_ref().unwrap().len().saturating_add(self.metadata_text.len()).saturating_add(12)>self.maximum_bytes {return Err(format!("mesh metadata preview exceeds {} bytes",self.maximum_bytes));}
                    continue;
                }
                let out=self.bytes.as_mut().unwrap();let text=self.metadata_text.as_bytes();
                if self.cursor==0 {crate::os_pack::write_varint_u64(out,MESH_PACK_FIELD_METADATA as u64);out.push(0x08);crate::os_pack::write_varint_u64(out,text.len() as u64);}
                let end=(self.cursor+1024).min(text.len());out.extend_from_slice(&text[self.cursor..end]);self.cursor=end;
                if end==text.len() {return Ok(self.bytes.take());}
                continue;
            }
            let len = self.field_len(self.field);
            if len == 0 { self.field += 1; continue; }
            let out = self.bytes.as_mut().unwrap();
            if self.cursor == 0 {
                crate::os_pack::write_varint_u64(out,self.field as u64); out.push(0x08); crate::os_pack::write_varint_u64(out,len as u64);
            }
            let width = if self.field == 11 { 1 } else { 4 };
            let end = (self.cursor + 1024/width).min(len/width);
            for index in self.cursor..end {
                let bytes = match self.field {
                    1 => self.mesh.positions[index].to_le_bytes(), 2 => self.mesh.normals[index].to_le_bytes(),
                    3 => self.mesh.indices[index].to_le_bytes(), 4 => self.mesh.colors[index].to_le_bytes(),
                    5 => self.mesh.uvs[index].to_le_bytes(), 6 => self.mesh.face_ids[index].to_le_bytes(),
                    7 => self.mesh.vertex_ids[index].to_le_bytes(), 8 => self.mesh.edge_positions[index].to_le_bytes(),
                    9 => self.mesh.edge_ids[index].to_le_bytes(), 10 => self.mesh.edge_uvs[index].to_le_bytes(),
                    11 => [self.mesh.edge_is_seam[index],0,0,0], _ => unreachable!(),
                };
                out.extend_from_slice(&bytes[..width]);
            }
            self.cursor = end;
            if end*width == len { self.field += 1; self.cursor = 0; }
        }
        if budget>0 && self.field>11 && self.mesh.attributes.is_empty() && self.mesh.materials.is_empty() && self.mesh.textures.is_empty() && self.mesh.component_references.is_empty() {Ok(self.bytes.take())}else {Ok(None)}
    }
}

/// 🎒️ Decodes an [`encode_mesh_pack`] body back into `MeshData` — the exact inverse, byte-for-byte.
pub fn decode_mesh_pack(bytes: &[u8]) -> Result<semio_framework::MeshData, String> {
    use semio_framework_dsl_record::FieldValue;
    let record = crate::os_pack::decode_record_body_exact(bytes, &mesh_pack_spec(), &crate::os_pack::DecodeOptions::default()).map_err(|error| error.to_string())?;
    let blob = |id: u16| match record.get(id) {
        Some(semio_framework_dsl_record::FieldValue::Bytes64(bytes)) => bytes.as_slice(),
        _ => &[][..],
    };
    let metadata=blob(MESH_PACK_FIELD_METADATA);
    let (attributes,materials,textures,component_references)=if metadata.is_empty() {(Default::default(),Default::default(),Default::default(),Default::default())}else {
        use semio_framework_value::FromValue;
        if metadata.len()>16_000_000 {return Err("mesh metadata exceeds 16 MB".into());}
        let value=semio_framework_pack_json::parse(core::str::from_utf8(metadata).map_err(|error|error.to_string())?,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())?;
        if value.as_object().is_none_or(|object|object.iter().any(|(name,_)|!["attributes","materials","textures","componentReferences"].contains(&name))) {return Err("unknown mesh metadata field".into());}
        let decode=|name|semio_framework_pack_json::to_dsl_value(&value[name]);
        (if value["attributes"].is_null() {Default::default()}else {std::collections::BTreeMap::from_value(decode("attributes")).map_err(|error|error.into_message())?},
         if value["materials"].is_null() {Default::default()}else {std::collections::BTreeMap::from_value(decode("materials")).map_err(|error|error.into_message())?},
         if value["textures"].is_null() {Default::default()}else {std::collections::BTreeMap::from_value(decode("textures")).map_err(|error|error.into_message())?},
         if value["componentReferences"].is_null() {Default::default()}else {std::collections::BTreeMap::from_value(decode("componentReferences")).map_err(|error|error.into_message())?})
    };
    let mesh=semio_framework::MeshData {
        attributes,materials,textures,component_references,
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
            Some(semio_framework_dsl_record::FieldValue::Text(text)) => Some(text.clone()),
            _ => None,
        },
        ..Default::default()
    };
    mesh.validate_component_references()?;
    Ok(mesh)
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
    use semio_framework_pack_json::{object, Value};
    semio_framework_pack_json::to_string(&object([
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

impl semio_framework_value::retirement::RetireOwned for MeshPackEncodingJob {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::artifact_retirement_sequence![self.mesh,self.bytes,self.metadata_cursor,self.metadata_text]}
}
