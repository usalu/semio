//! ⏯️ Energy model mutation domain `run`: `change-run-end-day`, `change-run-start-day`, `change-run-year`, `change-run-start-month`, `change-run-end-month`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod change_run_end_day {
    #[path = "⏹️change-end-day/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⏹️change-end-day/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⏹️change-end-day/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_run_start_day {
    #[path = "▶️change-start-day/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "▶️change-start-day/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "▶️change-start-day/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_run_year {
    #[path = "📅️change-year/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📅️change-year/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📅️change-year/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_run_start_month {
    #[path = "🛫️change-start-month/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛫️change-start-month/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛫️change-start-month/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_run_end_month {
    #[path = "🛬️change-end-month/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛬️change-end-month/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛬️change-end-month/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_run_end_day::{change_run_end_day, ChangeRunEndDay};
pub use change_run_start_day::{change_run_start_day, ChangeRunStartDay};
pub use change_run_year::{change_run_year, ChangeRunYear};
pub use change_run_start_month::{change_run_start_month, ChangeRunStartMonth};
pub use change_run_end_month::{change_run_end_month, ChangeRunEndMonth};
