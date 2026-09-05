//! 📥️ `jpg` (jfif-1.01) → `s.stdio.semio/v1/image` — `decode_jpg` already normalizes to
//! canonical RGBA8 `pixels` (alpha forced opaque, JPEG has no alpha channel), so this leaf is a
//! pure struct remap.
//!
//! Honest lossy points (documented):
//! - `colorspace` is always recorded as `Rgb` (JPEG/JFIF has no alpha channel; the canonical
//!   `rgba8` buffer's alpha byte is a decode-time fabrication the jpg codec itself adds, not a
//!   real source channel).
//! - `icc`: not modeled by `JpgSnapshot` (only the JFIF APP0 thumbnail/density fields, SOF/DQT/
//!   DHT, and verbatim `other_segments` are typed) — always `None` on import.
//! - `metadata`: only `COM` (comment, marker `0xFE`) segments become metadata entries
//!   (`key: "comment"`); every other `other_segments` entry (unrecognized APPn, etc.) has no
//!   textual home on `SemioImageMetadataEntry` and is dropped.

use crate::artifacts::jpg::{schema::snapshot::JpgSegment, JpgSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

/// 🏷️ JPEG `COM` marker (ISO/IEC 10918-1 Annex B.2.4).
const COM_MARKER: u8 = 0xFE;

//#region 🔖️Deserializer
pub struct SemioImageFromJpg;

impl ArtifactDeserializer for SemioImageFromJpg {
    type From = JpgSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.pixels.len() != (from.width as usize) * (from.height as usize) * 4 {
            return Err(store::PackError::Schema("jpg→semio/image: pixels length does not match width*height*4".into()));
        }
        let bit_depth = from.frame.as_ref().map(|f| f.precision).unwrap_or(8);
        let metadata = from.other_segments.iter().filter(|s: &&JpgSegment| s.marker == COM_MARKER).map(|s| SemioImageMetadataEntry { key: "comment".into(), value: String::from_utf8_lossy(&s.data).into_owned() }).collect();
        Ok(SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: from.width,
            height: from.height,
            colorspace: SemioColorspace::Rgb,
            bit_depth,
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_jpg() -> JpgSnapshot {
        JpgSnapshot { width: 2, height: 1, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255], other_segments: vec![JpgSegment { marker: COM_MARKER, data: b"semio fixture".to_vec() }], ..JpgSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn maps_pixels_and_comment() {
        let semio = semio_framework_plugin::resolve_ready(SemioImageFromJpg::deserialize(&sample_jpg())).expect("deserialize");
        assert_eq!(semio.width, 2);
        assert_eq!(semio.height, 1);
        assert_eq!(semio.colorspace, SemioColorspace::Rgb);
        assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
        assert_eq!(semio.metadata.len(), 1);
        assert_eq!(semio.metadata[0].key, "comment");
        assert_eq!(semio.metadata[0].value, "semio fixture");
    }
}
//#endregion 🔖️Tests
