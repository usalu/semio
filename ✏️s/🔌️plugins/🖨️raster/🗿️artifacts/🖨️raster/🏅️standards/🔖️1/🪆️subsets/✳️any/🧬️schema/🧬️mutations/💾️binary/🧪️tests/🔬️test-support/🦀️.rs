
use super::{RASTER_OWNED_FIELD_BYTES, RasterSnapshot, RasterSnapshotRetirementFactory};

pub(crate) fn retire_raster_snapshot(snapshot: RasterSnapshot) {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&RasterSnapshotRetirementFactory, snapshot);
    let maximum_bytes = RASTER_OWNED_FIELD_BYTES.max(crate::RASTER_OWNED_MAP_PAGE_BACKING_BYTES);
    let mut steps = 0_u64;
    let mut idle = 0_u64;
    loop {
        steps += 1;
        match retirement.close_step(1, maximum_bytes).expect("one Raster test snapshot owner retires") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= maximum_bytes);
                idle = if released_items == 0 && released_bytes == 0 { idle + 1 } else { 0 };
                assert!(idle <= 4_096, "Raster test snapshot retirement stalled: {idle} consecutive zero-release turns after {steps} turns");
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty(), "Raster test snapshot retirement reported a false terminal");
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Blocked => panic!("an unshared Raster test snapshot retirement cannot block"),
        }
    }
}
