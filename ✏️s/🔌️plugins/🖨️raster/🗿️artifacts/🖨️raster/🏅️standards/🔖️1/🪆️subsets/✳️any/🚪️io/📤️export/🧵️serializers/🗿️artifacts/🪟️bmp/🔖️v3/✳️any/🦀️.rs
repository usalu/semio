//! raster -> bmp
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    snapshot.require_empty_output_shell().map_err(str::to_owned)?;
    Ok(<RasterSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
