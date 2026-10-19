//! ☀️ Energy model mutation domain `pv-system`: `create-pv-system`, `delete-pv-system`, `change-pv-system-tilt`, `change-pv-system-area`, `change-pv-system-azimuth`, `change-pv-system-mounting`.

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
pub mod create_pv_system {
    #[path = "✨️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "✨️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✨️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_pv_system {
    #[path = "🌒️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌒️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌒️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_pv_system_tilt {
    #[path = "📈️change-tilt/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📈️change-tilt/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📈️change-tilt/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_pv_system_area {
    #[path = "🟨️change-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟨️change-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟨️change-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_pv_system_azimuth {
    #[path = "🧿️change-azimuth/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧿️change-azimuth/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧿️change-azimuth/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use create_pv_system::{create_pv_system, CreatePvSystem};
pub use delete_pv_system::{delete_pv_system, DeletePvSystem};
pub use change_pv_system_tilt::{change_pv_system_tilt, ChangePvSystemTilt};
pub use change_pv_system_area::{change_pv_system_area, ChangePvSystemArea};
pub use change_pv_system_azimuth::{change_pv_system_azimuth, ChangePvSystemAzimuth};

#[path = "."]
pub mod change_pv_system_mounting {
    #[path = "🏠️change-mounting/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏠️change-mounting/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏠️change-mounting/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_pv_system_mounting::{change_pv_system_mounting, ChangePvSystemMounting};
