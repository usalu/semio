//! 📥️ `gif` (89a) → `s.stdio.semio/v1/image` — GIF is palette-indexed, so unlike the other four
//! raster bridges this leaf does real work: it decodes each frame's `indices` through its own
//! (or the global) color table via `GifFrame::rgba` — the codec's OWN accessor, reused verbatim
//! (binary transparency: a transparent-index pixel normalizes to `[0,0,0,0]`, per that fn's doc).
//!
//! Honest lossy points (documented):
//! - `colorspace` is always recorded as `Indexed` (GIF's real on-disk representation).
//! - Per-frame region (`left`/`top`/sub-rectangle redraw) and `disposal` are dropped — semio's
//!   frame model has no region/disposal concept, only a full canonical RGBA8 canvas per frame.
//! - `icc`: GIF has no ICC chunk concept — always `None`.
//! - `metadata`: only `comments` (Comment Extension bodies) become entries (`key: "comment"`);
//!   `app_extensions` (NETSCAPE2.0 loop count is separately modeled, others verbatim) and
//!   `plain_text` blocks have no textual home on `SemioImageMetadataEntry` and are dropped.

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

//#region 🔖️Deserializer
pub struct SemioImageFromGif;

impl ArtifactDeserializer for SemioImageFromGif {
    type From = GifSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.frames.is_empty() {
            return Err(store::PackError::Schema("gif→semio/image: at least one frame is required".into()));
        }
        let frames = from.frames.iter().map(|f| SemioImageFrame { delay_ms: (f.delay_cs as u32) * 10, rgba8: f.rgba(from.gct.as_ref()) }).collect();
        let mut metadata: Vec<SemioImageMetadataEntry> = from.comments.iter().map(|c| SemioImageMetadataEntry { key: "comment".into(), value: c.clone() }).collect();
        if let Some(n) = from.loop_count {
            metadata.push(SemioImageMetadataEntry { key: "loopCount".into(), value: n.to_string() });
        }
        Ok(SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width: from.width, height: from.height, colorspace: SemioColorspace::Indexed, bit_depth: 8, frames, icc: None, metadata })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_gif() -> GifSnapshot {
        GifSnapshot {
            width: 2,
            height: 1,
            gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 255, g: 0, b: 0 }, GifRgb { r: 0, g: 255, b: 0 }] }),
            loop_count: Some(0),
            comments: vec!["semio fixture".into()],
            frames: vec![GifFrame { left: 0, top: 0, width: 2, height: 1, indices: vec![0, 1], delay_cs: 10, ..GifFrame::default() }],
            ..GifSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn decodes_indices_through_gct_and_maps_comments() {
        let semio = semio_framework_plugin::resolve_ready(SemioImageFromGif::deserialize(&sample_gif())).expect("deserialize");
        assert_eq!(semio.width, 2);
        assert_eq!(semio.height, 1);
        assert_eq!(semio.colorspace, SemioColorspace::Indexed);
        assert_eq!(semio.frames.len(), 1);
        assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
        assert_eq!(semio.frames[0].delay_ms, 100);
        assert!(semio.metadata.iter().any(|m| m.key == "comment" && m.value == "semio fixture"));
        assert!(semio.metadata.iter().any(|m| m.key == "loopCount" && m.value == "0"));
    }
}
//#endregion 🔖️Tests
