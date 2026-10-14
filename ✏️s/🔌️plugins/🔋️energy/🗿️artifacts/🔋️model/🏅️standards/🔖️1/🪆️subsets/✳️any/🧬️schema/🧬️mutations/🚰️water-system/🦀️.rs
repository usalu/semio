//! 🚰️ Energy model mutation domain `water-system`: `change-water-system-schedule`, `create-water-system`, `change-water-system-peak-flow`, `delete-water-system`, `change-water-system-fixture-count`.

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
pub mod change_water_system_schedule {
    #[path = "🕒️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕒️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕒️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_water_system {
    #[path = "🚽️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚽️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚽️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_water_system_peak_flow {
    #[path = "🚾️change-peak-flow/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚾️change-peak-flow/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚾️change-peak-flow/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_water_system {
    #[path = "🧼️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧼️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧼️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_water_system_fixture_count {
    #[path = "🪣️change-fixture-count/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪣️change-fixture-count/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪣️change-fixture-count/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_water_system_schedule::{change_water_system_schedule, ChangeWaterSystemSchedule};
pub use create_water_system::{create_water_system, CreateWaterSystem};
pub use change_water_system_peak_flow::{change_water_system_peak_flow, ChangeWaterSystemPeakFlow};
pub use delete_water_system::{delete_water_system, DeleteWaterSystem};
pub use change_water_system_fixture_count::{change_water_system_fixture_count, ChangeWaterSystemFixtureCount};
