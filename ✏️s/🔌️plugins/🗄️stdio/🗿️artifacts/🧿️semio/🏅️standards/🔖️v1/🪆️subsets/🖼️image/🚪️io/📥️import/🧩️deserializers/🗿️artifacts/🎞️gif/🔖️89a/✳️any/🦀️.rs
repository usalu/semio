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

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
