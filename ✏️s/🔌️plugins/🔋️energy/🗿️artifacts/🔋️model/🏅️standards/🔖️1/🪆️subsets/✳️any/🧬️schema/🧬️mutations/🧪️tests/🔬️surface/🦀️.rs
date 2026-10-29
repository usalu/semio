//! 🧪️ The `surface` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_surface as surface_mutations;
extern crate semio_s_artifact_energy_model_mut_surface_properties as surface_properties_mutations;

#[path = "."]
mod rename_surface {
    pub use crate::surface_mutations::rename_surface::*;
    #[path = "../../🧱️surface/🏳️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🏳️rename/🧪️tests/✅️renames-a-wall/🦀️.rs"]
    mod tests_renames_a_wall;
}

#[path = "."]
mod change_surface_multiplier {
    pub use crate::surface_properties_mutations::change_surface_multiplier::*;
    #[path = "../../🧱️surface/🔁️change-multiplier/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🔁️change-multiplier/🧪️tests/✅️repeats/🦀️.rs"]
    mod tests_repeats;
}

#[path = "."]
mod replace_surface_vertices {
    pub use crate::surface_properties_mutations::replace_surface_vertices::*;
    #[path = "../../🧱️surface/🔺️replace-vertices/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🔺️replace-vertices/🧪️tests/✅️narrows/🦀️.rs"]
    mod tests_narrows;
}

#[path = "."]
mod change_surface_zone {
    pub use crate::surface_mutations::change_surface_zone::*;
    #[path = "../../🧱️surface/🗜️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🗜️change-zone/🧪️tests/✅️moves-a-wall/🦀️.rs"]
    mod tests_moves_a_wall;
}

#[path = "."]
mod create_surface {
    pub use crate::surface_mutations::create_surface::*;
    #[path = "../../🧱️surface/🟫️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🟫️create/🧪️tests/✅️adds-a-south-wall/🦀️.rs"]
    mod tests_adds_a_south_wall;
}

#[path = "."]
mod change_surface_class {
    pub use crate::surface_properties_mutations::change_surface_class::*;
    #[path = "../../🧱️surface/🧩️change-class/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🧩️change-class/🧪️tests/✅️turns/🦀️.rs"]
    mod tests_turns;
}

#[path = "."]
mod change_surface_construction {
    pub use crate::surface_properties_mutations::change_surface_construction::*;
    #[path = "../../🧱️surface/🧰️change-construction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🧱️surface/🧰️change-construction/🧪️tests/✅️swaps/🦀️.rs"]
    mod tests_swaps;
}

#[path = "."]
mod delete_surface {
    pub use crate::surface_mutations::delete_surface::*;
    #[path = "../../🧱️surface/🪚️delete/🧪️tests/⛔️refuses-a-partner/🦀️.rs"]
    mod tests_refuses_a_partner;
    #[path = "../../🧱️surface/🪚️delete/🧪️tests/✅️cascades-a-window/🦀️.rs"]
    mod tests_cascades_a_window;
}
