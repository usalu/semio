//! 🧪️ The `humidistat` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_humidistat as humidistat_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_humidistat_humidifying_setpoint_schedule {
    pub use crate::humidistat_mutations::change_humidistat_humidifying_setpoint_schedule::*;
    #[path = "../../💧️humidistat/☔️change-humidifying-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/☔️change-humidifying-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod create_humidistat {
    pub use crate::humidistat_mutations::create_humidistat::*;
    #[path = "../../💧️humidistat/🌂️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/🌂️create/🧪️tests/✅️controls/🦀️.rs"]
    mod tests_controls;
}

#[path = "."]
mod change_humidistat_humidifying_throttle_range {
    pub use crate::humidistat_mutations::change_humidistat_humidifying_throttle_range::*;
    #[path = "../../💧️humidistat/🌧️change-humidifying-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/🌧️change-humidifying-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
    mod tests_widens;
}

#[path = "."]
mod change_humidistat_zone {
    pub use crate::humidistat_mutations::change_humidistat_zone::*;
    #[path = "../../💧️humidistat/🏙️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/🏙️change-zone/🧪️tests/✅️moves/🦀️.rs"]
    mod tests_moves;
}

#[path = "."]
mod delete_humidistat {
    pub use crate::humidistat_mutations::delete_humidistat::*;
    #[path = "../../💧️humidistat/🏜️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/🏜️delete/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}

#[path = "."]
mod change_humidistat_dehumidifying_setpoint_schedule {
    pub use crate::humidistat_mutations::change_humidistat_dehumidifying_setpoint_schedule::*;
    #[path = "../../💧️humidistat/🏝️dehumidify-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/🏝️dehumidify-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod change_humidistat_dehumidifying_throttle_range {
    pub use crate::humidistat_mutations::change_humidistat_dehumidifying_throttle_range::*;
    #[path = "../../💧️humidistat/🧻️change-dehumidifying-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💧️humidistat/🧻️change-dehumidifying-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
    mod tests_widens;
}
