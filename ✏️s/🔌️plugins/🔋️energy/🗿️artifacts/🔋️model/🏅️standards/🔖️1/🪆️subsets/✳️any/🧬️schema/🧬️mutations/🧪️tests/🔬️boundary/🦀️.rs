//! 🧪️ The `boundary` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_surface_sun_exposed {
    pub use crate::boundary_mutations::change_surface_sun_exposed::*;
    #[path = "../../🔲️boundary/🌅️change-sun-exposed/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔲️boundary/🌅️change-sun-exposed/🧪️tests/✅️shades/🦀️.rs"]
    mod tests_shades;
}

#[path = "."]
mod change_surface_wind_exposed {
    pub use crate::boundary_mutations::change_surface_wind_exposed::*;
    #[path = "../../🔲️boundary/🍃️change-wind-exposed/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔲️boundary/🍃️change-wind-exposed/🧪️tests/✅️shelters/🦀️.rs"]
    mod tests_shelters;
}

#[path = "."]
mod disconnect_surfaces {
    pub use crate::boundary_mutations::disconnect_surfaces::*;
    #[path = "../../🔲️boundary/💔️disconnect/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔲️boundary/💔️disconnect/🧪️tests/✅️parts/🦀️.rs"]
    mod tests_parts;
}

#[path = "."]
mod change_surface_boundary_condition {
    pub use crate::boundary_mutations::change_surface_boundary_condition::*;
    #[path = "../../🔲️boundary/🚧️change-boundary-condition/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔲️boundary/🚧️change-boundary-condition/🧪️tests/✅️grounds/🦀️.rs"]
    mod tests_grounds;
}

#[path = "."]
mod connect_surfaces {
    pub use crate::boundary_mutations::connect_surfaces::*;
    #[path = "../../🔲️boundary/🤝️connect/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔲️boundary/🤝️connect/🧪️tests/✅️joins-two-walls/🦀️.rs"]
    mod tests_joins_two_walls;
}
