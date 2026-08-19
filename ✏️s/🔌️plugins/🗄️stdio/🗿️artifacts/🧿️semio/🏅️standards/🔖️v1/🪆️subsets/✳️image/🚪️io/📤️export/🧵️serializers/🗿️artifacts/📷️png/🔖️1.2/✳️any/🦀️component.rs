//! 📤️ `s.stdio.semio/v1/image` → `png` (1.2) — mirrors the deserializer leaf in the same pair's
//! import directory. `encode_png` always writes canonical RGBA8 (see that leaf's doc comment), so
//! this side only needs to hand it a valid `width*height*4` pixel buffer plus a `chunk_order`
//! that actually references the `text_chunks`/markers it builds (png's own `encode_png` only
//! emits a chunk when `chunk_order` names it — see `⚙️engine::encode_png`).
//!
//! Honest lossy points (documented):
//! - Only the FIRST frame is exported (PNG 1.2/APNG-less is not an animated format under this
//!   codec's scope); additional `frames` are dropped.
//! - `icc` is dropped (PNG snapshot has no typed iCCP field to carry it — see the import leaf).
//! - `colorspace`/`bit_depth` are informational only: `encode_png` always emits RGBA8/8-bit
//!   regardless, so they are not fed back into the PNG snapshot's own fields beyond a best-effort
//!   `color_type`/`bit_depth` stamp for readers that inspect the typed snapshot directly (not the
//!   re-encoded bytes).

use crate::artifacts::png::{
    schema::snapshot::{PngChunkMarker, PngColorType, PngTextChunk, PngTextKind},
    PngSnapshot,
};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

fn colorspace_to_png(c: SemioColorspace) -> PngColorType {
    match c {
        SemioColorspace::Grayscale => PngColorType::Grayscale,
        SemioColorspace::Rgb => PngColorType::Rgb,
        SemioColorspace::Indexed => PngColorType::Palette,
        SemioColorspace::GrayscaleAlpha => PngColorType::GrayscaleAlpha,
        SemioColorspace::Rgba => PngColorType::Rgba,
    }
}

//#region 🔖️Serializer
pub struct SemioImageToPng;

impl ArtifactSerializer for SemioImageToPng {
    type From = SemioImageSnapshot;
    type Into = PngSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::Schema("semio/image→png: no frames to export".into()))?;
        if frame.rgba8.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("semio/image→png: frame pixel length does not match width*height*4".into()));
        }
        let mut chunk_order = vec![PngChunkMarker::Ihdr];
        let text_chunks: Vec<PngTextChunk> = from.metadata.iter().map(|m| PngTextChunk { keyword: m.key.clone(), value: m.value.clone(), kind: PngTextKind::Text, ..Default::default() }).collect();
        for i in 0..text_chunks.len() {
            chunk_order.push(PngChunkMarker::Text { index: i });
        }
        chunk_order.push(PngChunkMarker::Idat);
        chunk_order.push(PngChunkMarker::Iend);

        Ok(PngSnapshot {
            schema: crate::artifacts::png::STDIO_PNG_DOCUMENT_SCHEMA.into(),
            width: from.width,
            height: from.height,
            bit_depth: 8,
            color_type: colorspace_to_png(from.colorspace),
            interlace: false,
            pixels: frame.rgba8.clone(),
            text_chunks,
            chunk_order,
            ..PngSnapshot::default()
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageMetadataEntry};

    fn sample_semio() -> SemioImageSnapshot {
        SemioImageSnapshot {
            width: 2,
            height: 1,
            colorspace: SemioColorspace::Rgba,
            bit_depth: 8,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
            icc: None,
            metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "semio fixture".into() }],
            ..SemioImageSnapshot::default()
        }
    }

    #[test]
    fn maps_pixels_and_metadata_to_png() {
        let semio = sample_semio();
        let png = semio_framework_plugin::resolve_ready(SemioImageToPng::serialize(&semio)).expect("serialize");
        assert_eq!(png.width, 2);
        assert_eq!(png.height, 1);
        assert_eq!(png.pixels, semio.frames[0].rgba8);
        assert_eq!(png.text_chunks.len(), 1);
        assert_eq!(png.text_chunks[0].keyword, "Title");
    }

    /// 🧪️ Real round trip: semio → (this leaf) → `PngSnapshot` → (png's own real codec) → bytes →
    /// (png's own real codec) → `PngSnapshot` — proves the serializer produces a genuinely
    /// encodable/decodable PNG, not just a plausible-looking struct.
    #[test]
    fn real_byte_round_trip_through_png_codec() {
        let semio = sample_semio();
        let png = semio_framework_plugin::resolve_ready(SemioImageToPng::serialize(&semio)).expect("serialize");
        let bytes = crate::artifacts::png::engine::encode_png(&png).expect("encode real png bytes");
        let decoded = crate::artifacts::png::engine::decode_png(&bytes).expect("decode real png bytes");
        assert_eq!(decoded.pixels, semio.frames[0].rgba8);
        assert_eq!(decoded.width, semio.width);
        assert_eq!(decoded.height, semio.height);
        assert_eq!(decoded.text_chunks.len(), 1);
        assert_eq!(decoded.text_chunks[0].keyword, "Title");
        assert_eq!(decoded.text_chunks[0].value, "semio fixture");
    }
}
//#endregion 🔖️Tests
