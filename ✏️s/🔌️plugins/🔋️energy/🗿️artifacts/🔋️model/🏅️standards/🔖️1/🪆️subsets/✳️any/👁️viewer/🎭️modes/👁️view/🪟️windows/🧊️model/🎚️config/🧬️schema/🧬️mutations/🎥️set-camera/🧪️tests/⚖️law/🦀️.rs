//! ⚖️ `set-camera` (viewer 3d window) — the inverse diffs sum to the negative of the forward diff (L3).

use super::*;

/// ⚖️ Orbiting restates the camera slot and the inverse reads the previous pose back from `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = EnergyModelViewerWindowConfig { camera: EnergyModelViewerCameraPose { position: [4.0, -2.0, 6.0], target: [1.0, 1.0, 0.5], zoom: 1.5 } };
    let moved = EnergyModelViewerCameraPose { position: [1.0, 2.0, 3.0], target: [0.5, 0.5, 0.0], zoom: 2.0 };
    let zoomed = EnergyModelViewerCameraPose { zoom: 4.0, ..base.camera };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelViewerWindowConfigMutation::SetCamera(SetCamera { camera: moved }), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelViewerWindowConfigMutation::SetCamera(SetCamera { camera: zoomed }), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelViewerWindowConfigMutation::SetCamera(SetCamera { camera: moved }), &EnergyModelViewerWindowConfig::default()).await;
}
