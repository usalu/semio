//! 🧪️ The `site` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_site as site_mutations;
extern crate semio_s_artifact_energy_model_mut_site_ground as site_ground_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_ground_deep {
    pub use crate::site_ground_mutations::change_ground_deep::*;
    #[path = "../../📍️site/⛏️change-ground-deep/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/⛏️change-ground-deep/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_site_elevation {
    pub use crate::site_mutations::change_site_elevation::*;
    #[path = "../../📍️site/⛰️change-elevation/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/⛰️change-elevation/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_site_latitude {
    pub use crate::site_mutations::change_site_latitude::*;
    #[path = "../../📍️site/🌍️change-latitude/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/🌍️change-latitude/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_site_longitude {
    pub use crate::site_mutations::change_site_longitude::*;
    #[path = "../../📍️site/🌏️change-longitude/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/🌏️change-longitude/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_ground_building {
    pub use crate::site_ground_mutations::change_ground_building::*;
    #[path = "../../📍️site/🌡️change-ground-building/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/🌡️change-ground-building/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_ground_shallow {
    pub use crate::site_ground_mutations::change_ground_shallow::*;
    #[path = "../../📍️site/🌱️change-ground-shallow/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/🌱️change-ground-shallow/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}

#[path = "."]
mod change_site_north_axis {
    pub use crate::site_mutations::change_site_north_axis::*;
    #[path = "../../📍️site/🔝️change-north-axis/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
    #[path = "../../📍️site/🔝️change-north-axis/🧪️tests/🟰️same/🦀️.rs"]
    mod tests_same;
}

#[path = "."]
mod change_site_time_zone {
    pub use crate::site_mutations::change_site_time_zone::*;
    #[path = "../../📍️site/🕰️change-time-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📍️site/🕰️change-time-zone/🧪️tests/✅️sets/🦀️.rs"]
    mod tests_sets;
}
