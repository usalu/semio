//! 🪨️ Energy model mutation domain `material`: `change-material-specific-heat`, `change-material-density`, `change-material-thickness`, `change-material-conductivity`, `create-material`, `rename-material`, `delete-material`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod change_material_specific_heat {
    #[path = "♨️change-specific-heat/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "♨️change-specific-heat/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "♨️change-specific-heat/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_material_density {
    #[path = "⚖️change-density/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⚖️change-density/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⚖️change-density/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_material_thickness {
    #[path = "📏️change-thickness/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📏️change-thickness/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📏️change-thickness/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_material_conductivity {
    #[path = "🔥️change-conductivity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔥️change-conductivity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔥️change-conductivity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_material {
    #[path = "🧱️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧱️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧱️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_material {
    #[path = "🪧️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪧️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪧️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_material {
    #[path = "🪨️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪨️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪨️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_material_specific_heat::{change_material_specific_heat, ChangeMaterialSpecificHeat};
pub use change_material_density::{change_material_density, ChangeMaterialDensity};
pub use change_material_thickness::{change_material_thickness, ChangeMaterialThickness};
pub use change_material_conductivity::{change_material_conductivity, ChangeMaterialConductivity};
pub use create_material::{create_material, CreateMaterial};
pub use rename_material::{rename_material, RenameMaterial};
pub use delete_material::{delete_material, DeleteMaterial};
