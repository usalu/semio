use crate::artifacts::raster::diff::diff_set_snapshot;
use crate::artifacts::raster::mutations::RasterMutation;

pub fn diff(mutation: &RasterMutation) -> crate::artifacts::raster::diff::RasterDiff {
    match mutation {
        RasterMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        _ => unreachable!("set-snapshot diff only for SetSnapshot"),
    }
}
