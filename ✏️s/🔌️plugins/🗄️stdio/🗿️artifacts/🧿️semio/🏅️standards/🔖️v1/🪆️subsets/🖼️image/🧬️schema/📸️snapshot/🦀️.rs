//! 🧬️ SemioImageSnapshot — complete per the master plan's image subset spec: width/height/
//! colorspace/bit-depth + frames{delay_ms, rgba8 pixels} + embedded ICC profile + metadata
//! entries. Informed by png's typed IHDR/ancillary model and gif 89a's frame sequence; replaces
//! the pre-migration `RasterImage`. Ticket
//! 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT (W2b/image).
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's image wave replaces the old
//! hex-of-`serde_json` envelope passthrough with real hand-rolled text/binary codecs (this is a
//! NEUTRAL semio type, not itself an on-disk file format — real per-format bytes for png/gif/bmp/
//! jpg/tiff are produced by the semio↔format `🚪️io` leaves, W4).



use framework_schema::ArtifactSchema;

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — the semio design
/// (unlike gif 87a/89a's deliberately-split convention) uses the SAME literal for both, per the
/// master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`" note, one per
/// subset. Must stay repo-wide unique — `register_document_codec` duplicate-id detection is a
/// static policy check.
pub const STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA: &str = "s.stdio.semio.image";
//#endregion 🔖️Ids

//#region 🔖️Colorspace
/// 🎨️ Source pixel colorspace — every frame's `rgba8` buffer is always normalized to RGBA8 on
/// decode (per the master plan's snapshot spec), so this field records the SOURCE colorspace for
/// honest round-trip/re-encode decisions, not a second in-memory pixel layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum SemioColorspace {
    #[default]
    Rgb,
    Rgba,
    Grayscale,
    GrayscaleAlpha,
    Indexed,
}
//#endregion 🔖️Colorspace

//#region 🔖️Frame
/// 🖼️ One decoded frame: always-RGBA8 pixels (row-major, `width*height*4` bytes) plus its
/// animation delay. A single-frame image (png/jpg/bmp/tiff) has exactly one `SemioImageFrame`
/// with `delay_ms: 0`. Strong entity — per-field diffable (see `🔺️diff`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioImageFrame {
    pub delay_ms: u32,
    #[value(default)]
    pub rgba8: Vec<u8>,
}
//#endregion 🔖️Frame

//#region 🔖️Metadata
/// 🏷️ One textual metadata entry (png tEXt/iTXt, exif-as-text, gif comment-extension-derived, …)
/// — name-keyed by `key`. Weak/value entity: its "diff" is the whole new value, never sub-diffed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioImageMetadataEntry {
    pub key: String,
    #[value(default)]
    pub value: String,
}
//#endregion 🔖️Metadata

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image")]
pub struct SemioImageSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    pub colorspace: SemioColorspace,
    #[state(artifact)]
    #[value(default)]
    pub bit_depth: u8,
    #[state(artifact)]
    #[value(default)]
    pub frames: Vec<SemioImageFrame>,
    /// 🎨️ Embedded ICC color profile bytes, verbatim — `None` when the source carried none.
    #[state(artifact)]
    #[value(default)]
    pub icc: Option<Vec<u8>>,
    #[state(artifact)]
    #[value(default)]
    pub metadata: Vec<SemioImageMetadataEntry>,
}

impl Default for SemioImageSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width: 0, height: 0, colorspace: SemioColorspace::default(), bit_depth: 0, frames: Vec::new(), icc: None, metadata: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives























//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives










//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge







//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.image` document — one frame (16-byte RGBA8 pixel sweep across
/// red/green/blue/white), a non-default colorspace/bit-depth, a set ICC profile, and one metadata
/// entry — exercising every leaf/collection shape at least once. Single source of truth for
/// `📚️examples/…/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the conformance-law
/// tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_image_snapshot() -> SemioImageSnapshot {
    SemioImageSnapshot {
        schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width: 2,
        height: 2,
        colorspace: SemioColorspace::Rgba,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 100, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] }],
        icc: Some(vec![1, 2, 3, 4]),
        metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "Demo".into() }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests







