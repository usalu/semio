//! 📏️ Chooses how a picked raster reaches the editor: the inline data URL, the paged PNG placement or the paged DCT resource, each limit derived from the lane that serves it.
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::png_source::{PNG_GRAPH_MAXIMUM_BYTES, PNG_GRAPH_MAXIMUM_PIXELS};

/// 📦️ The largest paged DCT input: the proposed original owner capacity, within the raw media wire authority.
pub const PAGED_DCT_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;
/// 🧮️ Baseline gray and RGB DCT admits 64 Mi pixels.
pub const PAGED_DCT_MAXIMUM_PIXELS: u64 = 64 * 1024 * 1024;
const _: () = assert!(PAGED_DCT_MAXIMUM_BYTES <= semio_plugin::original_media::RAW_MEDIA_BYTES);
/// 🔤️ The inline data-URL lane: the picker's PNG file ceiling and sampled pixel ceiling.
pub const INLINE_MAXIMUM_BYTES: usize = 24_546;
pub const INLINE_MAXIMUM_PIXELS: usize = 4_096;
const _: () = assert!(PNG_GRAPH_MAXIMUM_BYTES >= INLINE_MAXIMUM_BYTES);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PickedRasterFormat { Png, Jpeg }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PickedRasterRoute { Inline, PagedPng, PagedDct, Refused }

/// 🧭️ Selects the lane from the declared format, pixel extent and encoded size; the inline lane stays the only route below its byte ceiling.
pub fn select_picked_raster_route(format: PickedRasterFormat, width: u32, height: u32, file_bytes: usize) -> PickedRasterRoute {
    let pixels = u64::from(width) * u64::from(height);
    if width == 0 || height == 0 || file_bytes == 0 {
        return PickedRasterRoute::Refused;
    }
    match format {
        PickedRasterFormat::Png if file_bytes <= INLINE_MAXIMUM_BYTES && pixels <= INLINE_MAXIMUM_PIXELS as u64 => PickedRasterRoute::Inline,
        PickedRasterFormat::Png if file_bytes <= PNG_GRAPH_MAXIMUM_BYTES && pixels <= PNG_GRAPH_MAXIMUM_PIXELS as u64 => PickedRasterRoute::PagedPng,
        PickedRasterFormat::Jpeg if file_bytes <= PAGED_DCT_MAXIMUM_BYTES && pixels <= PAGED_DCT_MAXIMUM_PIXELS => PickedRasterRoute::PagedDct,
        _ => PickedRasterRoute::Refused,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picked_raster_routes_match_the_pillow_measured_law() {
        let law: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🖼️image-route/🔣️.json")).unwrap();
        assert_eq!(law["inlineBytes"].as_u64().unwrap(), INLINE_MAXIMUM_BYTES as u64);
        assert_eq!(law["inlinePixels"].as_u64().unwrap(), INLINE_MAXIMUM_PIXELS as u64);
        assert_eq!(law["pngBytes"].as_u64().unwrap() as usize, PNG_GRAPH_MAXIMUM_BYTES);
        assert_eq!(law["pngPixels"].as_u64().unwrap() as usize, PNG_GRAPH_MAXIMUM_PIXELS);
        assert_eq!(law["dctBytes"].as_u64().unwrap() as usize, PAGED_DCT_MAXIMUM_BYTES);
        assert_eq!(law["dctPixels"].as_u64().unwrap(), PAGED_DCT_MAXIMUM_PIXELS);
        for row in law["cases"].as_array().unwrap() {
            let format = if row["format"] == "png" { PickedRasterFormat::Png } else { PickedRasterFormat::Jpeg };
            let route = select_picked_raster_route(format, row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32, row["fileBytes"].as_u64().unwrap() as usize);
            let expected = match row["route"].as_str().unwrap() { "inline" => PickedRasterRoute::Inline, "pagedPng" => PickedRasterRoute::PagedPng, "pagedDct" => PickedRasterRoute::PagedDct, _ => PickedRasterRoute::Refused };
            assert_eq!(route, expected, "{}", row["name"]);
        }
    }
}
