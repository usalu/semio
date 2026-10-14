//! 🧪️ The `week-schedule` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_week_schedule as week_schedule_mutations;

#[path = "."]
mod delete_weekly_schedule {
    pub use crate::week_schedule_mutations::delete_weekly_schedule::*;
    #[path = "../../🗓️week-schedule/🕖️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🗓️week-schedule/🕖️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_weekly_schedule_day {
    pub use crate::week_schedule_mutations::change_weekly_schedule_day::*;
    #[path = "../../🗓️week-schedule/🕗️change-day/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🗓️week-schedule/🕗️change-day/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_weekly_schedule {
    pub use crate::week_schedule_mutations::create_weekly_schedule::*;
    #[path = "../../🗓️week-schedule/🗓️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🗓️week-schedule/🗓️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
