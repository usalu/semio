//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_block_3d_model::binary_snapshot as snapshot;

pub use semio_s_artifact_block_3d_mutations::binary_mutations as mutations;

#[path = "🔺️diff/🦀️.rs"]
pub mod diff;

#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
