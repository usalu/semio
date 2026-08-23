//! 🖼️ Raster-image oracles. Reference implementations for PNG and GIF creation, mutation and
//! projection, wrapped behind an owned interface so no external type escapes this module.
//!
//! The `semantic-raster-v1` profile compares dimensions, colour model, bit depth and the DECODED
//! samples. Filtering, interlacing, chunk order, compression level and ancillary metadata are
//! encoder choices, not normative content, and are canonicalized away.
//!
//! @see 📇️registry/🔣️component.json — the approved oracle registry these functions implement.

use semio_repo_test_host::Json;

//#region 🔖️RasterSpec
/// 🖼️ Owned description of a raster image: dimensions and 8-bit RGBA samples, row-major.
#[derive(Debug, Clone)]
pub struct RasterSpec {
    pub width: u32,
    pub height: u32,
    /// 🎨️ `width * height * 4` bytes, RGBA order.
    pub rgba: Vec<u8>,
}

impl RasterSpec {
    /// 🎨️ A deterministic gradient of the requested size — a fixed input both producers can be given.
    pub fn gradient(width: u32, height: u32) -> RasterSpec {
        let mut rgba = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height {
            for x in 0..width {
                rgba.push((x * 255 / width.max(1)) as u8);
                rgba.push((y * 255 / height.max(1)) as u8);
                rgba.push(((x + y) * 255 / (width + height).max(1)) as u8);
                rgba.push(255);
            }
        }
        RasterSpec { width, height, rgba }
    }

    /// 🎨️ Reads a spec out of a scenario's owned JSON payload.
    pub fn from_json(value: &Json) -> RasterSpec {
        let number = |key: &str, fallback: u32| match value.get(key) {
            Some(Json::Number(found)) => *found as u32,
            _ => fallback,
        };
        RasterSpec::gradient(number("width", 4), number("height", 4))
    }

    /// 🔁️ The projection every raster producer is compared through.
    pub fn projection(&self, format: &str) -> Json {
        Json::Object(vec![
            ("format".to_string(), Json::String(format.to_string())),
            ("width".to_string(), Json::Number(self.width as f64)),
            ("height".to_string(), Json::Number(self.height as f64)),
            ("channels".to_string(), Json::Number(4.0)),
            ("bitDepth".to_string(), Json::Number(8.0)),
            ("samples".to_string(), Json::Array(self.rgba.iter().map(|byte| Json::Number(*byte as f64)).collect())),
        ])
    }
}
//#endregion 🔖️RasterSpec

//#region 🔖️Png
/// 🔮️ Creates a PNG with the registered `png` reference implementation.
/// @see https://github.com/image-rs/image-png
#[cfg(feature = "oracles")]
pub fn oracle_create_png(spec: &RasterSpec) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, spec.width, spec.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|error| format!("png header: {}", error))?;
        writer.write_image_data(&spec.rgba).map_err(|error| format!("png data: {}", error))?;
    }
    Ok(out)
}

/// 👁️ Projects PNG bytes with the INDEPENDENT `png` decoder onto the owned `semantic-raster-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_png(input: &[u8]) -> Result<Json, String> {
    let decoder = png::Decoder::new(std::io::Cursor::new(input));
    let mut reader = decoder.read_info().map_err(|error| format!("independent reader could not parse the PNG: {}", error))?;
    let mut buffer = vec![0; reader.output_buffer_size().unwrap_or(0)];
    let frame = reader.next_frame(&mut buffer).map_err(|error| format!("independent reader could not decode the PNG: {}", error))?;
    let info = reader.info();
    let samples = rgba_from(&buffer[..frame.buffer_size()], frame.color_type, info.palette.as_deref(), info.trns.as_deref())?;
    Ok(RasterSpec { width: frame.width, height: frame.height, rgba: samples }.projection("png"))
}

#[cfg(feature = "oracles")]
fn rgba_from(buffer: &[u8], color: png::ColorType, palette: Option<&[u8]>, transparency: Option<&[u8]>) -> Result<Vec<u8>, String> {
    match color {
        png::ColorType::Rgba => Ok(buffer.to_vec()),
        png::ColorType::Rgb => Ok(buffer.chunks_exact(3).flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 255]).collect()),
        png::ColorType::Grayscale => Ok(buffer.iter().flat_map(|value| [*value, *value, *value, 255]).collect()),
        png::ColorType::GrayscaleAlpha => Ok(buffer.chunks_exact(2).flat_map(|pixel| [pixel[0], pixel[0], pixel[0], pixel[1]]).collect()),
        png::ColorType::Indexed => {
            let table = palette.ok_or("indexed PNG without a palette")?;
            Ok(buffer
                .iter()
                .flat_map(|index| {
                    let base = (*index as usize) * 3;
                    let alpha = transparency.and_then(|values| values.get(*index as usize).copied()).unwrap_or(255);
                    [table.get(base).copied().unwrap_or(0), table.get(base + 1).copied().unwrap_or(0), table.get(base + 2).copied().unwrap_or(0), alpha]
                })
                .collect())
        }
    }
}
//#endregion 🔖️Png

//#region 🔖️Gif
/// 🔮️ Creates a single-frame GIF with the registered `gif` reference implementation.
/// @see https://github.com/image-rs/image-gif
#[cfg(feature = "oracles")]
pub fn oracle_create_gif(spec: &RasterSpec) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    {
        let mut pixels = spec.rgba.clone();
        let frame = gif::Frame::from_rgba_speed(spec.width as u16, spec.height as u16, &mut pixels, 10);
        let mut encoder = gif::Encoder::new(&mut out, spec.width as u16, spec.height as u16, &[]).map_err(|error| format!("gif header: {}", error))?;
        encoder.write_frame(&frame).map_err(|error| format!("gif frame: {}", error))?;
    }
    Ok(out)
}

