//! 🕒️ Energy model mutation domain `day-schedule`: `delete-daily-schedule`, `replace-daily-schedule-hourly-values`, `change-daily-schedule-interpolation`, `create-daily-schedule`, `change-daily-schedule-limits`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod delete_daily_schedule {
    #[path = "🌓️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌓️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌓️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌓️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_daily_schedule_hourly_values {
    #[path = "🕔️replace-hourly-values/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕔️replace-hourly-values/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕔️replace-hourly-values/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕔️replace-hourly-values/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_daily_schedule_interpolation {
    #[path = "🕕️change-interpolation/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕕️change-interpolation/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕕️change-interpolation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕕️change-interpolation/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_daily_schedule {
    #[path = "🕞️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕞️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕞️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕞️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_daily_schedule_limits {
    #[path = "🕟️change-limits/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕟️change-limits/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕟️change-limits/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕟️change-limits/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use delete_daily_schedule::{delete_daily_schedule, DeleteDailySchedule};
pub use replace_daily_schedule_hourly_values::{replace_daily_schedule_hourly_values, ReplaceDailyScheduleHourlyValues};
pub use change_daily_schedule_interpolation::{change_daily_schedule_interpolation, ChangeDailyScheduleInterpolation};
pub use create_daily_schedule::{create_daily_schedule, CreateDailySchedule};
pub use change_daily_schedule_limits::{change_daily_schedule_limits, ChangeDailyScheduleLimits};
