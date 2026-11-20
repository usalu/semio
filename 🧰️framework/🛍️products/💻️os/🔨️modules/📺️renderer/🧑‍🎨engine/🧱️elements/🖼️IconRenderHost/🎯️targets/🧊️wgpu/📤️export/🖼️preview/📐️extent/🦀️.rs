//! 📐️ Preview texture extents under a straight-alpha RGBA byte budget.

const MAX_RASTER_EXTENT: f64 = 1024.0;
const RGBA_BYTES_PER_PIXEL: usize = 4;

pub(super) fn preview_extent(width: f32, height: f32, byte_budget: usize) -> Option<(u32, u32)> {
    let pixel_budget = byte_budget / RGBA_BYTES_PER_PIXEL;
    if pixel_budget == 0 {
        return None;
    }
    let [width, height] = [width, height].map(f64::from);
    let long_edge = width.max(height);
    let scale = 1.0_f64
        .min(MAX_RASTER_EXTENT / long_edge)
        .min((pixel_budget as f64 / (width * height)).sqrt())
        .min(pixel_budget as f64 / long_edge);
    let width = ((width * scale).floor() as usize).max(1).min(pixel_budget);
    let height = ((height * scale).floor() as usize).max(1).min(pixel_budget / width);
    Some((width as u32, height as u32))
}
