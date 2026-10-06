//! 📤️ SVG export embeds the canonical layer composite through the shared drawing serializer.
use crate::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let (svg, _width, _height) = crate::standards::v1::subsets::any::io::raster_document_json_to_svg(snapshot)?;
    Ok(svg.into_bytes())
}
