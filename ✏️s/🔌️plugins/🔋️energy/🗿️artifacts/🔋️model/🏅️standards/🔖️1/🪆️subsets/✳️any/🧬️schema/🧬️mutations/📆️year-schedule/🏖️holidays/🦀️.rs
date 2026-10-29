//! 🏖️ Energy model mutation domain `year-schedule-holidays`: `change-annual-schedule-holiday-daily-schedule`, `add-annual-schedule-holiday`, `remove-annual-schedule-holiday`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_annual_schedule_holiday_daily_schedule {
    #[path = "../🎄️change-holiday-daily-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎄️change-holiday-daily-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🎄️change-holiday-daily-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎄️change-holiday-daily-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_annual_schedule_holiday {
    #[path = "../🎉️add-holiday/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎉️add-holiday/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🎉️add-holiday/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎉️add-holiday/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_annual_schedule_holiday {
    #[path = "../🎊️remove-holiday/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🎊️remove-holiday/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🎊️remove-holiday/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎊️remove-holiday/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_annual_schedule_holiday_daily_schedule::{change_annual_schedule_holiday_daily_schedule, ChangeAnnualScheduleHolidayDailySchedule};
pub use add_annual_schedule_holiday::{add_annual_schedule_holiday, AddAnnualScheduleHoliday};
pub use remove_annual_schedule_holiday::{remove_annual_schedule_holiday, RemoveAnnualScheduleHoliday};
