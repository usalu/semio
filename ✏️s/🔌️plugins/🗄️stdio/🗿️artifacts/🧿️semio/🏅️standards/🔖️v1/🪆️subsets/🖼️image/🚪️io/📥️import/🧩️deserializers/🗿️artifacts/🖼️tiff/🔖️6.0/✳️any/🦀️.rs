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
//! - `metadata`: every OTHER IFD-0 tag (not width/height/bits/photometric/samples/extra-samples;
//!   native strip, tile and compression policy is never a semantic tag) becomes one entry keyed by its decimal tag id, valued
//!   by `first_u32()` when numeric or the raw `Ascii` string — real, lossless-enough for the
//!   common informational tags (`ImageDescription` 270, `Software` 305, …; several ASCII strings join with a newline), though non-numeric/
//!   non-ASCII typed values (e.g. `Rational`) fall back to a `Debug`-formatted string (documented
//!   as a readable-but-not-machine-parseable representation).

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use {semio_framework_plugin::ArtifactDeserializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_tiff::{
    schema::snapshot::{TiffValues, TAG_BITS_PER_SAMPLE, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_SAMPLES_PER_PIXEL},
    TiffSnapshot,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

/// 🫥️ TIFF6 §18 `ExtraSamples`: what the fourth sample of a pixel means (2 = unassociated alpha).
const TAG_EXTRA_SAMPLES: u16 = 338;

/// 🕳️ Core geometry tags — already surfaced as typed `SemioImageSnapshot` fields, so they
/// don't ALSO become generic metadata entries (would duplicate the same information twice).
const CORE_TAGS: [u16; 6] = [TAG_IMAGE_WIDTH, TAG_IMAGE_LENGTH, TAG_BITS_PER_SAMPLE, TAG_PHOTOMETRIC, TAG_SAMPLES_PER_PIXEL, TAG_EXTRA_SAMPLES];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_to_metadata_string(v: &TiffValues) -> String {
    match v {
        TiffValues::Ascii(texts) => texts.join("\n"),
        other => other.first_u32().map_or_else(|| format!("{other:?}"), |n| n.to_string()),
    }
}

//#region 🔖️Deserializer
pub struct SemioImageFromTiff;

impl ArtifactDeserializer for SemioImageFromTiff {
    type From = TiffSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let width = from.width().ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "tiff→semio/image: missing ImageWidth tag in ifds[0]")))?;
        let height = from.height().ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "tiff→semio/image: missing ImageLength tag in ifds[0]")))?;
        let page = semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::decode_tiff_page_rgba(from, 0).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("tiff→semio/image: {error}"))))?;
        let samples_per_pixel = from.tag(TAG_SAMPLES_PER_PIXEL).and_then(|t| t.values.first_u32());
        let photometric = from.tag(TAG_PHOTOMETRIC).and_then(|t| t.values.first_u32());
        let colorspace = match (photometric, samples_per_pixel) {
            (Some(0), _) | (Some(1), _) => SemioColorspace::Grayscale,
            (_, Some(4)) => SemioColorspace::Rgba,
            _ => SemioColorspace::Rgb,
        };
        let bit_depth = from.tag(TAG_BITS_PER_SAMPLE).and_then(|t| t.values.first_u32()).unwrap_or(0).min(u8::MAX as u32) as u8;
        let metadata = from.ifds.first().map(|ifd| ifd.entries.iter().filter(|t| !CORE_TAGS.contains(&t.tag)).map(|t| SemioImageMetadataEntry { key: t.tag.to_string(), value: value_to_metadata_string(&t.values) }).collect()).unwrap_or_default();
        Ok(SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace, bit_depth, frames: vec![SemioImageFrame { delay_ms: 0, rgba8: page.pixels }], icc: None, metadata })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
