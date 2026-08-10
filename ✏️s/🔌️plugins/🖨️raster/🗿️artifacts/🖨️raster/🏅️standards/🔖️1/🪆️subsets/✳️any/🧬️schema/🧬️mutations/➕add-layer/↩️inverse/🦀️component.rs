//! ➕add-layer `RasterMutation` inverse leaf.
use crate::artifacts::raster::RasterSnapshot;
use crate::artifacts::raster::mutations::RasterMutation;
use protocol::Mutation;

pub fn inverse(base: &RasterSnapshot, mutation: &RasterMutation) -> Vec<RasterMutation> {
    <RasterMutation as Mutation<RasterSnapshot>>::inverse(mutation, base)
}
