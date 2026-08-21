//! 🧬️ GifSnapshot schema (89a) — complete per GIF89a §18-27: logical screen descriptor + optional
//! Global Color Table + an ordered sequence of frames, each with its own Graphic Control Extension
//! fields (delay/disposal/transparent-index/user-input), optional Local Color Table, interlace
//! flag, and losslessly-retained palette indices (never decoded RGBA — the lossless-payload
//! exception). Also carries NETSCAPE2.0 loop count, comment extensions, plain-text extensions, and
//! any OTHER application extension verbatim (`GifAppExtension`) — nothing real on disk is silently
//! dropped. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: a REAL
//! rewrite of the prior decoded-rgba stub, which dropped palettes and every extension but GCE/loop.
//! Distinct from 87a's `GifImage`-shaped `GifSnapshot` (no GCE/animation concept at all) — the two
//! standards genuinely differ in shape, which is why 87a→89a is the plan's "Tier 2"
//! (snapshot-type-changing) evolution pilot rather than a same-type dialect move.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Ids
/// 🏷️ Document schema / DSL envelope id — distinct from 87a's `"stdio.gif"` so the two
/// standards' document codecs never collide in the shared `store::document_codec_registry`
/// (still keyed by a flat schema string pre-D4; see `engine::register`).
pub const STDIO_GIF89A_DOCUMENT_SCHEMA: &str = "stdio.gif.89a";
/// 🧬️ Artifact schema descriptor id — distinct from 87a's `"s.stdio.gif"` for the same reason.
pub const GIF89A_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gif.89a";
//#endregion Ids

//#region ColorTable
/// 🎨️ One color table entry (GCT/LCT), stored exactly as read from disk — including any
/// power-of-two padding entries past the meaningful palette, since those are real on-disk bytes.
/// 🧪️ F6-PILOT: `dsl::DslRecord` throughout this file — gives every nested snapshot/strong-entity
/// type `DslField` so `#[derive(dsl::DslOps)]` (on `GifMutation`) and `#[derive(dsl::DslDiff)]`
/// (on `GifDiff`, `GifFrameDiff`, ...) can embed them as variant/field payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 🎨️ A Global or Local Color Table. `colors.len()` must be a power of two in `2..=256` on encode.
/// `sorted` mirrors the packed byte's sort flag (decreasing importance ordering).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifColorTable {
    #[serde(default)]
    pub sorted: bool,
    #[serde(default)]
    pub colors: Vec<GifRgb>,
}
//#endregion ColorTable

//#region DisposalModel
/// 🗑️ GIF89a §23.c.4 disposal method: how the decoder should treat this frame's canvas region
/// before rendering the next one.
/// 🧪️ F6-PILOT: `dsl::DslScalar` — a plain unit-variant enum binds as `DslField` directly (no
/// `DslVariants`/`Statements` needed; this is the "enum but not a mutation-shaped one" case).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum GifDisposal {
    #[default]
    Unspecified,
    DoNotDispose,
    RestoreToBackground,
    RestoreToPrevious,
}

impl GifDisposal {
    /// 📐️ Decodes the GCE packed byte's 3-bit disposal field (values 4-7 are spec-reserved and
    /// fold back to `Unspecified` rather than erroring).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            1 => GifDisposal::DoNotDispose,
            2 => GifDisposal::RestoreToBackground,
            3 => GifDisposal::RestoreToPrevious,
            _ => GifDisposal::Unspecified,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_bits(self) -> u8 {
        match self {
            GifDisposal::Unspecified => 0,
            GifDisposal::DoNotDispose => 1,
            GifDisposal::RestoreToBackground => 2,
            GifDisposal::RestoreToPrevious => 3,
        }
    }
}
//#endregion DisposalModel

//#region PlainText
/// 📝️ Plain Text Extension (GIF89a §25) — a Graphic-Rendering Block alternative to a Table-Based
/// Image. Modeled as an optional companion on [`GifFrame`] per this ticket's design: a frame whose
/// `plain_text` is `Some` and `width == 0` IS a plain-text-only block (no image data); a frame with
/// both real image data and `plain_text` is a rare-but-legal combo the codec does not encode (a
/// documented deviation — see `engine::encode_gif`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifPlainText {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    pub cell_width: u8,
    pub cell_height: u8,
    pub fg_color_index: u8,
    pub bg_color_index: u8,
    #[serde(default)]
    pub text: String,
}
//#endregion PlainText

//#region AppExtension
/// 🧩️ Any application extension OTHER than NETSCAPE2.0 (which is modeled separately via
/// `GifSnapshot::loop_count`, to avoid representing the same on-disk bytes twice), retained
/// verbatim — typed raw-retention for a spec-real-but-semantically-opaque region.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifAppExtension {
    pub identifier: [u8; 8],
    pub auth_code: [u8; 3],
    #[serde(default)]
    #[dsl(base64)]
    pub data: Vec<u8>,
}

impl Default for GifAppExtension {
    fn default() -> Self {
        Self { identifier: [0; 8], auth_code: [0; 3], data: Vec::new() }
    }
}
//#endregion AppExtension

//#region FrameModel
/// 🎞️ One animation frame: its own region of the logical screen (real GIFs commonly only redraw
/// the changed sub-rectangle per frame, confirmed against the `dancing.gif` fixture), an optional
/// Local Color Table, interlace flag, losslessly-retained palette indices (NOT decoded RGBA — the
/// lossless-payload exception), and the Graphic Control Extension fields that preceded it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifFrame {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub interlace: bool,
    #[serde(default)]
    #[dsl(block)]
    pub lct: Option<GifColorTable>,
    /// 🎞️ Palette indices, row-major, natural (non-interlaced) order — length must equal
    /// `width * height` for a real-image frame (empty for a plain-text-only frame).
    #[serde(default)]
    #[dsl(base64)]
    pub indices: Vec<u8>,
    /// ⏱️ GCE delay time in 1/100s units.
    #[serde(default)]
    pub delay_cs: u16,
    #[serde(default)]
    pub disposal: GifDisposal,
    /// 👁️ GCE transparent color index — `None` when the transparent-color flag is clear.
    #[serde(default)]
    pub transparent_index: Option<u8>,
    /// ⌨️ GCE user-input flag.
    #[serde(default)]
    pub user_input: bool,
    #[serde(default)]
    #[dsl(block)]
    pub plain_text: Option<GifPlainText>,
}

impl GifFrame {
    /// 🖌️ Derived RGBA accessor — decodes `indices` through `lct` (falling back to `gct`).
    /// `transparent_index`-matching pixels normalize to `[0,0,0,0]`. NOT a stored field.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rgba(&self, gct: Option<&GifColorTable>) -> Vec<u8> {
        let table = self.lct.as_ref().or(gct);
        let mut out = Vec::with_capacity(self.indices.len() * 4);
        for &idx in &self.indices {
            if Some(idx) == self.transparent_index {
                out.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            let rgb = table.and_then(|t| t.colors.get(idx as usize)).copied().unwrap_or_default();
            out.extend_from_slice(&[rgb.r, rgb.g, rgb.b, 255]);
        }
        out
    }
}
//#endregion FrameModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a")]
pub struct GifSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(block)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[serde(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[serde(default)]
    pub pixel_aspect_ratio: u8,
    /// 🔁️ NETSCAPE2.0 application extension loop count: `None` = no looping extension present
    /// (plays once); `Some(0)` = loop forever; `Some(n)` = loop `n` additional times.
    #[state(artifact)]
    #[serde(default)]
    pub loop_count: Option<u16>,
    #[state(artifact)]
    #[serde(default)]
    pub frames: Vec<GifFrame>,
    /// 💬️ Comment Extension bodies, in file order (positionally normalized to appear right after
    /// the screen descriptor on re-encode — see `engine::encode_gif`'s documented normal form).
    #[state(artifact)]
    #[serde(default)]
    pub comments: Vec<String>,
    /// 🧩️ Every application extension OTHER than NETSCAPE2.0, verbatim.
    #[state(artifact)]
    #[serde(default)]
    pub app_extensions: Vec<GifAppExtension>,
}

impl Default for GifSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: 0, height: 0, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, loop_count: None, frames: Vec::new(), comments: Vec::new(), app_extensions: Vec::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for GifSnapshot {
    const EXTENSION: &'static str = "gif";
    fn envelope_id() -> &'static str {
        STDIO_GIF89A_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::gif::standards::v89a::engine::decode_gif(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::gif::standards::v89a::engine::encode_gif(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GifSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::gif::standards::v89a::engine::encode_gif(self).map_err(store::PackError::Schema)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::gif::standards::v89a::engine::decode_gif(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion HandcraftedArtifactCodecs
