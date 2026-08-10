//! 🧬️ StepSnapshot schema — persistent fields + real codecs.

use crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️BrepModel
/// 📍 B-rep vertex.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔺 B-rep face as polygon vertex indices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepFace {
    #[serde(default)]
    pub indices: Vec<usize>,
}

/// 📐 Neutral B-rep mesh extracted from CAD text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepMesh {
    #[serde(default)]
    pub vertices: Vec<BrepVertex>,
    #[serde(default)]
    pub faces: Vec<BrepFace>,
}
//#endregion 🔖️BrepModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.step` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub brep: BrepMesh,
}

impl Default for StepSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
            brep: BrepMesh::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️CadTextCodec

fn step_collect_entities(text: &str) -> std::collections::HashMap<u64, String> {
    let mut map = std::collections::HashMap::new();
    let mut cur_id: Option<u64> = None;
    let mut cur_body = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('#') {
            if let Some((id_s, after)) = rest.split_once('=') {
                if let Ok(id) = id_s.trim().parse::<u64>() {
                    if let Some(prev) = cur_id.take() {
                        map.insert(prev, cur_body.trim().trim_end_matches(';').to_string());
                    }
                    cur_id = Some(id);
                    cur_body = after.trim().to_string();
                    if cur_body.ends_with(';') {
                        map.insert(id, cur_body.trim_end_matches(';').to_string());
                        cur_id = None;
                        cur_body.clear();
                    }
                    continue;
                }
            }
        }
        if cur_id.is_some() {
            cur_body.push(' ');
            cur_body.push_str(line);
            if line.ends_with(';') {
                if let Some(id) = cur_id.take() {
                    map.insert(id, cur_body.trim().trim_end_matches(';').to_string());
                    cur_body.clear();
                }
            }
        }
    }
    if let Some(id) = cur_id {
        map.insert(id, cur_body.trim().trim_end_matches(';').to_string());
    }
    map
}

fn step_parse_cartesian(body: &str) -> Option<BrepVertex> {
    let open = body.find('(')?;
    let inner = &body[open + 1..];
    let close = inner.rfind(')')?;
    let tuple = inner[..close].trim();
    let nums: Vec<f64> = tuple
        .split(',')
        .filter_map(|p| p.trim().parse().ok())
        .collect();
    if nums.len() >= 3 {
        Some(BrepVertex { x: nums[0], y: nums[1], z: nums[2] })
    } else {
        None
    }
}

fn step_parse_poly_loop(body: &str, id_to_idx: &std::collections::HashMap<u64, usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for part in body.split('#') {
        let id_s: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(id) = id_s.parse::<u64>() {
            if let Some(&idx) = id_to_idx.get(&id) {
                out.push(idx);
            }
        }
    }
    out
}

pub fn step_brep_from_text(text: &str) -> Result<BrepMesh, String> {
    let entities = step_collect_entities(text);
    let mut id_to_idx = std::collections::HashMap::new();
    let mut mesh = BrepMesh::default();
    for (id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("CARTESIAN_POINT") {
            if let Some(v) = step_parse_cartesian(body) {
                id_to_idx.insert(*id, mesh.vertices.len());
                mesh.vertices.push(v);
            }
        }
    }
    for (_id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("POLY_LOOP") {
            let idx = step_parse_poly_loop(body, &id_to_idx);
            if idx.len() >= 3 {
                mesh.faces.push(BrepFace { indices: idx });
            }
        }
    }
    Ok(mesh)
}

pub fn step_brep_to_text(mesh: &BrepMesh) -> String {
    let mut out = String::from("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','',(''),(''),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n");
    let mut next_id = 1u64;
    let mut v_ids = Vec::new();
    for v in &mesh.vertices {
        out.push_str(&format!(
            "#{next_id}=CARTESIAN_POINT('',({},{},{}));\n",
            v.x, v.y, v.z
        ));
        v_ids.push(next_id);
        next_id += 1;
    }
    for face in &mesh.faces {
        let refs: Vec<String> = face.indices.iter().filter_map(|&i| v_ids.get(i).map(|id| format!("#{id}"))).collect();
        if refs.len() < 3 {
            continue;
        }
        let loop_id = next_id;
        next_id += 1;
        out.push_str(&format!("#{loop_id}=POLY_LOOP('',({}));\n", refs.join(",")));
        let face_id = next_id;
        next_id += 1;
        out.push_str(&format!("#{face_id}=FACE_OUTER_BOUND('',#{loop_id},.T.);\n"));
        let adv = next_id;
        next_id += 1;
        out.push_str(&format!("#{adv}=ADVANCED_FACE('',(#{face_id}),.F.);\n"));
    }
    out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    out
}


impl store::DocumentDsl for StepSnapshot {
    const EXTENSION: &'static str = "step";
    fn envelope_id() -> &'static str { "stdio.step" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let brep = step_brep_from_text(body).map_err(|e| {
            store::TextError::new(format!("step parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), brep })
    }
    fn print_dsl(&self) -> String {
        let body = step_brep_to_text(&self.brep);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for StepSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = step_brep_to_text(&self.brep).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let brep = step_brep_from_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), brep })
    }
}
//#endregion 🔖️CadTextCodec
