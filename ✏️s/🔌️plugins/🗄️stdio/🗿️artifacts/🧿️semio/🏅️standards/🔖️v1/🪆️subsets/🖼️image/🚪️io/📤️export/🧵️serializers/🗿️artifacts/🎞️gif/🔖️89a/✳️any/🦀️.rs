//! 📤️ `s.stdio.semio/v1/image` → `gif` (89a) — the real, honest impedance mismatch: GIF is
//! palette-indexed (max 256 colors/frame) while semio's frame is canonical RGBA8. This leaf does
//! real 1:1 color quantization (build a palette from the frame's actual distinct colors — exact,
//! not lossy-approximated — and error out, per the recipe's "error out" allowance, when a frame
//! genuinely has more than 256 distinct colors, since GIF's palette hard-caps there and silently
//! downsampling would fabricate color data). A fully transparent source pixel (`a == 0`) is
//! normalized to RGB `(0,0,0)` before quantizing (matching `GifFrame::rgba`'s own decode-side
//! normalization, so re-importing the produced GIF is a fixed point) and reserves the palette's
//! transparent index.
//!
//! Honest lossy points (documented):
//! - `delay_ms` → `delay_cs` truncates to 10ms (GCE delay unit) precision.
//! - A single shared Global Color Table is built from the UNION of every frame's colors (kept
//!   simple — no per-frame Local Color Tables); `metadata` keys other than `comment`/`loopCount`
//!   have no home on `GifSnapshot` and are dropped; per-frame region/disposal are not
//!   reconstructed (every frame covers the full canvas, matching the import leaf's own
//!   normalization).
/// 🎞️ Shared palette, indexed frames, and optional transparency index.
pub type QuantizedFrames = (GifColorTable, Vec<Vec<u8>>, Option<u8>);

use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb, GifSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId::ANY };

//#region 🔖️Quantize
/// 🎨️ Real 1:1 RGBA→indexed quantization over ALL frames at once (shared GCT): every distinct
/// opaque `(r,g,b)` (and one canonical `(0,0,0)` slot for any transparent pixel) becomes exactly
/// one palette entry, in first-seen order. Errors — never silently drops colors — once a 257th
/// distinct entry would be needed (GIF's real `2..=256` palette-size ceiling).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn quantize(frames: &[&[u8]]) -> Result<QuantizedFrames, store::PackError> {
    let mut colors: Vec<(u8, u8, u8)> = Vec::new();
    let mut transparent_index: Option<u8> = None;
    let mut indexed_frames = Vec::with_capacity(frames.len());
    for rgba in frames {
        let mut indices = Vec::with_capacity(rgba.len() / 4);
        for px in rgba.as_chunks::<4>().0 {
            let is_transparent = px[3] == 0;
            let rgb = if is_transparent { (0u8, 0u8, 0u8) } else { (px[0], px[1], px[2]) };
            let idx = match colors.iter().position(|&c| c == rgb) {
                Some(pos) => pos,
                None => {
                    if colors.len() >= 256 {
                        return Err(store::PackError::Schema("semio/image→gif: frame has more than 256 distinct colors — GIF's palette cannot represent it losslessly".into()));
                    }
                    colors.push(rgb);
                    colors.len() - 1
                }
            };
            if is_transparent {
                transparent_index = Some(idx as u8);
            }
            indices.push(idx as u8);
        }
        indexed_frames.push(indices);
    }
    let mut padded = colors.clone();
    let target = padded.len().max(2).next_power_of_two().min(256);
    padded.resize(target, (0, 0, 0));
    let table = GifColorTable { sorted: false, colors: padded.into_iter().map(|(r, g, b)| GifRgb { r, g, b }).collect() };
    Ok((table, indexed_frames, transparent_index))
}
//#endregion 🔖️Quantize

//#region 🔖️Serializer
pub struct SemioImageToGif;

impl ArtifactSerializer for SemioImageToGif {
    type From = SemioImageSnapshot;
    type Into = GifSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.frames.is_empty() {
            return Err(store::PackError::Schema("semio/image→gif: no frames to export".into()));
        }
        let expected_len = (from.width as usize) * (from.height as usize) * 4;
        let refs: Vec<&[u8]> = from.frames.iter().map(|f| f.rgba8.as_slice()).collect();
        for r in &refs {
            if r.len() != expected_len {
                return Err(store::PackError::Schema("semio/image→gif: a frame's pixel length does not match width*height*4".into()));
            }
        }
        let (gct, indexed_frames, transparent_index) = quantize(&refs)?;
        let frames = from.frames.iter().zip(indexed_frames).map(|(f, indices)| GifFrame { left: 0, top: 0, width: from.width, height: from.height, indices, delay_cs: (f.delay_ms / 10) as u16, transparent_index, ..GifFrame::default() }).collect();
        let comments = from.metadata.iter().filter(|m| m.key == "comment").map(|m| m.value.clone()).collect();
        let loop_count = from.metadata.iter().find(|m| m.key == "loopCount").and_then(|m| m.value.parse::<u16>().ok());
        Ok(GifSnapshot {
            schema: semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
            width: from.width,
            height: from.height,
            gct: Some(gct),
            loop_count,
            comments,
            frames,
            ..GifSnapshot::default()
        })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
