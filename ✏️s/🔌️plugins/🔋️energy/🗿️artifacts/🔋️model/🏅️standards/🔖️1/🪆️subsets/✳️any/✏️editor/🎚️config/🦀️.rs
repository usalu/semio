//! 🎚️ Energy model editor config — the simulation run settings (zone and system timestep, warmup days).
//! They are not model data (`Model` owns the run period and schedules), so they live in the config
//! store: a settings publication bumps its generation, which is exactly what the tool run driver watches
//! to apply the simulation run's `reconfigure` policy (`📋️tool-run-contract.md` §3.3, §3.7.5).
//! Source of record: `🧬️schema/🔣️.json`.

use crate::SimulationConfig;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Config
/// 🎚️ `EnergyModelEditor::Config`: the run settings every simulation run of this editor instance uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValueDerive, FromValueDerive, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(extension = "energycfg")]
#[dsl(id = "energy.model.config")]
#[dsl(layout = "lines")]
pub struct EnergyModelConfig {
    pub zone_timestep_minutes: u32,
    pub system_timestep_minutes: u32,
    pub warmup_days: u32,
}

impl Default for EnergyModelConfig {
    fn default() -> Self {
        Self { zone_timestep_minutes: 60, system_timestep_minutes: 60, warmup_days: 7 }
    }
}

impl EnergyModelConfig {
    /// 🎛️ Accepts settings only inside the engine's own admissible ranges.
    pub fn is_valid(self) -> bool {
        (1..=60).contains(&self.zone_timestep_minutes) && (1..=60).contains(&self.system_timestep_minutes) && self.warmup_days <= 365
    }

    /// ⚙️ The engine configuration template a run folds the captured model's run period and schedules into.
    pub fn simulation_template(self) -> SimulationConfig {
        SimulationConfig { zone_timestep_minutes: self.zone_timestep_minutes, system_timestep_minutes: self.system_timestep_minutes, warmup_days: self.warmup_days, ..SimulationConfig::default() }
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl over the derive's `__dsl_*` helpers.
impl store::ArtifactDsl for EnergyModelConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack: envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for EnergyModelConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

store::impl_whole_record_config!(EnergyModelConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
