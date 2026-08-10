//! 🧬️ BmpSnapshot schema — persistent fields + real codecs.

use crate::artifacts::bmp::STDIO_BMP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for BmpSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width: 0, height: 0, pixels: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️BmpCodec
pub fn decode_bmp(bytes: &[u8]) -> Result<BmpSnapshot, String> {
    if bytes.len() < 54 || &bytes[0..2] != b"BM" {
        return Err("invalid BMP signature".into());
    }
    let data_off = u32::from_le_bytes(bytes[10..14].try_into().unwrap()) as usize;
    let width = i32::from_le_bytes(bytes[18..22].try_into().unwrap());
    let height = i32::from_le_bytes(bytes[22..26].try_into().unwrap()).unsigned_abs();
    let bpp = u16::from_le_bytes(bytes[28..30].try_into().unwrap());
    if bpp != 24 {
        return Err("only 24-bit BMP supported".into());
    }
    let w = width as u32;
    let h = height as u32;
    let row = ((w * 3 + 3) / 4) * 4;
    let mut pixels = vec![0u8; (w * h * 3) as usize];
    let mut off = data_off;
    for y in 0..h {
        let row_start = off;
        for x in 0..w {
            let i = ((h - 1 - y) * w + x) as usize * 3;
            if row_start + (x as usize) * 3 + 2 >= bytes.len() {
                return Err("bmp pixel overrun".into());
            }
            let b = bytes[row_start + (x as usize) * 3];
            let g = bytes[row_start + (x as usize) * 3 + 1];
            let r = bytes[row_start + (x as usize) * 3 + 2];
            pixels[i] = r;
            pixels[i + 1] = g;
            pixels[i + 2] = b;
        }
        off += row as usize;
    }
    Ok(BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels })
}

pub fn encode_bmp(snap: &BmpSnapshot) -> Result<Vec<u8>, String> {
    if snap.width == 0 || snap.height == 0 {
        return Err("empty image".into());
    }
    let w = snap.width;
    let h = snap.height;
    let row = ((w * 3 + 3) / 4) * 4;
    let pixel_bytes = (row * h) as usize;
    let file_size = 54 + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(-(h as i32)).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 16]);
    for y in 0..h {
        let mut row_buf = vec![0u8; row as usize];
        for x in 0..w {
            let i = ((y * w + x) * 3) as usize;
            if i + 2 >= snap.pixels.len() {
                return Err("pixel buffer short".into());
            }
            let o = (x as usize) * 3;
            row_buf[o] = snap.pixels[i + 2];
            row_buf[o + 1] = snap.pixels[i + 1];
            row_buf[o + 2] = snap.pixels[i];
        }
        out.extend_from_slice(&row_buf);
    }
    Ok(out)
}
//#endregion 🔖️BmpCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for BmpSnapshot {
    const EXTENSION: &'static str = "bmp";
    fn envelope_id() -> &'static str { "stdio.bmp" }

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
        decode_bmp(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let raw = encode_bmp(self).unwrap_or_default();
        let body: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for BmpSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_bmp(self).map_err(|e| store::PackError::Schema(e))?;
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
        decode_bmp(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
