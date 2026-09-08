//! 📤️ raster → pdf (1.4) — HONESTLY UNSUPPORTED, registered so the router answers with THIS
//! sentence instead of a bare "no route". `PdfSnapshot`'s entire per-page model is `{width, height, text}` (see stdio's own `📖️pdf/🏅️standards/4️⃣1.4/…/🧬️schema/📸️snapshot/🦀️.rs`) — there is no image XObject, no content-stream painting operator and no `encode_pdf` path that could carry one pixel. stdio's own `s.stdio.semio/v1/drawing` → pdf serializer says the same thing in its own module doc: it drops every `Path` and `Image` node and writes text only.
use crate::RasterSnapshot;
pub fn register() {}
/// 🚫️ A raster composite has no representable form in this repo's PDF model.
pub const RASTER_PDF_EXPORT_UNSUPPORTED: &str = "pdf export not supported for a raster document: this repo's PdfSnapshot models a page as {width, height, text} only — it has no image XObject or path-painting operator, so a pixel composite has nowhere to go. Export png/bmp/tiff/jpg/gif instead.";
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let _ = snapshot;
    Err(RASTER_PDF_EXPORT_UNSUPPORTED.to_string())
}
