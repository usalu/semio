//! 📤️ `s.stdio.semio/v1/image` → `tiff` (6.0) — `encode_tiff` recomputes every core strip tag
//! fresh from `pixels`/`width()`/`height()` and carries over any OTHER `ifds[0]` tag verbatim
//! (see that engine's own `EncodeScopeNote`), so this leaf only needs to plant `TAG_IMAGE_WIDTH`/
//! `TAG_IMAGE_LENGTH` in `ifds[0]` (required — `encode_tiff` errors without them) plus rebuild
//! the non-core tags this leaf's import side extracted into `metadata`.
//!
//! Honest lossy points (documented):
//! - Only the FIRST frame is exported (TIFF baseline single-IFD encode here is not animated).
//! - `encode_tiff` drops alpha (`rgba_to_rgb`) and always writes 8-bit/sample RGB — `colorspace`/
//!   `bit_depth` are not fed back beyond the required width/length tags (matching the codec's own
//!   real encode scope, documented in its module header).
//! - Metadata entries round-trip back as `Ascii` tags (best-effort — numeric-looking values that
//!   came from a non-Ascii source type on import re-emit as text, a real, honest normalization,
//!   not a byte-exact inverse of every possible TIFF field type).

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::tiff::{
    schema::snapshot::{TiffFieldType, TiffIfd, TiffTag, TiffValues, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH},
    TiffSnapshot,
};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId::ANY };

//#region 🔖️Serializer
pub struct SemioImageToTiff;

impl ArtifactSerializer for SemioImageToTiff {
    type From = SemioImageSnapshot;
    type Into = TiffSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::Schema("semio/image→tiff: no frames to export".into()))?;
        if frame.rgba8.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("semio/image→tiff: frame pixel length does not match width*height*4".into()));
        }
        let mut entries = vec![TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![from.width]) }, TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![from.height]) }];
        for m in &from.metadata {
            if let Ok(tag) = m.key.parse::<u16>() {
                entries.push(TiffTag { tag, kind: TiffFieldType::Ascii, values: TiffValues::Ascii(m.value.clone()) });
            }
        }
        entries.sort_by_key(|t| t.tag);
        Ok(TiffSnapshot { schema: crate::artifacts::tiff::STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: Default::default(), ifds: vec![TiffIfd { entries }], pixels: frame.rgba8.clone() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_semio() -> SemioImageSnapshot {
        SemioImageSnapshot {
            width: 2,
            height: 1,
            colorspace: SemioColorspace::Rgb,
            bit_depth: 8,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
            icc: None,
            metadata: vec![SemioImageMetadataEntry { key: "270".into(), value: "semio fixture".into() }],
            ..SemioImageSnapshot::default()
        }
    }

    /// 🧪️ Real round trip through tiff's own codec (drops alpha, per the engine's own documented
    /// encode scope — RGB channels and the description tag must survive).
    #[semio_framework_async_macros::async_test]
    async fn real_byte_round_trip_through_tiff_codec() {
        let semio = sample_semio();
        let tiff = semio_framework_plugin::resolve_ready(SemioImageToTiff::serialize(&semio)).expect("serialize");
        let bytes = crate::artifacts::tiff::engine::encode_tiff(&tiff).await.expect("encode real tiff bytes");
        let decoded = crate::artifacts::tiff::engine::decode_tiff(&bytes).await.expect("decode real tiff bytes");
        assert_eq!(decoded.width(), Some(2));
        assert_eq!(decoded.height(), Some(1));
        for (a, b) in decoded.pixels.chunks_exact(4).zip(semio.frames[0].rgba8.chunks_exact(4)) {
            assert_eq!(&a[0..3], &b[0..3], "RGB must survive exactly");
        }
        assert!(decoded.ifds[0].entries.iter().any(|t| t.tag == 270 && matches!(&t.values, TiffValues::Ascii(s) if s == "semio fixture")));
    }
}
//#endregion 🔖️Tests
