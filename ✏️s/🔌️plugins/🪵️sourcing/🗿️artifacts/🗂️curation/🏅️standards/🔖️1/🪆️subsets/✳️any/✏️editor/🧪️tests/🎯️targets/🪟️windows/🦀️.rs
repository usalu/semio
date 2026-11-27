//! 🪟️ The window laws of the sourcing curation editor: the curated, grid, pool and preview windows render and edit their bodies, mounted through the shared harness.
#![allow(unused_imports)]
extern crate semio_os_kernel as pack;
pub use semio_s_artifact_sourcing_curation::*;
pub mod editor_unit_tests {
    pub(crate) mod context {
        pub use semio_s_artifact_sourcing_curation_harness_actions::*;
        pub use semio_s_artifact_sourcing_curation_harness_dispatch::*;
        pub use semio_s_artifact_sourcing_curation_harness_mount::*;
        pub use semio_s_artifact_sourcing_curation_harness_windows::*;
    }
}
#[path = "../../../🎭️modes/✏️edit/🪟️windows/🧺️curated/🧪️tests/🔬️unit/🦀️.rs"]
mod curated_tests;
#[path = "../../../🎭️modes/✏️edit/🪟️windows/🔢️grid/🧪️tests/🔬️unit/🦀️.rs"]
mod grid_tests;
#[path = "../../../🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs"]
mod pool_tests;
#[path = "../../../🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod preview_tests;
