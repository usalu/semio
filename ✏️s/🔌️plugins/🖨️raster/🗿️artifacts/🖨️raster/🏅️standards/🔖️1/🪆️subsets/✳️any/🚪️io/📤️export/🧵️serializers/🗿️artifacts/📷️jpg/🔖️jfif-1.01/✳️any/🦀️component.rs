//! raster -> jpg
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
