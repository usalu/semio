//! 🚫️ Fixture-only refusal artifact authority.
pub const KIND:&str="fixture.neutral-host-fixture.snapshot-refusal";
#[path="🧬️schema/📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::Snapshot;
#[path="🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::Diff;
#[path="🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::Mutation;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
