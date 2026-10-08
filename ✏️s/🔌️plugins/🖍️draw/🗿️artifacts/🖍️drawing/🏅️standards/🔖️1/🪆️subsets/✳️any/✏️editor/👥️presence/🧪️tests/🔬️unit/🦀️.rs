//! 🧪️ Drawing presence — the sparse `Set` mutation's concrete inverse sums to exactly the negative diff.
use super::{DrawingPresence, DrawingPresenceDiff, DrawingPresenceMutation};
use protocol::os_spr::protocol_laws::{assert_mutation_inverse_sum_law};
use protocol::Mutation;

fn peer() -> DrawingPresence {
    DrawingPresence { engagement_input: "Peer layer".into(), camera: store::Viewport2d { x: 2.0, y: 3.0, zoom: 1.25 } }
}

/// ⚖️ A set of both fields restores both exactly.
#[semio_framework_async_macros::async_test]
async fn setting_both_fields_inverts_to_the_negative_diff() {
    let mutation = DrawingPresenceMutation::Set { engagement_input: "Layer A".into(), camera: store::Viewport2d { x: 9.0, y: -4.0, zoom: 3.0 } };
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}

/// ⚖️ A set that changes only the camera carries only the camera in its diff and its negative.
#[semio_framework_async_macros::async_test]
async fn setting_one_field_carries_only_that_field() {
    let mutation = DrawingPresenceMutation::Set { engagement_input: peer().engagement_input, camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } };
    let outcome = mutation.diff(&peer());
    assert!(outcome.diff().engagement_input.is_none() && outcome.diff().camera.is_some());
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}

