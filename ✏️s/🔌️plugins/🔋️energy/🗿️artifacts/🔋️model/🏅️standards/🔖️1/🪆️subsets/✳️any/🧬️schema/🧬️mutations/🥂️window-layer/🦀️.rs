//! 🥂️ Energy model mutation domain `window-layer`: `rename-gas-material`, `change-glazing-material-thickness`, `change-glazing-material-conductivity`, `rename-glazing-material`, `change-gas-material-thickness`, `change-gas-material-gas`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod rename_gas_material {
    #[path = "🔘️rename-gas/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔘️rename-gas/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔘️rename-gas/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔘️rename-gas/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_thickness {
    #[path = "🔷️change-glazing-thickness/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔷️change-glazing-thickness/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔷️change-glazing-thickness/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔷️change-glazing-thickness/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_conductivity {
    #[path = "🟠️change-glazing-conductivity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟠️change-glazing-conductivity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟠️change-glazing-conductivity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟠️change-glazing-conductivity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_glazing_material {
    #[path = "🟢️rename-glazing/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟢️rename-glazing/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟢️rename-glazing/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟢️rename-glazing/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_gas_material_thickness {
    #[path = "🟣️change-gas-thickness/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟣️change-gas-thickness/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟣️change-gas-thickness/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟣️change-gas-thickness/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_gas_material_gas {
    #[path = "🟤️change-gas-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟤️change-gas-type/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟤️change-gas-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟤️change-gas-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use rename_gas_material::{rename_gas_material, RenameGasMaterial};
pub use change_glazing_material_thickness::{change_glazing_material_thickness, ChangeGlazingMaterialThickness};
pub use change_glazing_material_conductivity::{change_glazing_material_conductivity, ChangeGlazingMaterialConductivity};
pub use rename_glazing_material::{rename_glazing_material, RenameGlazingMaterial};
pub use change_gas_material_thickness::{change_gas_material_thickness, ChangeGasMaterialThickness};
pub use change_gas_material_gas::{change_gas_material_gas, ChangeGasMaterialGas};
