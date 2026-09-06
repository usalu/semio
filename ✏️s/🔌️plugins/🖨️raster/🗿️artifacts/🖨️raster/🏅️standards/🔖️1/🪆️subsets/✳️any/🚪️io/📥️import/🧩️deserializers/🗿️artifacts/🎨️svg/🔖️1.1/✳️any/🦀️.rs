//! 📥️ raster ← svg (1.1) — REAL, host-tiered. A raster document holds pixels, so importing vector
//! markup requires actually RASTERIZING it. The only real vector renderer in this repo is
//! `semio_framework_os::rasterize_svg_to_png_base64` (usvg/resvg behind the framework's own
//! interface); its raw PNG output is then canonicalized through the real
//! `s.stdio.semio/v1/image` ↔ png codec, exactly as `raster_document_json_from_dwg` already does
//! for the DWG path (which is itself an SVG rasterization underneath).
//!
//! 🧾️ That renderer is native-tier only: inside a `wasm32-wasip2` guest it returns its own
//! "SVG rasterization requires the native semio-framework-os host" error, which this leaf
//! propagates verbatim rather than substituting a blank canvas.
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let svg = std::str::from_utf8(bytes).map_err(|error| format!("svg import: payload is not UTF-8 XML: {error}"))?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(svg, 0, 0)?;
    let raw = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| error.to_string())?;
    let image = crate::artifacts::raster::io::semio_image_from_png_bytes(&raw)?;
    crate::artifacts::raster::io::raster_document_from_semio_image(&image, "svg-import", "Imported svg")
}
