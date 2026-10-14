//! 🧪️ The `solar-thermal` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_solar_thermal as solar_thermal_mutations;

#[path = "."]
mod change_solar_thermal_system_azimuth {
    pub use crate::solar_thermal_mutations::change_solar_thermal_system_azimuth::*;
    #[path = "../../🔥️solar-thermal/⛵️change-azimuth/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/⛵️change-azimuth/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_solar_thermal_system {
    pub use crate::solar_thermal_mutations::create_solar_thermal_system::*;
    #[path = "../../🔥️solar-thermal/🌄️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/🌄️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_solar_thermal_system {
    pub use crate::solar_thermal_mutations::delete_solar_thermal_system::*;
    #[path = "../../🔥️solar-thermal/🌆️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/🌆️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_solar_thermal_system_efficiency {
    pub use crate::solar_thermal_mutations::change_solar_thermal_system_efficiency::*;
    #[path = "../../🔥️solar-thermal/🏅️change-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/🏅️change-efficiency/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_solar_thermal_system_tilt {
    pub use crate::solar_thermal_mutations::change_solar_thermal_system_tilt::*;
    #[path = "../../🔥️solar-thermal/🔼️change-tilt/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/🔼️change-tilt/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_solar_thermal_system_collector_area {
    pub use crate::solar_thermal_mutations::change_solar_thermal_system_collector_area::*;
    #[path = "../../🔥️solar-thermal/🟩️change-collector-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/🟩️change-collector-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_solar_thermal_system_storage_volume {
    pub use crate::solar_thermal_mutations::change_solar_thermal_system_storage_volume::*;
    #[path = "../../🔥️solar-thermal/🧃️change-storage-volume/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔥️solar-thermal/🧃️change-storage-volume/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
