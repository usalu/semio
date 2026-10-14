//! 🧪️ The `thermostat` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_thermostat as thermostat_mutations;

#[path = "."]
mod change_thermostat_heating_throttle_range {
    pub use crate::thermostat_mutations::change_thermostat_heating_throttle_range::*;
    #[path = "../../🌡️thermostat/🎚️change-heating-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🎚️change-heating-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
    mod tests_widens;
}

#[path = "."]
mod change_thermostat_cooling_throttle_range {
    pub use crate::thermostat_mutations::change_thermostat_cooling_throttle_range::*;
    #[path = "../../🌡️thermostat/🎛️change-cooling-throttle-range/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🎛️change-cooling-throttle-range/🧪️tests/✅️widens/🦀️.rs"]
    mod tests_widens;
}

#[path = "."]
mod change_thermostat_cooling_setpoint_schedule {
    pub use crate::thermostat_mutations::change_thermostat_cooling_setpoint_schedule::*;
    #[path = "../../🌡️thermostat/🐧️change-cooling-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🐧️change-cooling-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod delete_thermostat {
    pub use crate::thermostat_mutations::delete_thermostat::*;
    #[path = "../../🌡️thermostat/🛑️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🛑️delete/🧪️tests/✅️frees-zone-one/🦀️.rs"]
    mod tests_frees_zone_one;
}

#[path = "."]
mod change_thermostat_zone {
    pub use crate::thermostat_mutations::change_thermostat_zone::*;
    #[path = "../../🌡️thermostat/🛖️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🛖️change-zone/🧪️tests/✅️moves/🦀️.rs"]
    mod tests_moves;
}

#[path = "."]
mod change_thermostat_heating_setpoint_schedule {
    pub use crate::thermostat_mutations::change_thermostat_heating_setpoint_schedule::*;
    #[path = "../../🌡️thermostat/🥵️change-heating-setpoint-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🥵️change-heating-setpoint-schedule/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod create_thermostat {
    pub use crate::thermostat_mutations::create_thermostat::*;
    #[path = "../../🌡️thermostat/🩺️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌡️thermostat/🩺️create/🧪️tests/✅️controls/🦀️.rs"]
    mod tests_controls;
}
