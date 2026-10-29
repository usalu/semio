//! 💡️ Energy model mutation domain `lighting-gain`: `change-lighting-gain-schedule`, `create-lighting-gain`, `change-lighting-gain-zone`, `delete-lighting-gain`

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_lighting_gain_schedule {
    #[path = "⏱️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⏱️change-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⏱️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⏱️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_lighting_gain {
    #[path = "💡️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💡️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "💡️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💡️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_lighting_gain_zone {
    #[path = "🔦️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔦️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔦️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔦️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_lighting_gain {
    #[path = "🕯️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕯️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕯️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕯️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_lighting_gain_schedule::{change_lighting_gain_schedule, ChangeLightingGainSchedule};
pub use create_lighting_gain::{create_lighting_gain, CreateLightingGain};
pub use change_lighting_gain_zone::{change_lighting_gain_zone, ChangeLightingGainZone};
pub use delete_lighting_gain::{delete_lighting_gain, DeleteLightingGain};
