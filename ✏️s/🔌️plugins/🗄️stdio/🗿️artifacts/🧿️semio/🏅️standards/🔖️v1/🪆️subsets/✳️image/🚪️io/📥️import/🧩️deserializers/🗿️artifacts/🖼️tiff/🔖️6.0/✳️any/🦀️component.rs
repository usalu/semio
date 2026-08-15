//! 📥️ `tiff` (6.0) → `s.stdio.semio/v1/image` — `decode_tiff` already canonicalizes `pixels` to
//! RGBA8 decoded from IFD 0 (see that engine's module doc), so this leaf is a pure struct remap
//! over the well-known tag accessors `TiffSnapshot::width`/`height`/`tag`.
//!
//! Honest lossy points (documented):
//! - `colorspace` is inferred from IFD 0's `PhotometricInterpretation`/`SamplesPerPixel` tags
//!   (`Photometric` 0/1 → `Grayscale`, `SamplesPerPixel` 4 → `Rgba`, otherwise `Rgb`) —
//!   informational only; `pixels` is always already-decoded canonical RGBA8.
//! - `bit_depth` reads `BitsPerSample`'s first value (0 if untagged) — TIFF's real per-channel
//!   depth, so unlike BMP this one IS directly comparable to PNG's.
//! - `icc`: no ICC tag (34675, `ICCProfile`) extraction is attempted here — TIFF6.0's core tag
//!   table (this codec's typed scope) doesn't include it; always `None`.
//! - `metadata`: every OTHER IFD-0 tag (not width/height/bits/compression/photometric/samples/
//!   rows-per-strip/strip offsets or counts) becomes one entry keyed by its decimal tag id, valued
//!   by `first_u32()` when numeric or the raw `Ascii` string — real, lossless-enough for the
//!   common informational tags (`ImageDescription` 270, `Software` 305, …), though non-numeric/
//!   non-ASCII typed values (e.g. `Rational`) fall back to a `Debug`-formatted string (documented
//!   as a readable-but-not-machine-parseable representation).

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use crate::artifacts::tiff::{
    schema::snapshot::{TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_ROWS_PER_STRIP, TAG_SAMPLES_PER_PIXEL, TAG_STRIP_BYTE_COUNTS, TAG_STRIP_OFFSETS},
    TiffSnapshot,
};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

/// 🕳️ Core strip/geometry tags — already surfaced as typed `SemioImageSnapshot` fields, so they
/// don't ALSO become generic metadata entries (would duplicate the same information twice).
const CORE_TAGS: [u16; 9] = [TAG_IMAGE_WIDTH, TAG_IMAGE_LENGTH, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_SAMPLES_PER_PIXEL, TAG_ROWS_PER_STRIP, TAG_STRIP_BYTE_COUNTS];

fn value_to_metadata_string(v: &TiffValues) -> String {
    match v {
        TiffValues::Ascii(s) => s.clone(),
        other => other.first_u32().map(|n| n.to_string()).unwrap_or_else(|| format!("{other:?}")),
    }
}

//#region 🔖️Deserializer
pub struct SemioImageFromTiff;

impl ArtifactDeserializer for SemioImageFromTiff {
    type From = TiffSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let width = from.width().ok_or_else(|| store::PackError::Schema("tiff→semio/image: missing ImageWidth tag in ifds[0]".into()))?;
        let height = from.height().ok_or_else(|| store::PackError::Schema("tiff→semio/image: missing ImageLength tag in ifds[0]".into()))?;
        if from.pixels.len() != (width as usize) * (height as usize) * 4 {
            return Err(store::PackError::Schema("tiff→semio/image: pixels length does not match width*height*4".into()));
        }
        let samples_per_pixel = from.tag(TAG_SAMPLES_PER_PIXEL).and_then(|t| t.values.first_u32());
        let photometric = from.tag(TAG_PHOTOMETRIC).and_then(|t| t.values.first_u32());
        let colorspace = match (photometric, samples_per_pixel) {
            (Some(0), _) | (Some(1), _) => SemioColorspace::Grayscale,
            (_, Some(4)) => SemioColorspace::Rgba,
            _ => SemioColorspace::Rgb,
        };
        let bit_depth = from.tag(TAG_BITS_PER_SAMPLE).and_then(|t| t.values.first_u32()).unwrap_or(0).min(u8::MAX as u32) as u8;
        let metadata = from.ifds.first().map(|ifd| ifd.entries.iter().filter(|t| !CORE_TAGS.contains(&t.tag)).map(|t| SemioImageMetadataEntry { key: t.tag.to_string(), value: value_to_metadata_string(&t.values) }).collect()).unwrap_or_default();
        Ok(SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace, bit_depth, frames: vec![SemioImageFrame { delay_ms: 0, rgba8: from.pixels.clone() }], icc: None, metadata })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::tiff::schema::snapshot::{TiffFieldType, TiffIfd, TiffTag};

    fn sample_tiff() -> TiffSnapshot {
        TiffSnapshot {
            ifds: vec![TiffIfd {
                entries: vec![
                    TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![2]) },
                    TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) },
                    TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8]) },
                    TiffTag { tag: TAG_PHOTOMETRIC, kind: TiffFieldType::Short, values: TiffValues::Short(vec![2]) },
                    TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) },
                    TiffTag { tag: 270, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("semio fixture".into()) },
                ],
            }],
            pixels: vec![255, 0, 0, 255, 0, 255, 0, 255],
            ..TiffSnapshot::default()
        }
    }

    #[test]
    fn maps_pixels_and_description_tag() {
        let semio = SemioImageFromTiff::deserialize(&sample_tiff()).expect("deserialize");
        assert_eq!(semio.width, 2);
        assert_eq!(semio.height, 1);
        assert_eq!(semio.colorspace, SemioColorspace::Rgb);
        assert_eq!(semio.bit_depth, 8);
        assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
        assert!(semio.metadata.iter().any(|m| m.key == "270" && m.value == "semio fixture"));
    }
}
//#endregion 🔖️Tests
