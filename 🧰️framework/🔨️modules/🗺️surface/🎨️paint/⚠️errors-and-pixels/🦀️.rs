//! ⚠️ Errors and pixels of surface paint: the paint error with its conversions and the checkerboard, image and test-only brightness and blur pixel helpers. See [`super`].

use super::*;

//#region ⚠️ Errors
/// ⚠️ Raster-paint host errors — JSON decode failures, unsupported document schema, and image decode failures.
#[derive(Debug)]
pub enum FrameworkSurfacePaintError {
    Json(serde_json::Error),
    Image(String),
    UnsupportedSchema(String),
    Composite(String),
}

impl std::fmt::Display for FrameworkSurfacePaintError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => error.fmt(formatter),
            Self::Image(message) => formatter.write_str(message),
            Self::UnsupportedSchema(schema) => write!(formatter, "unsupported schema {schema}"),
            Self::Composite(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for FrameworkSurfacePaintError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Image(_) | Self::UnsupportedSchema(_) | Self::Composite(_) => None,
        }
    }
}

impl From<serde_json::Error> for FrameworkSurfacePaintError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<semio_framework_pixels::RasterError> for FrameworkSurfacePaintError {
    fn from(error: semio_framework_pixels::RasterError) -> Self {
        Self::Image(error.to_string())
    }
}
//#endregion ⚠️ Errors

// #region 🔖️Pixels
pub(super) fn checkerboard_rgba(width: u32, height: u32, light_cell: u8, dark_cell: u8) -> Vec<u8> {
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    let cell = 16u32;
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let on = ((x / cell) + (y / cell)).is_multiple_of(2);
            let v = if on { light_cell } else { dark_cell };
            rgba[idx] = v;
            rgba[idx + 1] = v;
            rgba[idx + 2] = v;
            rgba[idx + 3] = 255;
        }
    }
    rgba
}

pub(super) fn image_from_rgba(width: u32, height: u32, rgba: Vec<u8>) -> RasterImage {
    RasterImage::rgba8(width, height, Arc::new(rgba))
}

#[cfg(test)]
pub(super) fn apply_brightness_contrast(rgba: &mut [u8], brightness: f32, contrast: f32) {
    let b = brightness;
    let c = contrast;
    for px in rgba.chunks_exact_mut(4) {
        for ch in 0..3 {
            let v = px[ch] as f32 / 255.0;
            let adjusted = ((v - 0.5) * (1.0 + c) + 0.5 + b).clamp(0.0, 1.0);
            px[ch] = (adjusted * 255.0).round() as u8;
        }
    }
}

#[cfg(test)]
pub(super) fn apply_blur_box(rgba: &mut [u8], width: u32, height: u32, radius: u32) {
    if radius == 0 {
        return;
    }
    let r = radius.min(8);
    let src = rgba.to_vec();
    for y in 0..height {
        for x in 0..width {
            let mut acc = [0u32; 4];
            let mut count = 0u32;
            for dy in 0..=(r * 2) {
                for dx in 0..=(r * 2) {
                    let sx = x.saturating_add(dx).saturating_sub(r).min(width - 1);
                    let sy = y.saturating_add(dy).saturating_sub(r).min(height - 1);
                    let idx = ((sy * width + sx) * 4) as usize;
                    for c in 0..4 {
                        acc[c] += src[idx + c] as u32;
                    }
                    count += 1;
                }
            }
            let idx = ((y * width + x) * 4) as usize;
            for c in 0..4 {
                rgba[idx + c] = (acc[c] / count) as u8;
            }
        }
    }
}
// #endregion 🔖️Pixels
