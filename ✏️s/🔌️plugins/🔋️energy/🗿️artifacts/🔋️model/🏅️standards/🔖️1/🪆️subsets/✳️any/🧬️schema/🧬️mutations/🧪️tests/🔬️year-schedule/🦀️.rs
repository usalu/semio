//! 🧪️ The `year-schedule` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_year_schedule as year_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_year_schedule_holidays as year_schedule_holidays_mutations;

#[path = "."]
mod change_annual_schedule_holiday_daily_schedule {
    pub use crate::year_schedule_holidays_mutations::change_annual_schedule_holiday_daily_schedule::*;
    #[path = "../../📆️year-schedule/🎄️change-holiday-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/🎄️change-holiday-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod add_annual_schedule_holiday {
    pub use crate::year_schedule_holidays_mutations::add_annual_schedule_holiday::*;
    #[path = "../../📆️year-schedule/🎉️add-holiday/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/🎉️add-holiday/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_annual_schedule_holiday {
    pub use crate::year_schedule_holidays_mutations::remove_annual_schedule_holiday::*;
    #[path = "../../📆️year-schedule/🎊️remove-holiday/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/🎊️remove-holiday/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_annual_schedule_default_daily_schedule {
    pub use crate::year_schedule_mutations::change_annual_schedule_default_daily_schedule::*;
    #[path = "../../📆️year-schedule/🎌️change-default-daily-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/🎌️change-default-daily-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_annual_schedule {
    pub use crate::year_schedule_mutations::delete_annual_schedule::*;
    #[path = "../../📆️year-schedule/📕️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/📕️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod insert_annual_schedule_rule {
    pub use crate::year_schedule_mutations::insert_annual_schedule_rule::*;
    #[path = "../../📆️year-schedule/📗️insert-rule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/📗️insert-rule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_annual_schedule_rule {
    pub use crate::year_schedule_mutations::remove_annual_schedule_rule::*;
    #[path = "../../📆️year-schedule/📙️remove-rule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/📙️remove-rule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_annual_schedule {
    pub use crate::year_schedule_mutations::create_annual_schedule::*;
    #[path = "../../📆️year-schedule/📚️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/📚️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod reorder_annual_schedule_rules {
    pub use crate::year_schedule_mutations::reorder_annual_schedule_rules::*;
    #[path = "../../📆️year-schedule/🗂️reorder-rules/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📆️year-schedule/🗂️reorder-rules/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
