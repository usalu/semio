//! 🧪️ The `finish` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_finish as finish_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_material_solar_absorptance {
    pub use crate::finish_mutations::change_material_solar_absorptance::*;
    #[path = "../../🎨️finish/☀️change-solar-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎨️finish/☀️change-solar-absorptance/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_material_visible_absorptance {
    pub use crate::finish_mutations::change_material_visible_absorptance::*;
    #[path = "../../🎨️finish/👁️change-visible-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎨️finish/👁️change-visible-absorptance/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_material_thermal_absorptance {
    pub use crate::finish_mutations::change_material_thermal_absorptance::*;
    #[path = "../../🎨️finish/🔆️change-thermal-absorptance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎨️finish/🔆️change-thermal-absorptance/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_material_roughness {
    pub use crate::finish_mutations::change_material_roughness::*;
    #[path = "../../🎨️finish/🗻️change-roughness/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎨️finish/🗻️change-roughness/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
