//! raster <- svg
use crate::artifacts::raster::schema::{empty_raster_snapshot, create_raster_id};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_raster_snapshot();
    snap.id = create_raster_id("svg-import");
    snap.title = Some(format!("Imported svg"));
    Ok(snap)
}
