//! 📤️ raster → gif (87a) — REAL. The composite goes through stdio's own registered
//! `s.stdio.semio/v1/image` → `s.stdio.gif@89a` serializer (whose `quantize` does the real,
//! exact 1:1 RGBA→palette reduction and errors past 256 distinct colors rather than approximating),
//! then `io::gif87a::from_89a` remaps that snapshot onto stdio's own `87a` model, which stdio's own
//! `encode_gif` writes as genuine `GIF87a` bytes.
//!
//! 🧾️ GIF87a has no Graphic Control Extension: per-frame delay, disposal and transparency cannot
//! exist in this dialect at all, and the remap drops them for that reason. A raster composite is a
//! single still frame, so only transparency is materially lost — a fully transparent source pixel
//! is normalized to opaque black by the 89a quantizer's own documented rule.
use crate::artifacts::raster::io::{gif87a, raster_composite_image, semio_image_to_format, GIF89A_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
use semio_s_plugin_stdio::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot as Gif89aSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let image = raster_composite_image(snapshot).map_err(|reason| format!("gif export not available for this raster document: {reason}"))?;
    let gif89a: Gif89aSnapshot = semio_image_to_format(&image, GIF89A_DIALECT)?;
    semio_s_plugin_stdio::artifacts::gif::standards::v87a::subsets::any::io::encode_gif(&gif87a::from_89a(&gif89a))
}
