//! 🪫️ Energy model mutation domain `battery`: `change-battery-max-charge`, `change-battery-max-discharge`, `delete-battery`, `change-battery-round-trip-efficiency`, `change-battery-capacity`, `create-battery`.

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
pub mod change_battery_max_charge {
    #[path = "⏫️change-max-charge/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⏫️change-max-charge/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⏫️change-max-charge/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⏫️change-max-charge/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_battery_max_discharge {
    #[path = "⏬️change-max-discharge/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⏬️change-max-discharge/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⏬️change-max-discharge/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⏬️change-max-discharge/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_battery {
    #[path = "♒️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "♒️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "♒️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "♒️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_battery_round_trip_efficiency {
    #[path = "🥉️change-round-trip-efficiency/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🥉️change-round-trip-efficiency/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🥉️change-round-trip-efficiency/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🥉️change-round-trip-efficiency/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_battery_capacity {
    #[path = "🥫️change-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🥫️change-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🥫️change-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🥫️change-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_battery {
    #[path = "🪙️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪙️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪙️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪙️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_battery_max_charge::{change_battery_max_charge, ChangeBatteryMaxCharge};
pub use change_battery_max_discharge::{change_battery_max_discharge, ChangeBatteryMaxDischarge};
pub use delete_battery::{delete_battery, DeleteBattery};
pub use change_battery_round_trip_efficiency::{change_battery_round_trip_efficiency, ChangeBatteryRoundTripEfficiency};
pub use change_battery_capacity::{change_battery_capacity, ChangeBatteryCapacity};
pub use create_battery::{create_battery, CreateBattery};
