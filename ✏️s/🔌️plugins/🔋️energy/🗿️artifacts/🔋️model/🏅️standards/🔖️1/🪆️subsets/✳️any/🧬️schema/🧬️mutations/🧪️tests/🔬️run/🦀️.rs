//! 🧪️ The `run` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_run as run_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_run_end_day {
    pub use crate::run_mutations::change_run_end_day::*;
    #[path = "../../⏯️run/⏹️change-end-day/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⏯️run/⏹️change-end-day/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_run_start_day {
    pub use crate::run_mutations::change_run_start_day::*;
    #[path = "../../⏯️run/▶️change-start-day/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⏯️run/▶️change-start-day/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_run_year {
    pub use crate::run_mutations::change_run_year::*;
    #[path = "../../⏯️run/📅️change-year/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⏯️run/📅️change-year/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_run_start_month {
    pub use crate::run_mutations::change_run_start_month::*;
    #[path = "../../⏯️run/🛫️change-start-month/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⏯️run/🛫️change-start-month/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_run_end_month {
    pub use crate::run_mutations::change_run_end_month::*;
    #[path = "../../⏯️run/🛬️change-end-month/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⏯️run/🛬️change-end-month/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}
