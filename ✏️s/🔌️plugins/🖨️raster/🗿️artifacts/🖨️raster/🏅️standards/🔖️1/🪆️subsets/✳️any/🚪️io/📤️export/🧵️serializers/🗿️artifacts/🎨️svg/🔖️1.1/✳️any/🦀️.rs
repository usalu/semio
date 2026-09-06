//! 📤️ raster → svg (1.1) — REAL. `raster_document_json_to_svg` maps the document's own visible
//! layer stack into a real `SemioDrawingSnapshot` (one `DrawNode::Image` per pixel layer, carrying
//! that layer's actual asset bytes and its own transform) and composes it to SVG text through
//! stdio's registered `s.stdio.semio/v1/drawing` → `s.stdio.svg` serializer. The bytes are a bare
//! `<svg>…</svg>` XML document — never `print_dsl` output.
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let (svg, _width, _height) = crate::artifacts::raster::io::raster_document_json_to_svg(snapshot)?;
    Ok(svg.into_bytes())
}
