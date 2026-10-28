//! 🌈️ Energy model mutation domain `window-layer-optics`: `change-glazing-material-solar-transmittance`, `change-glazing-material-visible-transmittance`, `change-glazing-material-infrared-emissivity`, `change-glazing-material-solar-reflectance-front`, `change-glazing-material-solar-reflectance-back`, `change-glazing-material-visible-reflectance-front`, `change-glazing-material-visible-reflectance-back`, `change-glazing-material-infrared-transmittance`.

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
pub mod change_glazing_material_solar_transmittance {
    #[path = "../🥂️window-layer/🟡️change-glazing-solar-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🟡️change-glazing-solar-transmittance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🟡️change-glazing-solar-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🟡️change-glazing-solar-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_visible_transmittance {
    #[path = "../🥂️window-layer/🥽️change-glazing-visible-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🥽️change-glazing-visible-transmittance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🥽️change-glazing-visible-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🥽️change-glazing-visible-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_infrared_emissivity {
    #[path = "../🥂️window-layer/🩻️change-glazing-infrared-emissivity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🩻️change-glazing-infrared-emissivity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🩻️change-glazing-infrared-emissivity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🩻️change-glazing-infrared-emissivity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_solar_transmittance::{change_glazing_material_solar_transmittance, ChangeGlazingMaterialSolarTransmittance};
pub use change_glazing_material_visible_transmittance::{change_glazing_material_visible_transmittance, ChangeGlazingMaterialVisibleTransmittance};
pub use change_glazing_material_infrared_emissivity::{change_glazing_material_infrared_emissivity, ChangeGlazingMaterialInfraredEmissivity};

#[path = "."]
pub mod change_glazing_material_solar_reflectance_front {
    #[path = "../🥂️window-layer/🔅️solar-front/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🔅️solar-front/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🔅️solar-front/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🔅️solar-front/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_solar_reflectance_front::{change_glazing_material_solar_reflectance_front, ChangeGlazingMaterialSolarReflectanceFront};

#[path = "."]
pub mod change_glazing_material_solar_reflectance_back {
    #[path = "../🥂️window-layer/🔆️solar-back/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🔆️solar-back/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🔆️solar-back/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🔆️solar-back/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_solar_reflectance_back::{change_glazing_material_solar_reflectance_back, ChangeGlazingMaterialSolarReflectanceBack};

#[path = "."]
pub mod change_glazing_material_visible_reflectance_front {
    #[path = "../🥂️window-layer/🔎️visible-front/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🔎️visible-front/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🔎️visible-front/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🔎️visible-front/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_visible_reflectance_front::{change_glazing_material_visible_reflectance_front, ChangeGlazingMaterialVisibleReflectanceFront};

#[path = "."]
pub mod change_glazing_material_visible_reflectance_back {
    #[path = "../🥂️window-layer/🪞️visible-back/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🪞️visible-back/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🪞️visible-back/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🪞️visible-back/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_visible_reflectance_back::{change_glazing_material_visible_reflectance_back, ChangeGlazingMaterialVisibleReflectanceBack};

#[path = "."]
pub mod change_glazing_material_infrared_transmittance {
    #[path = "../🥂️window-layer/🌡️infrared-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🥂️window-layer/🌡️infrared-transmittance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🥂️window-layer/🌡️infrared-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🥂️window-layer/🌡️infrared-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_glazing_material_infrared_transmittance::{change_glazing_material_infrared_transmittance, ChangeGlazingMaterialInfraredTransmittance};
