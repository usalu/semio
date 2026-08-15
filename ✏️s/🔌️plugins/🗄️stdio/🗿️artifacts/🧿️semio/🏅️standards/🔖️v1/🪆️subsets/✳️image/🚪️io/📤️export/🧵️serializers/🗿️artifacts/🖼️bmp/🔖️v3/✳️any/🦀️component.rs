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

use crate::artifacts::bmp::{schema::snapshot::BmpRowOrder, BmpSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };

//#region 🔖️Serializer
pub struct SemioImageToBmp;

impl ArtifactSerializer for SemioImageToBmp {
    type From = SemioImageSnapshot;
    type Into = BmpSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::Schema("semio/image→bmp: no frames to export".into()))?;
        if frame.rgba8.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("semio/image→bmp: frame pixel length does not match width*height*4".into()));
        }
        let x_pixels_per_meter = from.metadata.iter().find(|m| m.key == "xPixelsPerMeter").and_then(|m| m.value.parse::<i32>().ok()).unwrap_or(0);
        let y_pixels_per_meter = from.metadata.iter().find(|m| m.key == "yPixelsPerMeter").and_then(|m| m.value.parse::<i32>().ok()).unwrap_or(0);
        Ok(BmpSnapshot {
            schema: crate::artifacts::bmp::STDIO_BMP_DOCUMENT_SCHEMA.into(),
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};

    fn sample_semio() -> SemioImageSnapshot {
        SemioImageSnapshot {
            width: 2,
            height: 1,
            colorspace: SemioColorspace::Rgb,
            bit_depth: 24,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
            icc: None,
            metadata: vec![SemioImageMetadataEntry { key: "xPixelsPerMeter".into(), value: "2835".into() }],
            ..SemioImageSnapshot::default()
        }
    }

    /// 🧪️ Real round trip through bmp's own codec — alpha is expected to drop (documented, the
    /// codec's own real behavior), RGB channels must survive exactly.
    #[test]
    fn real_byte_round_trip_through_bmp_codec() {
        let semio = sample_semio();
        let bmp = SemioImageToBmp::serialize(&semio).expect("serialize");
        assert_eq!(bmp.x_pixels_per_meter, 2835);
        let bytes = crate::artifacts::bmp::engine::encode_bmp(&bmp).expect("encode real bmp bytes");
        let decoded = crate::artifacts::bmp::engine::decode_bmp(&bytes).expect("decode real bmp bytes");
        assert_eq!(decoded.width, semio.width);
        assert_eq!(decoded.height, semio.height);
        for (a, b) in decoded.pixels.chunks_exact(4).zip(semio.frames[0].rgba8.chunks_exact(4)) {
            assert_eq!(&a[0..3], &b[0..3], "RGB must survive exactly");
        }
    }
}
//#endregion 🔖️Tests
