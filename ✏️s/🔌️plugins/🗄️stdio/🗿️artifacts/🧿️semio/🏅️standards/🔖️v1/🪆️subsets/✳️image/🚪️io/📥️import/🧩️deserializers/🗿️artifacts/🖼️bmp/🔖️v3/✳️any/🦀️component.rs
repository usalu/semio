//! 📥️ `bmp` (v3) → `s.stdio.semio/v1/image` — `decode_bmp` already canonicalizes `pixels` to
//! row-major top-down RGBA8 (regardless of the source `row_order`), so this leaf is a pure
//! struct remap.
//!
//! Honest lossy points (documented):
//! - `colorspace` is inferred from `bits_per_pixel` (`<= 8` → `Indexed` via the on-disk palette;
//!   `24` → `Rgb`; `32` → `Rgba`; anything else defaults to `Rgb`) — informational only, since
//!   `pixels` is always already-decoded canonical RGBA8 regardless.
//! - `bit_depth` carries the source `bits_per_pixel` (a whole-pixel bit count, e.g. 24), not a
//!   per-channel depth like PNG's — a real, documented unit difference between these two formats.
//! - `icc`: BMP v3 (`BITMAPINFOHEADER`) has no ICC profile field at all — always `None`.
//! - `metadata`: `x_pixels_per_meter`/`y_pixels_per_meter` (the only descriptive scalars BMP
//!   carries) become `xPixelsPerMeter`/`yPixelsPerMeter` entries; the palette itself has no
//!   textual home and is dropped (pixels are already palette-resolved).

use crate::artifacts::bmp::BmpSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

fn colorspace_from_bpp(bpp: u16) -> SemioColorspace {
    match bpp {
        0..=8 => SemioColorspace::Indexed,
        24 => SemioColorspace::Rgb,
        32 => SemioColorspace::Rgba,
        _ => SemioColorspace::Rgb,
    }
}

//#region 🔖️Deserializer
pub struct SemioImageFromBmp;

impl ArtifactDeserializer for SemioImageFromBmp {
    type From = BmpSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.pixels.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("bmp→semio/image: pixels length does not match width*height*4".into()));
        }
        let mut metadata = Vec::new();
        if from.x_pixels_per_meter != 0 {
            metadata.push(SemioImageMetadataEntry { key: "xPixelsPerMeter".into(), value: from.x_pixels_per_meter.to_string() });
        }
        if from.y_pixels_per_meter != 0 {
            metadata.push(SemioImageMetadataEntry { key: "yPixelsPerMeter".into(), value: from.y_pixels_per_meter.to_string() });
        }
        Ok(SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: from.width,
            height: from.height,
            colorspace: colorspace_from_bpp(from.bits_per_pixel),
            bit_depth: from.bits_per_pixel.min(u8::MAX as u16) as u8,
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

    fn sample_bmp() -> BmpSnapshot {
        BmpSnapshot { width: 2, height: 1, bits_per_pixel: 24, row_order: BmpRowOrder::BottomUp, x_pixels_per_meter: 2835, y_pixels_per_meter: 2835, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255], ..BmpSnapshot::default() }
    }

    #[test]
    fn maps_pixels_and_resolution() {
        let semio = SemioImageFromBmp::deserialize(&sample_bmp()).expect("deserialize");
        assert_eq!(semio.width, 2);
        assert_eq!(semio.height, 1);
        assert_eq!(semio.colorspace, SemioColorspace::Rgb);
        assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
        assert!(semio.metadata.iter().any(|m| m.key == "xPixelsPerMeter" && m.value == "2835"));
    }
}
//#endregion 🔖️Tests
