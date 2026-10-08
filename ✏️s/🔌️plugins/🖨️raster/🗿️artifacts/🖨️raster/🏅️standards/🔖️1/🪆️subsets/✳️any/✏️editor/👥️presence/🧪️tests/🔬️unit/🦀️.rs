//! 🧪️ Raster presence — the sparse `Set` mutation's concrete inverse sums to exactly the negative diff.
use super::{RasterPresence, RasterPresenceDiff, RasterPresenceMutation};
use crate::RasterCamera;
use protocol::os_spr::protocol_laws::{assert_mutation_inverse_sum_law};
use protocol::Mutation;

fn peer() -> RasterPresence {
    RasterPresence { brush_size: 48.0, brush_opacity: 0.5, camera: RasterCamera { x: 1.0, y: 2.0, zoom: 2.0 } }
}

/// ⚖️ A set that changes several fields restores every one of them exactly.
#[semio_framework_async_macros::async_test]
async fn setting_several_fields_inverts_to_the_negative_diff() {
    let mutation = RasterPresenceMutation::Set { presence: RasterPresence { brush_size: 12.0, camera: RasterCamera { x: 0.0, y: 0.0, zoom: 1.0 }, ..peer() } };
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}

/// ⚖️ A set that changes one field carries only that field in its diff.
#[semio_framework_async_macros::async_test]
async fn setting_one_field_carries_only_that_field() {
    let mutation = RasterPresenceMutation::Set { presence: RasterPresence { brush_opacity: 1.0, ..peer() } };
    assert_eq!(*mutation.diff(&peer()).diff(), RasterPresenceDiff { brush_opacity: Some(1.0), ..Default::default() });
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}

