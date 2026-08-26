//! raster <- png
use crate::artifacts::raster::schema::{create_raster_id, empty_raster_snapshot};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_raster_snapshot();
    snap.id = create_raster_id("png-import");
    snap.title = Some(format!("Imported png"));
    Ok(snap)
}
