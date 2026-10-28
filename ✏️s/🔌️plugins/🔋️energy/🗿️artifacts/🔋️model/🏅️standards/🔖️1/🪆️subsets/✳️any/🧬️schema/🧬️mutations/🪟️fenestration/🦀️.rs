//! 🪟️ Energy model mutation domain `fenestration`: `change-fenestration-height`, `change-fenestration-sill-height`, `rename-fenestration`, `replace-fenestration-vertices`, `delete-fenestration`, `change-fenestration-area`, `change-fenestration-surface`, `create-fenestration`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_geometry as geometry;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_fenestration_height {
    #[path = "⬆️change-height/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⬆️change-height/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⬆️change-height/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⬆️change-height/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_sill_height {
    #[path = "⬇️change-sill-height/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⬇️change-sill-height/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⬇️change-sill-height/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⬇️change-sill-height/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_fenestration {
    #[path = "🏁️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏁️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏁️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏁️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_fenestration_vertices {
    #[path = "🔶️replace-vertices/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔶️replace-vertices/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔶️replace-vertices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔶️replace-vertices/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_fenestration {
    #[path = "🚪️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚪️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚪️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚪️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_area {
    #[path = "🟥️change-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟥️change-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟥️change-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟥️change-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_surface {
    #[path = "🧲️change-surface/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧲️change-surface/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧲️change-surface/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧲️change-surface/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_fenestration {
    #[path = "🪟️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪟️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪟️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪟️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_fenestration_height::{change_fenestration_height, ChangeFenestrationHeight};
pub use change_fenestration_sill_height::{change_fenestration_sill_height, ChangeFenestrationSillHeight};
pub use rename_fenestration::{rename_fenestration, RenameFenestration};
pub use replace_fenestration_vertices::{replace_fenestration_vertices, ReplaceFenestrationVertices};
pub use delete_fenestration::{delete_fenestration, DeleteFenestration};
pub use change_fenestration_area::{change_fenestration_area, ChangeFenestrationArea};
pub use change_fenestration_surface::{change_fenestration_surface, ChangeFenestrationSurface};
pub use create_fenestration::{create_fenestration, CreateFenestration};
