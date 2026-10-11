//! 📤️ `s.stdio.semio/v1/image` → `tiff` (6.0) — the owned page is one logical RGBA8 sample block
//! plus its semantic tags (`TAG_IMAGE_WIDTH`/`TAG_IMAGE_LENGTH`, 8-bit samples, photometric RGB,
//! `ExtraSamples`) and the non-core tags this leaf's import side extracted into `metadata`;
//! `encode_tiff` chooses the native strip, tile and compression layout on its own.
//!
//! Honest lossy points (documented):
//! - Only the FIRST frame is exported (TIFF baseline single-IFD encode here is not animated).
//! - The frame is written as ONE sample block of 8-bit RGBA with unassociated alpha (`ExtraSamples` = 2) —
//!   `colorspace`/`bit_depth` are not fed back beyond that.
//! - Metadata entries round-trip back as `Ascii` tags (best-effort — numeric-looking values that
//!   came from a non-Ascii source type on import re-emit as text, a real, honest normalization,
//!   not a byte-exact inverse of every possible TIFF field type). A metadata value that is not
//!   ASCII or contains a NUL has no owned TIFF text form and is refused by the page validation.

use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use {semio_framework_plugin::ArtifactSerializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_tiff::{
    schema::snapshot::{TiffIfd, TiffSampleBlock, TiffTag, TiffValues, TiffWord64, TAG_BITS_PER_SAMPLE, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_SAMPLES_PER_PIXEL},
    TiffSnapshot,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId::ANY };

/// 🫥️ TIFF6 §18 `ExtraSamples`: what the fourth sample of a pixel means (2 = unassociated alpha).
const TAG_EXTRA_SAMPLES: u16 = 338;

//#region 🔖️Serializer
pub struct SemioImageToTiff;

impl ArtifactSerializer for SemioImageToTiff {
    type From = SemioImageSnapshot;
    type Into = TiffSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→tiff: no frames to export")))?;
        if frame.rgba8.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→tiff: frame pixel length does not match width*height*4")));
        }
        let mut entries = vec![
            TiffTag { tag: TAG_IMAGE_WIDTH, values: TiffValues::Long(vec![from.width]) },
            TiffTag { tag: TAG_IMAGE_LENGTH, values: TiffValues::Long(vec![from.height]) },
            TiffTag { tag: TAG_BITS_PER_SAMPLE, values: TiffValues::Short(vec![8, 8, 8, 8]) },
            TiffTag { tag: TAG_PHOTOMETRIC, values: TiffValues::Short(vec![2]) },
            TiffTag { tag: TAG_SAMPLES_PER_PIXEL, values: TiffValues::Short(vec![4]) },
            TiffTag { tag: TAG_EXTRA_SAMPLES, values: TiffValues::Short(vec![2]) },
        ];
        for m in &from.metadata {
            if let Ok(tag) = m.key.parse::<u16>() {
                entries.push(TiffTag { tag, values: TiffValues::Ascii(vec![m.value.clone()]) });
            }
        }
        entries.sort_by_key(|t| t.tag);
        entries.dedup_by_key(|t| t.tag);
        let block = TiffSampleBlock { x: 0, y: 0, width: from.width, height: from.height, channels: 4, samples: frame.rgba8.iter().map(|byte| TiffWord64::from_word(u64::from(*byte))).collect() };
        let snapshot = TiffSnapshot { schema: semio_s_artifact_stdio_tiff::STDIO_TIFF_DOCUMENT_SCHEMA.into(), ifds: vec![TiffIfd { entries, blocks: vec![block] }] };
        snapshot.validate().map_err(|message| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("semio/image→tiff: {message}"))))?;
        Ok(snapshot)
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
