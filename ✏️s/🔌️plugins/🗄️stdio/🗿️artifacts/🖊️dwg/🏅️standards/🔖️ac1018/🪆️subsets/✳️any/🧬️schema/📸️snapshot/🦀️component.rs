//! 🧬️ DwgSnapshot schema — persistent fields + real codecs.

use crate::artifacts::dwg::STDIO_DWG_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub version: String,
    #[state(persistent)]
    #[serde(default)]
    pub bytes: Vec<u8>,
    #[state(persistent)]
    #[serde(default)]
    pub section_names: Vec<String>,
}

impl Default for DwgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: String::new(),
            bytes: Vec::new(),
            section_names: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DwgCodec
const KNOWN_SECTIONS: [&str; 6] = [
    "AcDb:Header",
    "AcDb:Classes",
    "AcDb:Handles",
    "AcDb:ObjFreeSpace",
    "AcDb:Template",
    "AcDb:AcDs:Summary",
];

fn dwg_version_sentinel(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() < 6 {
        return Err("DWG too short for AC10xx header".into());
    }
    let head = &bytes[0..6];
    if head[0] != b'A' || head[1] != b'C' || !head[2].is_ascii_digit() || !head[3].is_ascii_digit() {
        return Err("missing AC10xx DWG version sentinel".into());
    }
    if !head[4].is_ascii_digit() || !head[5].is_ascii_digit() {
        return Err("invalid AC10xx version digits".into());
    }
    Ok(String::from_utf8_lossy(head).into_owned())
}

fn parse_section_names(bytes: &[u8]) -> Vec<String> {
    let mut found = Vec::new();
    for name in KNOWN_SECTIONS {
        if bytes.windows(name.len()).any(|w| w == name.as_bytes()) {
            found.push(name.into());
        }
    }
    if found.is_empty() && bytes.len() >= 0x80 + 0x60 {
        let base = 0x80;
        let count = u32::from_le_bytes(bytes[base + 0x4C..base + 0x50].try_into().unwrap_or([0, 0, 0, 0]));
        if count > 0 && count <= 16 {
            let mut off = base + 0x50;
            for _ in 0..count {
                if off + 32 > bytes.len() {
                    break;
                }
                let slice = &bytes[off..off + 32];
                let end = slice.iter().position(|&b| b == 0).unwrap_or(32);
                if end > 0 {
                    let label = String::from_utf8_lossy(&slice[..end]).into_owned();
                    if !label.is_empty() {
                        found.push(label);
                    }
                }
                off += 32;
            }
        }
    }
    found
}

pub fn decode_dwg(bytes: &[u8]) -> Result<DwgSnapshot, String> {
    let version = dwg_version_sentinel(bytes)?;
    let section_names = parse_section_names(bytes);
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version,
        bytes: bytes.to_vec(),
        section_names,
    })
}

pub fn encode_dwg(snap: &DwgSnapshot) -> Result<Vec<u8>, String> {
    if snap.bytes.is_empty() {
        return Err("empty DWG payload".into());
    }
    dwg_version_sentinel(&snap.bytes)?;
    if snap.version.is_empty() {
        return Err("missing DWG version".into());
    }
    if snap.bytes.get(0..6) != Some(snap.version.as_bytes()) {
        return Err("version field disagrees with AC10xx header".into());
    }
    Ok(snap.bytes.clone())
}
//#endregion 🔖️DwgCodec

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DwgSnapshot {
    const EXTENSION: &'static str = "dwg";
    fn envelope_id() -> &'static str { "stdio.dwg" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i + 1 < hex.len() {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
            i += 2;
        }
        decode_dwg(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let raw = encode_dwg(self).unwrap_or_default();
        let body: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DwgSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_dwg(self).map_err(|e| store::PackError::Schema(e))?;
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
        decode_dwg(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
