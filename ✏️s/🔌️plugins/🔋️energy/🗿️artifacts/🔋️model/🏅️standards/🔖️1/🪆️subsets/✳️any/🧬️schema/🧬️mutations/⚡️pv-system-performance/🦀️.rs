//! ⚡️ Energy model mutation domain `pv-system-performance`: `change-pv-system-inverter-efficiency`, `change-pv-system-dc-capacity`, `change-pv-system-module-efficiency`, `change-pv-system-temperature-coefficient`, `change-pv-system-faiman-heat-loss-constant`, `change-pv-system-faiman-heat-loss-wind`.

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
pub mod change_pv_system_inverter_efficiency {
    #[path = "../☀️pv-system/♌️change-inverter-efficiency/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../☀️pv-system/♌️change-inverter-efficiency/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../☀️pv-system/♌️change-inverter-efficiency/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../☀️pv-system/♌️change-inverter-efficiency/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_pv_system_dc_capacity {
    #[path = "../☀️pv-system/⚛️change-dc-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../☀️pv-system/⚛️change-dc-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../☀️pv-system/⚛️change-dc-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../☀️pv-system/⚛️change-dc-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_pv_system_module_efficiency {
    #[path = "../☀️pv-system/🎖️change-module-efficiency/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../☀️pv-system/🎖️change-module-efficiency/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../☀️pv-system/🎖️change-module-efficiency/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../☀️pv-system/🎖️change-module-efficiency/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_pv_system_inverter_efficiency::{change_pv_system_inverter_efficiency, ChangePvSystemInverterEfficiency};
pub use change_pv_system_dc_capacity::{change_pv_system_dc_capacity, ChangePvSystemDcCapacity};
pub use change_pv_system_module_efficiency::{change_pv_system_module_efficiency, ChangePvSystemModuleEfficiency};

#[path = "."]
pub mod change_pv_system_temperature_coefficient {
    #[path = "../☀️pv-system/🌡️change-temperature/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../☀️pv-system/🌡️change-temperature/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../☀️pv-system/🌡️change-temperature/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../☀️pv-system/🌡️change-temperature/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_pv_system_temperature_coefficient::{change_pv_system_temperature_coefficient, ChangePvSystemTemperatureCoefficient};

#[path = "."]
pub mod change_pv_system_faiman_heat_loss_constant {
    #[path = "../☀️pv-system/🧊️change-heat-loss/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../☀️pv-system/🧊️change-heat-loss/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../☀️pv-system/🧊️change-heat-loss/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../☀️pv-system/🧊️change-heat-loss/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_pv_system_faiman_heat_loss_constant::{change_pv_system_faiman_heat_loss_constant, ChangePvSystemFaimanHeatLossConstant};

#[path = "."]
pub mod change_pv_system_faiman_heat_loss_wind {
    #[path = "../☀️pv-system/💨️change-wind-loss/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../☀️pv-system/💨️change-wind-loss/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../☀️pv-system/💨️change-wind-loss/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../☀️pv-system/💨️change-wind-loss/↩️inverse/🦀️.rs"]
    pub mod inverse;
}
pub use change_pv_system_faiman_heat_loss_wind::{change_pv_system_faiman_heat_loss_wind, ChangePvSystemFaimanHeatLossWind};
