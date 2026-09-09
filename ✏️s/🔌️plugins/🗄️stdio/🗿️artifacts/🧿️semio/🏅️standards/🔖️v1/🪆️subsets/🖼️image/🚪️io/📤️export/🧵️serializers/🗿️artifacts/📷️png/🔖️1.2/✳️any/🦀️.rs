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

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_png::{
    schema::snapshot::{PngChunkMarker, PngColorType, PngTextChunk, PngTextKind},
    PngSnapshot,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
            schema: semio_s_artifact_stdio_png::STDIO_PNG_DOCUMENT_SCHEMA.into(),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
