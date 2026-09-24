use super::*;
use crate::standards::v1::subsets::any::schema::empty_raster_snapshot;

#[test]
fn demo_example_media_operation_mints_the_emblem() {
    let operations = example_media_operations(crate::examples::art_raster_demo::ID, &empty_raster_snapshot());
    assert_eq!(operations.len(), 1);
    assert!(matches!(&operations[0], RasterMutation::AddLayerAsset(_)));
}

/// ⚖️ LAW: the replace batch is self-contained — it removes the open asset pool, so it must plant
/// the example's own media back in the SAME batch, whatever the open pool held. Reading the
/// `current` pool for that decision (which `example_media_operations` does, correctly, for the
/// "nothing to do" check in `handle`) made re-selecting the demo over an already-materialized
/// document emit `remove-layer-asset semio-emblem` and nothing else: a pixel-less handle pool and
/// a blank composite (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
#[test]
fn replacing_a_materialized_document_plants_the_example_media_it_just_removed() {
    let current = crate::standards::v1::subsets::any::schema::semio_example_document();
    assert!(crate::raster_asset(&current.assets, "semio-emblem").is_some(), "the open document starts with real pixels");
    let example = raster_example_document(crate::examples::art_raster_demo::ID).expect("the demo example document");
    let operations = replace_document_operations(&current, example, crate::examples::art_raster_demo::ID);
    let removed = operations.iter().filter(|operation| matches!(operation, RasterMutation::RemoveLayerAsset(remove) if remove.asset_id == "semio-emblem")).count();
    let planted = operations.iter().any(|operation| matches!(operation, RasterMutation::AddLayerAsset(add) if add.asset_id == "semio-emblem" && !add.asset.data.is_empty()));
    assert_eq!(removed, 1, "the batch removes the open pool");
    assert!(planted, "and plants real emblem pixels back in the same batch");
    for operation in operations {
        crate::standards::v1::subsets::any::schema::mutations::retire_raster_mutation(operation);
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(current);
}
