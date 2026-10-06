//! 📤️ `s.stdio.semio/v1/image` → `tiff` (6.0) — `encode_tiff` recomputes every core strip tag
//! fresh from `pixels`/`width()`/`height()` and carries over any OTHER `ifds[0]` tag verbatim
//! (see that engine's own `EncodeScopeNote`), so this leaf only needs to plant `TAG_IMAGE_WIDTH`/
//! `TAG_IMAGE_LENGTH` in `ifds[0]` (required — `encode_tiff` errors without them) plus rebuild
//! the non-core tags this leaf's import side extracted into `metadata`.
//!
//! Honest lossy points (documented):
//! - Only the FIRST frame is exported (TIFF baseline single-IFD encode here is not animated).
//! - The frame is written as ONE uncompressed strip of 8-bit RGBA with unassociated alpha (`ExtraSamples` = 2) —
//!   `colorspace`/`bit_depth` are not fed back beyond that.
//! - Metadata entries round-trip back as `Ascii` tags (best-effort — numeric-looking values that
//!   came from a non-Ascii source type on import re-emit as text, a real, honest normalization,
//!   not a byte-exact inverse of every possible TIFF field type).

use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_tiff::{
    schema::snapshot::{TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_ROWS_PER_STRIP, TAG_SAMPLES_PER_PIXEL},
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
            TiffTag { tag: TAG_COMPRESSION, values: TiffValues::Short(vec![1]) },
            TiffTag { tag: TAG_PHOTOMETRIC, values: TiffValues::Short(vec![2]) },
            TiffTag { tag: TAG_SAMPLES_PER_PIXEL, values: TiffValues::Short(vec![4]) },
            TiffTag { tag: TAG_ROWS_PER_STRIP, values: TiffValues::Long(vec![from.height]) },
            TiffTag { tag: TAG_EXTRA_SAMPLES, values: TiffValues::Short(vec![2]) },
        ];
        for m in &from.metadata {
            if let Ok(tag) = m.key.parse::<u16>() {
                let mut bytes = m.value.as_bytes().to_vec();
                if !bytes.ends_with(&[0]) { bytes.push(0); }
                entries.push(TiffTag { tag, values: TiffValues::Ascii(bytes) });
            }
        }
        entries.sort_by_key(|t| t.tag);
        Ok(TiffSnapshot {
            schema: semio_s_artifact_stdio_tiff::STDIO_TIFF_DOCUMENT_SCHEMA.into(),
            byte_order: Default::default(),
            ifds: vec![TiffIfd {
                entries,
                storage: TiffStorage { kind: TiffStorageKind::Strips, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks: vec![frame.rgba8.clone()] },
            }],
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
