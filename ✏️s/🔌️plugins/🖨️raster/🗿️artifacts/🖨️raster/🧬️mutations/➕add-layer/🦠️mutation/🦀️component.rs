//! ➕add-layer `RasterMutation` apply leaf.
use crate::artifacts::raster::RasterProjection;
use crate::artifacts::raster::mutations::RasterMutation;

pub fn apply(projection: &mut RasterProjection, mutation: &RasterMutation) {
    *projection = crate::artifacts::raster::mutations::apply_raster_mutation(projection, mutation);
}
