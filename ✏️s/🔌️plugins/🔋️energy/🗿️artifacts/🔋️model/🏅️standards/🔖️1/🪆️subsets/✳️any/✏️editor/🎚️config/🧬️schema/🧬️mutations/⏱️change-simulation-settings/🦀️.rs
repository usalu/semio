//! ⏱️ Change the simulation run settings in the energy model editor config.

use super::{EnergyModelConfig, EnergyModelConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// ⏱️ `change-simulation-settings`: replaces the three run settings at once; its inverse restores the base's.
#[derive(Clone, Debug, PartialEq, Eq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
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

    /// 🎚️ The settings this change installs, over a default `resultField` — the standalone validity
    /// probe a command uses before publishing. Applying the change to a real base goes through
    /// [`Self::config_over`], which keeps that base's own `resultField`.
    pub fn config(&self) -> EnergyModelConfig {
        self.config_over(&EnergyModelConfig::default())
    }

    /// 🎚️ The settings this change installs over `base`, leaving every field it does not own alone.
    pub fn config_over(&self, base: &EnergyModelConfig) -> EnergyModelConfig {
        EnergyModelConfig { zone_timestep_minutes: self.zone_timestep_minutes, system_timestep_minutes: self.system_timestep_minutes, warmup_days: self.warmup_days, result_field: base.result_field.clone() }
    }
}

impl protocol::MutationKind<EnergyModelConfig, EnergyModelConfigMutation> for ChangeSimulationSettings {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "simulation-settings", kind: "change-simulation-settings", record: "ChangedSimulationSettings" };
    fn diff(&self, base: &EnergyModelConfig) -> protocol::MutationOutcome<EnergyModelConfig> {
        protocol::MutationOutcome::new(self.config_over(base))
    }
    fn inverse(&self, base: &EnergyModelConfig) -> Vec<EnergyModelConfigMutation> {
        vec![EnergyModelConfigMutation::ChangeSimulationSettings(Self::of(base))]
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change simulation settings to {} / {} min, {} warmup days", self.zone_timestep_minutes, self.system_timestep_minutes, self.warmup_days), &format!("Simulationseinstellungen auf {} / {} min, {} Einschwingtage ändern", self.zone_timestep_minutes, self.system_timestep_minutes, self.warmup_days))
    }
    fn target(&self) -> Vec<String> {
        vec!["simulation-settings".into()]
    }
}
