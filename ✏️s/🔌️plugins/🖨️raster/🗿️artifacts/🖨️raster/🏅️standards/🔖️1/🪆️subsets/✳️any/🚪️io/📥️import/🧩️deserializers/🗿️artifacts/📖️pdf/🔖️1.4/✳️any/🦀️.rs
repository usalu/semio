//! 📥️ raster ← pdf (1.4) — HONESTLY UNSUPPORTED, registered so the router answers with THIS
//! sentence instead of a bare "no route". The same page model runs the other way: `decode_pdf` yields `{width, height, text}` per page and no pixels, so there is nothing a raster document could be built out of.
use crate::RasterSnapshot;
pub fn register() {}
/// 🚫️ This repo's PDF model decodes no pixels.
pub const RASTER_PDF_IMPORT_UNSUPPORTED: &str =
    "pdf import not supported for a raster document: this repo's PdfSnapshot decodes a page as {width, height, text} only and carries no pixel data or embedded image, so no raster layer can be built from it. Import png/bmp/tiff/jpg/gif/svg instead.";
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let _ = bytes;
    Err(RASTER_PDF_IMPORT_UNSUPPORTED.to_string())
}
