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
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", content = "payload", rename_all = "camelCase"))]
#[mutations(snapshot = EnergyModelConfig, diff = EnergyModelConfigDiff, schema = "energy.model.config")]
pub enum EnergyModelConfigMutation {
    ChangeSimulationSettings(ChangeSimulationSettings),
    ChangeResultField(ChangeResultField),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
