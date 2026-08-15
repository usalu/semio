//! 📤️ `s.stdio.semio/v1/image` → `jpg` (jfif-1.01) — `encode_jpg` only reads `width`/`height`/
//! `pixels`/`re_encode_quality` (it recomputes SOF/DQT/DHT itself, a real baseline JPEG encoder),
//! so this leaf only needs to hand it a valid canonical RGBA8 buffer.
//!
//! Honest lossy points (documented):
//! - Only the FIRST frame is exported (JPEG is not an animated format).
//! - Alpha is silently ignored by `encode_jpg` itself (JPEG has no alpha channel) — not
//!   re-checked here since dropping it is the underlying codec's own real behavior.
//! - `icc` is dropped (no typed field on `JpgSnapshot`).
//! - Only `metadata` entries with `key == "comment"` round-trip (as a `COM` segment); any other
//!   key has no textual home on `JpgSnapshot` and is dropped.

use crate::artifacts::jpg::{schema::snapshot::JpgSegment, JpgSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId::ANY };

const COM_MARKER: u8 = 0xFE;

//#region 🔖️Serializer
pub struct SemioImageToJpg;

impl ArtifactSerializer for SemioImageToJpg {
    type From = SemioImageSnapshot;
    type Into = JpgSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::Schema("semio/image→jpg: no frames to export".into()))?;
        if frame.rgba8.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("semio/image→jpg: frame pixel length does not match width*height*4".into()));
        }
        let other_segments = from.metadata.iter().filter(|m| m.key == "comment").map(|m| JpgSegment { marker: COM_MARKER, data: m.value.clone().into_bytes() }).collect();
        Ok(JpgSnapshot { schema: crate::artifacts::jpg::STDIO_JPG_DOCUMENT_SCHEMA.into(), width: from.width, height: from.height, pixels: frame.rgba8.clone(), other_segments, ..JpgSnapshot::default() })
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
            bit_depth: 8,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
            icc: None,
            metadata: vec![SemioImageMetadataEntry { key: "comment".into(), value: "semio fixture".into() }],
            ..SemioImageSnapshot::default()
        }
    }

    #[test]
    fn real_byte_round_trip_through_jpg_codec() {
        let semio = sample_semio();
        let jpg = SemioImageToJpg::serialize(&semio).expect("serialize");
        assert_eq!(jpg.width, 2);
        assert_eq!(jpg.height, 1);
        assert_eq!(jpg.other_segments.len(), 1);
        let bytes = crate::artifacts::jpg::engine::encode_jpg(&jpg).expect("encode real jpg bytes");
        let decoded = crate::artifacts::jpg::engine::decode_jpg(&bytes).expect("decode real jpg bytes");
        assert_eq!(decoded.width, semio.width);
        assert_eq!(decoded.height, semio.height);
        assert_eq!(decoded.pixels.len(), semio.frames[0].rgba8.len(), "lossy DCT — length matches, exact bytes need not");
    }
}
//#endregion 🔖️Tests
