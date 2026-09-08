//! 📤️ raster → dwg (ac1018) — HONESTLY UNSUPPORTED, registered so the router answers with THIS
//! sentence instead of a bare "no route". DWG is a vector entity model. stdio's own `s.stdio.semio/v1/drawing` → dwg serializer states in its own module doc that `Image` nodes have no DWG entity equivalent and are dropped — and a raster composite is nothing BUT image nodes, so the produced DWG would be an empty drawing. The IMPORT direction of this same format is genuinely real (see the sibling leaf); the asymmetry is the format's, not a gap here.
use crate::RasterSnapshot;
pub fn register() {}
/// 🚫️ Every node a raster document contributes is an Image node, which DWG cannot hold.
pub const RASTER_DWG_EXPORT_UNSUPPORTED: &str = "dwg export not supported for a raster document: a raster document contributes only DrawNode::Image nodes, and stdio's semio/drawing to dwg serializer has no DWG entity for those (its own module doc says they are dropped), so the result would be an empty drawing. Export png/bmp/tiff/jpg/gif for pixels, or svg for a vector container that does carry embedded images.";
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let _ = snapshot;
    Err(RASTER_DWG_EXPORT_UNSUPPORTED.to_string())
}
