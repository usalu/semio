//! raster <- pdf
use crate::artifacts::raster::engine::{empty_raster_snapshot, create_raster_id};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_raster_snapshot();
    snap.id = create_raster_id("pdf-import", b"pdf");
    snap.title = Some(format!("Imported pdf"));
    Ok(snap)
}
