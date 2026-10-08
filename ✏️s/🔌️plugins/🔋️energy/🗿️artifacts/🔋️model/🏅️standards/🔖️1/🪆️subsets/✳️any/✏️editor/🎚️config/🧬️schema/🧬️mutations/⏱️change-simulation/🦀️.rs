//! ⏱️ Change the simulation run settings in the energy model editor config.

use super::{EnergyModelConfig, EnergyModelConfigDiff, EnergyModelConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// ⏱️ `change-simulation-settings`: replaces the three run settings at once; its inverse restores the base's three values.
#[derive(Clone, Debug, PartialEq, Eq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "change-simulation-settings")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSimulationSettings {
    pub zone_timestep_minutes: u32,
    pub system_timestep_minutes: u32,
    pub warmup_days: u32,
}

impl ChangeSimulationSettings {
    fn of(config: &EnergyModelConfig) -> Self {
        Self { zone_timestep_minutes: config.zone_timestep_minutes, system_timestep_minutes: config.system_timestep_minutes, warmup_days: config.warmup_days }
    }

    /// 🎚️ The settings this change installs, over a default `resultField` — the standalone validity probe a command
    /// uses before publishing.
    pub fn config(&self) -> EnergyModelConfig {
        EnergyModelConfig { zone_timestep_minutes: self.zone_timestep_minutes, system_timestep_minutes: self.system_timestep_minutes, warmup_days: self.warmup_days, ..EnergyModelConfig::default() }
    }
}

impl protocol::MutationKind<EnergyModelConfig, EnergyModelConfigMutation> for ChangeSimulationSettings {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "simulation-settings", kind: "change-simulation-settings", record: "ChangedSimulationSettings" };
    fn diff(&self, base: &EnergyModelConfig) -> protocol::MutationOutcome<EnergyModelConfigDiff> {
        protocol::MutationOutcome::new(EnergyModelConfigDiff {
            zone_timestep_minutes: (base.zone_timestep_minutes != self.zone_timestep_minutes).then_some(self.zone_timestep_minutes),
            system_timestep_minutes: (base.system_timestep_minutes != self.system_timestep_minutes).then_some(self.system_timestep_minutes),
            warmup_days: (base.warmup_days != self.warmup_days).then_some(self.warmup_days),
            ..EnergyModelConfigDiff::default()
        })
    }
    fn inverse(&self, base: &EnergyModelConfig) -> Result<Vec<EnergyModelConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![EnergyModelConfigMutation::ChangeSimulationSettings(Self::of(base))]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change simulation settings to {} / {} min, {} warmup days", self.zone_timestep_minutes, self.system_timestep_minutes, self.warmup_days), &format!("Simulationseinstellungen auf {} / {} min, {} Einschwingtage ändern", self.zone_timestep_minutes, self.system_timestep_minutes, self.warmup_days))
    }
    fn target(&self) -> Vec<String> {
        vec!["simulation-settings".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = EnergyModelConfig::default();
        let change = EnergyModelConfigMutation::ChangeSimulationSettings(ChangeSimulationSettings { zone_timestep_minutes: 15, system_timestep_minutes: 60, warmup_days: 1 });
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&change, &base).await;
    }
}
