//! 🕐️ Energy model mutation domain `fixed-schedule`: `delete-constant-schedule`, `create-constant-schedule`, `change-constant-schedule-value`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod delete_constant_schedule {
    #[path = "📍️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📍️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📍️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📍️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_constant_schedule {
    #[path = "🕜️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕜️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕜️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕜️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_constant_schedule_value {
    #[path = "🕝️change-value/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕝️change-value/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕝️change-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕝️change-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use delete_constant_schedule::{delete_constant_schedule, DeleteConstantSchedule};
pub use create_constant_schedule::{create_constant_schedule, CreateConstantSchedule};
pub use change_constant_schedule_value::{change_constant_schedule_value, ChangeConstantScheduleValue};
