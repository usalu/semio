//! 📆️ Energy model mutation domain `year-schedule`: `change-annual-schedule-holiday-daily-schedule`, `add-annual-schedule-holiday`, `remove-annual-schedule-holiday`, `change-annual-schedule-default-daily-schedule`, `delete-annual-schedule`, `insert-annual-schedule-rule`, `remove-annual-schedule-rule`, `create-annual-schedule`, `reorder-annual-schedule-rules`.

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
pub mod change_annual_schedule_holiday_daily_schedule {
    #[path = "🎄️change-holiday-daily-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎄️change-holiday-daily-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎄️change-holiday-daily-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_annual_schedule_holiday {
    #[path = "🎉️add-holiday/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎉️add-holiday/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎉️add-holiday/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_annual_schedule_holiday {
    #[path = "🎊️remove-holiday/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎊️remove-holiday/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎊️remove-holiday/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_annual_schedule_default_daily_schedule {
    #[path = "🎌️change-default-daily-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎌️change-default-daily-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎌️change-default-daily-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_annual_schedule {
    #[path = "📕️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📕️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📕️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod insert_annual_schedule_rule {
    #[path = "📗️insert-rule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📗️insert-rule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📗️insert-rule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_annual_schedule_rule {
    #[path = "📙️remove-rule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📙️remove-rule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📙️remove-rule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_annual_schedule {
    #[path = "📚️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📚️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📚️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod reorder_annual_schedule_rules {
    #[path = "🗂️reorder-rules/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗂️reorder-rules/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗂️reorder-rules/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_annual_schedule_holiday_daily_schedule::{change_annual_schedule_holiday_daily_schedule, ChangeAnnualScheduleHolidayDailySchedule};
pub use add_annual_schedule_holiday::{add_annual_schedule_holiday, AddAnnualScheduleHoliday};
pub use remove_annual_schedule_holiday::{remove_annual_schedule_holiday, RemoveAnnualScheduleHoliday};
pub use change_annual_schedule_default_daily_schedule::{change_annual_schedule_default_daily_schedule, ChangeAnnualScheduleDefaultDailySchedule};
pub use delete_annual_schedule::{delete_annual_schedule, DeleteAnnualSchedule};
pub use insert_annual_schedule_rule::{insert_annual_schedule_rule, InsertAnnualScheduleRule};
pub use remove_annual_schedule_rule::{remove_annual_schedule_rule, RemoveAnnualScheduleRule};
pub use create_annual_schedule::{create_annual_schedule, CreateAnnualSchedule};
pub use reorder_annual_schedule_rules::{reorder_annual_schedule_rules, ReorderAnnualScheduleRules};
