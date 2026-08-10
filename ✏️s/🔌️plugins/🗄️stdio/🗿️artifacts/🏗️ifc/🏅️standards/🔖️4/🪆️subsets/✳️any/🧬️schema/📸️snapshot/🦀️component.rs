//! 🧬️ IfcSnapshot schema — persistent fields + real codecs.

use crate::artifacts::ifc::STDIO_IFC_DOCUMENT_SCHEMA;
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
/// 📸️ Persisted `stdio.ifc` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc")]
pub struct IfcSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub brep: BrepMesh,
}

impl Default for IfcSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
            brep: BrepMesh::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️CadTextCodec

fn ifc_collect_entities(text: &str) -> std::collections::HashMap<u64, String> {
    let mut map = std::collections::HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with('#') {
            continue;
        }
        let rest = &line[1..];
        let Some((id_s, after)) = rest.split_once('=') else { continue };
        let Ok(id) = id_s.trim().parse::<u64>() else { continue };
        let body = after.trim().trim_end_matches(';').to_string();
        map.insert(id, body);
    }
    map
}

fn ifc_parse_point(body: &str) -> Option<BrepVertex> {
    let upper = body.to_ascii_uppercase();
    if !upper.starts_with("IFCCARTESIANPOINT") {
        return None;
    }
    let open = body.find('(')?;
    let inner = &body[open..];
    let nums: Vec<f64> = inner
        .split(|c: char| c == '(' || c == ')' || c == ',')
        .filter_map(|p| p.trim().parse().ok())
        .collect();
    if nums.len() >= 3 {
        Some(BrepVertex { x: nums[0], y: nums[1], z: nums[2] })
    } else {
        None
    }
}

fn ifc_refs(body: &str) -> Vec<u64> {
    let mut out = Vec::new();
    for part in body.split('#') {
        let id_s: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(id) = id_s.parse() {
            out.push(id);
        }
    }
    out
}

pub fn ifc_brep_from_text(text: &str) -> Result<BrepMesh, String> {
    let entities = ifc_collect_entities(text);
    let mut id_to_idx = std::collections::HashMap::new();
    let mut mesh = BrepMesh::default();
    for (id, body) in &entities {
        if let Some(v) = ifc_parse_point(body) {
            id_to_idx.insert(*id, mesh.vertices.len());
            mesh.vertices.push(v);
        }
    }
    for (_id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("IFCPOLYLOOP") {
            let mut idx = Vec::new();
            for rid in ifc_refs(body) {
                if let Some(&i) = id_to_idx.get(&rid) {
                    idx.push(i);
                }
            }
            if idx.len() >= 3 {
                mesh.faces.push(BrepFace { indices: idx });
            }
        }
    }
    for (_id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("IFCFACE") && mesh.faces.is_empty() {
            for rid in ifc_refs(body) {
                if let Some(bound) = entities.get(&rid) {
                    let bu = bound.to_ascii_uppercase();
                    if bu.starts_with("IFCFACEOUTERBOUND") {
                        for br in ifc_refs(bound) {
                            if let Some(loop_body) = entities.get(&br) {
                                if loop_body.to_ascii_uppercase().starts_with("IFCPOLYLOOP") {
                                    let mut idx = Vec::new();
                                    for pr in ifc_refs(loop_body) {
                                        if let Some(&i) = id_to_idx.get(&pr) {
                                            idx.push(i);
                                        }
                                    }
                                    if idx.len() >= 3 {
                                        mesh.faces.push(BrepFace { indices: idx });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(mesh)
}

pub fn ifc_brep_to_text(mesh: &BrepMesh) -> String {
    let mut out = String::from("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');\nFILE_NAME('semio.ifc','',(''),(''),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n");
    let mut next = 1u64;
    let mut pt_ids = Vec::new();
    for v in &mesh.vertices {
        out.push_str(&format!("#{next}=IFCCARTESIANPOINT(({},{},{}));\n", v.x, v.y, v.z));
        pt_ids.push(next);
        next += 1;
    }
    for face in &mesh.faces {
        let refs: Vec<String> = face.indices.iter().filter_map(|&i| pt_ids.get(i).map(|id| format!("#{id}"))).collect();
        if refs.len() < 3 {
            continue;
        }
        let loop_id = next;
        next += 1;
        out.push_str(&format!("#{loop_id}=IFCPOLYLOOP(({refs}));\n", refs = refs.join(",")));
        let bound_id = next;
        next += 1;
        out.push_str(&format!("#{bound_id}=IFCFACEOUTERBOUND(#{loop_id},.T.);\n"));
        let face_id = next;
        next += 1;
        out.push_str(&format!("#{face_id}=IFCFACE((#{bound_id}));\n"));
    }
    out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    out
}


impl store::DocumentDsl for IfcSnapshot {
    const EXTENSION: &'static str = "ifc";
    fn envelope_id() -> &'static str { "stdio.ifc" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let brep = ifc_brep_from_text(body).map_err(|e| {
            store::TextError::new(format!("ifc parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), brep })
    }
    fn print_dsl(&self) -> String {
        let body = ifc_brep_to_text(&self.brep);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for IfcSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = ifc_brep_to_text(&self.brep).into_bytes();
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
        let brep = ifc_brep_from_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), brep })
    }
}
//#endregion 🔖️CadTextCodec
