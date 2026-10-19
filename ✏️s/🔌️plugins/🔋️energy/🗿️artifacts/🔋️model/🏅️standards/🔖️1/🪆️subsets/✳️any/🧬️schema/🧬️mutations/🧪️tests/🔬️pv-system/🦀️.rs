//! 🧪️ The `pv-system` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system as pv_system_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system_performance as pv_system_performance_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_pv_system_inverter_efficiency {
    pub use crate::pv_system_performance_mutations::change_pv_system_inverter_efficiency::*;
    #[path = "../../☀️pv-system/♌️change-inverter-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/♌️change-inverter-efficiency/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_pv_system_dc_capacity {
    pub use crate::pv_system_performance_mutations::change_pv_system_dc_capacity::*;
    #[path = "../../☀️pv-system/⚛️change-dc-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/⚛️change-dc-capacity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_pv_system {
    pub use crate::pv_system_mutations::create_pv_system::*;
    #[path = "../../☀️pv-system/✨️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/✨️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_pv_system {
    pub use crate::pv_system_mutations::delete_pv_system::*;
    #[path = "../../☀️pv-system/🌒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/🌒️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_pv_system_module_efficiency {
    pub use crate::pv_system_performance_mutations::change_pv_system_module_efficiency::*;
    #[path = "../../☀️pv-system/🎖️change-module-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/🎖️change-module-efficiency/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_pv_system_tilt {
    pub use crate::pv_system_mutations::change_pv_system_tilt::*;
    #[path = "../../☀️pv-system/📈️change-tilt/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/📈️change-tilt/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_pv_system_area {
    pub use crate::pv_system_mutations::change_pv_system_area::*;
    #[path = "../../☀️pv-system/🟨️change-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/🟨️change-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_pv_system_azimuth {
    pub use crate::pv_system_mutations::change_pv_system_azimuth::*;
    #[path = "../../☀️pv-system/🧿️change-azimuth/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../☀️pv-system/🧿️change-azimuth/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
