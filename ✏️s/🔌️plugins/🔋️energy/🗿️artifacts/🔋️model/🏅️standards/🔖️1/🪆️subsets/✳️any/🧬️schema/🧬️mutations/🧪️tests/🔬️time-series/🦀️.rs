//! 🧪️ The `time-series` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_time_series as time_series_mutations;

#[path = "."]
mod delete_time_series_schedule {
    pub use crate::time_series_mutations::delete_time_series_schedule::*;
    #[path = "../../📈️time-series/🎞️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📈️time-series/🎞️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod replace_time_series_schedule_values {
    pub use crate::time_series_mutations::replace_time_series_schedule_values::*;
    #[path = "../../📈️time-series/🕘️replace-values/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📈️time-series/🕘️replace-values/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_time_series_schedule_timestep {
    pub use crate::time_series_mutations::change_time_series_schedule_timestep::*;
    #[path = "../../📈️time-series/🕙️change-timestep/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📈️time-series/🕙️change-timestep/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_time_series_schedule {
    pub use crate::time_series_mutations::create_time_series_schedule::*;
    #[path = "../../📈️time-series/🪗️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📈️time-series/🪗️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
