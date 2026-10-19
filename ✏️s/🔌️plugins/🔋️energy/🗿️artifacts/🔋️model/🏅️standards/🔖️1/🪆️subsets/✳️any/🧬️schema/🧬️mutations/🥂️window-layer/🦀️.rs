//! 🥂️ Energy model mutation domain `window-layer`: `rename-gas-material`, `change-glazing-material-thickness`, `change-glazing-material-conductivity`, `change-glazing-material-solar-transmittance`, `rename-glazing-material`, `change-gas-material-thickness`, `change-gas-material-gas`, `change-glazing-material-visible-transmittance`, `change-glazing-material-infrared-emissivity`.

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
pub mod rename_gas_material {
    #[path = "🔘️rename-gas/🦀️.rs"]
    mod component;
    pub use component::*;
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
    #[path = "🟠️change-glazing-conductivity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟠️change-glazing-conductivity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_solar_transmittance {
    #[path = "🟡️change-glazing-solar-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟡️change-glazing-solar-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟡️change-glazing-solar-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_glazing_material {
    #[path = "🟢️rename-glazing/🦀️.rs"]
    mod component;
    pub use component::*;
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
    #[path = "🟤️change-gas-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟤️change-gas-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_visible_transmittance {
    #[path = "🥽️change-glazing-visible-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🥽️change-glazing-visible-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🥽️change-glazing-visible-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_infrared_emissivity {
    #[path = "🩻️change-glazing-infrared-emissivity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🩻️change-glazing-infrared-emissivity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🩻️change-glazing-infrared-emissivity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use rename_gas_material::{rename_gas_material, RenameGasMaterial};
pub use change_glazing_material_thickness::{change_glazing_material_thickness, ChangeGlazingMaterialThickness};
pub use change_glazing_material_conductivity::{change_glazing_material_conductivity, ChangeGlazingMaterialConductivity};
pub use change_glazing_material_solar_transmittance::{change_glazing_material_solar_transmittance, ChangeGlazingMaterialSolarTransmittance};
pub use rename_glazing_material::{rename_glazing_material, RenameGlazingMaterial};
pub use change_gas_material_thickness::{change_gas_material_thickness, ChangeGasMaterialThickness};
pub use change_gas_material_gas::{change_gas_material_gas, ChangeGasMaterialGas};
pub use change_glazing_material_visible_transmittance::{change_glazing_material_visible_transmittance, ChangeGlazingMaterialVisibleTransmittance};
pub use change_glazing_material_infrared_emissivity::{change_glazing_material_infrared_emissivity, ChangeGlazingMaterialInfraredEmissivity};

#[path = "."]
pub mod change_glazing_material_solar_reflectance_front {
    #[path = "🔅️solar-front/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔅️solar-front/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔅️solar-front/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_solar_reflectance_front::{change_glazing_material_solar_reflectance_front, ChangeGlazingMaterialSolarReflectanceFront};

#[path = "."]
pub mod change_glazing_material_solar_reflectance_back {
    #[path = "🔆️solar-back/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔆️solar-back/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔆️solar-back/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_solar_reflectance_back::{change_glazing_material_solar_reflectance_back, ChangeGlazingMaterialSolarReflectanceBack};

#[path = "."]
pub mod change_glazing_material_visible_reflectance_front {
    #[path = "🔎️visible-front/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔎️visible-front/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔎️visible-front/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_visible_reflectance_front::{change_glazing_material_visible_reflectance_front, ChangeGlazingMaterialVisibleReflectanceFront};

#[path = "."]
pub mod change_glazing_material_visible_reflectance_back {
    #[path = "🪞️visible-back/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪞️visible-back/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪞️visible-back/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_visible_reflectance_back::{change_glazing_material_visible_reflectance_back, ChangeGlazingMaterialVisibleReflectanceBack};

#[path = "."]
pub mod change_glazing_material_infrared_transmittance {
    #[path = "🌡️infrared-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌡️infrared-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌡️infrared-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_infrared_transmittance::{change_glazing_material_infrared_transmittance, ChangeGlazingMaterialInfraredTransmittance};
