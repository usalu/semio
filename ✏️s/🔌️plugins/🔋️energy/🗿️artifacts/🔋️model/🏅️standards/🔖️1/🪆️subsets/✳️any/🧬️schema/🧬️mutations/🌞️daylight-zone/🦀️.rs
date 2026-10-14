//! 🌞️ Energy model mutation domain `daylight-zone`: `delete-daylight-zone`, `change-daylight-zone-zone`, `create-daylight-zone`, `change-daylight-zone-glare-limit`, `change-daylight-zone-window-transmittance`, `change-daylight-zone-illuminance-target`.

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
pub mod delete_daylight_zone {
    #[path = "🌗️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌗️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌗️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_daylight_zone_zone {
    #[path = "🏫️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏫️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏫️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_daylight_zone {
    #[path = "🔭️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔭️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔭️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_daylight_zone_glare_limit {
    #[path = "🕶️change-glare-limit/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕶️change-glare-limit/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕶️change-glare-limit/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_daylight_zone_window_transmittance {
    #[path = "🥃️change-window-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🥃️change-window-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🥃️change-window-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_daylight_zone_illuminance_target {
    #[path = "🪔️change-illuminance-target/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪔️change-illuminance-target/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪔️change-illuminance-target/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use delete_daylight_zone::{delete_daylight_zone, DeleteDaylightZone};
pub use change_daylight_zone_zone::{change_daylight_zone_zone, ChangeDaylightZoneZone};
pub use create_daylight_zone::{create_daylight_zone, CreateDaylightZone};
pub use change_daylight_zone_glare_limit::{change_daylight_zone_glare_limit, ChangeDaylightZoneGlareLimit};
pub use change_daylight_zone_window_transmittance::{change_daylight_zone_window_transmittance, ChangeDaylightZoneWindowTransmittance};
pub use change_daylight_zone_illuminance_target::{change_daylight_zone_illuminance_target, ChangeDaylightZoneIlluminanceTarget};
