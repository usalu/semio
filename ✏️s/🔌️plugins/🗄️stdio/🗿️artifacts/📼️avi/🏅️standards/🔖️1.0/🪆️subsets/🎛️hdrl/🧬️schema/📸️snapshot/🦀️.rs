//! 🧬️ AviSnapshot — RIFF/AVI 1.0: `avih` (MainAVIHeader) typed, per-stream `strh` typed + `strf`
//! discriminated by `fccType` (`BitmapInfo` for `vids`, `WaveFormat` for `auds`, `Raw` otherwise),
//! `movi` chunks assigned to their owning stream with `idx1`-derived keyframe flags, everything
//! else (non-`hdrl`/`movi`/`idx1` top-level RIFF children) typed-raw retained (`unknown_chunks`).
//! Nested auxiliary children real encoders also write are retained the same typed-raw way, one
//! level down: `hdrl`'s own non-`avih`/`strl` children (e.g. `JUNK` padding) in `hdrl_extra`, and
//! each `strl`'s non-`strh`/`strf` children (e.g. `vprp`, `JUNK`) in that stream's `strl_extra` —
//! both real, both present in ffmpeg's own AVI-1.0 output, neither addressable by a dedicated
//! mutation kind (see `AviMutation`'s module doc comment for why).
//! Real binary codec (`ArtifactPack`/`ArtifactDsl` wrap the REAL RIFF bytes `⚙️engine::{decode_avi,
//! encode_avi}` produce/consume, mirrors mp4's/`stdio.png`'s pattern — NOT JSON-pack passthrough).

use crate::standards::v1_0::subsets::any::io as engine;
use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_AVI_DOCUMENT_SCHEMA: &str = "stdio.avi";
//#endregion 🔖️Ids

//#region 🔖️MainHeader
/// 🏷️ `avih` — MainAVIHeader, all 14 DWORDs typed (56 bytes). <https://learn.microsoft.com/🪟️windows/win32/directshow/avimainheader>
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviMainHeader {
    pub micro_sec_per_frame: u32,
    pub max_bytes_per_sec: u32,
    pub padding_granularity: u32,
    pub flags: u32,
    pub total_frames: u32,
    pub initial_frames: u32,
    pub streams: u32,
    pub suggested_buffer_size: u32,
    pub width: u32,
    pub height: u32,
    /// 🕳️ `dwReserved[4]` — verbatim, never fabricated.
    #[value(default)]
    pub reserved: Vec<u32>,
}
//#endregion 🔖️MainHeader

//#region 🔖️StreamHeader
/// 🏷️ `strh` — AVISTREAMHEADER. The 13 DWORD/WORD fields up to `dwSampleSize` (48 bytes) are fixed;
/// the trailing `rcFrame` rectangle is NOT: real encoders (ffmpeg's own AVI-1.0 muxer included)
/// still write the classic pre-Win32 form with `rcFrame` as 4 16-bit `SHORT`s (56 bytes total), not
/// only the modern 4 `LONG`s form (64 bytes) most docs describe. <https://learn.microsoft.com/🪟️windows/win32/directshow/avistreamheader>
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviStreamHeader {
    pub fcc_type: String,
    pub fcc_handler: String,
    pub flags: u32,
    pub priority: u16,
    pub language: u16,
    pub initial_frames: u32,
    pub scale: u32,
    pub rate: u32,
    pub start: u32,
    pub length: u32,
    pub suggested_buffer_size: u32,
    pub quality: i32,
    pub sample_size: u32,
    /// 🖼️ `rcFrame`, always widened to `i32` regardless of the wire width it was read at.
    pub rc_frame_left: i32,
    pub rc_frame_top: i32,
    pub rc_frame_right: i32,
    pub rc_frame_bottom: i32,
    /// 📏 The wire width `encode_avi` re-serializes `rcFrame` as — `0` (omitted; a bare 48-byte
    /// `strh`), `8` (4 `SHORT`s; the classic 56-byte form), or `16` (4 `LONG`s; the modern 64-byte
    /// form). `decode_avi` records whichever width the source actually used so a real 56-byte
    /// `strh` round-trips byte-for-byte instead of being silently promoted to 64 bytes. Hand-built
    /// headers default to `16`, the complete/preferred form.
    #[value(default = "default_rc_frame_width")]
    pub rc_frame_width: u8,
    /// 📎 Any bytes beyond the documented 64-byte `AVISTREAMHEADER`, verbatim — rare, retained so
    /// an unusually padded real `strh` round-trips losslessly rather than being silently truncated.
    #[value(default)]
    pub strh_extra: Vec<u8>,
}

