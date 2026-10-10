//! 🚪️ Replication representation contracts independent of semantic mutation behavior.

#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
pub use binary::{DiffBinary, OpBinary};
pub use text::{DiffText, OpText};

/// 🧬️ Requires both native diff representations.
pub trait DiffCodec: DiffText + DiffBinary {}
impl<T: DiffText + DiffBinary> DiffCodec for T {}

#[path = "👥️presence/🔍️borrowed/🦀️.rs"]
pub mod presence;
