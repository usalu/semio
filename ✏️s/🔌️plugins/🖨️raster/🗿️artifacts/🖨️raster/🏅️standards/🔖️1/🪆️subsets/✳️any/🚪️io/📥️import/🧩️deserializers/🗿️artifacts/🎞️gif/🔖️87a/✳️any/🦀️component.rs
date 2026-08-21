//! raster <- gif
use crate::artifacts::raster::schema::{create_raster_id, empty_raster_snapshot};
use crate::artifacts::raster::RasterSnapshot;
pub async fn register() {}
pub async fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_raster_snapshot();
    snap.id = create_raster_id("gif-import");
    snap.title = Some(format!("Imported gif"));
    Ok(snap)
}
