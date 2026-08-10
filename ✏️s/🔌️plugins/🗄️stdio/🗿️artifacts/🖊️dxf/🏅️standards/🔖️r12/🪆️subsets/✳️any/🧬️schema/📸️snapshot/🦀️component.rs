//! 🧬️ DxfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DrawingModel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxfLine {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
}
//#endregion 🔖️DrawingModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub lines: Vec<DxfLine>,
}

impl Default for DxfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️FormatCodec
pub fn parse_dxf_text(text: &str) -> Result<Vec<DxfLine>, String> {
    let mut lines_out = Vec::new();
    let codes: Vec<&str> = text.lines().collect();
    let mut i = 0usize;
    while i + 1 < codes.len() {
        let code = codes[i].trim();
        let val = codes[i + 1].trim();
        i += 2;
        if code == "0" && val == "LINE" {
            let mut line = DxfLine::default();
            while i + 1 < codes.len() {
                let c = codes[i].trim();
                let v = codes[i + 1].trim();
                if c == "0" {
                    break;
                }
                i += 2;
                match c {
                    "10" => line.x1 = v.parse::<f64>().map_err(|e| e.to_string())?,
                    "20" => line.y1 = v.parse::<f64>().map_err(|e| e.to_string())?,
                    "30" => line.z1 = v.parse::<f64>().map_err(|e| e.to_string())?,
                    "11" => line.x2 = v.parse::<f64>().map_err(|e| e.to_string())?,
                    "21" => line.y2 = v.parse::<f64>().map_err(|e| e.to_string())?,
                    "31" => line.z2 = v.parse::<f64>().map_err(|e| e.to_string())?,
                    _ => {}
                }
            }
            lines_out.push(line);
            continue;
        }
    }
    Ok(lines_out)
}

pub fn write_dxf_text(lines: &[DxfLine]) -> String {
    let mut out = String::from("0\nSECTION\n2\nENTITIES\n");
    for ln in lines {
        out.push_str("0\nLINE\n8\n0\n");
        out.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", ln.x1, ln.y1, ln.z1));
        out.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", ln.x2, ln.y2, ln.z2));
    }
    out.push_str("0\nENDSEC\n0\nEOF\n");
    out
}
//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DxfSnapshot {
    const EXTENSION: &'static str = "dxf";
    fn envelope_id() -> &'static str { "stdio.dxf" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let lines = parse_dxf_text(body).map_err(|e| store::TextError::new(format!("dxf parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
    }
    fn print_dsl(&self) -> String {
        let body = write_dxf_text(&self.lines);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DxfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_dxf_text(&self.lines).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let lines = parse_dxf_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
