//! 🔲️ Energy model mutation domain `boundary`: `change-surface-sun-exposed`, `change-surface-wind-exposed`, `disconnect-surfaces`, `change-surface-boundary-condition`, `connect-surfaces`.

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
pub mod change_surface_sun_exposed {
    #[path = "🌅️change-sun-exposed/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌅️change-sun-exposed/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌅️change-sun-exposed/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌅️change-sun-exposed/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_wind_exposed {
    #[path = "🍃️change-wind-exposed/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🍃️change-wind-exposed/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🍃️change-wind-exposed/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🍃️change-wind-exposed/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod disconnect_surfaces {
    #[path = "💔️disconnect/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💔️disconnect/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "💔️disconnect/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💔️disconnect/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_boundary_condition {
    #[path = "🚧️change-boundary-condition/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚧️change-boundary-condition/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚧️change-boundary-condition/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚧️change-boundary-condition/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod connect_surfaces {
    #[path = "🤝️connect/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🤝️connect/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🤝️connect/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🤝️connect/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_surface_sun_exposed::{change_surface_sun_exposed, ChangeSurfaceSunExposed};
pub use change_surface_wind_exposed::{change_surface_wind_exposed, ChangeSurfaceWindExposed};
pub use disconnect_surfaces::{disconnect_surfaces, DisconnectSurfaces};
pub use change_surface_boundary_condition::{change_surface_boundary_condition, ChangeSurfaceBoundaryCondition};
pub use connect_surfaces::{connect_surfaces, ConnectSurfaces};
