//! ⚠️ Energy model mutation domain `fault`: `create-fault`, `change-fault-severity`, `change-fault-target-equipment`, `change-fault-type`, `change-fault-start-schedule`, `delete-fault`.

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
pub mod create_fault {
    #[path = "⚠️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⚠️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⚠️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⚠️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fault_severity {
    #[path = "🌶️change-severity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌶️change-severity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌶️change-severity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌶️change-severity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fault_target_equipment {
    #[path = "🎣️change-target-equipment/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎣️change-target-equipment/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🎣️change-target-equipment/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎣️change-target-equipment/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fault_type {
    #[path = "🐛️change-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🐛️change-type/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🐛️change-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🐛️change-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fault_start_schedule {
    #[path = "🕓️change-start-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕓️change-start-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕓️change-start-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕓️change-start-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_fault {
    #[path = "🩹️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🩹️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🩹️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🩹️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use create_fault::{create_fault, CreateFault};
pub use change_fault_severity::{change_fault_severity, ChangeFaultSeverity};
pub use change_fault_target_equipment::{change_fault_target_equipment, ChangeFaultTargetEquipment};
pub use change_fault_type::{change_fault_type, ChangeFaultType};
pub use change_fault_start_schedule::{change_fault_start_schedule, ChangeFaultStartSchedule};
pub use delete_fault::{delete_fault, DeleteFault};
