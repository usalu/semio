//! 🧪️ The `day-schedule` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_day_schedule as day_schedule_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod delete_daily_schedule {
    pub use crate::day_schedule_mutations::delete_daily_schedule::*;
    #[path = "../../🕒️day-schedule/🌓️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕒️day-schedule/🌓️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod replace_daily_schedule_hourly_values {
    pub use crate::day_schedule_mutations::replace_daily_schedule_hourly_values::*;
    #[path = "../../🕒️day-schedule/🕔️replace-hourly-values/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕒️day-schedule/🕔️replace-hourly-values/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_daily_schedule_interpolation {
    pub use crate::day_schedule_mutations::change_daily_schedule_interpolation::*;
    #[path = "../../🕒️day-schedule/🕕️change-interpolation/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕒️day-schedule/🕕️change-interpolation/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_daily_schedule {
    pub use crate::day_schedule_mutations::create_daily_schedule::*;
    #[path = "../../🕒️day-schedule/🕞️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕒️day-schedule/🕞️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_daily_schedule_limits {
    pub use crate::day_schedule_mutations::change_daily_schedule_limits::*;
    #[path = "../../🕒️day-schedule/🕟️change-limits/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕒️day-schedule/🕟️change-limits/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
