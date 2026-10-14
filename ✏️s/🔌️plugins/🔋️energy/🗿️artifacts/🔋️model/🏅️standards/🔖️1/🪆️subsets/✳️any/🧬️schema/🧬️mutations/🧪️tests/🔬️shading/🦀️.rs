//! 🧪️ The `shading` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_shading as shading_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_shading_surface_transmittance_schedule {
    pub use crate::shading_mutations::change_shading_surface_transmittance_schedule::*;
    #[path = "../../🌳️shading/⛱️change-transmittance-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌳️shading/⛱️change-transmittance-schedule/🧪️tests/✅️lets/🦀️.rs"]
    mod tests_lets;
}

#[path = "."]
mod create_shading_surface {
    pub use crate::shading_mutations::create_shading_surface::*;
    #[path = "../../🌳️shading/🌳️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌳️shading/🌳️create/🧪️tests/✅️adds/🦀️.rs"]
    mod tests_adds;
}

#[path = "."]
mod rename_shading_surface {
    pub use crate::shading_mutations::rename_shading_surface::*;
    #[path = "../../🌳️shading/🏕️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌳️shading/🏕️rename/🧪️tests/✅️renames/🦀️.rs"]
    mod tests_renames;
}

#[path = "."]
mod replace_shading_surface_vertices {
    pub use crate::shading_mutations::replace_shading_surface_vertices::*;
    #[path = "../../🌳️shading/🗺️replace-vertices/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌳️shading/🗺️replace-vertices/🧪️tests/✅️deepens/🦀️.rs"]
    mod tests_deepens;
}

#[path = "."]
mod delete_shading_surface {
    pub use crate::shading_mutations::delete_shading_surface::*;
    #[path = "../../🌳️shading/🪵️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌳️shading/🪵️delete/🧪️tests/✅️removes/🦀️.rs"]
    mod tests_removes;
}
