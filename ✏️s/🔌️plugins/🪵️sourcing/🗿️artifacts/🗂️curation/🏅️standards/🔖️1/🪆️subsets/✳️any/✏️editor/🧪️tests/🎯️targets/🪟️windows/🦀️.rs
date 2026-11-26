//! 🪟️ The window laws of the sourcing curation editor: the curated, grid, pool and preview windows render and edit their bodies, mounted through the shared harness.
#![allow(unused_imports)]
pub use semio_s_artifact_sourcing_curation::*;
#[path = "../../🧰️harness/🦀️.rs"]
pub mod harness;
pub mod editor_unit_tests {
    pub(crate) use crate::harness as context;
}
#[path = "../../../🎭️modes/✏️edit/🪟️windows/🧺️curated/🧪️tests/🔬️unit/🦀️.rs"]
mod curated_tests;
#[path = "../../../🎭️modes/✏️edit/🪟️windows/🔢️grid/🧪️tests/🔬️unit/🦀️.rs"]
mod grid_tests;
#[path = "../../../🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs"]
mod pool_tests;
#[path = "../../../🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod preview_tests;
