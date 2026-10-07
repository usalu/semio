//! 📤️ raster → jpg — REAL. The document's visible layer stack is flattened to one canonical
//! RGBA8 canvas by `raster_composite_image`, handed to stdio's own registered
//! `s.stdio.semio/v1/image` → `s.stdio.jpg` serializer, and written by stdio's own byte encoder.
//! This plugin owns no jpg byte codec and never will.
//!
//! 🧾️ JPEG is lossy by construction and carries no alpha; stdio's own codec forces alpha opaque on decode and re-quantizes on encode. Both are the FORMAT's losses, documented by that codec.
use crate::standards::v1::subsets::any::io::{raster_composite_image, semio_image_to_format, JPG_DIALECT};
use crate::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let image = raster_composite_image(snapshot).map_err(|reason| format!("jpg export not available for this raster document: {reason}"))?;
    let target: semio_s_artifact_stdio_jpg::JpgSnapshot = semio_image_to_format(&image, JPG_DIALECT)?;
    semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::io::encode_jpg(&target, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::from_frame(target.frame.as_ref())).map_err(|error| format!("{error:?}"))
}
