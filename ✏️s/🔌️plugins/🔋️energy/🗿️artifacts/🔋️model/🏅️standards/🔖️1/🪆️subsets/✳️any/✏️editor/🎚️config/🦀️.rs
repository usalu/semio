//! 🎚️ Energy model editor config — the simulation run settings (zone and system timestep, warmup days)
//! and the result field the 3d model window colours its surfaces by.
//! They are not model data (`Model` owns the run period and schedules), so they live in the config
//! store: a settings publication bumps its generation, which is exactly what the tool run driver watches
//! to apply the simulation run's `reconfigure` policy (`📋️tool-run-contract.md` §3.3, §3.7.5).
//! Source of record: `🧬️schema/🔣️.json`.
//!
//! ⚠️ `resultField` deliberately does NOT appear in `ENERGY_SIMULATION_RUN_SETTINGS`
//! (`🧵️simulation-session/🦀️.rs`): the run reads the three timestep/warmup pointers only, so
//! recolouring the finished result never restarts a live run.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
#[cfg(test)]
extern crate semio_s_artifact_energy_model_session as session;

use crate::kernel::SimulationConfig;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Config
/// 🎚️ `EnergyModelEditor::Config`: the run settings every simulation run of this editor instance uses,
/// plus the published result field the 3d model window colours by.
///
/// 🌱️ No longer `Copy` — `result_field` is an owned `String`, so the three `EnergyModelConfig`-by-value
/// call sites (`render`, `settings_nodes`, `simulation_template`) take it by reference or clone.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Eq, ToValueDerive, FromValueDerive, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "energycfg")]
#[artifact(id = "energy.model.config")]
#[dsl(layout = "lines")]
pub struct EnergyModelConfig {
    pub zone_timestep_minutes: u32,
    pub system_timestep_minutes: u32,
    pub warmup_days: u32,
    /// 🎨️ Wire id of the published per-surface field the 3d model window colours by — one of
    /// `conductionLoss` | `conductionGain` | `solarTransmitted` | `solarAbsorbed`
    /// ([`ResultField`]). An unrecognized string reads as the default.
    pub result_field: String,
}

/// 🎨️ The `resultField` every fresh config starts on.
pub const DEFAULT_RESULT_FIELD: &str = "conductionLoss";

impl Default for EnergyModelConfig {
    fn default() -> Self {
        Self { zone_timestep_minutes: 60, system_timestep_minutes: 60, warmup_days: 7, result_field: DEFAULT_RESULT_FIELD.to_string() }
    }
}

impl EnergyModelConfig {
    /// 🎛️ Accepts settings only inside the engine's own admissible ranges, and a `result_field` this
    /// build actually knows how to colour by.
    pub fn is_valid(&self) -> bool {
        (1..=60).contains(&self.zone_timestep_minutes) && (1..=60).contains(&self.system_timestep_minutes) && self.warmup_days <= 365 && crate::ResultField::from_id(&self.result_field).is_some()
    }

    /// ⚙️ The engine configuration template a run folds the captured model's run period and schedules into.
    pub fn simulation_template(&self) -> SimulationConfig {
        SimulationConfig { zone_timestep_minutes: self.zone_timestep_minutes, system_timestep_minutes: self.system_timestep_minutes, warmup_days: self.warmup_days, ..SimulationConfig::default() }
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl over the derive's `__dsl_*` helpers.
impl pack::snapshot::ArtifactDsl for EnergyModelConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack: envelope-wrapped pack body via `__dsl_*` record lowering.
impl pack::snapshot::ArtifactPack for EnergyModelConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for EnergyModelConfig {

}
//#endregion 🔖️ArtifactCodec

impl store::ConfigRecord for EnergyModelConfig {}

/// 🔺️ Sparse field delta over [`EnergyModelConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct EnergyModelConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub zone_timestep_minutes: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub system_timestep_minutes: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub warmup_days: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub result_field: Option<String>,
}

impl protocol::MutationDiff<EnergyModelConfig> for EnergyModelConfigDiff {



}

impl protocol::ColdMutationDiff<EnergyModelConfig> for EnergyModelConfigDiff {
    fn absorb(&mut self, other: Self) {
        if other.zone_timestep_minutes.is_some() {
            self.zone_timestep_minutes = other.zone_timestep_minutes;
        }
        if other.system_timestep_minutes.is_some() {
            self.system_timestep_minutes = other.system_timestep_minutes;
        }
        if other.warmup_days.is_some() {
            self.warmup_days = other.warmup_days;
        }
        if other.result_field.is_some() {
            self.result_field = other.result_field;
        }
    }

    fn apply(&self, next: &mut EnergyModelConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(value) = self.zone_timestep_minutes {
            next.zone_timestep_minutes = value;
        }
        if let Some(value) = self.system_timestep_minutes {
            next.system_timestep_minutes = value;
        }
        if let Some(value) = self.warmup_days {
            next.warmup_days = value;
        }
        if let Some(value) = &self.result_field {
            next.result_field = value.clone();
        }
        Ok(())
    }
}

impl protocol::DiffAlgebra<EnergyModelConfig> for EnergyModelConfigDiff {


    fn is_empty(&self) -> bool {
        self.zone_timestep_minutes.is_none() && self.system_timestep_minutes.is_none() && self.warmup_days.is_none() && self.result_field.is_none()
    }
}

impl protocol::ColdDiffAlgebra<EnergyModelConfig> for EnergyModelConfigDiff {
    fn inverse(&self, base: &EnergyModelConfig) -> Self {
        Self {
            zone_timestep_minutes: self.zone_timestep_minutes.map(|_| base.zone_timestep_minutes),
            system_timestep_minutes: self.system_timestep_minutes.map(|_| base.system_timestep_minutes),
            warmup_days: self.warmup_days.map(|_| base.warmup_days),
            result_field: self.result_field.as_ref().map(|_| base.result_field.clone()),
        }
    }
}
//#endregion 🔖️Config

//#region 🔖️Field
/// 🔖️ Which published quantity the 3d model window colours by. The wire ids are the values the
/// `set-result-field` action carries and the `resultField` config string stores.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResultField {
    #[default]
    ConductionLoss,
    ConductionGain,
    SolarTransmitted,
    SolarAbsorbed,
}

impl ResultField {
    pub const ALL: [Self; 4] = [Self::ConductionLoss, Self::ConductionGain, Self::SolarTransmitted, Self::SolarAbsorbed];

    pub fn id(self) -> &'static str {
        match self {
            Self::ConductionLoss => "conductionLoss",
            Self::ConductionGain => "conductionGain",
            Self::SolarTransmitted => "solarTransmitted",
            Self::SolarAbsorbed => "solarAbsorbed",
        }
    }

    /// 🔖️ The field a wire id names; unknown ids fall back to the default rather than refusing.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|field| field.id() == id)
    }

    /// 🏷️ English caption stem, used by `semio_s_artifact_energy_model_editor_results::legend_caption`.
    pub fn label(self) -> &'static str {
        match self {
            Self::ConductionLoss => "Conduction loss",
            Self::ConductionGain => "Conduction gain",
            Self::SolarTransmitted => "Solar transmitted",
            Self::SolarAbsorbed => "Solar absorbed",
        }
    }
}

/// 🎚️ The field the editor config currently selects. An unrecognized stored string (an older
/// document, a hand-edited config) reads as the default rather than blanking the window.
pub fn result_field(cfg: &EnergyModelConfig) -> ResultField {
    ResultField::from_id(&cfg.result_field).unwrap_or_default()
}
//#endregion 🔖️Field

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
