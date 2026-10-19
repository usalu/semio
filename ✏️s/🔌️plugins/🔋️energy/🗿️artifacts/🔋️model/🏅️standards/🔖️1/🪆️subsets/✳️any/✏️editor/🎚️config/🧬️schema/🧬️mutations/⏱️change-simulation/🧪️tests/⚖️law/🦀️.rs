//! ⚖️ `change-simulation-settings` — the inverse diffs sum to the negative of the forward diff (L3).

use super::*;

/// ⚖️ Changing the run settings diffs only the slots that differ and the inverse reads the three base values back.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let base = EnergyModelConfig { zone_timestep_minutes: 10, system_timestep_minutes: 30, warmup_days: 3, result_field: "solarTransmitted".into() };
    let all = EnergyModelConfigMutation::ChangeSimulationSettings(ChangeSimulationSettings { zone_timestep_minutes: 15, system_timestep_minutes: 60, warmup_days: 1 });
    let one_slot = EnergyModelConfigMutation::ChangeSimulationSettings(ChangeSimulationSettings { zone_timestep_minutes: 10, system_timestep_minutes: 30, warmup_days: 7 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&all, &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&one_slot, &base).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&all, &EnergyModelConfig::default()).await;
}
