//! 🔍️ Energy model mutation domain `glazing`: `change-fenestration-shgc`, `change-fenestration-vlt`, `change-fenestration-u-value`, `change-fenestration-frame-conductance`, `bind-fenestration-glazing-construction`, `change-fenestration-divider-conductance`, `clear-fenestration-glazing-construction`.

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
pub mod change_fenestration_shgc {
    #[path = "🌇️change-shgc/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌇️change-shgc/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌇️change-shgc/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_vlt {
    #[path = "🌈️change-vlt/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌈️change-vlt/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌈️change-vlt/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_u_value {
    #[path = "🌐️change-u-value/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌐️change-u-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌐️change-u-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_frame_conductance {
    #[path = "🖼️change-frame-conductance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🖼️change-frame-conductance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖼️change-frame-conductance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod bind_fenestration_glazing_construction {
    #[path = "🧊️bind-glazing-construction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧊️bind-glazing-construction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧊️bind-glazing-construction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_divider_conductance {
    #[path = "🧷️change-divider-conductance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧷️change-divider-conductance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧷️change-divider-conductance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod clear_fenestration_glazing_construction {
    #[path = "🫗️clear-glazing-construction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🫗️clear-glazing-construction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🫗️clear-glazing-construction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_fenestration_shgc::{change_fenestration_shgc, ChangeFenestrationShgc};
pub use change_fenestration_vlt::{change_fenestration_vlt, ChangeFenestrationVlt};
pub use change_fenestration_u_value::{change_fenestration_u_value, ChangeFenestrationUValue};
pub use change_fenestration_frame_conductance::{change_fenestration_frame_conductance, ChangeFenestrationFrameConductance};
pub use bind_fenestration_glazing_construction::{bind_fenestration_glazing_construction, BindFenestrationGlazingConstruction};
pub use change_fenestration_divider_conductance::{change_fenestration_divider_conductance, ChangeFenestrationDividerConductance};
pub use clear_fenestration_glazing_construction::{clear_fenestration_glazing_construction, ClearFenestrationGlazingConstruction};
