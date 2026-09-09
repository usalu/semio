//! 📤️ `s.stdio.semio/v1/image` → `bmp` (v3) — `encode_bmp` always writes 24bpp BGR rows (it
//! reads only `width`/`height`/`pixels`/`row_order`/`x_pixels_per_meter`/`y_pixels_per_meter`,
//! confirmed in `⚙️engine::encode_bmp`; alpha is silently dropped by the codec itself, not here).
//!
//! Honest lossy points (documented):
//! - Only the FIRST frame is exported (BMP is not animated).
//! - Alpha is dropped (the underlying BMP v3/`BITMAPINFOHEADER` codec's own real behavior).
//! - Only `xPixelsPerMeter`/`yPixelsPerMeter` metadata entries round-trip (parsed back into the
//!   matching header fields); any other key is dropped (no other textual field exists on
//!   `BmpSnapshot`).

use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_bmp::{schema::snapshot::BmpRowOrder, BmpSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };

//#region 🔖️Serializer
pub struct SemioImageToBmp;

impl ArtifactSerializer for SemioImageToBmp {
    type From = SemioImageSnapshot;
    type Into = BmpSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::Schema("semio/image→bmp: no frames to export".into()))?;
        if frame.rgba8.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("semio/image→bmp: frame pixel length does not match width*height*4".into()));
        }
        let x_pixels_per_meter = from.metadata.iter().find(|m| m.key == "xPixelsPerMeter").and_then(|m| m.value.parse::<i32>().ok()).unwrap_or(0);
        let y_pixels_per_meter = from.metadata.iter().find(|m| m.key == "yPixelsPerMeter").and_then(|m| m.value.parse::<i32>().ok()).unwrap_or(0);
        Ok(BmpSnapshot {
            schema: semio_s_artifact_stdio_bmp::STDIO_BMP_DOCUMENT_SCHEMA.into(),
            width: from.width,
            height: from.height,
            row_order: BmpRowOrder::BottomUp,
            planes: 1,
            bits_per_pixel: 24,
            compression: 0,
            x_pixels_per_meter,
            y_pixels_per_meter,
            pixels: frame.rgba8.clone(),
            ..BmpSnapshot::default()
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
