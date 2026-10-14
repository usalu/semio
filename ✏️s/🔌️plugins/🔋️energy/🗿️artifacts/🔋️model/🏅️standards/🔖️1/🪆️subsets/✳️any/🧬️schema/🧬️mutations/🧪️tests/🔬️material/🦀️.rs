//! 🧪️ The `material` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_material as material_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_material_specific_heat {
    pub use crate::material_mutations::change_material_specific_heat::*;
    #[path = "../../🪨️material/♨️change-specific-heat/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/♨️change-specific-heat/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_material_density {
    pub use crate::material_mutations::change_material_density::*;
    #[path = "../../🪨️material/⚖️change-density/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/⚖️change-density/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_material_thickness {
    pub use crate::material_mutations::change_material_thickness::*;
    #[path = "../../🪨️material/📏️change-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/📏️change-thickness/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_material_conductivity {
    pub use crate::material_mutations::change_material_conductivity::*;
    #[path = "../../🪨️material/🔥️change-conductivity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/🔥️change-conductivity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_material {
    pub use crate::material_mutations::create_material::*;
    #[path = "../../🪨️material/🧱️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/🧱️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod rename_material {
    pub use crate::material_mutations::rename_material::*;
    #[path = "../../🪨️material/🪧️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/🪧️rename/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_material {
    pub use crate::material_mutations::delete_material::*;
    #[path = "../../🪨️material/🪨️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪨️material/🪨️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
