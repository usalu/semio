//! 🌪️ Energy model mutation domain `ventilation`: `create-mechanical-ventilation`, `change-mechanical-ventilation-fan-delta-pressure`, `change-mechanical-ventilation-fan-total-efficiency`, `change-mechanical-ventilation-schedule`, `delete-mechanical-ventilation`, `change-mechanical-ventilation-design-flow`, `change-mechanical-ventilation-zone`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod create_mechanical_ventilation {
    #[path = "🌪️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌪️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌪️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌪️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_mechanical_ventilation_fan_delta_pressure {
    #[path = "🎈️change-fan-delta-pressure/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎈️change-fan-delta-pressure/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🎈️change-fan-delta-pressure/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎈️change-fan-delta-pressure/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_mechanical_ventilation_fan_total_efficiency {
    #[path = "💠️change-fan-total-efficiency/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💠️change-fan-total-efficiency/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "💠️change-fan-total-efficiency/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💠️change-fan-total-efficiency/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_mechanical_ventilation_schedule {
    #[path = "📆️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📆️change-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📆️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📆️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_mechanical_ventilation {
    #[path = "🚫️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚫️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚫️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚫️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_mechanical_ventilation_design_flow {
    #[path = "🚿️change-design-flow/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚿️change-design-flow/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚿️change-design-flow/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚿️change-design-flow/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_mechanical_ventilation_zone {
    #[path = "🧭️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧭️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧭️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧭️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use create_mechanical_ventilation::{create_mechanical_ventilation, CreateMechanicalVentilation};
pub use change_mechanical_ventilation_fan_delta_pressure::{change_mechanical_ventilation_fan_delta_pressure, ChangeMechanicalVentilationFanDeltaPressure};
pub use change_mechanical_ventilation_fan_total_efficiency::{change_mechanical_ventilation_fan_total_efficiency, ChangeMechanicalVentilationFanTotalEfficiency};
pub use change_mechanical_ventilation_schedule::{change_mechanical_ventilation_schedule, ChangeMechanicalVentilationSchedule};
pub use delete_mechanical_ventilation::{delete_mechanical_ventilation, DeleteMechanicalVentilation};
pub use change_mechanical_ventilation_design_flow::{change_mechanical_ventilation_design_flow, ChangeMechanicalVentilationDesignFlow};
pub use change_mechanical_ventilation_zone::{change_mechanical_ventilation_zone, ChangeMechanicalVentilationZone};
