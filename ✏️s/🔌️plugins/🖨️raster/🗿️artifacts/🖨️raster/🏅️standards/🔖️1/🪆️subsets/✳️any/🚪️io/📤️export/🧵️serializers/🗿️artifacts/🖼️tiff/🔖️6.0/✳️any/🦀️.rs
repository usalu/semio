//! 📤️ raster → tiff — REAL. The document's visible layer stack is flattened to one canonical
//! RGBA8 canvas by `raster_composite_image`, handed to stdio's own registered
//! `s.stdio.semio/v1/image` → `s.stdio.tiff` serializer, and written by stdio's own byte encoder.
//! This plugin owns no tiff byte codec and never will.
//!
//! 🧾️ stdio's TIFF codec decodes/encodes IFD 0 as canonical RGBA8 strips.
use crate::artifacts::raster::io::{raster_composite_image, semio_image_to_format, TIFF_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let image = raster_composite_image(snapshot).map_err(|reason| format!("tiff export not available for this raster document: {reason}"))?;
    let target: semio_s_artifact_stdio_tiff::TiffSnapshot = semio_image_to_format(&image, TIFF_DIALECT)?;
    semio_s_artifact_stdio_tiff::io::encode_tiff(&target)
}