impl Default for AviStreamHeader {
    fn default() -> Self {
        Self {
            fcc_type: String::new(),
            fcc_handler: String::new(),
            flags: 0,
            priority: 0,
            language: 0,
            initial_frames: 0,
            scale: 0,
            rate: 0,
            start: 0,
            length: 0,
            suggested_buffer_size: 0,
            quality: 0,
            sample_size: 0,
            rc_frame_left: 0,
            rc_frame_top: 0,
            rc_frame_right: 0,
            rc_frame_bottom: 0,
            rc_frame_width: 16,
            strh_extra: Vec::new(),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_rc_frame_width() -> u8 {
    16
}
//#endregion 🔖️StreamHeader

//#region 🔖️StreamFormat
/// 🎨️ `strf`, discriminated by the owning stream's `fccType`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "format", rename_all = "camelCase")]
pub enum AviStreamFormat {
    /// 🖼️ `BITMAPINFOHEADER` (40 bytes; `vids`). <https://learn.microsoft.com/🪟️windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader>
    BitmapInfo { size: u32, width: i32, height: i32, planes: u16, bit_count: u16, compression: String, size_image: u32, x_pels_per_meter: i32, y_pels_per_meter: i32, colors_used: u32, colors_important: u32 },
    /// 🔊️ `WAVEFORMATEX`-shaped (`auds`).
    WaveFormat {
        format_tag: u16,
        channels: u16,
        samples_per_sec: u32,
        avg_bytes_per_sec: u32,
        block_align: u16,
        bits_per_sample: u16,
        #[value(default)]
        extra: Vec<u8>,
    },
    /// 📦 Any other `fccType` — verbatim `strf` payload bytes.
    Raw { data: Vec<u8> },
}

impl Default for AviStreamFormat {
    fn default() -> Self {
        Self::Raw { data: Vec::new() }
    }
}
//#endregion 🔖️StreamFormat

//#region 🔖️Chunk
/// 🎞️ One `movi` chunk belonging to this stream — fourcc (e.g. `"00dc"`), payload bytes, and
/// whether `idx1` (or the no-`idx1` fallback, per spec: absent index ⇒ every scanned chunk is
/// treated as a sync point) marks it a keyframe.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviChunk {
    pub fourcc: String,
    #[value(default)]
    pub data: Vec<u8>,
    pub keyframe: bool,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AviStream {
    pub strh: AviStreamHeader,
    pub strf: AviStreamFormat,
    #[value(default)]
    pub chunks: Vec<AviChunk>,
    /// 📦️ Typed-raw retention for this stream's `strl` children besides `strh`/`strf` (e.g. a
    /// `vprp` video-properties chunk, `JUNK` padding) — verbatim fourcc + payload, replayed after
    /// `strh`/`strf` on encode. Real ffmpeg AVI-1.0 output carries both of these inside `strl`.
    #[value(default)]
    pub strl_extra: Vec<RiffChunk>,
}
//#endregion 🔖️Chunk

//#region 🔖️RawChunk
/// 📦️ Typed-raw retention for a top-level RIFF child this codec doesn't otherwise type (any
/// entry inside `AVI `'s body besides `hdrl`/`movi`/`idx1`) — verbatim fourcc + payload, replayed
/// at the same relative position (after `idx1`) on encode.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RiffChunk {
    pub fourcc: String,
    #[value(default)]
    pub data: Vec<u8>,
}
//#endregion 🔖️RawChunk

//#region 🔖️Snapshot
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.avi")]
pub struct AviSnapshot {
    #[state(artifact)]
    #[value(default = "default_schema")]
    pub schema: String,
    #[state(artifact)]
    pub main_header: AviMainHeader,
    #[state(artifact)]
    #[value(default)]
    pub streams: Vec<AviStream>,
    #[state(artifact)]
    pub idx1_present: bool,
    #[state(artifact)]
    #[value(default)]
    pub unknown_chunks: Vec<RiffChunk>,
    /// 📦️ Typed-raw retention for `hdrl` children besides `avih`/`strl` (e.g. `JUNK` padding
    /// directly inside `hdrl`) — verbatim fourcc + payload, replayed after every `strl` on encode.
    #[state(artifact)]
    #[value(default)]
    pub hdrl_extra: Vec<RiffChunk>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_schema() -> String {
    STDIO_AVI_DOCUMENT_SCHEMA.into()
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for AviSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_AVI_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if !hex.len().is_multiple_of(2) {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        engine::decode_avi(&bytes).map_err(|e| store::TextError::new(format!("avi decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = engine::encode_avi(self);
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for AviSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = engine::encode_avi(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        engine::decode_avi(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
