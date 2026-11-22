//! 🖼️ A [`VideoRenderProgram`] as the vector scenes the wgpu rasterizer paints: its path table as Bézier paths, its pictures decoded to
//! straight RGBA8 by the framework's own PNG and JPEG codecs, and each distinct scene as one [`VectorScene`].

use semio_framework_geometry::{Affine, BezPath, Point};
use semio_framework_kernel_media_jobs::{VideoRenderOp, VideoRenderPath, VideoRenderProgram};
use semio_framework_pixels::image_decoding::{decode_encoded_blocking, EncodedDecodeLimits};
use semio_framework_raster::{ImageOp, VectorScene};
use std::sync::Arc;

/// 🖼️ One decoded picture of a program: straight RGBA8 rows.
#[derive(Clone, Debug)]
pub struct VideoRenderPicture {
    pub pixels: Arc<Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

/// ✏️ A program path (`M`, `L`, `Q`, `C`, `Z` over flat points) as a [`BezPath`].
pub fn video_render_path(path: &VideoRenderPath) -> BezPath {
    let mut shape = BezPath::new();
    let mut cursor = 0;
    let mut next = || {
        let value = path.points.get(cursor).copied().unwrap_or(0.0);
        cursor += 1;
        value
    };
    let mut point = || Point::new(next(), next());
    for verb in path.verbs.chars() {
        match verb {
            'M' => shape.move_to(point()),
            'L' => shape.line_to(point()),
            'Q' => shape.quad_to(point(), point()),
            'C' => shape.curve_to(point(), point(), point()),
            _ => shape.close_path(),
        }
    }
    shape
}

fn percent_decode(payload: &str) -> Result<Vec<u8>, String> {
    let bytes = payload.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'%' {
            let pair = payload.get(at + 1..at + 3).ok_or("data URL ends inside a percent escape")?;
            out.push(u8::from_str_radix(pair, 16).map_err(|_| "data URL holds a malformed percent escape")?);
            at += 3;
        } else {
            out.push(bytes[at]);
            at += 1;
        }
    }
    Ok(out)
}

/// 🔗️ The encoded bytes behind a program picture URL: an inline `data:image/…` URL is read here, a same-origin path is asked of
/// `resolve` (the host's own asset door).
pub fn video_render_picture_bytes(url: &str, resolve: &dyn Fn(&str) -> Result<Vec<u8>, String>) -> Result<Vec<u8>, String> {
    let Some(body) = url.strip_prefix("data:") else { return resolve(url) };
    let (meta, payload) = body.split_once(',').ok_or("data URL has no payload")?;
    if meta.split(';').any(|part| part.eq_ignore_ascii_case("base64")) {
        semio_framework_io_base64::base64_standard_decode(payload).map_err(|error| error.to_string())
    } else {
        percent_decode(payload)
    }
}

/// 🖼️ Decodes picture `url` (PNG or JPEG, by signature) upright, stopping with an error as soon as `cancelled` answers true.
pub fn decode_video_render_picture(url: &str, resolve: &dyn Fn(&str) -> Result<Vec<u8>, String>, cancelled: &dyn Fn() -> bool) -> Result<VideoRenderPicture, String> {
    let bytes = video_render_picture_bytes(url, resolve)?;
    let decoded = decode_encoded_blocking(bytes, EncodedDecodeLimits::MAXIMUM, cancelled).map_err(|error| error.to_string())?.upright();
    Ok(VideoRenderPicture { width: decoded.width, height: decoded.height, pixels: Arc::new(decoded.pixels) })
}

/// 🎞️ Scene `scene` of `program` over `paths` (its path table, built once) and `pictures`, every op in list order.
pub fn video_render_scene(program: &VideoRenderProgram, scene: usize, paths: &[BezPath], pictures: &[VideoRenderPicture]) -> VectorScene {
    let mut out = VectorScene::new();
    let affine = |t: &[f64; 6]| Affine::new(*t);
    let tint = |color: &[f64; 4]| [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
    for op in program.scenes.get(scene).map_or(&[][..], |scene| scene.ops.as_slice()) {
        match op {
            VideoRenderOp::Fill { path, transform, color } => {
                if let Some(path) = paths.get(*path as usize) {
                    out.fill(path.clone(), affine(transform), tint(color));
                }
            }
            VideoRenderOp::Stroke { path, transform, color, width } => {
                if let Some(path) = paths.get(*path as usize) {
                    out.stroke(path.clone(), affine(transform), tint(color), *width);
                }
            }
            VideoRenderOp::Image { image, crop, transform, opacity } => {
                if let Some(picture) = pictures.get(*image as usize) {
                    out.image(ImageOp { pixels: picture.pixels.clone(), width: picture.width, height: picture.height, crop: *crop, transform: affine(transform), opacity: *opacity as f32 });
                }
            }
        }
    }
    out
}
