//! 🗓️ Energy model mutation domain `week-schedule`: `delete-weekly-schedule`, `change-weekly-schedule-day`, `create-weekly-schedule`.

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
pub mod delete_weekly_schedule {
    #[path = "🕖️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕖️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕖️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕖️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_weekly_schedule_day {
    #[path = "🕗️change-day/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕗️change-day/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕗️change-day/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕗️change-day/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_weekly_schedule {
    #[path = "🗓️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗓️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🗓️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗓️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use delete_weekly_schedule::{delete_weekly_schedule, DeleteWeeklySchedule};
pub use change_weekly_schedule_day::{change_weekly_schedule_day, ChangeWeeklyScheduleDay};
pub use create_weekly_schedule::{create_weekly_schedule, CreateWeeklySchedule};
