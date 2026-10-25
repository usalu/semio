//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_block_2d_model::text_snapshot as snapshot;

pub use semio_s_artifact_block_2d_mutations::text_mutations as mutations;

#[path = "🔺️diff/🦀️.rs"]
pub mod diff;

#[path = "💡️inferences/🦀️.rs"]
pub mod inferences;
