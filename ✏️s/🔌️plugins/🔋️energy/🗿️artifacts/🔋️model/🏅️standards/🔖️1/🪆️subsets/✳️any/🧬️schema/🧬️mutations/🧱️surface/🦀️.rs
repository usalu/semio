//! 🧱️ Energy model mutation domain `surface`: `rename-surface`, `change-surface-zone`, `create-surface`, `delete-surface`

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration_geometry as fenestration_geometry_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod rename_surface {
    #[path = "🏳️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏳️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏳️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏳️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_zone {
    #[path = "🗜️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗜️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🗜️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗜️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_surface {
    #[path = "🟫️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟫️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟫️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟫️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_surface {
    #[path = "🪚️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪚️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪚️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪚️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use rename_surface::{rename_surface, RenameSurface};
pub use change_surface_zone::{change_surface_zone, ChangeSurfaceZone};
pub use create_surface::{create_surface, CreateSurface};
pub use delete_surface::{delete_surface, DeleteSurface};

pub use fenestration_mutations::{CreateFenestration};
pub use fenestration_geometry_mutations::{ReplaceFenestrationVertices};
pub use boundary_mutations::ConnectSurfaces;
