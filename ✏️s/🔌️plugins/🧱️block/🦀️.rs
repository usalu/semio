//! 🧱️ Block plugin schema authority has no artifact dependencies.
#[path = "🧬️schema/🧱️shared/🦀️.rs"]
mod schema;
pub use schema::{BlockKindIdentity, BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation, BlockCamera2d, BlockCamera3d, BlockMeta};
#[path = "🧬️schema/🧱️shared/♻️retirement/🦀️.rs"]
mod retirement;
