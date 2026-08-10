//! ➕add-layer `RasterMutation` apply leaf.
use crate::artifacts::raster::RasterSnapshot;
use crate::artifacts::raster::mutations::RasterMutation;

pub fn apply(snapshot: &mut RasterSnapshot, mutation: &RasterMutation) {
    *snapshot = crate::artifacts::raster::mutations::apply_raster_mutation(snapshot, mutation);
}
