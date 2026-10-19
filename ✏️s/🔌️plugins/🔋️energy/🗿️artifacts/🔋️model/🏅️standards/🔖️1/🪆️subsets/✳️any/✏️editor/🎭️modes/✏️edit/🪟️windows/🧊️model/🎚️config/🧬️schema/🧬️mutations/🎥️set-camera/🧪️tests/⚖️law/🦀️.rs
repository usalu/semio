//! ⚖️ `set-camera` (editor 3d window) — the inverse diffs sum to the negative of the forward diff (L3).

use super::*;

/// ⚖️ Orbiting restates the camera slot and the inverse reads the previous pose back from `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = EnergyModelWindowConfig { camera: EnergyModelCameraPose { position: [4.0, -2.0, 6.0], target: [1.0, 1.0, 0.5], zoom: 1.5 } };
    let moved = EnergyModelCameraPose { position: [1.0, 2.0, 3.0], target: [0.5, 0.5, 0.0], zoom: 2.0 };
    let zoomed = EnergyModelCameraPose { zoom: 4.0, ..base.camera };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: moved }), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: zoomed }), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelWindowConfigMutation::SetCamera(SetCamera { camera: moved }), &EnergyModelWindowConfig::default()).await;
}
