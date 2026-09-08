//! 📥️ raster ← gif (87a) — REAL. stdio's own `87a` `decode_gif` reads the file, `io::gif87a::to_89a`
//! remaps it onto the model stdio's own registered `s.stdio.gif@89a` → `s.stdio.semio/v1/image`
//! deserializer accepts (that leaf owns the palette→RGBA decode, via the codec's own `GifFrame::rgba`
//! accessor), and the resulting canvas becomes one `Pixel` layer with a materialized asset child.
//!
//! 🧾️ Only the FIRST image of a multi-image GIF87a reaches the document: a raster document has no
//! frame/animation concept, so the semio/image hub's own frame list collapses to its first frame.
use crate::artifacts::raster::io::{gif87a, raster_document_from_semio_image, semio_image_from_format, GIF89A_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let gif87a_snapshot = semio_s_artifact_stdio_gif::standards::v87a::subsets::any::io::decode_gif(bytes)?;
    let image = semio_image_from_format(&gif87a::to_89a(&gif87a_snapshot), GIF89A_DIALECT)?;
    raster_document_from_semio_image(&image, "gif-import", "Imported gif")
}
