//! 🧬️ Semantic energy model config mutation vocabulary and codecs.

use super::{EnergyModelConfig, EnergyModelConfigDiff};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[path = "⏱️change-simulation/🦀️.rs"]
mod change_simulation_settings;
pub use change_simulation_settings::ChangeSimulationSettings;

#[path = "🎨️change-result/🦀️.rs"]
mod change_result_field;
pub use change_result_field::ChangeResultField;

/// 🧬️ `EnergyModelEditor::ConfigMutation`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = EnergyModelConfig, diff = EnergyModelConfigDiff, schema = "energy.model.config")]
pub enum EnergyModelConfigMutation {
    ChangeSimulationSettings(ChangeSimulationSettings),
    ChangeResultField(ChangeResultField),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<EnergyModelConfig> for EnergyModelConfigMutation {
    fn exchange(self, post: &mut EnergyModelConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::ChangeSimulationSettings(ChangeSimulationSettings { zone_timestep_minutes, system_timestep_minutes, warmup_days }) => Self::ChangeSimulationSettings(ChangeSimulationSettings {
                zone_timestep_minutes: std::mem::replace(&mut post.zone_timestep_minutes, zone_timestep_minutes),
                system_timestep_minutes: std::mem::replace(&mut post.system_timestep_minutes, system_timestep_minutes),
                warmup_days: std::mem::replace(&mut post.warmup_days, warmup_days),
            }),
            Self::ChangeResultField(ChangeResultField { field }) => Self::ChangeResultField(ChangeResultField { field: std::mem::replace(&mut post.result_field, field) }),
        })
    }

    fn admissible(&self) -> bool {
        match self {
            Self::ChangeSimulationSettings(change) => change.config().is_valid(),
            Self::ChangeResultField(ChangeResultField { field }) => crate::editor::model::results::ResultField::from_id(field).is_some(),
        }
    }

    fn payload_bytes(&self) -> usize {
        match self {
            Self::ChangeSimulationSettings(_) => 0,
            Self::ChangeResultField(ChangeResultField { field }) => field.len(),
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
