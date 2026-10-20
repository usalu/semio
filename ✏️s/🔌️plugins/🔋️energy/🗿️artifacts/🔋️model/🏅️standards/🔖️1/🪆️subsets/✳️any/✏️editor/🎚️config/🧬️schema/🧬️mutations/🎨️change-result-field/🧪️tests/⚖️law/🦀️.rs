//! ⚖️ `change-result-field` — the inverse diffs sum to the negative of the forward diff (L3).

use super::*;

/// ⚖️ Recolouring diffs only `resultField` and leaves the run settings alone; the inverse reads the old field back.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = EnergyModelConfig { zone_timestep_minutes: 10, system_timestep_minutes: 30, warmup_days: 3, result_field: "solarTransmitted".into() };
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: "airTemperature".into() }), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: String::new() }), &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: "solarTransmitted".into() }), &EnergyModelConfig::default()).await;
}
