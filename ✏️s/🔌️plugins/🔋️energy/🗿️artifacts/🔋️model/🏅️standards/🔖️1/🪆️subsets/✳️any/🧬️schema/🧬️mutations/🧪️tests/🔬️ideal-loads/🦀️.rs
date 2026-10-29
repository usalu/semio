//! 🧪️ The `ideal-loads` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_ideal_loads as ideal_loads_mutations;
extern crate semio_s_artifact_energy_model_mut_ideal_loads_limits as ideal_loads_limits_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_ideal_loads_system_max_heating_capacity {
    pub use crate::ideal_loads_limits_mutations::change_ideal_loads_system_max_heating_capacity::*;
    #[path = "../../💯️ideal-loads/⛽️change-max-heating-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/⛽️change-max-heating-capacity/🧪️tests/✅️caps/🦀️.rs"]
    mod tests_caps;
}

#[path = "."]
mod change_ideal_loads_system_zone {
    pub use crate::ideal_loads_mutations::change_ideal_loads_system_zone::*;
    #[path = "../../💯️ideal-loads/🏢️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🏢️change-zone/🧪️tests/✅️moves/🦀️.rs"]
    mod tests_moves;
}

#[path = "."]
mod change_ideal_loads_system_outdoor_air_per_area {
    pub use crate::ideal_loads_mutations::change_ideal_loads_system_outdoor_air_per_area::*;
    #[path = "../../💯️ideal-loads/🔳️change-outdoor-air-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🔳️change-outdoor-air-per-area/🧪️tests/✅️ventilates/🦀️.rs"]
    mod tests_ventilates;
}

#[path = "."]
mod change_ideal_loads_system_max_heating_supply_air_temp {
    pub use crate::ideal_loads_limits_mutations::change_ideal_loads_system_max_heating_supply_air_temp::*;
    #[path = "../../💯️ideal-loads/🔴️change-max-heating-supply-air-temp/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🔴️change-max-heating-supply-air-temp/🧪️tests/✅️cools/🦀️.rs"]
    mod tests_cools;
}

#[path = "."]
mod change_ideal_loads_system_min_cooling_supply_air_temp {
    pub use crate::ideal_loads_limits_mutations::change_ideal_loads_system_min_cooling_supply_air_temp::*;
    #[path = "../../💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🔵️change-min-cooling-supply-air-temp/🧪️tests/✅️lowers/🦀️.rs"]
    mod tests_lowers;
}

#[path = "."]
mod change_ideal_loads_system_max_cooling_capacity {
    pub use crate::ideal_loads_limits_mutations::change_ideal_loads_system_max_cooling_capacity::*;
    #[path = "../../💯️ideal-loads/🟧️change-max-cooling-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🟧️change-max-cooling-capacity/🧪️tests/✅️caps/🦀️.rs"]
    mod tests_caps;
}

#[path = "."]
mod change_ideal_loads_system_outdoor_air_per_person {
    pub use crate::ideal_loads_mutations::change_ideal_loads_system_outdoor_air_per_person::*;
    #[path = "../../💯️ideal-loads/🧍️change-outdoor-air-per-person/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🧍️change-outdoor-air-per-person/🧪️tests/✅️ventilates/🦀️.rs"]
    mod tests_ventilates;
}

#[path = "."]
mod create_ideal_loads_system {
    pub use crate::ideal_loads_mutations::create_ideal_loads_system::*;
    #[path = "../../💯️ideal-loads/🫁️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🫁️create/🧪️tests/✅️serves/🦀️.rs"]
    mod tests_serves;
}

#[path = "."]
mod delete_ideal_loads_system {
    pub use crate::ideal_loads_mutations::delete_ideal_loads_system::*;
    #[path = "../../💯️ideal-loads/🫥️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💯️ideal-loads/🫥️delete/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}
