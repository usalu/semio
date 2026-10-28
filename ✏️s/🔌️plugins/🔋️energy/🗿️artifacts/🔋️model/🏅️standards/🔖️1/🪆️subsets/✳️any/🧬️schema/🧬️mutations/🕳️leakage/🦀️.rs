//! 🕳️ Energy model mutation domain `leakage`: `change-infiltration-constant-term-coefficient`, `change-infiltration-temperature-term-coefficient`, `change-infiltration-velocity-term-coefficient`, `change-infiltration-velocity-squared-term-coefficient`, `change-infiltration-stack-height`, `change-infiltration-effective-leakage-area`, `change-infiltration-discharge-coefficient`.

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
pub mod change_infiltration_constant_term_coefficient {
    #[path = "🅰️change-constant-term-coefficient/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🅰️change-constant-term-coefficient/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🅰️change-constant-term-coefficient/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🅰️change-constant-term-coefficient/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_temperature_term_coefficient {
    #[path = "🅱️change-temperature-term-coefficient/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🅱️change-temperature-term-coefficient/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🅱️change-temperature-term-coefficient/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🅱️change-temperature-term-coefficient/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_velocity_term_coefficient {
    #[path = "🆎️change-velocity-term-coefficient/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🆎️change-velocity-term-coefficient/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🆎️change-velocity-term-coefficient/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🆎️change-velocity-term-coefficient/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_velocity_squared_term_coefficient {
    #[path = "🆑️change-velocity-squared-term-coefficient/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🆑️change-velocity-squared-term-coefficient/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🆑️change-velocity-squared-term-coefficient/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🆑️change-velocity-squared-term-coefficient/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_stack_height {
    #[path = "🏭️change-stack-height/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏭️change-stack-height/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏭️change-stack-height/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏭️change-stack-height/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_effective_leakage_area {
    #[path = "🕳️change-effective-leakage-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕳️change-effective-leakage-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕳️change-effective-leakage-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕳️change-effective-leakage-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_discharge_coefficient {
    #[path = "🚰️change-discharge-coefficient/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚰️change-discharge-coefficient/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚰️change-discharge-coefficient/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚰️change-discharge-coefficient/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_infiltration_constant_term_coefficient::{change_infiltration_constant_term_coefficient, ChangeInfiltrationConstantTermCoefficient};
pub use change_infiltration_temperature_term_coefficient::{change_infiltration_temperature_term_coefficient, ChangeInfiltrationTemperatureTermCoefficient};
pub use change_infiltration_velocity_term_coefficient::{change_infiltration_velocity_term_coefficient, ChangeInfiltrationVelocityTermCoefficient};
pub use change_infiltration_velocity_squared_term_coefficient::{change_infiltration_velocity_squared_term_coefficient, ChangeInfiltrationVelocitySquaredTermCoefficient};
pub use change_infiltration_stack_height::{change_infiltration_stack_height, ChangeInfiltrationStackHeight};
pub use change_infiltration_effective_leakage_area::{change_infiltration_effective_leakage_area, ChangeInfiltrationEffectiveLeakageArea};
pub use change_infiltration_discharge_coefficient::{change_infiltration_discharge_coefficient, ChangeInfiltrationDischargeCoefficient};
