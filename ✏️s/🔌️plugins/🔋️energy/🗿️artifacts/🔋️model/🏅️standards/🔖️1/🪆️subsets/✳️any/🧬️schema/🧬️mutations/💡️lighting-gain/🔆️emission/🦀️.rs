//! 🔆️ Energy model mutation domain `lighting-gain-emission`: `change-lighting-gain-radiant-fraction`, `change-lighting-gain-return-air-fraction`, `change-lighting-gain-visible-fraction`, `change-lighting-gain-watts-per-area`


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
pub mod change_lighting_gain_radiant_fraction {
    #[path = "../🌟️change-radiant-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌟️change-radiant-fraction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🌟️change-radiant-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌟️change-radiant-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_lighting_gain_return_air_fraction {
    #[path = "../🎐️change-return-air-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎐️change-return-air-fraction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🎐️change-return-air-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎐️change-return-air-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_lighting_gain_visible_fraction {
    #[path = "../🔅️change-visible-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔅️change-visible-fraction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🔅️change-visible-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔅️change-visible-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_lighting_gain_watts_per_area {
    #[path = "../🔌️change-watts-per-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔌️change-watts-per-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🔌️change-watts-per-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔌️change-watts-per-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_lighting_gain_radiant_fraction::{change_lighting_gain_radiant_fraction, ChangeLightingGainRadiantFraction};
pub use change_lighting_gain_return_air_fraction::{change_lighting_gain_return_air_fraction, ChangeLightingGainReturnAirFraction};
pub use change_lighting_gain_visible_fraction::{change_lighting_gain_visible_fraction, ChangeLightingGainVisibleFraction};
pub use change_lighting_gain_watts_per_area::{change_lighting_gain_watts_per_area, ChangeLightingGainWattsPerArea};
