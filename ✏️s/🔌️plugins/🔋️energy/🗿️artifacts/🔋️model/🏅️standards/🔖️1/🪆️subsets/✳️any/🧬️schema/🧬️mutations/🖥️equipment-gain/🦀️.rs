//! 🖥️ Energy model mutation domain `equipment-gain`: `change-equipment-gain-schedule`, `change-equipment-gain-watts-per-area`, `change-equipment-gain-radiant-fraction`, `change-equipment-gain-latent-fraction`, `create-equipment-gain`, `change-equipment-gain-zone`, `delete-equipment-gain`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod change_equipment_gain_schedule {
    #[path = "⌛️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⌛️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⌛️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_equipment_gain_watts_per_area {
    #[path = "⚡️change-watts-per-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⚡️change-watts-per-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⚡️change-watts-per-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_equipment_gain_radiant_fraction {
    #[path = "🌠️change-radiant-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌠️change-radiant-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌠️change-radiant-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_equipment_gain_latent_fraction {
    #[path = "💦️change-latent-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💦️change-latent-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💦️change-latent-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_equipment_gain {
    #[path = "🖥️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🖥️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖥️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_equipment_gain_zone {
    #[path = "🖨️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🖨️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖨️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_equipment_gain {
    #[path = "🧯️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧯️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧯️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_equipment_gain_schedule::{change_equipment_gain_schedule, ChangeEquipmentGainSchedule};
pub use change_equipment_gain_watts_per_area::{change_equipment_gain_watts_per_area, ChangeEquipmentGainWattsPerArea};
pub use change_equipment_gain_radiant_fraction::{change_equipment_gain_radiant_fraction, ChangeEquipmentGainRadiantFraction};
pub use change_equipment_gain_latent_fraction::{change_equipment_gain_latent_fraction, ChangeEquipmentGainLatentFraction};
pub use create_equipment_gain::{create_equipment_gain, CreateEquipmentGain};
pub use change_equipment_gain_zone::{change_equipment_gain_zone, ChangeEquipmentGainZone};
pub use delete_equipment_gain::{delete_equipment_gain, DeleteEquipmentGain};
