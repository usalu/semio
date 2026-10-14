//! 🧱️ Energy model mutation domain `surface`: `rename-surface`, `change-surface-multiplier`, `replace-surface-vertices`, `change-surface-zone`, `create-surface`, `change-surface-class`, `change-surface-construction`, `delete-surface`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod rename_surface {
    #[path = "🏳️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏳️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏳️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_multiplier {
    #[path = "🔁️change-multiplier/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔁️change-multiplier/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔁️change-multiplier/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_surface_vertices {
    #[path = "🔺️replace-vertices/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔺️replace-vertices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔺️replace-vertices/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_zone {
    #[path = "🗜️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
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
    #[path = "🟫️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟫️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_class {
    #[path = "🧩️change-class/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧩️change-class/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧩️change-class/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_construction {
    #[path = "🧰️change-construction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧰️change-construction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧰️change-construction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_surface {
    #[path = "🪚️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪚️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪚️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use rename_surface::{rename_surface, RenameSurface};
pub use change_surface_multiplier::{change_surface_multiplier, ChangeSurfaceMultiplier};
pub use replace_surface_vertices::{replace_surface_vertices, ReplaceSurfaceVertices};
pub use change_surface_zone::{change_surface_zone, ChangeSurfaceZone};
pub use create_surface::{create_surface, CreateSurface};
pub use change_surface_class::{change_surface_class, ChangeSurfaceClass};
pub use change_surface_construction::{change_surface_construction, ChangeSurfaceConstruction};
pub use delete_surface::{delete_surface, DeleteSurface};
