//! 📥️ `png` (1.2) → `s.stdio.semio/v1/image` — PNG's own decoded `pixels` are already canonical
//! RGBA8 (png's `encode_png` hardcodes color-type 6/bit-depth 8 on write regardless of the
//! source IHDR, see that engine's own doc), so this leaf is a pure struct-to-struct remap: no
//! byte-level codec work happens here (that stays in `crate::artifacts::png::engine`).
//!
//! Honest lossy points (documented, never silently fabricated):
//! - `icc`: PNG's typed snapshot does not model the `iCCP` chunk (only IHDR/PLTE/tRNS/gAMA/cHRM/
//!   sRGB/pHYs/tIME/bKGD/tEXt are typed; anything else — including `iCCP` — lands in
//!   `unknown_chunks` untyped). Import always yields `icc: None`.
//! - `bit_depth`/`colorspace`: recorded from the SOURCE IHDR for informational round-trip, but
//!   `encode_png` always re-emits canonical RGBA8 regardless of these fields (confirmed in that
//!   engine: IHDR bytes are hardcoded `[8, 6, 0, 0, 0]`), so they are lossy-informational only.
//! - `text_chunks`: `keyword`/`value` map onto `SemioImageMetadataEntry{key,value}`; `kind`
//!   (tEXt/zTXt/iTXt)/`compressed`/`language_tag`/`translated_keyword` have no home on
//!   `SemioImageMetadataEntry` and are dropped on import.

use crate::artifacts::png::{
    schema::snapshot::{PngColorType, PngTextChunk},
    PngSnapshot,
};
#[cfg(test)]
use crate::artifacts::png::schema::snapshot::{PngChunkMarker, PngTextKind};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

//#region 🔖️ColorspaceMap
async fn colorspace_from_png(c: PngColorType) -> SemioColorspace {
    match c {
        PngColorType::Grayscale => SemioColorspace::Grayscale,
        PngColorType::Rgb => SemioColorspace::Rgb,
        PngColorType::Palette => SemioColorspace::Indexed,
        PngColorType::GrayscaleAlpha => SemioColorspace::GrayscaleAlpha,
        PngColorType::Rgba => SemioColorspace::Rgba,
    }
}
//#endregion 🔖️ColorspaceMap

//#region 🔖️Deserializer
pub struct SemioImageFromPng;

impl ArtifactDeserializer for SemioImageFromPng {
    type From = PngSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.pixels.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("png→semio/image: pixels length does not match width*height*4".into()));
        }
        let metadata = from.text_chunks.iter().map(|t: &PngTextChunk| SemioImageMetadataEntry { key: t.keyword.clone(), value: t.value.clone() }).collect();
        Ok(SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: from.width,
            height: from.height,
            colorspace: colorspace_from_png(from.color_type),
            bit_depth: from.bit_depth,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8: from.pixels.clone() }],
            icc: None,
            metadata,
        })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::png::schema::snapshot::PngRgb;

    async fn sample_png() -> PngSnapshot {
        PngSnapshot {
            width: 2,
            height: 1,
            bit_depth: 8,
            color_type: PngColorType::Rgba,
            pixels: vec![255, 0, 0, 255, 0, 255, 0, 255],
            text_chunks: vec![PngTextChunk { keyword: "Title".into(), value: "semio fixture".into(), kind: PngTextKind::Text, ..Default::default() }],
            chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Text { index: 0 }, PngChunkMarker::Idat, PngChunkMarker::Iend],
            plte: Some(vec![PngRgb::default()]),
            ..PngSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn maps_pixels_and_metadata() {
        let semio = semio_framework_plugin::resolve_ready(SemioImageFromPng::deserialize(&sample_png())).expect("deserialize");
        assert_eq!(semio.width, 2);
        assert_eq!(semio.height, 1);
        assert_eq!(semio.colorspace, SemioColorspace::Rgba);
        assert_eq!(semio.frames.len(), 1);
        assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
        assert_eq!(semio.icc, None, "png codec does not model iCCP — documented loss");
        assert_eq!(semio.metadata.len(), 1);
        assert_eq!(semio.metadata[0].key, "Title");
        assert_eq!(semio.metadata[0].value, "semio fixture");
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_pixel_length_mismatch() {
        let mut bad = sample_png();
        bad.pixels.pop();
        assert!(semio_framework_plugin::resolve_ready(SemioImageFromPng::deserialize(&bad)).is_err());
    }
}
//#endregion 🔖️Tests
