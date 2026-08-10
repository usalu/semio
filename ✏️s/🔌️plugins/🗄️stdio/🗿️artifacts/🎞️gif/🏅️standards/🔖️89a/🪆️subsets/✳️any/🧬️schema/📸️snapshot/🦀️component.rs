//! 🧬️ GifSnapshot schema (89a) — multi-frame animation model: Graphic Control Extension
//! (delay/transparency/disposal per frame) + NETSCAPE2.0 loop count, real byte codecs.
//! Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION. Distinct from
//! 87a's `RasterImage`-shaped `GifSnapshot` (single static image, no GCE/animation concept) --
//! the two standards genuinely differ in shape, which is why 87a→89a is the plan's "Tier 2"
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

//#region DisposalModel
/// 🗑️ GIF89a §23.c.4 disposal method: how the decoder should treat this frame's canvas region
/// before rendering the next one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
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
    /// fold back to `Unspecified` rather than erroring — an unrecognized-but-legal reserved value
    /// should not fail an otherwise-valid decode).
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            1 => GifDisposal::DoNotDispose,
            2 => GifDisposal::RestoreToBackground,
            3 => GifDisposal::RestoreToPrevious,
            _ => GifDisposal::Unspecified,
        }
    }
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

//#region FrameModel
/// 🎞️ One animation frame: its own region of the logical screen (`left`/`top`/`width`/`height`
/// -- real GIFs commonly only redraw the changed sub-rectangle per frame, confirmed against the
/// `dancing.gif` fixture, so frames are NOT forced to canvas size), straight (non-premultiplied)
/// RGBA pixels (`alpha==0` marks the GCE transparent index), and the Graphic Control Extension
/// fields that preceded it in the file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GifFrame {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub rgba: Vec<u8>,
    /// ⏱️ GCE delay time in 1/100s units.
    #[serde(default)]
    pub delay_cs: u16,
    #[serde(default)]
    pub disposal: GifDisposal,
    /// 👁️ GCE transparent-color flag (redundant with any `alpha==0` pixel present, but the flag
    /// itself is real on-disk state — a frame can set it with no transparent pixels actually
    /// present, and that bit should still round-trip).
    #[serde(default)]
    pub transparent: bool,
    /// ⌨️ GCE user-input flag.
    #[serde(default)]
    pub user_input: bool,
}
//#endregion FrameModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a")]
pub struct GifSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    /// 🔁️ NETSCAPE2.0 application extension loop count: `None` = no looping extension present
    /// (plays once); `Some(0)` = loop forever; `Some(n)` = loop `n` additional times.
    #[state(persistent)]
    #[serde(default)]
    pub loop_count: Option<u16>,
    #[state(persistent)]
    #[serde(default)]
    pub frames: Vec<GifFrame>,
}

impl Default for GifSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(), width: 0, height: 0, loop_count: None, frames: Vec::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for GifSnapshot {
    const EXTENSION: &'static str = "gif";
    fn envelope_id() -> &'static str { STDIO_GIF89A_DOCUMENT_SCHEMA }

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
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::gif::standards::v89a::engine::decode_gif(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::gif::standards::v89a::engine::encode_gif(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GifSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::gif::standards::v89a::engine::encode_gif(self).map_err(store::PackError::Schema)?;
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
        crate::artifacts::gif::standards::v89a::engine::decode_gif(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion HandcraftedArtifactCodecs
