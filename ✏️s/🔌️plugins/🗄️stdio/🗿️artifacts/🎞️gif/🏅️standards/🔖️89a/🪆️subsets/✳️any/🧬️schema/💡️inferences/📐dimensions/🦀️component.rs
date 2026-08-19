//! 📐 `dimensions` — one named inference: the GIF89a logical screen's geometry, a pure O(1) read
//! of already-decoded header fields — nothing here is per-entity/incremental, so this holds only
//! the value type + its pure `compute` fn (no `InferredField`).

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Dimensions
/// 📐️ GIF89a logical-screen-derived raster geometry. `bit_depth` reads the Global Color Table's
/// `colors.len()` (GIF89a §18's "size of Global Color Table" field is exactly this, log2'd),
/// falling back to `8` (the spec's own max indexed depth) when no GCT is present — a real GIF89a
/// file with no GCT still color-resolves every frame through its own per-frame Local Color Table,
/// which this whole-snapshot scalar doesn't drill into. `has_alpha` IS exact: it reads every
/// frame's Graphic Control Extension `transparent_index` (§23.c.4) directly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifDimensions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub has_alpha: bool,
    pub pixel_count: u64,
}

/// 🔢️ `ceil(log2(colors.max(2)))`, clamped to `8` — GIF89a §18's "size of Global/Local Color
/// Table" field is exactly this value (the on-disk field stores `size - 1`).
async fn color_table_bit_depth(colors_len: usize) -> u8 {
    let n = colors_len.max(2);
    ((usize::BITS - (n - 1).leading_zeros()) as u8).min(8)
}

/// 📐️ Computes [`GifDimensions`] from a snapshot's screen descriptor + GCT + frames' GCE — pure,
/// total, O(frames) (a single linear pass over `frames` for `transparent_index`).
pub async fn compute_gif_dimensions(snapshot: &GifSnapshot) -> GifDimensions {
    let bit_depth = match &snapshot.gct {
        Some(table) => color_table_bit_depth(table.colors.len()),
        None => 8,
    };
    let has_alpha = snapshot.frames.iter().any(|frame| frame.transparent_index.is_some());
    GifDimensions { width: snapshot.width, height: snapshot.height, bit_depth, has_alpha, pixel_count: snapshot.width as u64 * snapshot.height as u64 }
}
//#endregion 🔖️Dimensions

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifFrame, GifRgb};

    #[semio_framework_async_macros::async_test]
    async fn derives_bit_depth_from_global_color_table_size() {
        let gct = GifColorTable { sorted: false, colors: vec![GifRgb::default(); 4] };
        let snapshot = GifSnapshot { width: 3, height: 2, gct: Some(gct), ..GifSnapshot::default() };
        let dims = compute_gif_dimensions(&snapshot);
        assert_eq!(dims.bit_depth, 2);
        assert!(!dims.has_alpha);
    }

    #[semio_framework_async_macros::async_test]
    async fn has_alpha_when_any_frame_declares_a_transparent_index() {
        let frame = GifFrame { transparent_index: Some(0), ..GifFrame::default() };
        let snapshot = GifSnapshot { frames: vec![frame], ..GifSnapshot::default() };
        assert!(compute_gif_dimensions(&snapshot).has_alpha);
    }
}
//#endregion 🧪️Tests
