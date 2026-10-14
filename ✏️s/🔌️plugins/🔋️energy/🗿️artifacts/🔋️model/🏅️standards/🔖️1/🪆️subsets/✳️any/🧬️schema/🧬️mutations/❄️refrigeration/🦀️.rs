//! ❄️ Energy model mutation domain `refrigeration`: `create-refrigeration-system`, `change-refrigeration-system-design-load`, `change-refrigeration-system-defrost-schedule`, `change-refrigeration-system-case-count`, `delete-refrigeration-system`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod create_refrigeration_system {
    #[path = "❄️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "❄️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "❄️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_refrigeration_system_design_load {
    #[path = "🏋️change-design-load/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏋️change-design-load/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏋️change-design-load/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_refrigeration_system_defrost_schedule {
    #[path = "🕑️change-defrost-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕑️change-defrost-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕑️change-defrost-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_refrigeration_system_case_count {
    #[path = "🗄️change-case-count/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗄️change-case-count/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗄️change-case-count/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_refrigeration_system {
    #[path = "🫠️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🫠️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🫠️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use create_refrigeration_system::{create_refrigeration_system, CreateRefrigerationSystem};
pub use change_refrigeration_system_design_load::{change_refrigeration_system_design_load, ChangeRefrigerationSystemDesignLoad};
pub use change_refrigeration_system_defrost_schedule::{change_refrigeration_system_defrost_schedule, ChangeRefrigerationSystemDefrostSchedule};
pub use change_refrigeration_system_case_count::{change_refrigeration_system_case_count, ChangeRefrigerationSystemCaseCount};
pub use delete_refrigeration_system::{delete_refrigeration_system, DeleteRefrigerationSystem};