/// 👁️ Projects GIF bytes with the INDEPENDENT `gif` decoder. GIF is palette-based and lossy about
/// exact RGBA, so the projection reports the frame geometry and the palette-resolved sample count
/// rather than pretending the samples survive a quantizing round trip unchanged.
#[cfg(feature = "oracles")]
pub fn project_gif(input: &[u8]) -> Result<Json, String> {
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = options.read_info(input).map_err(|error| format!("independent reader could not parse the GIF: {}", error))?;
    let width = decoder.width();
    let height = decoder.height();
    let mut frames: Vec<Json> = Vec::new();
    while let Some(frame) = decoder.read_next_frame().map_err(|error| format!("independent reader could not decode a GIF frame: {}", error))? {
        frames.push(Json::Object(vec![
            ("width".to_string(), Json::Number(frame.width as f64)),
            ("height".to_string(), Json::Number(frame.height as f64)),
            ("left".to_string(), Json::Number(frame.left as f64)),
            ("top".to_string(), Json::Number(frame.top as f64)),
            ("opaqueSamples".to_string(), Json::Number(frame.buffer.chunks_exact(4).filter(|pixel| pixel[3] == 255).count() as f64)),
        ]));
    }
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("gif".to_string())),
        ("width".to_string(), Json::Number(width as f64)),
        ("height".to_string(), Json::Number(height as f64)),
        ("frameCount".to_string(), Json::Number(frames.len() as f64)),
        ("frames".to_string(), Json::Array(frames)),
    ]))
}
//#endregion 🔖️Gif

//#region 🔖️ImageCrateFormats
/// 🔮️ Creates a BMP, TIFF or JPEG with the registered `image` reference implementation. One crate is
/// the reference decoder/encoder for all three, so one oracle covers all three formats.
/// @see https://github.com/image-rs/image
#[cfg(feature = "oracles")]
pub fn oracle_create_image(spec: &RasterSpec, format: &str) -> Result<Vec<u8>, String> {
    let buffer = image::RgbaImage::from_raw(spec.width, spec.height, spec.rgba.clone()).ok_or("raster spec does not fill width * height * 4 bytes")?;
    let mut out = std::io::Cursor::new(Vec::new());
    match format {
        // 🧭️BMP and JPEG have no alpha channel, so the reference encoder is given RGB — the
        // projection compares what a conforming reader recovers, not a channel the format cannot carry.
        "bmp" => image::DynamicImage::ImageRgba8(buffer).to_rgb8().write_to(&mut out, image::ImageFormat::Bmp).map_err(|error| format!("bmp encode: {}", error))?,
        "jpg" => image::DynamicImage::ImageRgba8(buffer).to_rgb8().write_to(&mut out, image::ImageFormat::Jpeg).map_err(|error| format!("jpeg encode: {}", error))?,
        "tiff" => buffer.write_to(&mut out, image::ImageFormat::Tiff).map_err(|error| format!("tiff encode: {}", error))?,
        other => return Err(format!("no reference encoder registered for {}", other)),
    }
    Ok(out.into_inner())
}

/// 👁️ Projects BMP/TIFF/JPEG bytes with the INDEPENDENT `image` decoder.
///
/// JPEG is LOSSY, so its projection deliberately reports geometry and a coarse sample histogram
/// rather than exact samples: asserting byte-equal pixels through a lossy codec would be a test that
/// can only ever pass by accident.
#[cfg(feature = "oracles")]
pub fn project_image(input: &[u8], format: &str) -> Result<Json, String> {
    let decoded = image::load_from_memory(input).map_err(|error| format!("independent reader could not parse the {}: {}", format, error))?;
    let rgba = decoded.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    if format == "jpg" {
        let mut buckets = [0u32; 8];
        for pixel in rgba.chunks_exact(4) {
            let luma = (u32::from(pixel[0]) * 299 + u32::from(pixel[1]) * 587 + u32::from(pixel[2]) * 114) / 1000;
            buckets[(luma / 32).min(7) as usize] += 1;
        }
        return Ok(Json::Object(vec![
            ("format".to_string(), Json::String("jpg".to_string())),
            ("width".to_string(), Json::Number(width as f64)),
            ("height".to_string(), Json::Number(height as f64)),
            ("lossy".to_string(), Json::Bool(true)),
            ("lumaHistogram".to_string(), Json::Array(buckets.iter().map(|count| Json::Number(*count as f64)).collect())),
        ]));
    }
    Ok(RasterSpec { width, height, rgba: rgba.into_raw() }.projection(format))
}
//#endregion 🔖️ImageCrateFormats

//#region 🔖️Unavailable
/// 🚫️ Without the `oracles` feature the reference implementations are not linked, and every entry
/// point fails loudly. A missing oracle must never degrade into a silently skipped test.
#[cfg(not(feature = "oracles"))]
mod unavailable {
    use super::{Json, RasterSpec};
    const MESSAGE: &str = "the `oracles` feature is disabled — this host was not built with the registered reference implementations";

    pub fn create_png(_spec: &RasterSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_png(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn create_gif(_spec: &RasterSpec) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_gif(_input: &[u8]) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
    pub fn create_image(_spec: &RasterSpec, _format: &str) -> Result<Vec<u8>, String> {
        Err(MESSAGE.to_string())
    }
    pub fn project_image(_input: &[u8], _format: &str) -> Result<Json, String> {
        Err(MESSAGE.to_string())
    }
}

#[cfg(not(feature = "oracles"))]
pub use unavailable::{create_gif as oracle_create_gif, create_image as oracle_create_image, create_png as oracle_create_png, project_gif, project_image, project_png};
//#endregion 🔖️Unavailable
