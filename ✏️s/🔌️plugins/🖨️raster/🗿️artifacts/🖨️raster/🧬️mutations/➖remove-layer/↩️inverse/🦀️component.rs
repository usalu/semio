//! ➖remove-layer `RasterMutation` inverse leaf.
use crate::artifacts::raster::RasterProjection;
use crate::artifacts::raster::mutations::RasterMutation;
use protocol::Mutation;

pub fn inverse(base: &RasterProjection, mutation: &RasterMutation) -> Vec<RasterMutation> {
    <RasterMutation as Mutation<RasterProjection>>::inverse(mutation, base)
}
