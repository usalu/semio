//! 📆️ Energy model mutation domain `year-schedule`: `change-annual-schedule-default-daily-schedule`, `delete-annual-schedule`, `insert-annual-schedule-rule`, `remove-annual-schedule-rule`, `create-annual-schedule`, `reorder-annual-schedule-rules`

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_year_schedule_holidays as year_schedule_holidays_mutations;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_annual_schedule_default_daily_schedule {
    #[path = "🎌️change-default-daily-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎌️change-default-daily-schedule/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "📕️delete/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "📗️insert-rule/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "📙️remove-rule/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "📚️create/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "🗂️reorder-rules/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🗂️reorder-rules/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗂️reorder-rules/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_annual_schedule_default_daily_schedule::{change_annual_schedule_default_daily_schedule, ChangeAnnualScheduleDefaultDailySchedule};
pub use delete_annual_schedule::{delete_annual_schedule, DeleteAnnualSchedule};
pub use insert_annual_schedule_rule::{insert_annual_schedule_rule, InsertAnnualScheduleRule};
pub use remove_annual_schedule_rule::{remove_annual_schedule_rule, RemoveAnnualScheduleRule};
pub use create_annual_schedule::{create_annual_schedule, CreateAnnualSchedule};
pub use reorder_annual_schedule_rules::{reorder_annual_schedule_rules, ReorderAnnualScheduleRules};
pub use year_schedule_holidays_mutations::AddAnnualScheduleHoliday;
