//! 📍️ Energy model mutation domain `site`: `change-site-elevation`, `change-site-latitude`, `change-site-longitude`, `change-site-north-axis`, `change-site-time-zone`

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_site_elevation {
    #[path = "⛰️change-elevation/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⛰️change-elevation/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⛰️change-elevation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⛰️change-elevation/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_site_latitude {
    #[path = "🌍️change-latitude/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌍️change-latitude/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌍️change-latitude/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌍️change-latitude/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_site_longitude {
    #[path = "🌏️change-longitude/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌏️change-longitude/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌏️change-longitude/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌏️change-longitude/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_site_north_axis {
    #[path = "🔝️change-north-axis/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔝️change-north-axis/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔝️change-north-axis/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔝️change-north-axis/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_site_time_zone {
    #[path = "🕰️change-time-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕰️change-time-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕰️change-time-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕰️change-time-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_site_elevation::{change_site_elevation, ChangeSiteElevation};
pub use change_site_latitude::{change_site_latitude, ChangeSiteLatitude};
pub use change_site_longitude::{change_site_longitude, ChangeSiteLongitude};
pub use change_site_north_axis::{change_site_north_axis, ChangeSiteNorthAxis};
pub use change_site_time_zone::{change_site_time_zone, ChangeSiteTimeZone};
