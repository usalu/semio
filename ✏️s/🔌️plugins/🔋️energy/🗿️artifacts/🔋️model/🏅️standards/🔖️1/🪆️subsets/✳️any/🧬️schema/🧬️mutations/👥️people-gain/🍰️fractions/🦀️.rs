//! 🍰️ Energy model mutation domain `people-gain-fractions`: `change-people-gain-sensible-fraction`, `change-people-gain-latent-fraction`, `change-people-gain-radiant-fraction`


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
pub mod change_people_gain_sensible_fraction {
    #[path = "../🌞️change-sensible-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌞️change-sensible-fraction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🌞️change-sensible-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌞️change-sensible-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_latent_fraction {
    #[path = "../💧️change-latent-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../💧️change-latent-fraction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../💧️change-latent-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../💧️change-latent-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_radiant_fraction {
    #[path = "../📡️change-radiant-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../📡️change-radiant-fraction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../📡️change-radiant-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📡️change-radiant-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_people_gain_sensible_fraction::{change_people_gain_sensible_fraction, ChangePeopleGainSensibleFraction};
pub use change_people_gain_latent_fraction::{change_people_gain_latent_fraction, ChangePeopleGainLatentFraction};
pub use change_people_gain_radiant_fraction::{change_people_gain_radiant_fraction, ChangePeopleGainRadiantFraction};
