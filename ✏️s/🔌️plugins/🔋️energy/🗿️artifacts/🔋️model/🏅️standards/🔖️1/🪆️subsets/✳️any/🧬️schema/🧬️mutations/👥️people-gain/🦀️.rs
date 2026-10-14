//! 👥️ Energy model mutation domain `people-gain`: `change-people-gain-schedule`, `change-people-gain-sensible-fraction`, `change-people-gain-activity-schedule`, `create-people-gain`, `change-people-gain-people-per-area`, `change-people-gain-latent-fraction`, `change-people-gain-radiant-fraction`, `change-people-gain-zone`, `delete-people-gain`.

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
pub mod change_people_gain_schedule {
    #[path = "⏰️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⏰️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⏰️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_sensible_fraction {
    #[path = "🌞️change-sensible-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌞️change-sensible-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌞️change-sensible-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_activity_schedule {
    #[path = "🏃️change-activity-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏃️change-activity-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏃️change-activity-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_people_gain {
    #[path = "👤️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "👤️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "👤️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_people_per_area {
    #[path = "👥️change-people-per-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "👥️change-people-per-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "👥️change-people-per-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_latent_fraction {
    #[path = "💧️change-latent-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💧️change-latent-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💧️change-latent-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_radiant_fraction {
    #[path = "📡️change-radiant-fraction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📡️change-radiant-fraction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📡️change-radiant-fraction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_people_gain_zone {
    #[path = "🚶️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚶️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚶️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_people_gain {
    #[path = "🚷️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚷️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚷️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_people_gain_schedule::{change_people_gain_schedule, ChangePeopleGainSchedule};
pub use change_people_gain_sensible_fraction::{change_people_gain_sensible_fraction, ChangePeopleGainSensibleFraction};
pub use change_people_gain_activity_schedule::{change_people_gain_activity_schedule, ChangePeopleGainActivitySchedule};
pub use create_people_gain::{create_people_gain, CreatePeopleGain};
pub use change_people_gain_people_per_area::{change_people_gain_people_per_area, ChangePeopleGainPeoplePerArea};
pub use change_people_gain_latent_fraction::{change_people_gain_latent_fraction, ChangePeopleGainLatentFraction};
pub use change_people_gain_radiant_fraction::{change_people_gain_radiant_fraction, ChangePeopleGainRadiantFraction};
pub use change_people_gain_zone::{change_people_gain_zone, ChangePeopleGainZone};
pub use delete_people_gain::{delete_people_gain, DeletePeopleGain};
