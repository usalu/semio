//! 🧬️ ZipSnapshot schema — persistent fields + real ZIP codecs.

use crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region CompressionMethod
/// 🗜️ Per-entry compression method. Only the two methods the deflate artifact's own
/// codec can round-trip are modeled — anything else is a decode-time `ZipError::UnsupportedMethod`,
/// never a silently-dropped or fabricated entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ZipCompressionMethod {
    Stored,
    Deflate,
}

impl Default for ZipCompressionMethod {
    fn default() -> Self { Self::Stored }
}

impl ZipCompressionMethod {
    /// 🔢️ The on-disk method code (APPNOTE 4.4.5).
    pub const fn code(self) -> u16 {
        match self {
            Self::Stored => 0,
            Self::Deflate => 8,
        }
    }

    /// 🔢️ Inverse of `code` — `None` for any method this artifact can't decode.
    pub const fn from_code(code: u16) -> Option<Self> {
        match code {
            0 => Some(Self::Stored),
            8 => Some(Self::Deflate),
            _ => None,
        }
    }
}
//#endregion CompressionMethod

//#region ExtraField
/// 🧩️ One raw local/central "extra field" record (id + payload), kept verbatim for any id this
/// artifact doesn't specially interpret (ZIP64 sizes and the Info-ZIP `UT` timestamp are read for
/// convenience but their raw bytes are still kept here too — nothing genuinely unmodeled is dropped).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipExtraField {
    pub id: u16,
    #[serde(default)]
    pub payload: Vec<u8>,
}
//#endregion ExtraField

//#region Entry
/// 🎒️ One ZIP archive member: uncompressed payload plus every local-file-header/central-directory
/// field this artifact models (see `🎒️zip` D2 plan row — method, DOS+UTC times, attrs, flags, extra,
/// comment). `data` is always the decompressed payload; `method` drives how the writer re-compresses it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntry {
    pub name: String,
    #[serde(default)]
    pub data: Vec<u8>,
    #[serde(default)]
    pub method: ZipCompressionMethod,
    /// 📅️ Raw MS-DOS date (local-file-header layout, APPNOTE 4.4.6).
    #[serde(default)]
    pub dos_date: u16,
    /// 📅️ Raw MS-DOS time (local-file-header layout, APPNOTE 4.4.6).
    #[serde(default)]
    pub dos_time: u16,
    /// 🕰️ Real-world UTC mtime (seconds since epoch), decoded from an Info-ZIP `UT` (0x5455)
    /// extra-field record when present. `None` when no such extension exists — the DOS
    /// date/time above is still the ground truth for the on-disk local/central headers.
    #[serde(default)]
    pub unix_mtime: Option<i64>,
    /// 🚩️ General-purpose bit flags as read from the central directory (bit 3 = data-descriptor,
    /// bit 11 = UTF-8 filename/comment). The writer clears bit 3 (it always knows sizes up front)
    /// and sets bit 11 (it always emits `name`/`comment` as UTF-8) — see engine `encode_zip`.
    #[serde(default)]
    pub flags: u16,
    #[serde(default)]
    pub version_made_by: u16,
    #[serde(default)]
    pub version_needed: u16,
    #[serde(default)]
    pub internal_attrs: u16,
    #[serde(default)]
    pub external_attrs: u32,
    /// 🧩️ Extra-field records as they appeared in the local file header.
    #[serde(default)]
    pub local_extra: Vec<ZipExtraField>,
    /// 🧩️ Extra-field records as they appeared in the central directory header (legally distinct
    /// from `local_extra` — real writers occasionally disagree between the two).
    #[serde(default)]
    pub central_extra: Vec<ZipExtraField>,
    #[serde(default)]
    pub comment: String,
}
//#endregion Entry

//#region Snapshot
/// 📸️ Persisted `stdio.zip` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<ZipEntry>,
    /// 💬️ Archive-level comment (EOCD comment field).
    #[state(persistent)]
    #[serde(default)]
    pub comment: String,
}

impl Default for ZipSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: Vec::new(),
            comment: String::new(),
        }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for ZipSnapshot {
    const EXTENSION: &'static str = "zip";
    fn envelope_id() -> &'static str { "stdio.zip" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        // hex of zip bytes
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::zip::engine::decode_zip(&bytes).map_err(|e| {
            store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1))
        })
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::zip::engine::encode_zip(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ZipSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::zip::engine::encode_zip(self)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        crate::artifacts::zip::engine::decode_zip(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion HandcraftedArtifactCodecs
