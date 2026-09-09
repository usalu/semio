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

use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_tiff::{
    schema::snapshot::{TiffFieldType, TiffIfd, TiffTag, TiffValues, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH},
    TiffSnapshot,
};

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
        Ok(TiffSnapshot { schema: semio_s_artifact_stdio_tiff::STDIO_TIFF_DOCUMENT_SCHEMA.into(), byte_order: Default::default(), ifds: vec![TiffIfd { pixels: Vec::new(), entries }], pixels: frame.rgba8.clone() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
