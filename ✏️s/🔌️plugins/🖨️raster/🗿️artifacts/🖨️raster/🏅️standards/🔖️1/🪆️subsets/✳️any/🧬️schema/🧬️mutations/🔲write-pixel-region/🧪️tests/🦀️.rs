//! 🧪️ `write-pixel-region` laws: it writes exactly its rectangle, inverts to itself with the base samples of that rectangle,
//! and is a diagnosed no-op when the rectangle already holds the samples.

use super::*;
use crate::standards::v1::subsets::any::schema::{flatten_raster_layers, raster_image_test_snapshot};
use crate::RasterLayerNode;
use protocol::Mutation;

fn shown_pixels(snapshot: &RasterSnapshot) -> (String, String, crate::SemioImageSnapshot) {
    let layer = flatten_raster_layers(&snapshot.layers).into_iter().find_map(|layer| match layer {
        RasterLayerNode::Pixel { id, image_key: Some(key), .. } => Some((id.clone(), key.clone())),
        _ => None,
    });
    let (layer_id, key) = layer.expect("the test document shows an image");
    let image = crate::raster_image(&snapshot.assets, &key).expect("the shown image materializes");
    (layer_id, key, image)
}

fn mutation(snapshot: &RasterSnapshot, invert: bool) -> WritePixelRegion {
    let (layer_id, _, image) = shown_pixels(snapshot);
    let current = &image.frames[0].rgba8[..4];
    let samples = if invert { current.iter().map(|byte| !byte).collect() } else { current.to_vec() };
    WritePixelRegion { layer_id, target: "pixels".into(), x: 0, y: 0, width: 1, height: 1, samples }
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

#[semio_framework_async_macros::async_test]
async fn it_writes_exactly_its_rectangle_and_inverts_with_the_base_samples() {
    let base = raster_image_test_snapshot();
    let write = mutation(&base, true);
    let outcome = <WritePixelRegion as protocol::MutationKind<RasterSnapshot, RasterMutation>>::diff(&write, &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(outcome.diff().pixels.len(), 1, "one pixel region and nothing else");
    assert!(outcome.diff().layers.is_none() && outcome.diff().assets.is_none(), "the handle is derived by the applier, never carried");
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("the region applies");
    let (_, _, written) = shown_pixels(&applied);
    assert_eq!(&written.frames[0].rgba8[..4], write.samples.as_slice());
    let (_, _, original) = shown_pixels(&base);
    assert_eq!(&written.frames[0].rgba8[4..], &original.frames[0].rgba8[4..], "every other pixel is untouched");
    let inverse = <WritePixelRegion as protocol::MutationKind<RasterSnapshot, RasterMutation>>::inverse(&write, &base).expect("inverse");
    assert!(matches!(inverse.as_slice(), [RasterMutation::WritePixelRegion(restore)] if restore.samples == original.frames[0].rgba8[..4]), "{inverse:?}");
    retire(applied);
    retire(base);
}

#[semio_framework_async_macros::async_test]
async fn it_obeys_the_cold_inverse_sum_law() {
    let base = raster_image_test_snapshot();
    let write = RasterMutation::WritePixelRegion(mutation(&base, true));
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law_cold(&write, &base, <crate::diff::RasterDiff as protocol::MutationDiff<RasterSnapshot>>::retire_projection, <crate::diff::RasterDiff as protocol::MutationDiff<RasterSnapshot>>::retire_cold).await;
    retire(base);
}

#[semio_framework_async_macros::async_test]
async fn a_rectangle_already_holding_the_samples_is_a_noop() {
    let base = raster_image_test_snapshot();
    let same = mutation(&base, false);
    let outcome = <WritePixelRegion as protocol::MutationKind<RasterSnapshot, RasterMutation>>::diff(&same, &base);
    assert!(outcome.diff().pixels.is_empty());
    assert!(outcome.messages().iter().any(|message| format!("{:?}", message.level) == "Warning"));
    retire(base);
}
