//! 📈️ Energy model mutation domain `time-series`: `delete-time-series-schedule`, `replace-time-series-schedule-values`, `change-time-series-schedule-timestep`, `create-time-series-schedule`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod delete_time_series_schedule {
    #[path = "🎞️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎞️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎞️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_time_series_schedule_values {
    #[path = "🕘️replace-values/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕘️replace-values/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕘️replace-values/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_time_series_schedule_timestep {
    #[path = "🕙️change-timestep/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕙️change-timestep/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕙️change-timestep/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_time_series_schedule {
    #[path = "🪗️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪗️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪗️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use delete_time_series_schedule::{delete_time_series_schedule, DeleteTimeSeriesSchedule};
pub use replace_time_series_schedule_values::{replace_time_series_schedule_values, ReplaceTimeSeriesScheduleValues};
pub use change_time_series_schedule_timestep::{change_time_series_schedule_timestep, ChangeTimeSeriesScheduleTimestep};
pub use create_time_series_schedule::{create_time_series_schedule, CreateTimeSeriesSchedule};
