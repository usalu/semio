//! 🌳️ Energy model mutation domain `shading`: `change-shading-surface-transmittance-schedule`, `create-shading-surface`, `rename-shading-surface`, `replace-shading-surface-vertices`, `delete-shading-surface`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod change_shading_surface_transmittance_schedule {
    #[path = "⛱️change-transmittance-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⛱️change-transmittance-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⛱️change-transmittance-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_shading_surface {
    #[path = "🌳️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌳️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌳️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_shading_surface {
    #[path = "🏕️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏕️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏕️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_shading_surface_vertices {
    #[path = "🗺️replace-vertices/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗺️replace-vertices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗺️replace-vertices/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_shading_surface {
    #[path = "🪵️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪵️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪵️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_shading_surface_transmittance_schedule::{change_shading_surface_transmittance_schedule, ChangeShadingSurfaceTransmittanceSchedule};
pub use create_shading_surface::{create_shading_surface, CreateShadingSurface};
pub use rename_shading_surface::{rename_shading_surface, RenameShadingSurface};
pub use replace_shading_surface_vertices::{replace_shading_surface_vertices, ReplaceShadingSurfaceVertices};
pub use delete_shading_surface::{delete_shading_surface, DeleteShadingSurface};
